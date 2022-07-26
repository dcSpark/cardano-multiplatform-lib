use cbor_event::{
    self,
    de::Deserializer,
    se::{self, Serializer},
};
use std::io::{BufRead, Write};

use crate::byron::ProtocolMagic;

use super::{Ed25519Bip32, Signature, byron_tags};

type SignData = ();

type ProxyCert = Signature<(), Ed25519Bip32>;

#[derive(Debug, Clone)]
pub struct ProxySecretKey {
    pub omega: u64,
    pub issuer_pk: super::PublicKey<Ed25519Bip32>,
    pub delegate_pk: super::PublicKey<Ed25519Bip32>,
    pub cert: ProxyCert,
}

impl cbor_event::se::Serialize for ProxySecretKey {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer
            .write_array(cbor_event::Len::Len(4))?
            .serialize(&self.omega)?
            .serialize(&self.issuer_pk)?
            .serialize(&self.delegate_pk)?
            .serialize(&self.cert)
    }
}

impl cbor_event::de::Deserialize for ProxySecretKey {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        raw.tuple(4, "ProxySecretKey")?;

        let omega = cbor_event::de::Deserialize::deserialize(raw)?;
        let issuer_pk = cbor_event::de::Deserialize::deserialize(raw)?;
        let delegate_pk = cbor_event::de::Deserialize::deserialize(raw)?;
        let cert = cbor_event::de::Deserialize::deserialize(raw)?;

        Ok(ProxySecretKey {
            omega,
            issuer_pk,
            delegate_pk,
            cert,
        })
    }
}

impl ProxySecretKey {
    /// Verify that 'cert' is a signature from 'issuer_pk' over
    /// 'delegate_pk' and 'omega'.
    pub fn verify(&self, protocol_magic: ProtocolMagic) -> bool {
        let buf = Self::data_to_sign(&self.delegate_pk, self.omega, protocol_magic);
        // signature.0.verify_slice(&self.0, data) == crypto::Verification::Success
        self.cert.verify_slice(&self.issuer_pk, &buf) == super::sign::Verification::Success
    }

    /// Use 'issuer_prv' to sign 'delegate_pk' and 'omega' to create a
    /// ProxySecretKey.
    pub fn sign(
        issuer_prv: &super::SecretKey<Ed25519Bip32>,
        delegate_pk: super::PublicKey<Ed25519Bip32>,
        omega: u64,
        protocol_magic: ProtocolMagic,
    ) -> Self {
        let buf = Self::data_to_sign(&delegate_pk, omega, protocol_magic);

        Self {
            omega,
            issuer_pk: issuer_prv.to_public(),
            delegate_pk,
            cert: issuer_prv.sign_slice(&buf),
        }
    }

    fn data_to_sign(
        delegate_pk: &super::PublicKey<Ed25519Bip32>,
        omega: u64,
        protocol_magic: ProtocolMagic,
    ) -> Vec<u8> {
        // Yes, this really is
        // CBOR-in-byte-vector-in-CBOR-in-byte-vector...
        let mut buf2 = vec!['0' as u8, '0' as u8];
        buf2.extend(delegate_pk.as_ref());
        se::Serializer::new(&mut buf2).serialize(&omega).unwrap();

        let mut buf = vec![];
        buf.push(byron_tags::SigningTag::ProxySK as u8);
        se::Serializer::new(&mut buf)
            .serialize(&protocol_magic)
            .unwrap()
            .write_bytes(buf2)
            .unwrap();

        buf
    }
}

#[derive(Debug, Clone)]
pub struct ProxySignature {
    pub psk: ProxySecretKey,
    pub sig: Signature<(), Ed25519Bip32>,
}

impl cbor_event::se::Serialize for ProxySignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer
            .write_array(cbor_event::Len::Len(2))?
            .serialize(&self.psk)?
            .serialize(&self.sig)
    }
}

impl cbor_event::de::Deserialize for ProxySignature {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        raw.tuple(2, "ProxySignature")?;

        let psk = cbor_event::de::Deserialize::deserialize(raw)?;
        let sig = cbor_event::de::Deserialize::deserialize(raw)?;

        Ok(ProxySignature { psk, sig })
    }
}

