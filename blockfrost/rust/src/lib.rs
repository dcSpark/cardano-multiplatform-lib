use cml_chain::{
    builders::tx_builder::TxBuilderConfigField,
    plutus::{CostModels, Language},
    Int,
};
use std::collections::HashMap;

#[cfg(feature = "direct_api")]
pub mod direct_api;

#[derive(Debug, thiserror::Error)]
pub enum CostModelsError {
    #[error("Invalid object format: {0:?}")]
    InvalidFormat(serde_json::Value),
    #[error("Invalid cost format: {0:?}")]
    InvalidCost(Option<serde_json::Value>),
    #[error("Invalid op count: {0}")]
    OpCount(usize),
}

#[derive(Debug, thiserror::Error)]
pub enum BlockfrostParamsParseError {
    #[error("Missing field: {0:?}")]
    MissingField(TxBuilderConfigField),
    #[error("Incorrect format in field: {0:?}")]
    IncorrectFormat(TxBuilderConfigField),
    #[error("Invalid cost models {0:?} for language {1:?}")]
    CostModels(CostModelsError, Language),
}

pub fn parse_cost_model(
    serde_value: &serde_json::Value,
    language: Language,
) -> Result<Vec<Int>, CostModelsError> {
    if let serde_json::Value::Object(cost_obj) = serde_value {
        let mut costs = vec![];
        // bad idea to assume it's ordered - depends on feature enabled
        // and could possibly change
        let mut keys: Vec<String> = cost_obj.keys().cloned().collect();
        if keys.len() != CostModels::op_count(language) {
            return Err(CostModelsError::OpCount(keys.len()));
        }
        keys.sort();
        for key in keys {
            let cost = cost_obj
                .get(&key)
                .and_then(|c| c.as_i64())
                .ok_or_else(|| CostModelsError::InvalidCost(cost_obj.get(&key).cloned()))?;
            costs.push(Int::from(cost));
        }
        Ok(costs)
    } else {
        Err(CostModelsError::InvalidFormat(serde_value.clone()))
    }
}

pub fn parse_sancho_cost_model(
    serde_value: &serde_json::Value,
) -> Result<Vec<Int>, CostModelsError> {
    if let serde_json::Value::Object(cost_obj) = serde_value {
        let mut costs = vec![];
        for i in 0..CostModels::PLUTUS_V3_COUNT {
            let cost = cost_obj
                .get(&i.to_string())
                .and_then(|val| val.as_i64())
                .ok_or_else(|| {
                    CostModelsError::InvalidCost(cost_obj.get(&i.to_string()).cloned())
                })?;
            costs.push(cml_chain::Int::from(cost));
        }
        Ok(costs)
    } else {
        Err(CostModelsError::InvalidFormat(serde_value.clone()))
    }
}

pub fn parse_cost_models(
    costs: &HashMap<String, serde_json::Value>,
) -> Result<CostModels, BlockfrostParamsParseError> {
    let mut cost_models = CostModels::new();
    if let Some(plutus_v1) = costs.get("PlutusV1") {
        cost_models.plutus_v1 = Some(
            parse_cost_model(plutus_v1, Language::PlutusV1)
                .map_err(|e| BlockfrostParamsParseError::CostModels(e, Language::PlutusV1))?,
        );
    }
    if let Some(plutus_v2) = costs.get("PlutusV2") {
        cost_models.plutus_v2 = Some(
            parse_cost_model(plutus_v2, Language::PlutusV2)
                .map_err(|e| BlockfrostParamsParseError::CostModels(e, Language::PlutusV2))?,
        );
    }
    // Sancho testnet has a very different format for some reason
    if let Some(plutus_v3) = costs.get("PlutusV3") {
        cost_models.plutus_v3 = Some(
            parse_sancho_cost_model(plutus_v3)
                .map_err(|e| BlockfrostParamsParseError::CostModels(e, Language::PlutusV3))?,
        );
    }
    Ok(cost_models)
}
