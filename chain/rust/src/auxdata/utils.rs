use crate::auxdata::metadata::Metadata;

use crate::{
    plutus::{PlutusV1Script, PlutusV2Script},
    transaction::NativeScript,
};

use super::{AuxiliaryData, ConwayFormatAuxData, ShelleyMAFormatAuxData};

impl AuxiliaryData {
    pub fn new() -> Self {
        Self::new_shelley(Metadata::new())
    }

    pub fn metadata(&self) -> Option<&Metadata> {
        match self {
            Self::Shelley(shelley) => Some(shelley),
            Self::ShelleyMA(shelley_ma) => Some(&shelley_ma.transaction_metadata),
            Self::Conway(conway) => conway.metadata.as_ref(),
        }
    }

    /// Mut ref to the general tx metadata.
    /// Will be created if it didn't exist (i.e. Conway format)
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        match self {
            Self::Shelley(shelley) => shelley,
            Self::ShelleyMA(shelley_ma) => &mut shelley_ma.transaction_metadata,
            Self::Conway(conway) => {
                if conway.metadata.is_none() {
                    conway.metadata = Some(Metadata::new());
                }
                conway.metadata.as_mut().unwrap()
            }
        }
    }

    pub fn native_scripts(&self) -> Option<&Vec<NativeScript>> {
        match self {
            Self::Shelley { .. } => None,
            Self::ShelleyMA(shelley_ma) => Some(&shelley_ma.auxiliary_scripts),
            Self::Conway(conway) => conway.native_scripts.as_ref(),
        }
        .filter(|scripts| !scripts.is_empty())
    }

    pub fn plutus_v1_scripts(&self) -> Option<&Vec<PlutusV1Script>> {
        match self {
            Self::Shelley { .. } => None,
            Self::ShelleyMA(_shelley_ma) => None,
            Self::Conway(conway) => conway.plutus_v1_scripts.as_ref(),
        }
        .filter(|scripts| !scripts.is_empty())
    }

    pub fn plutus_v2_scripts(&self) -> Option<&Vec<PlutusV2Script>> {
        match self {
            Self::Shelley { .. } => None,
            Self::ShelleyMA(_shelley_ma) => None,
            Self::Conway(conway) => conway.plutus_v2_scripts.as_ref(),
        }
        .filter(|scripts| !scripts.is_empty())
    }

    /// Warning: overwrites any conflicting metadatum labels present
    pub fn add_metadata(&mut self, other: Metadata) {
        let metadata = match self {
            Self::Shelley(shelley) => shelley,
            Self::ShelleyMA(shelley_ma) => &mut shelley_ma.transaction_metadata,
            Self::Conway(conway) => {
                if conway.metadata.is_none() {
                    conway.metadata = Some(Metadata::new());
                }
                conway.metadata.as_mut().unwrap()
            }
        };
        metadata.entries.extend(other.entries);
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_native_scripts(&mut self, scripts: Vec<NativeScript>) {
        match self {
            Self::Shelley(shelley) => {
                *self = Self::ShelleyMA(ShelleyMAFormatAuxData::new(shelley.clone(), scripts));
            }
            Self::ShelleyMA(shelley_ma) => {
                shelley_ma.auxiliary_scripts.extend(scripts);
            }
            Self::Conway(conway) => {
                if let Some(old_scripts) = &mut conway.native_scripts {
                    old_scripts.extend(scripts);
                } else {
                    conway.native_scripts = Some(scripts);
                }
            }
        }
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_plutus_v1_scripts(&mut self, scripts: Vec<PlutusV1Script>) {
        match self {
            Self::Shelley(shelley) => {
                let mut conway = ConwayFormatAuxData::new();
                if !shelley.entries.is_empty() {
                    conway.metadata = Some(shelley.clone());
                }
                conway.plutus_v1_scripts = Some(scripts);
                *self = Self::Conway(conway);
            }
            Self::ShelleyMA(shelley_ma) => {
                let mut conway = ConwayFormatAuxData::new();
                if !shelley_ma.transaction_metadata.entries.is_empty() {
                    conway.metadata = Some(shelley_ma.transaction_metadata.clone());
                }
                if !shelley_ma.auxiliary_scripts.is_empty() {
                    conway.native_scripts = Some(shelley_ma.auxiliary_scripts.clone());
                }
                conway.plutus_v1_scripts = Some(scripts);
                *self = Self::Conway(conway);
            }
            Self::Conway(conway) => {
                if let Some(old_scripts) = &mut conway.plutus_v1_scripts {
                    old_scripts.extend(scripts);
                } else {
                    conway.plutus_v1_scripts = Some(scripts);
                }
            }
        }
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_plutus_v2_scripts(&mut self, scripts: Vec<PlutusV2Script>) {
        match self {
            Self::Shelley(shelley) => {
                let mut conway = ConwayFormatAuxData::new();
                if !shelley.entries.is_empty() {
                    conway.metadata = Some(shelley.clone());
                }
                conway.plutus_v2_scripts = Some(scripts);
                *self = Self::Conway(conway);
            }
            Self::ShelleyMA(shelley_ma) => {
                let mut conway = ConwayFormatAuxData::new();
                if !shelley_ma.transaction_metadata.entries.is_empty() {
                    conway.metadata = Some(shelley_ma.transaction_metadata.clone());
                }
                if !shelley_ma.auxiliary_scripts.is_empty() {
                    conway.native_scripts = Some(shelley_ma.auxiliary_scripts.clone());
                }
                conway.plutus_v2_scripts = Some(scripts);
                *self = Self::Conway(conway);
            }
            Self::Conway(conway) => {
                if let Some(old_scripts) = &mut conway.plutus_v2_scripts {
                    old_scripts.extend(scripts);
                } else {
                    conway.plutus_v2_scripts = Some(scripts);
                }
            }
        }
    }

    /// Adds everything present in other to self
    /// May change the era the aux data is in if necessary
    /// Warning: overwrites any metadatum labels present
    /// also does not check for duplicates in scripts
    pub fn add(&mut self, other: AuxiliaryData) {
        // to avoid redundant migrating of formats, we set the content with
        // plutus scripts first, then native scripts, then metadata in
        // reverse chronological (era format wise) order.
        match other {
            Self::Shelley(shelley) => {
                self.add_metadata(shelley);
            }
            Self::ShelleyMA(shelley_ma) => {
                self.add_native_scripts(shelley_ma.auxiliary_scripts);
                self.add_metadata(shelley_ma.transaction_metadata);
            }
            Self::Conway(conway) => {
                if let Some(scripts) = conway.plutus_v2_scripts {
                    self.add_plutus_v2_scripts(scripts);
                }
                if let Some(scripts) = conway.plutus_v1_scripts {
                    self.add_plutus_v1_scripts(scripts);
                }
                if let Some(scripts) = conway.native_scripts {
                    self.add_native_scripts(scripts);
                }
                if let Some(metadata) = conway.metadata {
                    self.add_metadata(metadata);
                }
            }
        }
    }
}

impl Default for AuxiliaryData {
    fn default() -> Self {
        Self::new()
    }
}
