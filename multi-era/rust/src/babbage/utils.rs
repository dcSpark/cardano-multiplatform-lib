use std::io::{BufRead, Seek, Write};

use cbor_event::{de::Deserializer, se::Serializer};
use cml_chain::{
    assets::{AssetName, Mint, NonZeroInt64},
    auxdata::{AuxiliaryData, ConwayFormatAuxData},
    plutus::Redeemers,
    transaction::TransactionWitnessSet,
    LenEncoding, PolicyId, Script, StringEncoding,
};

use super::{
    BabbageAuxiliaryData, BabbageScript, BabbageTransactionBody, BabbageTransactionWitnessSet,
};

use cml_core::{
    serialization::{fit_sz, Deserialize, Serialize},
    DeserializeError, DeserializeFailure,
};
use cml_crypto::{blake2b256, RawBytesEncoding, TransactionHash};

impl BabbageTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<BabbageScript> for Script {
    fn from(script: BabbageScript) -> Script {
        match script {
            BabbageScript::Native {
                script,
                len_encoding,
                tag_encoding,
            } => Script::Native {
                script,
                len_encoding,
                tag_encoding,
            },
            BabbageScript::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            } => Script::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            },
            BabbageScript::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            } => Script::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            },
        }
    }
}

impl From<BabbageAuxiliaryData> for AuxiliaryData {
    fn from(aux: BabbageAuxiliaryData) -> Self {
        match aux {
            BabbageAuxiliaryData::Shelley(md) => AuxiliaryData::new_shelley(md.clone()),
            BabbageAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_ma(md.clone()),
            BabbageAuxiliaryData::Babbage(md) => AuxiliaryData::new_conway({
                let mut conway = ConwayFormatAuxData::new();
                conway.metadata.clone_from(&md.metadata);
                conway.native_scripts.clone_from(&md.native_scripts);
                conway.plutus_v1_scripts.clone_from(&md.plutus_v1_scripts);
                conway.plutus_v2_scripts.clone_from(&md.plutus_v2_scripts);
                conway
            }),
        }
    }
}

impl From<BabbageTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: BabbageTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses.map(Into::into);
        new_wits.native_scripts = wits.native_scripts.map(Into::into);
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses.map(Into::into);
        new_wits.redeemers = wits.redeemers.map(|rs| {
            Redeemers::new_arr_legacy_redeemer(rs.into_iter().map(Into::into).collect::<Vec<_>>())
        });
        new_wits.plutus_datums = wits.plutus_datums.map(Into::into);
        new_wits.plutus_v1_scripts = wits.plutus_v1_scripts.map(Into::into);
        new_wits.plutus_v2_scripts = wits.plutus_v2_scripts.map(Into::into);
        new_wits
    }
}

/// Babbage mints can have multiple maps resulting in different encodings so this works around it
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageMint {
    pub assets: Vec<(PolicyId, Vec<(AssetName, NonZeroInt64)>)>,
    #[serde(skip)]
    pub encodings: Option<BabbageMintEncoding>,
}

impl BabbageMint {
    pub fn to_mint(&self) -> Mint {
        // the only on-chain values found here are well within i64's limits
        let mut mint = Mint::new();
        for (policy_id, assets) in self.assets.iter() {
            for (asset_name, coin) in assets {
                let new_coin = *coin + mint.get(policy_id, asset_name).unwrap_or(0);
                mint.set(*policy_id, asset_name.clone(), new_coin);
            }
        }
        mint
    }
}

