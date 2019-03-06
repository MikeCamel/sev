use codicon::{Decoder, Encoder};
use std::num::NonZeroU128;
use super::super::*;

#[test]
fn v1() {
    let bytes = include_bytes!("ark.cert");

    let ark = Certificate::decode(&mut &bytes[..], Kind::Ca).unwrap();
    assert_eq!(ark, Certificate {
        version: 1,
        firmware: None,
        key: PublicKey {
            usage: Usage::AmdRootKey,
            algo: SigAlgo::RsaSha256.into(),
            key: Key::Rsa(RsaKey {
                pubexp: bytes[0x040..][..256].to_vec(),
                modulus: bytes[0x140..][..256].to_vec(),
            }),
            id: NonZeroU128::new(122178821951678173525318614033703090459),
        },
        sigs: vec! {
            Signature {
                usage: Usage::AmdRootKey,
                algo: SigAlgo::RsaSha256,
                sig: bytes[0x240..][..256].to_vec(),
                id: NonZeroU128::new(122178821951678173525318614033703090459),
            }
        }
    });

    let mut output = Vec::new();
    ark.encode(&mut output, ()).unwrap();
    assert_eq!(bytes.len(), output.len());
    assert_eq!(bytes.to_vec(), output);
}