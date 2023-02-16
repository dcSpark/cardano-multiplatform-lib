use std::convert::TryFrom;

pub use cml_core::{
    error::*,
    metadata::{Metadata, TransactionMetadatum},
    serialization::*,
};

use crate::{CIP25Metadata, ChunkableString, LabelMetadata, String64};

pub static CIP25_METADATA_LABEL: u64 = 721;

impl CIP25Metadata {
    /// Create a Metadata containing only the CIP25 schema
    pub fn to_metadata(&self) -> Result<Metadata, DeserializeError> {
        use std::convert::TryInto;
        self.try_into()
    }

    /// Read the CIP25 schema from a Metadata. Ignores all other data besides CIP25
    /// Can fail if the Metadata does not conform to CIP25
    pub fn from_metadata(metadata: &Metadata) -> Result<CIP25Metadata, DeserializeError> {
        Self::try_from(metadata)
    }

    /// Add to an existing metadata (could be empty) the full CIP25 metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        let cip25_metadatum = TransactionMetadatum::from_cbor_bytes(&self.key_721.to_bytes())?;
        metadata.insert(CIP25_METADATA_LABEL, cip25_metadatum);
        Ok(())
    }
}

impl std::convert::TryFrom<&Metadata> for CIP25Metadata {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        let cip25_metadatum = metadata.get(&CIP25_METADATA_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(CIP25_METADATA_LABEL))
        })?;
        Ok(Self {
            key_721: LabelMetadata::from_cbor_bytes(&cip25_metadatum.to_original_cbor_bytes())?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &CIP25Metadata {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}

impl String64 {
    pub fn new_str(inner: &str) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "String64",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self(inner.to_owned()))
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for String64 {
    type Error = DeserializeError;

    fn try_from(inner: &str) -> Result<Self, Self::Error> {
        String64::new_str(inner)
    }
}

impl From<&str> for ChunkableString {
    fn from(s: &str) -> Self {
        String64::new_str(s)
            .map(Self::Single)
            .unwrap_or_else(|_err| {
                let mut chunks = Vec::with_capacity(s.len() / 64);
                for i in (0..s.len()).step_by(64) {
                    let j = std::cmp::min(s.len(), i + 64);
                    chunks.push(String64::new_str(&s[i..j]).unwrap());
                }
                Self::Chunked(chunks)
            })
    }
}

impl From<&ChunkableString> for String {
    fn from(chunkable: &ChunkableString) -> Self {
        match chunkable {
            ChunkableString::Single(chunk) => chunk.to_str().to_owned(),
            ChunkableString::Chunked(chunks) => chunks
                .iter()
                .map(|chunk| chunk.to_str().to_owned())
                .collect(),
        }
    }
}


/// A subset of MetadataDetails where the keys are optional
/// Useful to extract the key fields (name & image) of incorrectly formatted cip25
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MiniMetadataDetails {
    pub name: Option<String64>,
    pub image: Option<ChunkableString>,
}

impl MiniMetadataDetails {
    pub fn new(name: Option<String64>, image: Option<ChunkableString>) -> Self {
        Self {
            name,
            image,
        }
    }

    /// loose parsing of CIP25 metadata to allow for common exceptions to the format
    /// TODO: this is not an ideal solution
    ///       ideally: we would have a function that takes in a policy ID
    ///       and would have a lookup map to know which lambda to call to get the name & image depending on the policy ID
    ///       with a fallback to the standard CIP25 definition
    ///       however, since this is a lot of work, we use this temporary solution instead
    pub fn loose_parse(data: Vec<u8>) -> Result<Self, DeserializeError> {
        let metadatum = TransactionMetadatum::from_bytes(
            data,
        )?;
        match metadatum {
            TransactionMetadatum::Map { entries, .. } => {
                let name: Option<String64> = entries
                .get(&TransactionMetadatum::new_text("name".to_owned()))
                .or_else(|| entries.get(&TransactionMetadatum::new_text("name".to_owned())))
                // for some reason, 1% of NFTs seem to use the wrong case
                .or_else(|| entries.get(&TransactionMetadatum::new_text("Name".to_owned())))
                // for some reason, 0.5% of NFTs use "title" instead of name
                .or_else(|| entries.get(&TransactionMetadatum::new_text("title".to_owned())))
                // for some reason, 0.3% of NFTs use "id" instead of name
                .or_else(|| entries.get(&TransactionMetadatum::new_text("id".to_owned())))
                .map(|result| match result {
                    TransactionMetadatum::Text { text, .. } => String64::new_str(&text).ok(),
                    _ => None,
                })
                .flatten();
    
                let image_base = entries
                .get(&TransactionMetadatum::new_text("image".to_owned()));
                let image = match image_base {
                    None => None,
                    Some(base) => match base {
                        TransactionMetadatum::Text { text, .. } => match String64::new_str(&text) {
                            Ok(str64) => Some(ChunkableString::Single(str64)),
                            Err(_) => None,
                        },
                        TransactionMetadatum::List { elements, .. } => {
                            (|| {
                                let mut chunks: Vec<String64> = vec![];
                                for i in 0..elements.len() {
                                    match elements.get(i) {
                                        Some(TransactionMetadatum::Text { text, .. }) => {
                                            match String64::new_str(&text) {
                                                Ok(str64) => chunks.push(str64),
                                                Err(_) => return None,
                                            }
                                        },
                                        _ => return None
                                    };
                                };
                                Some(ChunkableString::Chunked(chunks))
                            })()
                        },
                        _ => None,
                    },
                };
    
                Ok(MiniMetadataDetails::new(name, image))
            },
            _ => Err(DeserializeError::new("MiniMetadataDetails", DeserializeFailure::NoVariantMatched))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{AssetNameV2, Data, FilesDetails, LabelMetadataV2, MetadataDetails, PolicyIdV2};

    use super::*;

    #[test]
    fn create() {
        let mut details = MetadataDetails::new(
            String64::try_from("Metadata Name").unwrap(),
            ChunkableString::from("htts://some.website.com/image.png"),
        );
        details.description = Some(ChunkableString::from("description of this NFT"));
        details.media_type = Some(String64::try_from("image/*").unwrap());
        details.files = Some(vec![
            FilesDetails::new(
                String64::new_str("filename1").unwrap(),
                String64::new_str("filetype1").unwrap(),
                ChunkableString::from("src1"),
            ),
            FilesDetails::new(
                String64::new_str("filename2").unwrap(),
                String64::new_str("filetype2").unwrap(),
                ChunkableString::from("src2"),
            ),
        ]);
        let mut v2 = Data::new();
        let mut v2_inner = BTreeMap::new();
        v2_inner.insert(AssetNameV2::from(vec![0xCA, 0xFE, 0xD0, 0x0D]), details);
        v2.insert(PolicyIdV2::from(vec![0xBA, 0xAD, 0xF0, 0x0D]), v2_inner);
        let metadata = CIP25Metadata::new(LabelMetadata::new_label_metadata_v2(
            LabelMetadataV2::new(v2),
        ));
        let metadata_bytes = metadata.to_bytes();
        let roundtrip = CIP25Metadata::from_cbor_bytes(&metadata_bytes).unwrap();
        assert_eq!(metadata_bytes, roundtrip.to_bytes());
        let as_metadata = metadata.to_metadata().unwrap();
        let from_metadata = CIP25Metadata::from_metadata(&as_metadata).unwrap();
        assert_eq!(metadata_bytes, from_metadata.to_bytes());
    }

    #[test]
    fn parse_metadata_details() {
        {
            // {
            //  "arweaveId": "6srpXZOTfK_62KUrJKh4VdCFG0YS271pq20OMRpE5Ts",
            //  "image": "ipfs://QmUWP6xGHucgBUv514gwgbt4yijg36aUQunEP61z5D8RKS",
            //  "name": "SpaceBud #1507",
            //  "traits": ["Star Suit", "Chestplate", "Belt", "Flag", "Pistol"],
            //  "type": "Alien",
            // }
            let bytes = "a569617277656176654964782b36737270585a4f54664b5f36324b55724a4b68345664434647305953323731707132304f4d52704535547365696d6167657835697066733a2f2f516d5557503678474875636742557635313467776762743479696a673336615551756e455036317a354438524b53646e616d656e53706163654275642023313530376674726169747385695374617220537569746a4368657374706c6174656442656c7464466c616766506973746f6c647479706565416c69656e";
            MetadataDetails::from_bytes(hex::decode(bytes).unwrap()).unwrap();
        }
        {
            // {
            //     "color": "#EC97B6",
            //     "image": "ipfs://ipfs/QmUvbF2siHFGGRtZ5za1VwNQ8y49bbtjmYfFYhgE89hCq2",
            //     "name": "Berry Alba",
            // }
            let bytes = "a365636f6c6f72672345433937423665696d616765783a697066733a2f2f697066732f516d557662463273694846474752745a357a613156774e51387934396262746a6d59664659686745383968437132646e616d656a426572727920416c6261";
            MetadataDetails::from_bytes(hex::decode(bytes).unwrap()).unwrap();
        }
    }

    #[test]
    fn just_name() {
        // {"name":"Metaverse"}
        let details = MiniMetadataDetails::loose_parse(hex::decode("a1646e616d65694d6574617665727365").unwrap()).unwrap();
        assert_eq!(details.name.unwrap().0, "Metaverse");
    }

    #[test]
    fn uppercase_name() {
        // {"Date":"9 May 2021","Description":"Happy Mother's Day to all the Cardano Moms!","Image":"ipfs.io/ipfs/Qmah6QPKUKvp6K9XQB2SA42Q3yrffCbYBbk8EoRrB7FN2g","Name":"Mother's Day 2021","Ticker":"MOM21","URL":"ipfs.io/ipfs/Qmah6QPKUKvp6K9XQB2SA42Q3yrffCbYBbk8EoRrB7FN2g"}
        let details = MiniMetadataDetails::loose_parse(hex::decode("a664446174656a39204d617920323032316b4465736372697074696f6e782b4861707079204d6f7468657227732044617920746f20616c6c207468652043617264616e6f204d6f6d732165496d616765783b697066732e696f2f697066732f516d61683651504b554b7670364b39585142325341343251337972666643625942626b38456f52724237464e3267644e616d65714d6f746865722773204461792032303231665469636b6572654d4f4d32316355524c783b697066732e696f2f697066732f516d61683651504b554b7670364b39585142325341343251337972666643625942626b38456f52724237464e3267").unwrap()).unwrap();
        assert_eq!(details.name.unwrap().0, "Mother's Day 2021");
    }

    #[test]
    fn id_no_name() {
        // {"id":"00","image":"ipfs://QmSfYTF8B4ua6hFdr6URdRDZBZ9FjCQNUdDcLr2f7P8xn3"}
        let details = MiniMetadataDetails::loose_parse(hex::decode("a262696462303065696d6167657835697066733a2f2f516d5366595446384234756136684664723655526452445a425a39466a43514e556444634c723266375038786e33").unwrap()).unwrap();
        assert_eq!(details.name.unwrap().0, "00");
    }

    #[test]
    fn just_image() {
        // {"image":"ipfs://QmSfYTF8B4ua6hFdr6URdRDZBZ9FjCQNUdDcLr2f7P8xn3"}
        let details = MiniMetadataDetails::loose_parse(hex::decode("a165696d6167657835697066733a2f2f516d5366595446384234756136684664723655526452445a425a39466a43514e556444634c723266375038786e33").unwrap()).unwrap();
        assert_eq!(String::from(&details.image.unwrap()), "ipfs://QmSfYTF8B4ua6hFdr6URdRDZBZ9FjCQNUdDcLr2f7P8xn3");
    }

    #[test]
    fn noisy_metadata() {
        // generatd by adding this to the create() test case
        // as_metadata.insert(1337, TransactionMetadatum::new_list(vec![
        //     TransactionMetadatum::new_bytes(vec![0xBA, 0xAD, 0xF0, 0x0D]),
        // ]));
        // let label_metadatum_entries: &mut _ = match as_metadata.get_mut(&721).unwrap() {
        //     TransactionMetadatum::Map { entries, .. } => entries,
        //     _ => panic!(),
        // };
        // let mut filler_map = OrderedHashMap::new();
        // filler_map.insert(
        //     TransactionMetadatum::new_bytes(vec![]),
        //     TransactionMetadatum::new_int(cml_core::Int::new_nint(100))
        // );
        // label_metadatum_entries.insert(TransactionMetadatum::new_map(filler_map.clone()), TransactionMetadatum::new_map(filler_map.clone()));
        // let data_entries: &mut _ = match label_metadatum_entries.get_mut(&TransactionMetadatum::new_text("data".to_owned())).unwrap() {
        //     TransactionMetadatum::Map{ entries, .. } => entries,
        //     _ => panic!(),
        // };
        // data_entries.insert(TransactionMetadatum::new_map(filler_map.clone()), TransactionMetadatum::new_map(filler_map.clone()));
        // let policy_entries: &mut _ = match data_entries.get_mut(&TransactionMetadatum::new_bytes(vec![0xBA, 0xAD, 0xF0, 0x0D])).unwrap() {
        //     TransactionMetadatum::Map{ entries, .. } => entries,
        //     _ => panic!(),
        // };
        // policy_entries.insert(TransactionMetadatum::new_map(filler_map.clone()), TransactionMetadatum::new_map(filler_map.clone()));
        // policy_entries.insert(
        //     TransactionMetadatum::new_list(vec![TransactionMetadatum::new_map(filler_map.clone())]),
        //     TransactionMetadatum::new_list(vec![TransactionMetadatum::new_text("dskjfaks".to_owned())])
        // );
        // let details: &mut _ = match policy_entries.get_mut(&TransactionMetadatum::new_bytes(vec![0xCA, 0xFE, 0xD0, 0x0D])).unwrap() {
        //     TransactionMetadatum::Map { entries, .. } => entries,
        //     _ => panic!(),
        // };
        // details.insert(
        //     TransactionMetadatum::new_map(filler_map.clone()),
        //     TransactionMetadatum::new_int(cml_core::Int::new_uint(50))
        // );
        // let file_details: &mut _ = match details.get_mut(&TransactionMetadatum::new_text("files".to_owned())).unwrap() {
        //     TransactionMetadatum::List{ elements, .. } => match elements.get_mut(0).unwrap() {
        //         TransactionMetadatum::Map{ entries, .. } => entries,
        //         _ => panic!(),
        //     },
        //     _ => panic!(),
        // };
        // file_details.insert(
        //     TransactionMetadatum::new_list(vec![TransactionMetadatum::new_text("dskjfaks".to_owned())]),
        //     TransactionMetadatum::new_list(vec![TransactionMetadatum::new_map(filler_map.clone())])
        // );
        // let mut buf = cbor_event::se::Serializer::new_vec();
        // buf.write_map(cbor_event::Len::Indefinite).unwrap();
        // for (label, datum) in as_metadata.iter() {
        //     buf.write_unsigned_integer(*label).unwrap();
        //     datum.serialize(&mut buf, false).unwrap();
        // }
        // buf.write_special(cbor_event::Special::Break).unwrap();
        // panic!("{}", hex::encode(buf.finalize()));
        let bytes = "bf1902d1a36464617461a244baadf00da344cafed00da6646e616d656d4d65746164617461204e616d656566696c657382a4637372636473726331646e616d656966696c656e616d6531696d65646961547970656966696c657479706531816864736b6a66616b7381a1403864a3637372636473726332646e616d656966696c656e616d6532696d65646961547970656966696c65747970653265696d6167657821687474733a2f2f736f6d652e776562736974652e636f6d2f696d6167652e706e67696d656469615479706567696d6167652f2a6b6465736372697074696f6e776465736372697074696f6e206f662074686973204e4654a14038641832a1403864a140386481a1403864816864736b6a66616b73a1403864a14038646776657273696f6e02a1403864a14038641905398144baadf00dff";
        let _ = CIP25Metadata::from_bytes(hex::decode(bytes).unwrap()).unwrap();
    }
}
