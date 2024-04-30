use std::io::{BufRead, Seek, Write};

use cbor_event::{de::Deserializer, se::Serializer};
use cml_chain::{
    assets::{AssetName, Mint, NonZeroInt64},
    auxdata::{AuxiliaryData, ConwayFormatAuxData},
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
            BabbageAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_m_a(md.clone()),
            BabbageAuxiliaryData::Babbage(md) => AuxiliaryData::new_conway({
                let mut conway = ConwayFormatAuxData::new();
                conway.metadata = md.metadata.clone();
                conway.native_scripts = md.native_scripts.clone();
                conway.plutus_v1_scripts = md.plutus_v1_scripts.clone();
                conway.plutus_v2_scripts = md.plutus_v2_scripts.clone();
                conway
            }),
        }
    }
}

impl From<BabbageTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: BabbageTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses;
        new_wits.native_scripts = wits.native_scripts;
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses;
        new_wits.redeemers = wits.redeemers;
        new_wits.plutus_datums = wits.plutus_datums;
        new_wits.plutus_v1_scripts = wits.plutus_v1_scripts;
        new_wits.plutus_v2_scripts = wits.plutus_v2_scripts;
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

            let mut inner_key_order = self
                .assets
                .iter()
                .enumerate()
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if force_canonical {
                inner_key_order.sort_by(|i, j| assets[*i].0.get().cmp(assets[*j].0.get()));
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
        }
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
}
