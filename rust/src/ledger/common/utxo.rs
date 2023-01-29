use std::io::Write;
use std::io::{Seek, BufRead};
use cbor_event::Special as CBORSpecial;
use crate::error::JsError;

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"), not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use crate::{to_from_bytes, TransactionInput, TransactionOutput, error::{DeserializeFailure, DeserializeError}};

use super::binary::*;


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionUnspentOutput {
    pub(crate) input: TransactionInput,
    pub(crate) output: TransactionOutput
}

to_from_bytes!(TransactionUnspentOutput);

#[wasm_bindgen]
impl TransactionUnspentOutput {
    pub fn new(input: &TransactionInput, output: &TransactionOutput) -> TransactionUnspentOutput {
        Self {
            input: input.clone(),
            output: output.clone()
        }
    }

    pub fn input(&self) -> TransactionInput {
        self.input.clone()
    }

    pub fn output(&self) -> TransactionOutput {
        self.output.clone()
    }
}

impl cbor_event::se::Serialize for TransactionUnspentOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.input.serialize(serializer)?;
        self.output.serialize(serializer)
    }
}

impl Deserialize for TransactionUnspentOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Array => {
                    let len = raw.array()?;
                    let input = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionInput::deserialize(raw)?)
                    })().map_err(|e| e.annotate("input"))?;
                    let output = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionOutput::deserialize(raw)?)
                    })().map_err(|e| e.annotate("output"))?;
                    let ret = Ok(Self {
                        input,
                        output
                    });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 => /* it's ok */(),
                            n => return Err(DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()),
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => /* it's ok */(),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                },
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("TransactionUnspentOutput"))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct TransactionUnspentOutputs(pub(crate) Vec<TransactionUnspentOutput>);

#[wasm_bindgen]
impl TransactionUnspentOutputs {
    pub fn new() -> Self {
        // we have to provide new to expose it to WASM builds
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionUnspentOutput {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &TransactionUnspentOutput) {
        self.0.push(elem.clone());
    }
}
