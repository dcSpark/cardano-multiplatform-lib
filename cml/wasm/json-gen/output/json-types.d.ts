export type HDAddressPayloadJSON = number[];
export type ProtocolMagicJSON = number;
export type StakeDistributionJSON =
  | "BootstrapEra"
  | {
      SingleKey: string;
    };
export type AddressJSON = string;
export type ByronAddrTypeJSON = "PublicKey" | "Script" | "Redeem";
export type AnchorDocHashJSON = string;
export type AssetNameJSON = string;
export type CredentialJSON =
  | {
      PubKey: {
        hash: string;
        [k: string]: unknown;
      };
    }
  | {
      Script: {
        hash: string;
        [k: string]: unknown;
      };
    };
/**
 * This interface was referenced by `undefined`'s JSON-Schema definition
 * via the `patternProperty` "^\d+$".
 */
export type AuxiliaryDataJSON =
  | {
      Shelley: MetadataJSON;
    }
  | {
      ShelleyMA: ShelleyMAFormatAuxDataJSON;
    }
  | {
      Conway: ConwayFormatAuxDataJSON;
    };
export type NativeScriptJSON =
  | {
      ScriptPubkey: ScriptPubkeyJSON;
    }
  | {
      ScriptAll: ScriptAllJSON;
    }
  | {
      ScriptAny: ScriptAnyJSON;
    }
  | {
      ScriptNOfK: ScriptNOfKJSON;
    }
  | {
      ScriptInvalidBefore: ScriptInvalidBeforeJSON;
    }
  | {
      ScriptInvalidHereafter: ScriptInvalidHereafterJSON;
    };
export type AuxiliaryDataHashJSON = string;
export type BigIntegerJSON = string;
export type Bip32PublicKeyJSON = string;
/**
 * ED25519 key used as public key
 */
export type PublicKeyJSON = string;
export type CertificateJSON =
  | {
      StakeRegistration: StakeRegistrationJSON;
    }
  | {
      StakeDeregistration: StakeDeregistrationJSON;
    }
  | {
      StakeDelegation: StakeDelegationJSON;
    }
  | {
      PoolRegistration: PoolRegistrationJSON;
    }
  | {
      PoolRetirement: PoolRetirementJSON;
    }
  | {
      RegCert: RegCertJSON;
    }
  | {
      UnregCert: UnregCertJSON;
    }
  | {
      VoteDelegCert: VoteDelegCertJSON;
    }
  | {
      StakeVoteDelegCert: StakeVoteDelegCertJSON;
    }
  | {
      StakeRegDelegCert: StakeRegDelegCertJSON;
    }
  | {
      VoteRegDelegCert: VoteRegDelegCertJSON;
    }
  | {
      StakeVoteRegDelegCert: StakeVoteRegDelegCertJSON;
    }
  | {
      AuthCommitteeHotCert: AuthCommitteeHotCertJSON;
    }
  | {
      ResignCommitteeColdCert: ResignCommitteeColdCertJSON;
    }
  | {
      RegDrepCert: RegDrepCertJSON;
    }
  | {
      UnregDrepCert: UnregDrepCertJSON;
    }
  | {
      UpdateDrepCert: UpdateDrepCertJSON;
    };
export type RelayJSON =
  | {
      SingleHostAddr: SingleHostAddrJSON;
    }
  | {
      SingleHostName: SingleHostNameJSON;
    }
  | {
      MultiHostName: MultiHostNameJSON;
    };
