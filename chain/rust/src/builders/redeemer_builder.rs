use super::{
    certificate_builder::CertificateBuilderResult, input_builder::InputBuilderResult,
    mint_builder::MintBuilderResult, withdrawal_builder::WithdrawalBuilderResult,
};
use crate::{
    address::RewardAddress,
    plutus::{ExUnits, PlutusData, Redeemer, RedeemerTag},
    transaction::TransactionInput,
    PolicyId,
};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Clone, Copy, PartialOrd, Ord, Debug, PartialEq, Eq, Hash)]
pub struct RedeemerWitnessKey {
    tag: RedeemerTag,
    index: u64,
}

impl RedeemerWitnessKey {
    pub fn new(tag: RedeemerTag, index: u64) -> Self {
        Self { tag, index }
    }
}

impl From<&Redeemer> for RedeemerWitnessKey {
    fn from(redeemer: &Redeemer) -> Self {
        Self {
            tag: redeemer.tag,
            index: redeemer.index,
        }
    }
}

/// Redeemer without the tag of index
/// This allows builder code to return partial redeemers
/// and then later have them placed in the right context
#[derive(Clone, Debug)]
pub struct UntaggedRedeemer {
    pub data: PlutusData,
    pub ex_units: ExUnits,
}

impl UntaggedRedeemer {
    pub fn new(data: PlutusData, ex_units: ExUnits) -> Self {
        Self { data, ex_units }
    }
}

#[derive(Clone, Debug)]
enum UntaggedRedeemerPlaceholder {
    JustData(PlutusData),
    Full(UntaggedRedeemer),
}

impl UntaggedRedeemerPlaceholder {
    fn data(&self) -> &PlutusData {
        match self {
            Self::JustData(data) => data,
            Self::Full(untagged_redeemer) => &untagged_redeemer.data,
        }
    }
}

/// Possible errors during conversion from bytes
#[derive(Debug, thiserror::Error)]
pub enum MissingExunitError {
    #[error("Missing exunit for {0:?} with <key, index> values of <{1:?}, {2}>")]
    Key(RedeemerTag, usize, String),
}

#[derive(Debug, thiserror::Error)]
pub enum RedeemerBuilderError {
    #[error("Missing ExUnit: {0}")]
    MissingExUnit(#[from] MissingExunitError),
}

/// In order to calculate the index from the sorted set, "add_*" methods in this builder
/// must be called along with the "add_*" methods in transaction builder.
#[derive(Clone, Default, Debug)]
pub struct RedeemerSetBuilder {
    // the set of inputs is an ordered set (according to the order defined on the type TxIn) -
    // this also is the order in which the elements of the set are indexed (lex order on the pair of TxId and Ix).
    // All inputs of a transaction are included in the set being indexed (not just the ones that point to a Plutus script UTxO)
    spend: BTreeMap<TransactionInput, Option<UntaggedRedeemerPlaceholder>>,

    // the set of policy IDs is ordered according to the order defined on PolicyId (lex).
    // The index of a PolicyId in this set of policy IDs is computed according to this order.
    // Note that at the use site, the set of policy IDs passed to indexof is the (unfiltered)
    // domain of the Value map in the mint field of the transaction.
    mint: BTreeMap<PolicyId, Option<UntaggedRedeemerPlaceholder>>,

    // the index of a reward account ract in the reward withdrawals map is the index of ract as a key in the (unfiltered) map.
    // The keys of the Wdrl map are arranged in the order defined on the RewardAcnt type, which is a lexicographical (abbrv. lex)
    // order on the pair of the Network and the Credential.
    reward: BTreeMap<RewardAddress, Option<UntaggedRedeemerPlaceholder>>,

