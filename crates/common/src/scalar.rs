use crate::prelude::AResult;
use anyhow::anyhow;

#[cfg(feature = "scalar")]
pub use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::CompressedRistretto,
    ristretto::RistrettoPoint,
    // self,
    scalar::Scalar,
};

#[cfg(feature = "scalar")]
pub type PubRing = Vec<RistrettoPoint>;

#[cfg(feature = "scalar")]
pub trait PublicKeyComputable {
    fn compute_pubkey(&self) -> RistrettoPoint;
}

#[cfg(feature = "scalar")]
impl PublicKeyComputable for Scalar {
    fn compute_pubkey(&self) -> RistrettoPoint {
        self * RISTRETTO_BASEPOINT_POINT
    }
}

#[cfg(feature = "scalar")]
pub trait LocalByteConvertible {
    fn to_bytes(&self) -> [u8; 32];
    fn from_bytes(bytes: &[u8]) -> AResult<Self>
    where
        Self: Sized;
    fn to_base58(&self) -> String;
    fn from_base58(input: String) -> AResult<Self>
    where
        Self: Sized;
}

#[cfg(feature = "scalar")]
impl LocalByteConvertible for RistrettoPoint {
    fn to_bytes(&self) -> [u8; 32] {
        self.compress().to_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> AResult<Self> {
        let compressed = CompressedRistretto::from_slice(bytes)
            .map_err(|_| anyhow!("Invalid bytes {bytes:?} length or format"))?;
        let point = compressed
            .decompress()
            .ok_or_else(|| anyhow!("Bytes {bytes:?} do not represent a valid Ristretto point"))?;
        Ok(point)
    }

    fn to_base58(&self) -> String {
        bs58::encode(self.to_bytes()).into_string()
    }

    fn from_base58(input: String) -> AResult<Self> {
        let bytes = bs58::decode(input).into_vec()?;

        Self::from_bytes(&bytes)
    }
}
