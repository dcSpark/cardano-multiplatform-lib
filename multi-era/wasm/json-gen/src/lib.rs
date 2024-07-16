macro_rules! gen_json_schema {
    ($name:ty) => {
        let dest_path =
            std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
        std::fs::write(
            &dest_path,
            serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap(),
        )
        .unwrap();
    };
}

pub fn export_schemas() {
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    // allegra
    gen_json_schema!(cml_multi_era::allegra::AllegraAuxiliaryData);
    gen_json_schema!(cml_multi_era::allegra::AllegraBlock);
    gen_json_schema!(cml_multi_era::allegra::AllegraCertificate);
    gen_json_schema!(cml_multi_era::allegra::AllegraTransaction);
    gen_json_schema!(cml_multi_era::allegra::AllegraTransactionBody);
    gen_json_schema!(cml_multi_era::allegra::AllegraTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::allegra::MIRAction);
    gen_json_schema!(cml_multi_era::allegra::MIRPot);
    gen_json_schema!(cml_multi_era::allegra::MoveInstantaneousReward);
    gen_json_schema!(cml_multi_era::allegra::MoveInstantaneousRewardsCert);
    // alonzo
    gen_json_schema!(cml_multi_era::alonzo::AlonzoAuxiliaryData);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoBlock);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoFormatAuxData);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoProtocolParamUpdate);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoRedeemer);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoRedeemerTag);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoTransaction);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoTransactionBody);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoUpdate);
    // babbage
    gen_json_schema!(cml_multi_era::babbage::BabbageAuxiliaryData);
    gen_json_schema!(cml_multi_era::babbage::BabbageBlock);
    gen_json_schema!(cml_multi_era::babbage::BabbageFormatAuxData);
    gen_json_schema!(cml_multi_era::babbage::BabbageFormatTxOut);
    gen_json_schema!(cml_multi_era::babbage::BabbageProtocolParamUpdate);
    gen_json_schema!(cml_multi_era::babbage::BabbageScript);
    gen_json_schema!(cml_multi_era::babbage::BabbageTransaction);
    gen_json_schema!(cml_multi_era::babbage::BabbageTransactionBody);
    gen_json_schema!(cml_multi_era::babbage::BabbageTransactionOutput);
    gen_json_schema!(cml_multi_era::babbage::BabbageTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::babbage::BabbageUpdate);
    // byron
    gen_json_schema!(cml_multi_era::byron::Blake2b256);
    gen_json_schema!(cml_multi_era::byron::ByronSlotId);
    // byron::block
    gen_json_schema!(cml_multi_era::byron::block::BlockHeaderExtraData);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlock);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockBody);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockConsensusData);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockHeader);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockSignature);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockSignatureNormal);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy);
    gen_json_schema!(cml_multi_era::byron::block::ByronBlockSignatureProxyLight);
    gen_json_schema!(cml_multi_era::byron::block::ByronBodyProof);
    gen_json_schema!(cml_multi_era::byron::block::ByronDifficulty);
    gen_json_schema!(cml_multi_era::byron::block::ByronEbBlock);
    gen_json_schema!(cml_multi_era::byron::block::ByronMainBlock);
    gen_json_schema!(cml_multi_era::byron::block::EbbConsensusData);
    gen_json_schema!(cml_multi_era::byron::block::EbbHead);
    gen_json_schema!(cml_multi_era::byron::block::TxAux);
    // byron::delegation
    gen_json_schema!(cml_multi_era::byron::delegation::ByronDelegation);
    gen_json_schema!(cml_multi_era::byron::delegation::ByronDelegationSignature);
    gen_json_schema!(cml_multi_era::byron::delegation::EpochRange);
    gen_json_schema!(cml_multi_era::byron::delegation::LightWeightDelegationSignature);
    gen_json_schema!(cml_multi_era::byron::delegation::LightWeightDlg);
    // byron::mpc
    gen_json_schema!(cml_multi_era::byron::mpc::Ssc);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCert);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCertificatesPayload);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCertificatesProof);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCommitment);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCommitmentsPayload);
    gen_json_schema!(cml_multi_era::byron::mpc::SscCommitmentsProof);
    gen_json_schema!(cml_multi_era::byron::mpc::SscOpeningsPayload);
    gen_json_schema!(cml_multi_era::byron::mpc::SscOpeningsProof);
    gen_json_schema!(cml_multi_era::byron::mpc::SscProof);
    gen_json_schema!(cml_multi_era::byron::mpc::SscSharesPayload);
    gen_json_schema!(cml_multi_era::byron::mpc::SscSharesProof);
    gen_json_schema!(cml_multi_era::byron::mpc::SscSignedCommitment);
    gen_json_schema!(cml_multi_era::byron::mpc::VssEncryptedShare);
    gen_json_schema!(cml_multi_era::byron::mpc::VssProof);
    // byron::transaction
    gen_json_schema!(cml_multi_era::byron::utils::ByronAny);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronPkWitness);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronPkWitnessEntry);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronRedeemWitness);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronRedeemerScript);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronScriptWitness);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronScriptWitnessEntry);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTx);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxIn);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxInGenesis);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxInRegular);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxOutPtr);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxProof);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronTxWitness);
    gen_json_schema!(cml_multi_era::byron::transaction::ByronValidatorScript);
    // byron::update
    gen_json_schema!(cml_multi_era::byron::update::Bvermod);
    gen_json_schema!(cml_multi_era::byron::update::ByronBlockVersion);
    gen_json_schema!(cml_multi_era::byron::update::ByronSoftwareVersion);
    gen_json_schema!(cml_multi_era::byron::update::ByronTxFeePolicy);
    gen_json_schema!(cml_multi_era::byron::update::ByronUpdate);
    gen_json_schema!(cml_multi_era::byron::update::ByronUpdateData);
    gen_json_schema!(cml_multi_era::byron::update::ByronUpdateProposal);
    gen_json_schema!(cml_multi_era::byron::update::ByronUpdateVote);
    gen_json_schema!(cml_multi_era::byron::update::SoftForkRule);
    gen_json_schema!(cml_multi_era::byron::update::StdFeePolicy);
    // lib
    gen_json_schema!(cml_multi_era::MultiEraBlock);
    gen_json_schema!(cml_multi_era::MultiEraTransactionBody);
    // mary
    gen_json_schema!(cml_multi_era::mary::MaryBlock);
    gen_json_schema!(cml_multi_era::mary::MaryTransaction);
    gen_json_schema!(cml_multi_era::mary::MaryTransactionBody);
    gen_json_schema!(cml_multi_era::mary::MaryTransactionOutput);
    // shelley
    gen_json_schema!(cml_multi_era::shelley::MultisigAll);
    gen_json_schema!(cml_multi_era::shelley::MultisigAny);
    gen_json_schema!(cml_multi_era::shelley::MultisigNOfK);
    gen_json_schema!(cml_multi_era::shelley::MultisigPubkey);
    gen_json_schema!(cml_multi_era::shelley::MultisigScript);
    gen_json_schema!(cml_multi_era::shelley::ShelleyBlock);
    gen_json_schema!(cml_multi_era::shelley::ShelleyCertificate);
    gen_json_schema!(cml_multi_era::shelley::ShelleyDNSName);
    gen_json_schema!(cml_multi_era::shelley::ShelleyHeader);
    gen_json_schema!(cml_multi_era::shelley::ShelleyHeaderBody);
    gen_json_schema!(cml_multi_era::shelley::ShelleyMoveInstantaneousReward);
    gen_json_schema!(cml_multi_era::shelley::ShelleyMoveInstantaneousRewardsCert);
    gen_json_schema!(cml_multi_era::shelley::ShelleyMultiHostName);
    gen_json_schema!(cml_multi_era::shelley::ShelleyPoolParams);
    gen_json_schema!(cml_multi_era::shelley::ShelleyPoolRegistration);
    gen_json_schema!(cml_multi_era::shelley::ShelleyProtocolParamUpdate);
    gen_json_schema!(cml_multi_era::shelley::ShelleyRelay);
    gen_json_schema!(cml_multi_era::shelley::ShelleySingleHostName);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransaction);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionBody);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionOutput);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::shelley::ShelleyUpdate);
}