#[derive(Debug, Clone)]
pub enum BlockSignature {
    Signature(Signature<SignData, Ed25519Bip32>),
    ProxyLight(Vec<cbor_event::Value>), // TODO: decode
    ProxyHeavy(ProxySignature),
}
impl BlockSignature {
    pub fn to_bytes<'a>(&'a self) -> Option<&'a [u8; ed25519_bip32::SIGNATURE_SIZE]> {
        match self {
            BlockSignature::Signature(s) => {
                Some(s.signdata.to_bytes())
            },
            _ => None,
        }
    }
}
impl cbor_event::se::Serialize for BlockSignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            &BlockSignature::Signature(ref sig) => serializer
                .write_array(cbor_event::Len::Len(2))?
                .write_unsigned_integer(0)?
                .serialize(sig),
            &BlockSignature::ProxyLight(ref v) => {
                let serializer = serializer
                    .write_array(cbor_event::Len::Len(2))?
                    .write_unsigned_integer(1)?;
                cbor_event::se::serialize_fixed_array(v.iter(), serializer)
            }
            &BlockSignature::ProxyHeavy(ref v) => serializer
                .write_array(cbor_event::Len::Len(2))?
                .write_unsigned_integer(2)?
                .serialize(v),
        }
    }
}
impl cbor_event::de::Deserialize for BlockSignature {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        raw.tuple(2, "BlockSignature")?;
        let sum_type_idx = raw.unsigned_integer()?;
        match sum_type_idx {
            0 => Ok(BlockSignature::Signature(raw.deserialize()?)),
            1 => Ok(BlockSignature::ProxyLight(raw.deserialize()?)),
            2 => Ok(BlockSignature::ProxyHeavy(
                cbor_event::de::Deserialize::deserialize(raw)?,
            )),
            _ => Err(cbor_event::Error::CustomError(format!(
                "Unsupported BlockSignature: {}",
                sum_type_idx
            ))),
        }
    }
}

#[cfg(test)]
mod tests {

    use base64;
    use std::str::FromStr;

    use crate::chain_crypto::{Ed25519Bip32, SecretKey, PublicKey, byron_proxy_key::ProxySecretKey, Signature};

    #[test]
    fn test_psk_verify() {
        let mut psk = ProxySecretKey {
            omega: 0,
            issuer_pk: PublicKey::<Ed25519Bip32>::from_binary(&base64::decode(&"nFhj99RbuDjG5jU3XjRXlUbP+4LStPeiMh7E7l3oWWfwRqjxXg10jUFt+4pKRlnZTrmI4weBWMGpchDJA9MKnA==").unwrap()).unwrap(),
            delegate_pk: PublicKey::<Ed25519Bip32>::from_binary(&base64::decode(&"mLujHvc/6KIvUEt2IdnjmVRENEHx9ifl45ZmhZZ8e39+C4fe/HgnKjFtT1M5LjeeSn1Bp8tSAM4WZwL+ECWgsw==").unwrap()).unwrap(),
            cert: Signature::<(), Ed25519Bip32>::from_str("fd30c5ac3f77df733eabe48de391ad6727b6ecd7ee72cc85207075a9bba90365f10455b80f3dbf5cc821f71075f00ebdfcffd30b264b5262c1473fd70125ee05").unwrap()
        };

        let pm = 1097911063.into();

        assert!(psk.verify(pm));

        psk.omega = 1;

        assert!(!psk.verify(pm));
    }

    #[test]
    fn test_psk_sign() {
        let pm = 328429219.into();

        let issuer_prv = SecretKey::<Ed25519Bip32>::from_str("b8b054ec1b92dd4542db35e2f813f013a8d7ee9f53255b26f3ef3dafb74e11462545bd9c85aa0a6f6719a933eba16909c1a2fa0bbb58e9cd98bf9ddbb79f7d50fcfc22db8155f8d6ca0e3a975cb1b6aa5d6e7609b30c99877e469db06b5d5016").unwrap();
        let delegate_pk = PublicKey::<Ed25519Bip32>::from_str("695b380fc72ae7d830d46f902a7c9d4057a4b9a7a0be235b87fdf51e698619e033aac8d93fd4cb82785973bb943f2047ddd1e664d4e185e7be634722e108389a").unwrap();
        let expected_cert = Signature::<(), Ed25519Bip32>::from_str("a72bf0119afd1ba5bed56b6521544105b6077c884609666296dbc59275477149a1b8230ce5b6c0fa81e1ec61c717164be57422e86a8f2f5773cdc66da99fcc0e").unwrap();

        let psk = ProxySecretKey::sign(&issuer_prv, delegate_pk, 0, pm);

        assert_eq!(psk.cert, expected_cert);

        assert!(psk.verify(pm));
    }
}
