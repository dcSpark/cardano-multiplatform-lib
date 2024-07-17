use crate::RequiredSigners;

use super::witness_builder::RequiredWitnessSet;

pub(crate) fn required_wits_from_required_signers(
    required_signers: &RequiredSigners,
) -> RequiredWitnessSet {
    let mut required_wits = RequiredWitnessSet::default();
    required_signers
        .as_ref()
        .iter()
        .for_each(|required_signer| required_wits.add_vkey_key_hash(*required_signer));
    required_wits
}
