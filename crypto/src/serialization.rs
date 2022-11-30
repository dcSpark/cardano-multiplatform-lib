use super::*;
use std::io::{BufRead, Seek, SeekFrom, Write};
use cardano_multiplatform_lib_core::{
  error::Key,
  serialization::{
      fit_sz,
      CBORReadLen,
      Deserialize,
      DeserializeEmbeddedGroup,
      Serialize,
      SerializeEmbeddedGroup,
  }
}; 

impl Serialize for BootstrapWitness {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
      self.vkey.serialize(serializer, force_canonical)?;
      self.signature.serialize(serializer, force_canonical)?;
      serializer.write_bytes_sz(&self.chain_code, self.encodings.as_ref().map(|encs| encs.chain_code_encoding.clone()).unwrap_or_default().to_str_len_sz(self.chain_code.len() as u64, force_canonical))?;
      serializer.write_bytes_sz(&self.attributes, self.encodings.as_ref().map(|encs| encs.attributes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.attributes.len() as u64, force_canonical))?;
      self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
  }
}

impl Deserialize for BootstrapWitness {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let len = raw.array_sz()?;
          let len_encoding: LenEncoding = len.into();
          let mut read_len = CBORReadLen::new(len);
          read_len.read_elems(4)?;
          let vkey = (|| -> Result<_, DeserializeError> {
              Ok(Vkey::deserialize(raw)?)
          })().map_err(|e| e.annotate("vkey"))?;
          let signature = (|| -> Result<_, DeserializeError> {
              Ok(Ed25519Signature::deserialize(raw)?)
          })().map_err(|e| e.annotate("signature"))?;
          let (chain_code, chain_code_encoding) = (|| -> Result<_, DeserializeError> {
              Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
          })().map_err(|e| e.annotate("chain_code"))?;
          let (attributes, attributes_encoding) = (|| -> Result<_, DeserializeError> {
              Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
          })().map_err(|e| e.annotate("attributes"))?;
          match len {
              cbor_event::LenSz::Len(_, _) => (),
              cbor_event::LenSz::Indefinite => match raw.special()? {
                  cbor_event::Special::Break => (),
                  _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
              },
          }
          Ok(BootstrapWitness {
              vkey,
              signature,
              chain_code,
              attributes,
              encodings: Some(BootstrapWitnessEncoding {
                  len_encoding,
                  chain_code_encoding,
                  attributes_encoding,
              }),
          })
      })().map_err(|e| e.annotate("BootstrapWitness"))
  }
}

impl Serialize for KesSignature {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
  }
}

impl Deserialize for KesSignature {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
      if inner.len() != 32 {
          return Err(DeserializeError::new("KesSignature", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(32), max: Some(32) }));
      }
      Ok(Self {
          inner,
          encodings: Some(KesSignatureEncoding {
              inner_encoding,
          }),
      })
  }
}

impl Serialize for KesVkey {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
  }
}

impl Deserialize for KesVkey {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
      if inner.len() != 8 {
          return Err(DeserializeError::new("KesVkey", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(8), max: Some(8) }));
      }
      Ok(Self {
          inner,
          encodings: Some(KesVkeyEncoding {
              inner_encoding,
          }),
      })
  }
}

impl Serialize for SignkeyKES {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
  }
}

impl Deserialize for SignkeyKES {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
      if inner.len() != 16 {
          return Err(DeserializeError::new("SignkeyKES", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
      }
      Ok(Self {
          inner,
          encodings: Some(SignkeyKESEncoding {
              inner_encoding,
          }),
      })
  }
}

impl Serialize for Nonce {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      match self {
          Nonce::I0{ i0_encoding, outer_len_encoding } => {
              serializer.write_array_sz(outer_len_encoding.to_len_sz(1, force_canonical))?;
              serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))?;
              outer_len_encoding.end(serializer, force_canonical)?;
              Ok(serializer)
          },
          Nonce::Nonce1(nonce1) => nonce1.serialize(serializer, force_canonical),
      }
  }
}

impl Deserialize for Nonce {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let len = raw.array_sz()?;
          let outer_len_encoding: LenEncoding = len.into();
          let mut read_len = CBORReadLen::new(len);
          let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
              if i0_value != 0 {
                  return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
              }
              Ok(Some(i0_encoding))
          })(raw)
          {
              Ok(i0_encoding) => return Ok(Self::I0 {
                  i0_encoding,
                  outer_len_encoding,
              }),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              Ok(Nonce1::deserialize_as_embedded_group(raw, &mut read_len, len)?)
          })(raw)
          {
              Ok(nonce1) => return Ok(Self::Nonce1(nonce1)),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match len {
              cbor_event::LenSz::Len(_, _) => (),
              cbor_event::LenSz::Indefinite => match raw.special()? {
                  cbor_event::Special::Break => (),
                  _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
              },
          }
          Err(DeserializeError::new("Nonce", DeserializeFailure::NoVariantMatched.into()))
      })().map_err(|e| e.annotate("Nonce"))
  }
}

impl Serialize for Nonce1 {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
      self.serialize_as_embedded_group(serializer, force_canonical)
  }
}

