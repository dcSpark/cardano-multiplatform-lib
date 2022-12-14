use super::*;
use cardano_multiplatform_lib_core::{
    serialization::{LenEncoding, StringEncoding},
};
use cbor_event::Sz;

#[derive(Clone, Debug, Default)]
pub struct AddressEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct AlonzoAuxDataEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<Sz>,
    pub orig_deser_order: Vec<usize>,
    pub key_0_encoding: LenEncoding,
    pub key_0_key_encodings: BTreeMap<u64, Option<cbor_event::Sz>>,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_encoding: LenEncoding,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
    pub key_2_encoding: LenEncoding,
    pub key_2_elem_encodings: Vec<StringEncoding>,
    pub key_2_key_encoding: Option<cbor_event::Sz>,
    pub key_3_encoding: LenEncoding,
    pub key_3_elem_encodings: Vec<StringEncoding>,
    pub key_3_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct AlonzoTxOutEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct AssetNameEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageTxOutEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
    pub key_2_key_encoding: Option<cbor_event::Sz>,
    pub key_3_tag_encoding: Option<cbor_event::Sz>,
    pub key_3_bytes_encoding: StringEncoding,
    pub key_3_encoding: StringEncoding,
    pub key_3_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BigIntEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub auxiliary_data_set_encoding: LenEncoding,
    pub auxiliary_data_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
    pub invalid_transactions_encoding: LenEncoding,
    pub invalid_transactions_elem_encodings: Vec<Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct ConstrPlutusDataEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<Sz>,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub plutus_datas_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct CostmdlsEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_0_encoding: LenEncoding,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_encoding: LenEncoding,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct DatumOption0Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct DatumOption1Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub data_tag_encoding: Option<cbor_event::Sz>,
    pub data_bytes_encoding: StringEncoding,
    pub data_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct DnsNameEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ExUnitPricesEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ExUnitsEncoding {
    pub len_encoding: LenEncoding,
    pub mem_encoding: Option<cbor_event::Sz>,
    pub steps_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct GenesisKeyDelegationEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct Hash28Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Hash32Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct HeaderEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct HeaderBodyEncoding {
    pub len_encoding: LenEncoding,
    pub block_number_encoding: Option<cbor_event::Sz>,
    pub slot_encoding: Option<cbor_event::Sz>,
    pub block_body_size_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct InvalidBeforeEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct InvalidHereafterEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct Ipv4Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Ipv6Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardEncoding {
    pub len_encoding: LenEncoding,
    pub index_1_encoding: LenEncoding,
    pub coin_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardsCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MultiHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct OperationalCertEncoding {
    pub len_encoding: LenEncoding,
    pub sequence_number_encoding: Option<cbor_event::Sz>,
    pub kes_period_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PoolMetadataEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PoolParamsEncoding {
    pub len_encoding: LenEncoding,
    pub pledge_encoding: Option<cbor_event::Sz>,
    pub cost_encoding: Option<cbor_event::Sz>,
    pub pool_owners_encoding: LenEncoding,
    pub relays_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PoolRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PoolRetirementEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub epoch_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PositiveIntervalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<Sz>,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolParamUpdateEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_0_encoding: Option<cbor_event::Sz>,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_encoding: Option<cbor_event::Sz>,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
    pub key_2_encoding: Option<cbor_event::Sz>,
    pub key_2_key_encoding: Option<cbor_event::Sz>,
    pub key_3_encoding: Option<cbor_event::Sz>,
    pub key_3_key_encoding: Option<cbor_event::Sz>,
    pub key_4_encoding: Option<cbor_event::Sz>,
    pub key_4_key_encoding: Option<cbor_event::Sz>,
    pub key_5_encoding: Option<cbor_event::Sz>,
    pub key_5_key_encoding: Option<cbor_event::Sz>,
    pub key_6_encoding: Option<cbor_event::Sz>,
    pub key_6_key_encoding: Option<cbor_event::Sz>,
    pub key_7_encoding: Option<cbor_event::Sz>,
    pub key_7_key_encoding: Option<cbor_event::Sz>,
    pub key_8_encoding: Option<cbor_event::Sz>,
    pub key_8_key_encoding: Option<cbor_event::Sz>,
    pub key_9_key_encoding: Option<cbor_event::Sz>,
    pub key_10_key_encoding: Option<cbor_event::Sz>,
    pub key_11_key_encoding: Option<cbor_event::Sz>,
    pub key_14_key_encoding: Option<cbor_event::Sz>,
    pub key_16_encoding: Option<cbor_event::Sz>,
    pub key_16_key_encoding: Option<cbor_event::Sz>,
    pub key_17_encoding: Option<cbor_event::Sz>,
    pub key_17_key_encoding: Option<cbor_event::Sz>,
    pub key_18_key_encoding: Option<cbor_event::Sz>,
    pub key_19_key_encoding: Option<cbor_event::Sz>,
    pub key_20_key_encoding: Option<cbor_event::Sz>,
    pub key_21_key_encoding: Option<cbor_event::Sz>,
    pub key_22_encoding: Option<cbor_event::Sz>,
    pub key_22_key_encoding: Option<cbor_event::Sz>,
    pub key_23_encoding: Option<cbor_event::Sz>,
    pub key_23_key_encoding: Option<cbor_event::Sz>,
    pub key_24_encoding: Option<cbor_event::Sz>,
    pub key_24_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolVersionEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolVersionStructEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct RationalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<Sz>,
    pub numerator_encoding: Option<cbor_event::Sz>,
    pub denominator_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RedeemerEncoding {
    pub len_encoding: LenEncoding,
    pub index_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RequiredSignersEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct RewardAccountEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct Script0Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct Script1Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub plutus_v1_script_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Script2Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub plutus_v2_script_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptAllEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptAnyEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptNOfKEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub n_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptPubkeyEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyMaAuxDataEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_metadata_encoding: LenEncoding,
    pub transaction_metadata_key_encodings: BTreeMap<u64, Option<cbor_event::Sz>>,
    pub auxiliary_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyTxOutEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SingleHostAddrEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub port_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct SingleHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub port_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeCredentialEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeDelegationEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeDeregistrationEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionBodyEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_0_encoding: LenEncoding,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_encoding: LenEncoding,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
    pub key_2_encoding: Option<cbor_event::Sz>,
    pub key_2_key_encoding: Option<cbor_event::Sz>,
    pub key_3_encoding: Option<cbor_event::Sz>,
    pub key_3_key_encoding: Option<cbor_event::Sz>,
    pub key_4_encoding: LenEncoding,
    pub key_4_key_encoding: Option<cbor_event::Sz>,
    pub key_5_encoding: LenEncoding,
    pub key_5_value_encodings: BTreeMap<RewardAccount, Option<cbor_event::Sz>>,
    pub key_5_key_encoding: Option<cbor_event::Sz>,
    pub key_6_key_encoding: Option<cbor_event::Sz>,
    pub key_7_key_encoding: Option<cbor_event::Sz>,
    pub key_8_encoding: Option<cbor_event::Sz>,
    pub key_8_key_encoding: Option<cbor_event::Sz>,
    pub key_9_encoding: LenEncoding,
    pub key_9_value_encodings: BTreeMap<PolicyId, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
    pub key_9_key_encoding: Option<cbor_event::Sz>,
    pub key_11_key_encoding: Option<cbor_event::Sz>,
    pub key_13_encoding: LenEncoding,
    pub key_13_key_encoding: Option<cbor_event::Sz>,
    pub key_14_key_encoding: Option<cbor_event::Sz>,
    pub key_15_key_encoding: Option<cbor_event::Sz>,
    pub key_16_key_encoding: Option<cbor_event::Sz>,
    pub key_17_encoding: Option<cbor_event::Sz>,
    pub key_17_key_encoding: Option<cbor_event::Sz>,
    pub key_18_encoding: LenEncoding,
    pub key_18_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionInputEncoding {
    pub len_encoding: LenEncoding,
    pub index_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionWitnessSetEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_0_encoding: LenEncoding,
    pub key_0_key_encoding: Option<cbor_event::Sz>,
    pub key_1_encoding: LenEncoding,
    pub key_1_key_encoding: Option<cbor_event::Sz>,
    pub key_2_encoding: LenEncoding,
    pub key_2_key_encoding: Option<cbor_event::Sz>,
    pub key_3_encoding: LenEncoding,
    pub key_3_elem_encodings: Vec<StringEncoding>,
    pub key_3_key_encoding: Option<cbor_event::Sz>,
    pub key_4_encoding: LenEncoding,
    pub key_4_key_encoding: Option<cbor_event::Sz>,
    pub key_5_encoding: LenEncoding,
    pub key_5_key_encoding: Option<cbor_event::Sz>,
    pub key_6_encoding: LenEncoding,
    pub key_6_elem_encodings: Vec<StringEncoding>,
    pub key_6_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UnitIntervalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<Sz>,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub index_1_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateEncoding {
    pub len_encoding: LenEncoding,
    pub proposed_protocol_parameter_updates_encoding: LenEncoding,
    pub epoch_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UrlEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ValueEncoding {
    pub len_encoding: LenEncoding,
    pub coin_encoding: Option<cbor_event::Sz>,
    pub multiasset_encoding: LenEncoding,
    pub multiasset_value_encodings: BTreeMap<PolicyId, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
}


#[derive(Clone, Debug, Default)]
pub struct BootstrapWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub chain_code_encoding: StringEncoding,
    pub attributes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KesSignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KesVkeyEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Nonce1Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SignkeyKESEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeyEncoding {
    pub pubkey_bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeywitnessEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VrfCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: StringEncoding,
    pub bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VrfVkeyEncoding {
    pub inner_encoding: StringEncoding,
}