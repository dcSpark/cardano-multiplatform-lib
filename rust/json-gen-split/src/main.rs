use cddl_lib_core::*;

fn main() {
    macro_rules! gen_json_schema {
        ($name:ident) =>  {
            let dest_path = std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
            std::fs::write(&dest_path, serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap()).unwrap();
        }
    }
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    gen_json_schema!(Address);
    gen_json_schema!(AlonzoAuxData);
    gen_json_schema!(AlonzoTxOut);
    gen_json_schema!(AssetName);
    gen_json_schema!(AuxiliaryData);
    gen_json_schema!(BabbageTxOut);
    gen_json_schema!(BigInt);
    gen_json_schema!(Block);
    gen_json_schema!(BootstrapWitness);
    gen_json_schema!(Certificate);
    gen_json_schema!(ConstrPlutusData);
    gen_json_schema!(Costmdls);
    gen_json_schema!(DatumOption);
    gen_json_schema!(DatumOption0);
    gen_json_schema!(DatumOption1);
    gen_json_schema!(DnsName);
    gen_json_schema!(ExUnitPrices);
    gen_json_schema!(ExUnits);
    gen_json_schema!(GenesisKeyDelegation);
    gen_json_schema!(Hash28);
    gen_json_schema!(Hash32);
    gen_json_schema!(Header);
    gen_json_schema!(HeaderBody);
    gen_json_schema!(I0OrI1);
    gen_json_schema!(Int);
    gen_json_schema!(InvalidBefore);
    gen_json_schema!(InvalidHereafter);
    gen_json_schema!(Ipv4);
    gen_json_schema!(Ipv6);
    gen_json_schema!(KesSignature);
    gen_json_schema!(KesVkey);
    gen_json_schema!(Language);
    gen_json_schema!(MoveInstantaneousReward);
    gen_json_schema!(MoveInstantaneousRewardsCert);
    gen_json_schema!(MultiHostName);
    gen_json_schema!(NativeScript);
    gen_json_schema!(NetworkId);
    gen_json_schema!(Nonce);
    gen_json_schema!(Nonce1);
    gen_json_schema!(OperationalCert);
    gen_json_schema!(PlutusData);
    gen_json_schema!(PoolMetadata);
    gen_json_schema!(PoolParams);
    gen_json_schema!(PoolRegistration);
    gen_json_schema!(PoolRetirement);
    gen_json_schema!(PositiveInterval);
    gen_json_schema!(ProtocolParamUpdate);
    gen_json_schema!(ProtocolVersion);
    gen_json_schema!(ProtocolVersionStruct);
    gen_json_schema!(Rational);
    gen_json_schema!(Redeemer);
    gen_json_schema!(RedeemerTag);
    gen_json_schema!(Relay);
    gen_json_schema!(RequiredSigners);
    gen_json_schema!(RewardAccount);
    gen_json_schema!(Script);
    gen_json_schema!(Script0);
    gen_json_schema!(Script1);
    gen_json_schema!(Script2);
    gen_json_schema!(ScriptAll);
    gen_json_schema!(ScriptAny);
    gen_json_schema!(ScriptNOfK);
    gen_json_schema!(ScriptPubkey);
    gen_json_schema!(ShelleyMaAuxData);
    gen_json_schema!(ShelleyTxOut);
    gen_json_schema!(Signature);
    gen_json_schema!(SignkeyKES);
    gen_json_schema!(SingleHostAddr);
    gen_json_schema!(SingleHostName);
    gen_json_schema!(StakeCredential);
    gen_json_schema!(StakeCredential0);
    gen_json_schema!(StakeCredential1);
    gen_json_schema!(StakeDelegation);
    gen_json_schema!(StakeDeregistration);
    gen_json_schema!(StakeRegistration);
    gen_json_schema!(Transaction);
    gen_json_schema!(TransactionBody);
    gen_json_schema!(TransactionInput);
    gen_json_schema!(TransactionMetadatum);
    gen_json_schema!(TransactionOutput);
    gen_json_schema!(TransactionWitnessSet);
    gen_json_schema!(UnitInterval);
    gen_json_schema!(Update);
    gen_json_schema!(Url);
    gen_json_schema!(Value);
    gen_json_schema!(Vkey);
    gen_json_schema!(Vkeywitness);
    gen_json_schema!(VrfCert);
    gen_json_schema!(VrfVkey);
}