impl SerializeEmbeddedGroup for Nonce1 {
  fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
      serializer.write_bytes_sz(&self.bytes, self.encodings.as_ref().map(|encs| encs.bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.bytes.len() as u64, force_canonical))?;
      self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
  }
}

impl Deserialize for Nonce1 {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let len = raw.array_sz()?;
          let mut read_len = CBORReadLen::new(len);
          read_len.read_elems(2)?;
          let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
          match len {
              cbor_event::LenSz::Len(_, _) => (),
              cbor_event::LenSz::Indefinite => match raw.special()? {
                  cbor_event::Special::Break => (),
                  _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
              },
          }
          ret
      })().map_err(|e| e.annotate("Nonce1"))
  }
}

impl DeserializeEmbeddedGroup for Nonce1 {
  fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
      let len_encoding = len.into();
      let index_0_encoding = (|| -> Result<_, DeserializeError> {
          let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
          if index_0_value != 1 {
              return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
          }
          Ok(Some(index_0_encoding))
      })().map_err(|e| e.annotate("index_0"))?;
      let (bytes, bytes_encoding) = (|| -> Result<_, DeserializeError> {
          Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
      })().map_err(|e| e.annotate("bytes"))?;
      Ok(Nonce1 {
          bytes,
          encodings: Some(Nonce1Encoding {
              len_encoding,
              index_0_encoding,
              bytes_encoding,
          }),
      })
  }
}

impl Serialize for Vkey {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      let pubkey_bytes = self.pubkey.as_bytes();
      serializer.write_bytes_sz(&pubkey_bytes, self.encodings.as_ref().map(|encs| encs.pubkey_bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(pubkey_bytes.len() as u64, force_canonical))
  }
}

impl Deserialize for Vkey {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      let (pubkey_bytes, pubkey_bytes_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
      let pubkey = chain_crypto::PublicKey::from_binary(pubkey_bytes.as_ref())
          .map(PublicKey)
          .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
      Ok(Self {
          pubkey,
          encodings: Some(VkeyEncoding {
              pubkey_bytes_encoding,
          }),
      })
  }
}

impl Serialize for Vkeywitness {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
      self.vkey.serialize(serializer, force_canonical)?;
      self.signature.serialize(serializer, force_canonical)?;
      self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
  }
}

impl Deserialize for Vkeywitness {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let len = raw.array_sz()?;
          let len_encoding: LenEncoding = len.into();
          let mut read_len = CBORReadLen::new(len);
          read_len.read_elems(2)?;
          let vkey = (|| -> Result<_, DeserializeError> {
              Ok(Vkey::deserialize(raw)?)
          })().map_err(|e| e.annotate("vkey"))?;
          let signature = (|| -> Result<_, DeserializeError> {
              Ok(Ed25519Signature::deserialize(raw)?)
          })().map_err(|e| e.annotate("signature"))?;
          match len {
              cbor_event::LenSz::Len(_, _) => (),
              cbor_event::LenSz::Indefinite => match raw.special()? {
                  cbor_event::Special::Break => (),
                  _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
              },
          }
          Ok(Vkeywitness {
              vkey,
              signature,
              encodings: Some(VkeywitnessEncoding {
                  len_encoding,
              }),
          })
      })().map_err(|e| e.annotate("Vkeywitness"))
  }
}

impl Serialize for VrfCert {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
      serializer.write_bytes_sz(&self.index_0, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default().to_str_len_sz(self.index_0.len() as u64, force_canonical))?;
      serializer.write_bytes_sz(&self.bytes, self.encodings.as_ref().map(|encs| encs.bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.bytes.len() as u64, force_canonical))?;
      self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
  }
}

impl Deserialize for VrfCert {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let len = raw.array_sz()?;
          let len_encoding: LenEncoding = len.into();
          let mut read_len = CBORReadLen::new(len);
          read_len.read_elems(2)?;
          let (index_0, index_0_encoding) = (|| -> Result<_, DeserializeError> {
              Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
          })().map_err(|e| e.annotate("index_0"))?;
          let (bytes, bytes_encoding) = (|| -> Result<_, DeserializeError> {
              Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
          })().map_err(|e| e.annotate("bytes"))?;
          match len {
              cbor_event::LenSz::Len(_, _) => (),
              cbor_event::LenSz::Indefinite => match raw.special()? {
                  cbor_event::Special::Break => (),
                  _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
              },
          }
          Ok(VrfCert {
              index_0,
              bytes,
              encodings: Some(VrfCertEncoding {
                  len_encoding,
                  index_0_encoding,
                  bytes_encoding,
              }),
          })
      })().map_err(|e| e.annotate("VrfCert"))
  }
}

impl Serialize for VrfVkey {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
  }
}

impl Deserialize for VrfVkey {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
      if inner.len() != 8 {
          return Err(DeserializeError::new("VrfVkey", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(8), max: Some(8) }));
      }
      Ok(Self {
          inner,
          encodings: Some(VrfVkeyEncoding {
              inner_encoding,
          }),
      })
  }
}