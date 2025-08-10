#[cfg(feature = "scalar")]
pub mod scalar;
pub mod http_client;

pub use toml;

pub use unic_langid::langid;

#[cfg(feature = "scalar")]
pub use scalar::*;

pub use color_eyre;
pub use color_eyre::Result as CResult;

pub use bs58;

cfg_if!{
    if #[cfg(target_arch = "wasm32")] {
    } else {
        pub use std::{
            fs::{
                self,
                File,
            },
            io::{
                self,
                Write,
            },
        };
    }
}
pub use csv::{
    self,
    WriterBuilder,
};
pub use std::fmt::Display;
pub use std::fmt::Debug;
pub use std::{
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    sync::Arc,
    env,
    path::{
        Path,
        PathBuf,
    },
};
pub type ElectionCloseStatus = Arc<AtomicBool>;
pub type RegistrationCloseStatus = Arc<AtomicBool>;
pub use std::convert::AsRef;
pub use strum_macros::{
    self,
    AsRefStr,
};
pub use hex;
pub use dashmap::DashMap;
pub use passwords;
pub use derive_builder::Builder;
pub use serde_json;
pub use serde::{
    self,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
pub use derive_more::{
    self,
    derive,
    // Display,
};

pub use nazgul::{
    blsag::BLSAG,
    traits::{
        Verify,
        Link,
        Sign,
    },
};
pub use sha3::{
    Keccak512,
    Digest,
    Sha3_256,
    // Sha3_512,
};


pub use futures::{
    self,
    StreamExt,
};

pub use thiserror;

pub use uuid::{
    self,
    Uuid,
};

pub use chrono::{
    self,
    Utc,
};

pub use tracing::info;



pub use cfg_if::cfg_if;

pub use anyhow::bail;
pub use anyhow;
pub use anyhow::anyhow as aerr;
pub use anyhow::Error;
// pub use anyhow::Context;
pub type AResult<T> = anyhow::Result<T>;
#[derive(Debug, derive::Display, derive::Error, derive::From, derive::FromStr)]
#[display("anyhow::Error: {msg}")]
pub struct AnyErr {
    #[from]
    msg: String
}

impl From<&str> for AnyErr {
    fn from(msg: &str) -> AnyErr {
        Self {
            msg: msg.to_string()
        }
    }
}

pub fn msg<M>(message: M) -> anyhow::Error
where
    M: Display + Debug + Send + Sync + 'static,
{
    anyhow::Error::msg(message)
}

pub trait IntoAnyhowError<E>: core::error::Error {
    fn ae(&self) -> anyhow::Error {
        msg(format!("Core error: {:?}", self))
    }
}



pub mod base58 {
    use super::*;

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let res = bs58::encode(bytes)
            .into_string();

        serializer.serialize_str(&res)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let req: String = serde::Deserialize::deserialize(deserializer)?;
        let res  = bs58::decode(req)
            .into_vec()
            .map_err(|err| serde::de::Error::custom(err.to_string()))?;

        Ok(res)
    }
}

// pub use base64::engine::general_purpose::GeneralPurpose::encode as bs64encode;
pub use base64::engine::general_purpose::STANDARD as BS64ENGINE;
pub use base64::Engine as Base64Engine;
use qrcode::QrCode;
use image::Luma;
use image::ImageFormat;
use std::io::Cursor;

pub fn data_to_qr_png(data: &[u8]) -> AResult<String> {
    let code = QrCode::new(data)?;

    // Render the bits into an image.
    let luma_image = code.render::<Luma<u8>>().build();
    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    luma_image.write_to(&mut cursor, ImageFormat::Png)?;
    // let base64_encoded = bs64encode(png_bytes);
    let base64_encoded = BS64ENGINE.encode(png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_encoded))
}