    // certificates in the DCert list are indexed in the order in which they arranged in the (full, unfiltered)
    // list of certificates inside the transaction
    cert: Vec<Option<UntaggedRedeemerPlaceholder>>,
}

impl RedeemerSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.spend.is_empty()
            && self.mint.is_empty()
            && self.reward.is_empty()
            && self.cert.is_empty()
    }

    /// note: will override existing value if called twice with the same key
    pub fn update_ex_units(&mut self, key: RedeemerWitnessKey, ex_units: ExUnits) {
        match key.tag {
            RedeemerTag::Spend => {
                let entry = self.spend.iter_mut().nth(key.index as usize).unwrap().1;
                *entry = Some(UntaggedRedeemerPlaceholder::Full(UntaggedRedeemer::new(
                    entry.as_ref().unwrap().data().clone(),
                    ex_units,
                )));
            }
            RedeemerTag::Mint => {
                let entry = self.mint.iter_mut().nth(key.index as usize).unwrap().1;
                *entry = Some(UntaggedRedeemerPlaceholder::Full(UntaggedRedeemer::new(
                    entry.as_ref().unwrap().data().clone(),
                    ex_units,
                )));
            }
            RedeemerTag::Cert => {
                let entry = self.cert.get_mut(key.index as usize).unwrap();
                *entry = Some(UntaggedRedeemerPlaceholder::Full(UntaggedRedeemer::new(
                    entry.as_ref().unwrap().data().clone(),
                    ex_units,
                )));
            }
            RedeemerTag::Reward => {
                let entry = self.reward.iter_mut().nth(key.index as usize).unwrap().1;
                *entry = Some(UntaggedRedeemerPlaceholder::Full(UntaggedRedeemer::new(
                    entry.as_ref().unwrap().data().clone(),
                    ex_units,
                )));
            }
        }
    }

    pub fn add_spend(&mut self, result: &InputBuilderResult) {
        let plutus_data = {
            result
                .aggregate_witness
                .as_ref()
                .and_then(|data| data.redeemer_plutus_data())
        };
        if let Some(data) = plutus_data {
            self.spend.insert(
                result.input.clone(),
                Some(UntaggedRedeemerPlaceholder::JustData(data.clone())),
            );
        } else {
            self.spend.insert(result.input.clone(), None);
        }
    }

    pub fn add_mint(&mut self, result: &MintBuilderResult) {
        let plutus_data = {
            result
                .aggregate_witness
                .as_ref()
                .and_then(|data| data.redeemer_plutus_data())
        };
        if let Some(data) = plutus_data {
            self.mint.insert(
                result.policy_id,
                Some(UntaggedRedeemerPlaceholder::JustData(data.clone())),
            );
        } else {
            self.mint.insert(result.policy_id, None);
        }
    }

    pub fn add_reward(&mut self, result: &WithdrawalBuilderResult) {
        let plutus_data = {
            result
                .aggregate_witness
                .as_ref()
                .and_then(|data| data.redeemer_plutus_data())
        };
        if let Some(data) = plutus_data {
            self.reward.insert(
                result.address.clone(),
                Some(UntaggedRedeemerPlaceholder::JustData(data.clone())),
            );
        } else {
            self.reward.insert(result.address.clone(), None);
        }
    }

    pub fn add_cert(&mut self, result: &CertificateBuilderResult) {
        let plutus_data = {
            result
                .aggregate_witness
                .as_ref()
                .and_then(|data| data.redeemer_plutus_data())
        };
        if let Some(data) = plutus_data {
            self.cert
                .push(Some(UntaggedRedeemerPlaceholder::JustData(data.clone())));
        } else {
            self.cert.push(None);
        }
    }

    pub fn build(
        &self,
        default_to_dummy_exunits: bool,
    ) -> Result<Vec<Redeemer>, RedeemerBuilderError> {
        let mut redeemers = Vec::new();
        // Calling iter on a BTreeMap returns a list of sorted keys
        self.remove_placeholders_and_tag(
            &mut redeemers,
            RedeemerTag::Spend,
            &mut self.spend.iter(),
            default_to_dummy_exunits,
        )?;
        self.remove_placeholders_and_tag(
            &mut redeemers,
            RedeemerTag::Mint,
            &mut self.mint.iter(),
            default_to_dummy_exunits,
        )?;
        self.remove_placeholders_and_tag(
            &mut redeemers,
            RedeemerTag::Reward,
            &mut self.reward.iter(),
            default_to_dummy_exunits,
        )?;
        self.remove_placeholders_and_tag(
            &mut redeemers,
            RedeemerTag::Cert,
            &mut self.cert.iter().map(|entry| (&(), entry)),
            default_to_dummy_exunits,
        )?;

        Ok(redeemers)
    }

    fn remove_placeholders_and_tag<'a, K: Debug + Clone>(
        &self,
        redeemers: &mut Vec<Redeemer>,
        tag: RedeemerTag,
        entries: &mut dyn Iterator<Item = (&'a K, &'a Option<UntaggedRedeemerPlaceholder>)>,
        default_to_dummy_exunits: bool,
    ) -> Result<(), RedeemerBuilderError> {
        let mut result = vec![];
        for (i, entry) in entries.enumerate() {
            let key = (tag, i, entry.0);

            let redeemer = match entry.1 {
                Some(UntaggedRedeemerPlaceholder::JustData(data)) => {
                    if !default_to_dummy_exunits {
                        Err(RedeemerBuilderError::MissingExUnit(
                            MissingExunitError::Key(key.0, key.1, format!("{:?}", key.2)),
                        ))
                    } else {
                        Ok(Some(UntaggedRedeemer::new(data.clone(), ExUnits::dummy())))
                    }
                }
                Some(UntaggedRedeemerPlaceholder::Full(untagged_redeemer)) => {
                    Ok(Some(untagged_redeemer.clone()))
                }
                None => Ok(None),
            }?;
            result.push(redeemer);
        }
        redeemers.append(&mut Self::tag_redeemer(tag, &result));
        Ok(())
    }

    fn tag_redeemer(
        tag: RedeemerTag,
        untagged_redeemers: &[Option<UntaggedRedeemer>],
    ) -> Vec<Redeemer> {
        let mut result = Vec::new();

        for (index, untagged_redeemer) in untagged_redeemers.iter().enumerate() {
            if let Some(untagged_redeemer) = untagged_redeemer {
                result.push(Redeemer::new(
                    tag,
                    index as u64,
                    untagged_redeemer.data.clone(),
                    untagged_redeemer.ex_units.clone(),
                ));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        address::Address,
        builders::witness_builder::{
            InputAggregateWitnessData, PartialPlutusWitness, PlutusScriptWitness,
            RequiredWitnessSet,
        },
        plutus::{PlutusScript, PlutusV1Script},
        transaction::AlonzoFormatTxOut,
        Value,
    };
    use cml_crypto::{PublicKey, RawBytesEncoding, TransactionHash};

    use super::*;

    fn fake_raw_key_public(id: u8) -> PublicKey {
        PublicKey::from_raw_bytes(&[
            id, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37,
            227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205,
        ])
        .unwrap()
    }

    #[test]
    fn test_redeemer_set_builder() {
        let mut builder = RedeemerSetBuilder::new();

        let data = {
            let witness = {
                let script = PlutusScript::PlutusV1(PlutusV1Script::new(vec![0]));
                PartialPlutusWitness {
                    script: PlutusScriptWitness::Script(script),
                    redeemer: PlutusData::new_big_int(0u64.into()),
                }
            };
            let missing_signers = vec![fake_raw_key_public(0).hash()];
            InputAggregateWitnessData::PlutusScript(witness, missing_signers, None)
        };

        let address = Address::from_bech32("addr1qxeqxcja25k8q05evyngf4f88xn89asl54x2zg3ephgj26ndyt5qk02xmmras5pe9jz2c7tc93wu4c96rqwvg6e2v50qlpmx70").unwrap();

        let input_result = InputBuilderResult {
            input: TransactionInput::new(TransactionHash::from([1; 32]), 1),
            utxo_info: AlonzoFormatTxOut::new(address.clone(), Value::zero()).into(),
            aggregate_witness: None,
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        let input_result = InputBuilderResult {
            input: TransactionInput::new(TransactionHash::from([1; 32]), 0),
            utxo_info: AlonzoFormatTxOut::new(address.clone(), Value::zero()).into(),
            aggregate_witness: None,
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        let input_result = InputBuilderResult {
            input: TransactionInput::new(TransactionHash::from([0; 32]), 0),
            utxo_info: AlonzoFormatTxOut::new(address, Value::zero()).into(),
            aggregate_witness: Some(data),
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        builder.update_ex_units(
            RedeemerWitnessKey::new(RedeemerTag::Spend, 0),
            ExUnits::new(10, 10),
        );

        let redeemers = builder.build(false).unwrap();

        assert_eq!(redeemers.len(), 1);

        let spend_redeemer = &redeemers[0];

        assert_eq!(spend_redeemer.tag, RedeemerTag::Spend);
        assert_eq!(spend_redeemer.index, 0);
    }
}