impl From<Mint> for BabbageMint {
    fn from(mint: Mint) -> Self {
        let mut assets = Vec::new();
        for (policy_id, policy_assets) in mint.iter() {
            assets.push((
                *policy_id,
                policy_assets
                    .iter()
                    .map(|(asset_name, coin)| (asset_name.clone(), *coin))
                    .collect(),
            ));
        }
        Self {
            assets,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BabbageMintEncoding {
    pub len_encoding: LenEncoding,
    pub assets_encodings: Vec<(StringEncoding, LenEncoding, Vec<cbor_event::Sz>)>,
}

impl Serialize for BabbageMint {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(self.assets.len() as u64, force_canonical),
        )?;
        // all keys same length so this is okay (will all use 1-byte byte len for canonical)
        let mut key_order = self
            .assets
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        if force_canonical {
            key_order.sort_by(|i, j| {
                self.assets[*i]
                    .0
                    .to_raw_bytes()
                    .cmp(self.assets[*j].0.to_raw_bytes())
            });
        }
        for (i, (policy_id, assets)) in key_order.into_iter().zip(self.assets.iter()) {
            let i_encs = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.assets_encodings.get(i))
                .cloned();
            let policy_id_encoding = i_encs
                .as_ref()
                .map(|(e, _, _)| e.clone())
                .unwrap_or_default()
                .to_str_len_sz(policy_id.to_raw_bytes().len() as u64, force_canonical);
            serializer.write_bytes_sz(policy_id.to_raw_bytes(), policy_id_encoding)?;

            serializer.write_map_sz(
                i_encs
                    .as_ref()
                    .map(|(_, e, _)| *e)
                    .unwrap_or_default()
                    .to_len_sz(assets.len() as u64, force_canonical),
            )?;

            let mut inner_key_order = assets
                .iter()
                .enumerate()
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if force_canonical {
                inner_key_order
                    .sort_by(|i, j| assets[*i].0.to_raw_bytes().cmp(assets[*j].0.to_raw_bytes()));
            }

            for (j, (asset_name, coin)) in inner_key_order.into_iter().zip(assets.iter()) {
                let coin_encoding = i_encs
                    .as_ref()
                    .and_then(|(_, _, encs)| encs.get(j))
                    .cloned();
                asset_name.serialize(serializer, force_canonical)?;
                if *coin >= 0 {
                    serializer.write_unsigned_integer_sz(
                        *coin as u64,
                        fit_sz(*coin as u64, coin_encoding, force_canonical),
                    )?;
                } else {
                    serializer.write_negative_integer_sz(
                        *coin as i128,
                        fit_sz((*coin + 1).unsigned_abs(), coin_encoding, force_canonical),
                    )?;
                }
            }
            i_encs
                .as_ref()
                .map(|(_, e, _)| *e)
                .unwrap_or_default()
                .end(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        Ok(serializer)
    }
}

impl Deserialize for BabbageMint {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let outer_len = raw.map_sz()?;
        let mut assets = Vec::new();
        let mut encodings = Vec::new();
        while match outer_len {
            cbor_event::LenSz::Len(n, _) => (assets.len() as u64) < n,
            cbor_event::LenSz::Indefinite => true,
        } {
            if raw.cbor_type()? == cbor_event::Type::Special {
                assert_eq!(raw.special()?, cbor_event::Special::Break);
                break;
            }
            let (policy_id, policy_id_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    PolicyId::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })?;
            let inner_len = raw.map_sz()?;
            let mut policy_assets = Vec::new();
            let mut inner_encodings = Vec::new();
            while match inner_len {
                cbor_event::LenSz::Len(n, _) => (policy_assets.len() as u64) < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                let asset_name = AssetName::deserialize(raw)?;
                let (coin, coin_encoding) = match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => {
                        let (x, enc) = raw.unsigned_integer_sz()?;
                        (x as i64, enc)
                    }
                    _ => {
                        let (x, enc) = raw.negative_integer_sz()?;
                        (x as i64, enc)
                    }
                };
                policy_assets.push((asset_name, coin));
                inner_encodings.push(coin_encoding);
            }
            assets.push((policy_id, policy_assets));
            encodings.push((policy_id_encoding, inner_len.into(), inner_encodings));
        }
        Ok(Self {
            assets,
            encodings: Some(BabbageMintEncoding {
                len_encoding: outer_len.into(),
                assets_encodings: encodings,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use cml_chain::{Deserialize, Serialize};

    use crate::babbage::BabbageBlock;

    #[test]
    fn babbage_mint_duplicate() {
        // See: https://github.com/dcSpark/cardano-multiplatform-lib/issues/326
        let bytes = hex::decode("85828a1a0018ba521a02ae24895820413ca2266bac4f438bb66be88bc805ca5d1f4d2a8c3bdeb583c42df7474e14b958200339649fecc4224156494435a46d783d72c8b47fe54e411e6b8fdadda3a7585058203997f8cde4ebadc8993fb0e73c88a7058ec089a6311cc1f930388b9114fc3c99825840180f8b92442933ddb1d6e7a51c2e446fec230c72ff2924df1211cd698212534b488c2b1af6d12109fd2880aa43bbdd92cd0c12f08e50826abb80de13d44b075d5850da506b142cc8d33ae6d2aecfdb327622e9721e49417679f1f05f5229a0924577b80bb0b457fd99fbabd117e758bb49edf17a9a0a52b3a40b923e0faa369e687125787b70361442be3d7b91cebb53da0c19093c58200e3d4834a4dd7a556937b6c1b36a9d766cfff56b9ca28431fe4c7271008fa473845820d35f6405c9d73869b4a9ee42ac578af5d32faa1d79b4013bc620b97a61a8b7da0419014358405c6face300da458ca9453363480a961522daa402e852ba808f2ca9360ffe81e16bc4a8619e0872aeb2b6108b0809c6a0c7b3333162a0c33ffcbde8db6dad440f8208005901c0db84609ce526a38235eea695e2ed42aa2dafc739b23d395edc8e65c33553f987386182dc2a02d8a098e0e688ed9361c036704d16781489e6adca39989b436a035f17db62b490762330be236cf92e5a1bf20a9b671d062b4b38f2581498cd75fbfc4f24e21239468edbea69397a03ce91c1682c08975fc92a4873370722d6fe50876ffb1bf5c2b541cee23ac6e716788630c00d94335589dfd6161efaaa67e56dbe773acbe8c8c8e63f3ad4490342670aac252efed07b51fd082d80c78c4acf07bcc60377ae9ec07b705131841c99e5eb297355b9196d9b63aa1737dd9fdbf7a9282251a6f2e8e157aa141640fd347e18194788e13496abe4b7b2339bdad4ce5c12c25effc103e9ea2b91adf81ef9b6efd23dfe1a86f9b8d594ec536c277876365cd43984ac6ebb89df693036fc01b70cec8f3d64ca49577dc924ab08c71ef3869916e8e934b81b95c427a9d04733707887064620f05869a556d043658324654684d6e8a3a439b5e50a9cac1684ca857fe73cc12fb9ab5ac91a4b12345152344cb5edf6d226a581bb29fea7947ee7b5f1639b2a3bd5032ba2a15308c10d3d3fd75516f7b3f3ed4e24a1c9985c934984cbf867f1c3787accd5034ffec5b26f3abf83aa008282582055ba023aaccefe6888628a8513805ddad2a65f8278ca5c3397a80da6c7890b66018258208d9181649b3671475686851153eadbb8c0b90103d0c10c2ac6a9b1973d4abf49000d818258207aa9cd4999a21c3951de9e330e0ce1fba5943d79ef5bfd90628c2d41e982becd001281825820c59678b6892ba0fbeeaaec22d4cbde17026ff614ed47cea02c47752e5853ebc8010182a200581d600a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07011a018d55b0a2005839007ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048011a001e848010a200581d600a11b0c7e25dc5d9c63171bdf39d9741b901dc903e12b4e162348e07011a0054cc87111a000a6361021a0006eceb031a02b574ff081a02ae246b0b58204e15e4b830f4f5f029a4779f51735b5dd8462c23a85319166ada33c56ea2ab5aa600818258201d3ae3ffec2d9228c7d5bb9870f5cff032d0f31c83c22c4bd7c5e3ffead4470c0001818258390079467c69a9ac66280174d09d62575ba955748b21dec3b483a9469a65cc339a35f9e0fe039cf510c761d4dd29040c48e9657fdac7e9c01d94821b0000000253e00ccca1581cf923673e7bc027eba929a9086b7b04a126a18ef524b41db6fb014a26a24a414c414c415f54455354024c4741474147415f544553543202021a0002c20e09a2581cf923673e7bc027eba929a9086b7b04a126a18ef524b41db6fb014a26a14a414c414c415f5445535401581cf923673e7bc027eba929a9086b7b04a126a18ef524b41db6fb014a26a14c4741474147415f5445535432010b5820e4af6dc836dada8e84d698b26252f8586f4261ddc6090fc22eac4a059bfe6b040d8382582078869651d619a1832d90b7c843f6d674707fc4c1c2a24e205145656aba7533f600825820c111c9d57fb9ee06f54f0f8bc035f9762db24eada643509afbdbbae98b3ae8f800825820ceff84520384d2f4cdf2017bab6b86a7b46b81ef8e40c3d4f08b699a8a27bade00ab00838258201388fc6611c1aea7ddb4071ce03ee041c7ae2a3b809f730c99ea5d737cb9836b03825820a4d5f23d70af6e494237da29ebd3d0c13f8fde2c3c3b169980a4b63417753a7700825820fd12b648ce816a3aee8f0f0235303958b57bccf3929c88b1f99d6605f1c648ca020183a300581d70f5542fa9d9c61b4dbccd71654cc5a72a9fddc9d563f503682e772ec901821a001e8480a1581c5e4a2431a465a00dc5d8181aaff63959bb235d97013e7acb50b55bc4a1484e6f64654665656401028201d818583cd87a9fd8799f581c4ad1571e7df63d4d6c49240c8372eb639f57c0ef669338c0d752f29bd8799fd8799f1a0025ebd11b0000018bf6a73cbdffffffff82581d604ad1571e7df63d4d6c49240c8372eb639f57c0ef669338c0d752f29b1a0089544082581d604ad1571e7df63d4d6c49240c8372eb639f57c0ef669338c0d752f29b1a005ed7c4021a00044e78031a02ae24e3081a02ae246b0b58205f658fa6994cd9157a48835760a4421da6fec59400f666b343e114283f454eea0d81825820a4d5f23d70af6e494237da29ebd3d0c13f8fde2c3c3b169980a4b63417753a77020e81581c4ad1571e7df63d4d6c49240c8372eb639f57c0ef669338c0d752f29b1082581d604ad1571e7df63d4d6c49240c8372eb639f57c0ef669338c0d752f29b1a004df682111a00370c3f12818258204b61acedca6426ec989ef78a85ed97a8b4ffd47d9c40d4155e453d7e6b25ae580083a30081825820669ed15b1bc5e97ec45af8951e9cbcbd33a3b5878943704d054a1a3ec46be28258407861d1a46731116eb7b140c1a9efbd19e2727f472d4e3503ff96d3125fcc3e11982320322e99fafb61d7016443ac815d909e573bd721efbbeba0cd7a78a11f020481d8799fd8799f40ffd8799fa1d8799fd8799fd87980d8799fd8799f581c7ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cffd8799fd8799fd8799f581cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048ffffffffffd8799f4040ffff1a001e8480a0a000ffd87c9f9fd8799fd87a9fd8799f5545766572797468696e6720697320616c7269676874d8799fd87980d8799fd8799f581c7ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cffd8799fd8799fd8799f581cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048ffffffffffff9fd8799f0000ffffffd87980ffd8799fd87a9fd8799f4e5265706f72742070726f626c656dd8799fd87980d8799fd8799f581c7ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cffd8799fd8799fd8799f581cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048ffffffffffff9fd8799f0101ffffffd87980ffd8799fd87a9fd8799f5243686f696365206265747765656e20312d33d8799fd87980d8799fd8799f581c7ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cffd8799fd8799fd8799f581cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048ffffffffffff9fd8799f0103ffffffd87980ffd8799fd87a9fd8799f5243686f696365206265747765656e20312d34d8799fd87980d8799fd8799f581c7ea04a6b4942fc0db5f81a86847d2545c3810717ca95f546901cd08cffd8799fd8799fd8799f581cfffa1d082a6132fd1789d48fd348fae9e88d5521c20fb8e1045b0048ffffffffffff9fd8799f0104ffffffd87980ffff1b0000018bf65a9496d87980ffff058184000080821a002f80a61a339742d1a30081825820489ef28ea97f719ee7768645fc74b811c271e5d7ef06c2310854db30158e945d584023dcbaf18dff2ed33a436cbc9036c70deacb573d810df73c41c72c0ebc6183f3d5b44d24d9689db9d3254f3a6720f71175bc1ac32607758276c2cf64ca7dbb0403815251010000322253330034a229309b2b2b9a0105828401000182190bb91a000c79f88401000282190bb91a000c79f8a200818258204b5d869e21e1d1d6f84fdc1e5811eda837ca9fdf2068c69255cd43f62e2711f25840888846f463da49f6f4fbbe4fbfc056901521852d203f1b8bf1a54af2c4efd1cf292ef983a8dbfd3b9266194025028ea59ce34453ddc6a4e0861ddc5db6ee01050581840001d87980821a0012c2f41a15f4bff0a080").unwrap();
        let babbage_block = BabbageBlock::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, babbage_block.to_cbor_bytes());
    }

    #[test]
    fn babbage_tx_hash_mismatch_duplicate_mint() {
        let bytes = hex::decode("85828a1a0002b9921a00a322425820581063827053ccc7d81dbac8141d9c2c509516bafd5599d9c15daa9b2eb15ce65820f270c16659735ec4b65d15d1cbef937e50e608585ac3b71a4cd117fdb761624e5820ebb507d7e0b0399ffced1a35297203134f8a2a85b616c3591c452cbefa61b88a825840199fd347919a2befa7995d869744808df54d5f9608a1de89d25e0b2837386861ea57394e0c13d858ae9e1b3bc5ffb952399fb258d2d670748ff0533a9547f03858503721a2acd37fc0e328f3749b7b4b797d37deb9c778d33b3058ee2fb631531f137d3320449dfedb74259269a1c5da254efe3afaf4626e86c470684353b7a9b7381d978a5a7d66abca9adad56b79bd6d02190bf458203648f02334769911ab0d9422311675c125cd9bd3871746b17c88ef7b0c3afe13845820e947674a4b27747fbdb5512190d79643e013cbb2ffc9346b49bd65e093cadd7301183c5840926b676216ae9db730e2d2ae9fe0308152dc5468acb3d573d7ddd0fedda8ed760f0058110a41c3db2b526e2365062f6f78f8397b5d76d5dacf07d3304d3bf7058207005901c0c3f9dcb6e03112878798fc303808f7c3ee12b6351674514d0ef0343ce9ed5011712431fc59b1c86a637e40c13191b29bdb29ece2ca14d44d21a5556528af640fb0b308ded9d94eca36532dea9fcb00f7f64792216090422ce127021da8a6fd3577d545c8cd4b7e7dcf219bbcda733e709ca8acf7f45990076820cfa8777776708422e34683c07b4f0e25689d990cc40626a44b7f6eddbcda58423416461648efb319189d242ee580efc4c89ec46de3a715ccd57c68d5e59b54759cee2d5be64b862eee54f19f06fc0e5388a7e1abbf7716cb7ef3bf8d4fad009a77de8233ba187cc0a4712c28b90b80885649eda709180911d23b71e1d494b80f4d6bb1aa5d6511dfdc7de150eed972facf595801eba808adb730ec5111bd36175a29c08e8643e2da2fcbe41a97049abd814bfbb12c895f1e55f6c9b8c81b3e3e1e276fc74ae1d1b5b7e8c26d763835c10880b6996d64aeb443e64c13c8cc1518ba9befce9660334d061489404e282ef00967f097cc64f9ef802193cecb31c04b1779a2605a5132ee4c168c60f152bda320a4c82c348559cb3600321930f5c9cbcbcddf64582ada49fc04c5a4f4ccdf58a47d4a025270e45dedf923f87baee972b0e3549b75d082a600838258207fc36e487fc641ad98963b7ec45d94ef6729acb15d993c67b564c1f845eb837e008258207fc36e487fc641ad98963b7ec45d94ef6729acb15d993c67b564c1f845eb837e028258207fc36e487fc641ad98963b7ec45d94ef6729acb15d993c67b564c1f845eb837e030184825839005ecffdc54a493a3f11b149c922ddb6c0f2622efc5e1651ccc4d4bfaf0298b4b8579f55b767c32f8137067d7f8a83879802317b10ce954e911a140cacd882583900377c5ff6ad6ab6814e108a9f7c298d467ab071d94fe02a8d0ce1f1812cf859673b01e57891f55a55ca600c7428f47e9b94a94ccbbfdb8c771a00839b68825839005ecffdc54a493a3f11b149c922ddb6c0f2622efc5e1651ccc4d4bfaf0298b4b8579f55b767c32f8137067d7f8a83879802317b10ce954e91821a001a4ec4a1581c99027c7af372116f0716ee02008739ec51224bcfdd54398ec16217efa5484d6565726b61743401484d6565726b61743601494d6565726b6174313501494d6565726b6174323201494d6565726b6174333501825839005ecffdc54a493a3f11b149c922ddb6c0f2622efc5e1651ccc4d4bfaf0298b4b8579f55b767c32f8137067d7f8a83879802317b10ce954e91821b000000023eeaf5a2a1581c99027c7af372116f0716ee02008739ec51224bcfdd54398ec16217efa3484d6565726b61743201494d6565726b6174313801494d6565726b6174333201021a00044271031a00a32588075820e17f14914a4c11738d1a391ac645b8a8c077acf8ef3b06e7be52d3007058002f09a1581c99027c7af372116f0716ee02008739ec51224bcfdd54398ec16217efa5484d6565726b61743401484d6565726b61743601494d6565726b6174313501494d6565726b6174323201494d6565726b6174333501a40081825820ad0b622e745b250833b2d9b5ba097f1eda98af919df7f79bf06db0200530df16000182a200581d609e5614893238cf85e284c61ec56d5efd9f9cdc4863ba7e1bf00c2c7d011b0067d5c8983f93cca2005839009e5614893238cf85e284c61ec56d5efd9f9cdc4863ba7e1bf00c2c7dd12d53af25a20a84981b873d3117cc21324ba035366a466c330ff790011b000000e8d4a51000021a00029309048182008200581cd12d53af25a20a84981b873d3117cc21324ba035366a466c330ff79082a2008282582033846103a1478022104669a9653dcca4c8b93ceb1ddc1045027df1a871ddabca584059e5b2c5032d044d522f7b15d5bbc957ac6d28f2f26b13ff294366a31da52dde6a3ce42b3c57719b5f98ff9e57c776e8bd3285f02ecfc69d4d1eaf2f98d27a0d825820ddc1c2e930eda0c6df23fd811b9ff40059b88338582611b966a3bd156a49cfff58401cd7f2e92a465f21981b74b253ceed7a7d48147f49de21dd9cfb99caf320a0e84a14e0b18e21fd83909c68a41d558eb3ab396dcd7a3bdc87cc4efcb2759d920001818201828200581c2b354356304a1ccb513a8bc12b278e3c4f421c692c89cf7855a2739e82051a056557faa1008182582069a14b724409e0ceef671c76ec4f8bce7509b5919bb971b3855bf92ca56532225840c180d20a83b5879609fa35c2a423b4bd08642add58e5782102e79e40f833fcb96f5546c3bd776da14717aa41360bb748bffc868a9303c7672814e9a503ccbd07a10082a11902d1a178383939303237633761663337323131366630373136656530323030383733396563353132323462636664643534333938656331363231376566a5684d6565726b617434ad63486174764469616d6f6e642048616e647320536e61706261636b64457965736b4e6f726d616c20426c7565644d61736b644e6f6e6564536b696e66446573657274646e616d656a4d6565726b6174202334654d6f757468724d6f7573746163686520416e6420506970656566696c657381a3637372637835697066733a2f2f516d614c6d566176334577427567657968425a357363387533746b61434a6851647575436b534c35654559647a35646e616d656a4d6565726b6174202334696d656469615479706569696d6167652f706e6765696d6167657835697066733a2f2f516d614c6d566176334577427567657968425a357363387533746b61434a6851647575436b534c35654559647a3566546174746f6f644e6f6e6568436c6f7468696e676a41726d79205368697274696d656469615479706569696d6167652f706e676a4261636b67726f756e646659656c6c6f776d456172204163636573736f7279644e6f6e65684d6565726b617436ad63486174644e6f6e6564457965736b4e6f726d616c20426c7565644d61736b644e6f6e6564536b696e6753747269706564646e616d656a4d6565726b6174202336654d6f757468644772696e6566696c657381a3637372637835697066733a2f2f516d59736d53564d75766245636d473545387873385466445065546f374b5634715346704d5932504a6974487852646e616d656a4d6565726b6174202336696d656469615479706569696d6167652f706e6765696d6167657835697066733a2f2f516d59736d53564d75766245636d473545387873385466445065546f374b5634715346704d5932504a697448785266546174746f6f66447261676f6e68436c6f7468696e676c426c61636b2054757865646f696d656469615479706569696d6167652f706e676a4261636b67726f756e646450696e6b6d456172204163636573736f7279644e6f6e65694d6565726b61743135ad6348617468526963652048617464457965736b4e6f726d616c20426c7565644d61736b644e6f6e6564536b696e6443616d6f646e616d656b4d6565726b617420233135654d6f75746865536e616b656566696c657381a3637372637835697066733a2f2f516d59364c5065766d714d4d3373716334374364637341353241784b4e626f3247527274354a66584b39484d4c33646e616d656b4d6565726b617420233135696d656469615479706569696d6167652f706e6765696d6167657835697066733a2f2f516d59364c5065766d714d4d3373716334374364637341353241784b4e626f3247527274354a66584b39484d4c3366546174746f6f644e6f6e6568436c6f7468696e676e486177616969616e205368697274696d656469615479706569696d6167652f706e676a4261636b67726f756e64694772617665796172646d456172204163636573736f7279644e6f6e65694d6565726b61743232ad63486174644e6f6e6564457965736c4e6f726d616c2042726f776e644d61736b644e6f6e6564536b696e6753747269706564646e616d656b4d6565726b617420233232654d6f7574686853747261696768746566696c657381a3637372637835697066733a2f2f516d665334707a7653466a76687a537a38356b44326547435453525252514e704d736e4b63436f66636f77325a74646e616d656b4d6565726b617420233232696d656469615479706569696d6167652f706e6765696d6167657835697066733a2f2f516d665334707a7653466a76687a537a38356b44326547435453525252514e704d736e4b63436f66636f77325a7466546174746f6f644e6f6e6568436c6f7468696e6766507269657374696d656469615479706569696d6167652f706e676a4261636b67726f756e64644c696d656d456172204163636573736f7279644e6f6e65694d6565726b61743335ad63486174764469616d6f6e642048616e647320536e61706261636b64457965736c4e6f726d616c2042726f776e644d61736b644e6f6e6564536b696e6a476f6c6420416c706861646e616d656b4d6565726b617420233335654d6f757468644772696e6566696c657381a3637372637835697066733a2f2f516d534766323873446f62765178727443453673364b323152776f5143756f7969574c544774436e7a76374e4337646e616d656b4d6565726b617420233335696d656469615479706569696d6167652f706e6765696d6167657835697066733a2f2f516d534766323873446f62765178727443453673364b323152776f5143756f7969574c544774436e7a76374e433766546174746f6f644e6f6e6568436c6f7468696e67694e6176792053756974696d656469615479706569696d6167652f706e676a4261636b67726f756e64694772617665796172646d456172204163636573736f72796d476f6c642045617272696e67738080").unwrap();
        let babbage_block = BabbageBlock::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, babbage_block.to_cbor_bytes());
    }
}
