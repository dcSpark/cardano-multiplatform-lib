use cml_crypto::Ed25519KeyHash;

use super::witness_builder::RequiredWitnessSet;

pub(crate) fn required_wits_from_required_signers(
    required_signers: &[Ed25519KeyHash],
) -> RequiredWitnessSet {
    let mut required_wits = RequiredWitnessSet::default();
    required_signers
        .iter()
        .for_each(|required_signer| required_wits.add_vkey_key_hash(*required_signer));
    required_wits
}