export type DRepJSON =
  | {
      Key: {
        pool: string;
        [k: string]: unknown;
      };
    }
  | {
      Script: {
        script_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      AlwaysAbstain: {
        [k: string]: unknown;
      };
    }
  | {
      AlwaysNoConfidence: {
        [k: string]: unknown;
      };
    };
export type TransactionOutputJSON =
  | {
      AlonzoFormatTxOut: AlonzoFormatTxOutJSON;
    }
  | {
      ConwayFormatTxOut: ConwayFormatTxOutJSON;
    };
export type DatumOptionJSON =
  | {
      Hash: {
        datum_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      Datum: {
        datum: PlutusDataJSON;
        [k: string]: unknown;
      };
    };
export type PlutusDataJSON =
  | {
      map: {
        k: PlutusDataJSON;
        v: PlutusDataJSON;
      }[];
    }
  | {
      list: PlutusDataJSON[];
    }
  | {
      /**
       * A Plutus integer: unbounded by the ledger's definition, which is why this is the one integer in the document with no `format` and no bounds. Values outside ±2^53 cannot be held exactly by a JavaScript `number`; read the JSON text if you need them intact.
       */
      int: number;
    }
  | {
      bytes: string;
    }
  | ConstrPlutusDataJSON;
export type ScriptRefJSON =
  | {
      Native: {
        script: NativeScriptJSON;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV1: {
        script: string;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV2: {
        script: string;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV3: {
        script: string;
        [k: string]: unknown;
      };
    };
export type GovActionJSON =
  | {
      ParameterChangeAction: ParameterChangeActionJSON;
    }
  | {
      HardForkInitiationAction: HardForkInitiationActionJSON;
    }
  | {
      TreasuryWithdrawalsAction: TreasuryWithdrawalsActionJSON;
    }
  | {
      NoConfidence: NoConfidenceJSON;
    }
  | {
      UpdateCommittee: UpdateCommitteeJSON;
    }
  | {
      NewConstitution: NewConstitutionJSON;
    }
  | {
      InfoAction: {
        [k: string]: unknown;
      };
    };
export type VoteJSON = "No" | "Yes" | "Abstain";
export type RedeemersJSON =
  | {
      ArrLegacyRedeemer: {
        arr_legacy_redeemer: LegacyRedeemerJSON[];
        [k: string]: unknown;
      };
    }
  | {
      MapRedeemerKeyToRedeemerVal: {
        map_redeemer_key_to_redeemer_val: {
          [k: string]: RedeemerValJSON;
        };
        [k: string]: unknown;
      };
    };
export type RedeemerTagJSON = "Spend" | "Mint" | "Cert" | "Reward" | "Voting" | "Proposing";
export type BlockBodyHashJSON = string;
export type BlockHeaderHashJSON = string;
export type ByronAddressJSON = string;
/**
 * A String that may or may not be chunked into 64-byte chunks to be able
 * to conform to Cardano TX Metadata limitations.
 * Unless you have good reasons, you should be using the From<&str> trait to construct this:
 * ```
 * use cml_cip25::CIP25ChunkableString;
 * // automatically chunks this too long string into two chunks:
 * let chunkable_string = CIP25ChunkableString::from("this can be any length and will automatically be chunked into 64-byte pieces when/if needed");
 * match chunkable_string {
 *     CIP25ChunkableString::Single(_) => panic!(),
 *     CIP25ChunkableString::Chunked(chunks) => {
 *         assert_eq!(chunks[0].to_str(), "this can be any length and will automatically be chunked into 64");
 *         assert_eq!(chunks[1].to_str(), "-byte pieces when/if needed");
 *     },
 * }
 * ```
 */
export type CIP25ChunkableStringJSON =
  | {
      Single: string;
    }
  | {
      Chunked: string[];
    };
/**
 * Which version of the CIP25 spec to use. See CIP25 for details.
 * This will change how things are encoded but for the most part contains
 * the same information.
 */
export type CIP25VersionJSON = "V1" | "V2";
export type CIP25String64JSON = string;
export type CIP36DelegationDistributionJSON =
  | {
      Weighted: {
        weighted: CIP36DelegationJSON[];
        [k: string]: unknown;
      };
    }
  | {
      Legacy: {
        legacy: PublicKeyJSON;
        [k: string]: unknown;
      };
    };
export type TransactionMetadatumJSON =
  | {
      map: {
        k: TransactionMetadatumJSON;
        v: TransactionMetadatumJSON;
      }[];
    }
  | {
      list: TransactionMetadatumJSON[];
    }
  | {
      /**
       * A CBOR int: -2^64 ..= 2^64-1. No JSON Schema `format` covers that range, hence the explicit bounds. Values outside ±2^53 cannot be held exactly by a JavaScript `number`; read the JSON text if you need them intact.
       */
      int: number;
    }
  | {
      bytes: string;
    }
  | {
      string: string;
    };
/**
 * structure to compute the CRC32 of chunks of bytes.
 *
 * This structure allows implements the `Write` trait making it easier
 * to compute the crc32 of a stream.
 */
export type Crc32JSON = number;
export type DNSNameJSON = string;
export type DatumHashJSON = string;
export type Ed25519KeyHashJSON = string;
export type Ed25519SignatureJSON = string;
export type GenesisDelegateHashJSON = string;
export type GenesisHashJSON = string;
export type IntJSON = string;
export type Ipv4JSON = string;
export type Ipv6JSON = string;
export type KESSignatureJSON = string;
export type KESVkeyJSON = string;
export type LanguageJSON = "PlutusV1" | "PlutusV2" | "PlutusV3";
export type NetworkIdJSON = number;
export type NonceJSON =
  | {
      Identity: {
        [k: string]: unknown;
      };
    }
  | {
      Hash: {
        hash: string;
        [k: string]: unknown;
      };
    };
export type NonceHashJSON = string;
export type NonemptySetBootstrapWitnessJSON = BootstrapWitnessJSON[];
export type NonemptySetCertificateJSON = CertificateJSON[];
export type NonemptySetEd25519KeyHashJSON = string[];
export type NonemptySetNativeScriptJSON = NativeScriptJSON[];
export type NonemptySetPlutusDataJSON = PlutusDataJSON[];
export type NonemptySetPlutusV1ScriptJSON = string[];
export type NonemptySetPlutusV2ScriptJSON = string[];
export type NonemptySetPlutusV3ScriptJSON = string[];
export type NonemptySetProposalProcedureJSON = ProposalProcedureJSON[];
export type NonemptySetTransactionInputJSON = TransactionInputJSON[];
export type NonemptySetVkeywitnessJSON = VkeywitnessJSON[];
export type PlutusV1ScriptJSON = string;
export type PlutusV2ScriptJSON = string;
export type PlutusV3ScriptJSON = string;
export type PoolMetadataHashJSON = string;
export type RewardAddressJSON = string;
export type ScriptJSON =
  | {
      Native: {
        script: NativeScriptJSON;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV1: {
        script: string;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV2: {
        script: string;
        [k: string]: unknown;
      };
    }
  | {
      PlutusV3: {
        script: string;
        [k: string]: unknown;
      };
    };
export type ScriptDataHashJSON = string;
export type ScriptHashJSON = string;
export type SetCommitteeColdCredentialJSON = CredentialJSON[];
export type SetEd25519KeyHashJSON = string[];
export type SetTransactionInputJSON = TransactionInputJSON[];
export type SpendingDataJSON =
  | {
      SpendingDataPubKey: Bip32PublicKeyJSON;
    }
  | {
      SpendingDataScript: string;
    }
  | {
      SpendingDataRedeem: PublicKeyJSON;
    };
export type StakeholderIdJSON = string;
export type TransactionHashJSON = string;
export type UrlJSON = string;
export type VRFKeyHashJSON = string;
export type VRFVkeyJSON = string;
export type VoterJSON =
  | {
      ConstitutionalCommitteeHotKeyHash: {
        ed25519_key_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      ConstitutionalCommitteeHotScriptHash: {
        script_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      DRepKeyHash: {
        ed25519_key_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      DRepScriptHash: {
        script_hash: string;
        [k: string]: unknown;
      };
    }
  | {
      StakingPoolKeyHash: {
        ed25519_key_hash: string;
        [k: string]: unknown;
      };
    };

export interface AddrAttributesJSON {
  derivation_path?: HDAddressPayloadJSON | null;
  protocol_magic?: ProtocolMagicJSON | null;
  stake_distribution?: StakeDistributionJSON | null;
}
export interface AddressContentJSON {
  addr_attributes: AddrAttributesJSON;
  addr_type: ByronAddrTypeJSON;
  address_id: string;
}
export interface AlonzoFormatTxOutJSON {
  address: string;
  amount: ValueJSON;
  datum_hash?: string | null;
}
export interface ValueJSON {
  coin: number;
  multiasset: MultiAssetJSON;
}
export interface MultiAssetJSON {
  [k: string]: {
    [k: string]: number;
  };
}
export interface AnchorJSON {
  anchor_doc_hash: string;
  anchor_url: string;
}
export interface AuthCommitteeHotCertJSON {
  committee_cold_credential: CredentialJSON;
  committee_hot_credential: CredentialJSON;
}
/**
 * Collection of TransactionMetadatums indexed by TransactionMetadatumLabels
 * Handles the extremely rare edge-case of in previous generations allowing
 * duplicate metadatum labels.
 */
export interface MetadataJSON {
  entries: [unknown, unknown][];
}
export interface ShelleyMAFormatAuxDataJSON {
  auxiliary_scripts: NativeScriptJSON[];
  transaction_metadata: MetadataJSON;
}
export interface ScriptPubkeyJSON {
  ed25519_key_hash: string;
}
export interface ScriptAllJSON {
  native_scripts: NativeScriptJSON[];
}
export interface ScriptAnyJSON {
  native_scripts: NativeScriptJSON[];
}
export interface ScriptNOfKJSON {
  n: number;
  native_scripts: NativeScriptJSON[];
}
export interface ScriptInvalidBeforeJSON {
  before: number;
}
export interface ScriptInvalidHereafterJSON {
  after: number;
}
export interface ConwayFormatAuxDataJSON {
  metadata?: MetadataJSON | null;
  native_scripts?: NativeScriptJSON[] | null;
  plutus_v1_scripts?: string[] | null;
  plutus_v2_scripts?: string[] | null;
  plutus_v3_scripts?: string[] | null;
}
export interface BlockJSON {
  auxiliary_data_set: {
    [k: string]: AuxiliaryDataJSON;
  };
  header: HeaderJSON;
  invalid_transactions: number[];
  transaction_bodies: TransactionBodyJSON[];
  transaction_witness_sets: TransactionWitnessSetJSON[];
}
export interface HeaderJSON {
  body_signature: string;
  header_body: HeaderBodyJSON;
}
export interface HeaderBodyJSON {
  block_body_hash: string;
  block_body_size: number;
  block_number: number;
  issuer_vkey: PublicKeyJSON;
  operational_cert: OperationalCertJSON;
  prev_hash?: string | null;
  protocol_version: ProtocolVersionJSON;
  slot: number;
  vrf_result: VRFCertJSON;
  vrf_vkey: string;
}
export interface OperationalCertJSON {
  hot_vkey: string;
  kes_period: number;
  sequence_number: number;
  sigma: string;
}
export interface ProtocolVersionJSON {
  major: number;
  minor: number;
}
export interface VRFCertJSON {
  output: number[];
  proof: number[];
}
export interface TransactionBodyJSON {
  auxiliary_data_hash?: string | null;
  certs?: CertificateJSON[] | null;
  collateral_inputs?: TransactionInputJSON[] | null;
  collateral_return?: TransactionOutputJSON | null;
  current_treasury_value?: number | null;
  donation?: number | null;
  fee: number;
  inputs: TransactionInputJSON[];
  mint?: MintJSON | null;
  network_id?: number | null;
  outputs: TransactionOutputJSON[];
  proposal_procedures?: ProposalProcedureJSON[] | null;
  reference_inputs?: TransactionInputJSON[] | null;
  required_signers?: string[] | null;
  script_data_hash?: string | null;
  total_collateral?: number | null;
  ttl?: number | null;
  validity_interval_start?: number | null;
  voting_procedures?: {
    [k: string]: {
      [k: string]: VotingProcedureJSON;
    };
  } | null;
  withdrawals?: {
    [k: string]: number;
  } | null;
}
export interface StakeRegistrationJSON {
  stake_credential: CredentialJSON;
}
export interface StakeDeregistrationJSON {
  stake_credential: CredentialJSON;
}
export interface StakeDelegationJSON {
  pool: string;
  stake_credential: CredentialJSON;
}
export interface PoolRegistrationJSON {
  pool_params: PoolParamsJSON;
}
export interface PoolParamsJSON {
  cost: number;
  margin: UnitIntervalJSON;
  operator: string;
  pledge: number;
  pool_metadata?: PoolMetadataJSON | null;
  pool_owners: string[];
  relays: RelayJSON[];
  reward_account: string;
  vrf_keyhash: string;
}
export interface UnitIntervalJSON {
  end: number;
  start: number;
}
export interface PoolMetadataJSON {
  pool_metadata_hash: string;
  url: string;
}
export interface SingleHostAddrJSON {
  ipv4?: string | null;
  ipv6?: string | null;
  port?: number | null;
}
export interface SingleHostNameJSON {
  /**
   * An A or AAAA DNS record
   */
  dns_name: string;
  port?: number | null;
}
export interface MultiHostNameJSON {
  /**
   * A SRV DNS record
   */
  dns_name: string;
}
export interface PoolRetirementJSON {
  epoch: number;
  pool: string;
}
export interface RegCertJSON {
  deposit: number;
  stake_credential: CredentialJSON;
}
export interface UnregCertJSON {
  deposit: number;
  stake_credential: CredentialJSON;
}
export interface VoteDelegCertJSON {
  d_rep: DRepJSON;
  stake_credential: CredentialJSON;
}
export interface StakeVoteDelegCertJSON {
  d_rep: DRepJSON;
  pool: string;
  stake_credential: CredentialJSON;
}
export interface StakeRegDelegCertJSON {
  deposit: number;
  pool: string;
  stake_credential: CredentialJSON;
}
export interface VoteRegDelegCertJSON {
  d_rep: DRepJSON;
  deposit: number;
  stake_credential: CredentialJSON;
}
export interface StakeVoteRegDelegCertJSON {
  d_rep: DRepJSON;
  deposit: number;
  pool: string;
  stake_credential: CredentialJSON;
}
export interface ResignCommitteeColdCertJSON {
  anchor?: AnchorJSON | null;
  committee_cold_credential: CredentialJSON;
}
export interface RegDrepCertJSON {
  anchor?: AnchorJSON | null;
  deposit: number;
  drep_credential: CredentialJSON;
}
export interface UnregDrepCertJSON {
  deposit: number;
  drep_credential: CredentialJSON;
}
export interface UpdateDrepCertJSON {
  anchor?: AnchorJSON | null;
  drep_credential: CredentialJSON;
}
export interface TransactionInputJSON {
  index: number;
  transaction_id: string;
}
export interface ConwayFormatTxOutJSON {
  address: string;
  amount: ValueJSON;
  datum_option?: DatumOptionJSON | null;
  script_reference?: ScriptRefJSON | null;
}
export interface ConstrPlutusDataJSON {
  constructor: number;
  fields: PlutusDataJSON[];
}
export interface MintJSON {
  [k: string]: {
    [k: string]: number;
  };
}
export interface ProposalProcedureJSON {
  anchor: AnchorJSON;
  deposit: number;
  gov_action: GovActionJSON;
  reward_account: string;
}
export interface ParameterChangeActionJSON {
  action_id?: GovActionIdJSON | null;
  policy_hash?: string | null;
  update: ProtocolParamUpdateJSON;
}
export interface GovActionIdJSON {
  gov_action_index: number;
  transaction_id: string;
}
export interface ProtocolParamUpdateJSON {
  ada_per_utxo_byte?: number | null;
  collateral_percentage?: number | null;
  committee_term_limit?: number | null;
  cost_models_for_script_languages?: {
    /**
     * This interface was referenced by `undefined`'s JSON-Schema definition
     * via the `patternProperty` "^\d+$".
     */
    [k: string]: number[];
  } | null;
  d_rep_deposit?: number | null;
  d_rep_inactivity_period?: number | null;
  d_rep_voting_thresholds?: DRepVotingThresholdsJSON | null;
  execution_costs?: ExUnitPricesJSON | null;
  expansion_rate?: UnitIntervalJSON | null;
  governance_action_deposit?: number | null;
  governance_action_validity_period?: number | null;
  key_deposit?: number | null;
  max_block_body_size?: number | null;
  max_block_ex_units?: ExUnitsJSON | null;
  max_block_header_size?: number | null;
  max_collateral_inputs?: number | null;
  max_transaction_size?: number | null;
  max_tx_ex_units?: ExUnitsJSON | null;
  max_value_size?: number | null;
  maximum_epoch?: number | null;
  min_committee_size?: number | null;
  min_fee_ref_script_cost_per_byte?: RationalJSON | null;
  min_pool_cost?: number | null;
  minfee_a?: number | null;
  minfee_b?: number | null;
  n_opt?: number | null;
  pool_deposit?: number | null;
  pool_pledge_influence?: RationalJSON | null;
  pool_voting_thresholds?: PoolVotingThresholdsJSON | null;
  treasury_growth_rate?: UnitIntervalJSON | null;
}
export interface DRepVotingThresholdsJSON {
  committee_no_confidence: UnitIntervalJSON;
  committee_normal: UnitIntervalJSON;
  hard_fork_initiation: UnitIntervalJSON;
  motion_no_confidence: UnitIntervalJSON;
  pp_economic_group: UnitIntervalJSON;
  pp_governance_group: UnitIntervalJSON;
  pp_network_group: UnitIntervalJSON;
  pp_technical_group: UnitIntervalJSON;
  treasury_withdrawal: UnitIntervalJSON;
  update_constitution: UnitIntervalJSON;
}
export interface ExUnitPricesJSON {
  mem_price: RationalJSON;
  step_price: RationalJSON;
}
export interface RationalJSON {
  denominator: number;
  numerator: number;
}
export interface ExUnitsJSON {
  mem: number;
  steps: number;
}
export interface PoolVotingThresholdsJSON {
  committee_no_confidence: UnitIntervalJSON;
  committee_normal: UnitIntervalJSON;
  hard_fork_initiation: UnitIntervalJSON;
  motion_no_confidence: UnitIntervalJSON;
  security_relevant_parameter_voting_threshold: UnitIntervalJSON;
}
export interface HardForkInitiationActionJSON {
  action_id?: GovActionIdJSON | null;
  version: ProtocolVersionJSON;
}
export interface TreasuryWithdrawalsActionJSON {
  policy_hash?: string | null;
  withdrawal: {
    [k: string]: number;
  };
}
export interface NoConfidenceJSON {
  action_id?: GovActionIdJSON | null;
}
export interface UpdateCommitteeJSON {
  action_id?: GovActionIdJSON | null;
  cold_credentials: CredentialJSON[];
  credentials: {
    [k: string]: number;
  };
  unit_interval: UnitIntervalJSON;
}
export interface NewConstitutionJSON {
  action_id?: GovActionIdJSON | null;
  constitution: ConstitutionJSON;
}
export interface ConstitutionJSON {
  anchor: AnchorJSON;
  script_hash?: string | null;
}
export interface VotingProcedureJSON {
  anchor?: AnchorJSON | null;
  vote: VoteJSON;
}
export interface TransactionWitnessSetJSON {
  bootstrap_witnesses?: BootstrapWitnessJSON[] | null;
  native_scripts?: NativeScriptJSON[] | null;
  plutus_datums?: PlutusDataJSON[] | null;
  plutus_v1_scripts?: string[] | null;
  plutus_v2_scripts?: string[] | null;
  plutus_v3_scripts?: string[] | null;
  redeemers?: RedeemersJSON | null;
  vkeywitnesses?: VkeywitnessJSON[] | null;
}
export interface BootstrapWitnessJSON {
  attributes: AddrAttributesJSON;
  chain_code: number[];
  public_key: PublicKeyJSON;
  signature: string;
}
export interface LegacyRedeemerJSON {
  data: PlutusDataJSON;
  ex_units: ExUnitsJSON;
  index: number;
  tag: RedeemerTagJSON;
}
export interface RedeemerValJSON {
  data: PlutusDataJSON;
  ex_units: ExUnitsJSON;
}
export interface VkeywitnessJSON {
  ed25519_signature: string;
  vkey: PublicKeyJSON;
}
export interface ByronTxOutJSON {
  address: string;
  amount: number;
}
export interface CIP25FilesDetailsJSON {
  media_type: string;
  name: string;
  src: CIP25ChunkableStringJSON;
}
export interface CIP25LabelMetadataJSON {
  nfts: {
    [k: string]: {
      [k: string]: CIP25MetadataDetailsJSON;
    };
  };
  version: CIP25VersionJSON;
}
export interface CIP25MetadataDetailsJSON {
  description?: CIP25ChunkableStringJSON | null;
  files?: CIP25FilesDetailsJSON[] | null;
  image: CIP25ChunkableStringJSON;
  media_type?: string | null;
  name: string;
}
/**
 * This is the entire metadata schema for CIP-25
 * It can be parsed by passing in the CBOR bytes of the entire transaction metadata
 * or by passing in an existing Metadata struct.
 * Parsing from CBOR bytes should be marginally faster.
 */
export interface CIP25MetadataJSON {
  key_721: CIP25LabelMetadataJSON;
}
/**
 * A subset of CIP25MetadataDetails where the keys are optional
 * Useful to extract the key fields (name & image) of incorrectly formatted cip25
 */
export interface CIP25MiniMetadataDetailsJSON {
  image?: CIP25ChunkableStringJSON | null;
  name?: string | null;
}
/**
 * Weighted delegation input.
 * This is the proportion of weight to assign to this public key relative to the weights
 * of all other Delegations where this is used.
 */
export interface CIP36DelegationJSON {
  voting_pub_key: PublicKeyJSON;
  weight: number;
}
/**
 * This is the entire metadata schema for CIP-36 deregistration.
 * It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
 */
export interface CIP36DeregistrationCborJSON {
  deregistration_witness: CIP36DeregistrationWitnessJSON;
  key_deregistration: CIP36KeyDeregistrationJSON;
  /**
   * This interface was referenced by `CIP36DeregistrationCborJSON`'s JSON-Schema definition
   * via the `patternProperty` "^\d+$".
   */
  [k: string]: TransactionMetadatumJSON | CIP36DeregistrationWitnessJSON | CIP36KeyDeregistrationJSON;
}
export interface CIP36DeregistrationWitnessJSON {
  stake_witness: string;
}
export interface CIP36KeyDeregistrationJSON {
  /**
   * Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
   */
  nonce: number;
  stake_credential: PublicKeyJSON;
  voting_purpose: number;
}
export interface CIP36KeyRegistrationJSON {
  delegation: CIP36DelegationDistributionJSON;
  /**
   * Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
   */
  nonce: number;
  payment_address: string;
  stake_credential: PublicKeyJSON;
  voting_purpose: number;
}
/**
 * This is the entire metadata schema for CIP-36 registration.
 * It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
 */
export interface CIP36RegistrationCborJSON {
  key_registration: CIP36KeyRegistrationJSON;
  registration_witness: CIP36RegistrationWitnessJSON;
  /**
   * This interface was referenced by `CIP36RegistrationCborJSON`'s JSON-Schema definition
   * via the `patternProperty` "^\d+$".
   */
  [k: string]: TransactionMetadatumJSON | CIP36KeyRegistrationJSON | CIP36RegistrationWitnessJSON;
}
export interface CIP36RegistrationWitnessJSON {
  stake_witness: string;
}
export interface CostModelsJSON {
  /**
   * This interface was referenced by `CostModelsJSON`'s JSON-Schema definition
   * via the `patternProperty` "^\d+$".
   */
  [k: string]: number[];
}
export interface RedeemerKeyJSON {
  index: number;
  tag: RedeemerTagJSON;
}
export interface TransactionJSON {
  auxiliary_data?: AuxiliaryDataJSON | null;
  body: TransactionBodyJSON;
  is_valid: boolean;
  witness_set: TransactionWitnessSetJSON;
}
