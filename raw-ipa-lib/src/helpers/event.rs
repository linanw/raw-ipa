use crate::error::{Error, Res};
use crate::helpers::Helpers;
use crate::threshold::ThresholdDecryptionKey;
use rand::thread_rng;
use rust_elgamal::EncryptionKey;
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "enable-serde")]
use std::fs;
use std::ops::{Deref, DerefMut};
#[cfg(feature = "enable-serde")]
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum Role {
    Source,
    Trigger,
}

/// All of the public information about an event helper.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct PublicHelper {
    role: Role,
    matchkey_encrypt: EncryptionKey,
}

impl PublicHelper {
    #[must_use]
    pub fn matchkey_encryption_key(&self) -> EncryptionKey {
        self.matchkey_encrypt
    }
}

/// A source or trigger event helper.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Helper {
    #[cfg_attr(feature = "enable-serde", serde(flatten))]
    public: PublicHelper,

    matchkey_decrypt: ThresholdDecryptionKey,
}

impl Helper {
    #[must_use]
    pub fn new(role: Role) -> Self {
        let matchkey_decrypt = ThresholdDecryptionKey::new(&mut thread_rng());
        Self {
            public: PublicHelper {
                role,
                matchkey_encrypt: matchkey_decrypt.encryption_key(),
            },
            matchkey_decrypt,
        }
    }

    /// # Errors
    /// Missing or badly formatted files.
    #[cfg(feature = "enable-serde")]
    pub fn load(dir: &Path, role: Role) -> Res<Self> {
        let s = fs::read_to_string(&Helpers::filename(dir, false))?;
        let v: Self = serde_json::from_str(&s)?;
        if role != v.public.role {
            return Err(Error::InvalidRole);
        }
        Ok(v)
    }

    /// # Errors
    /// Unable to write files.
    #[cfg(feature = "enable-serde")]
    pub fn save(&self, dir: &Path) -> Res<()> {
        let f = Helpers::filename(dir, true);
        fs::write(f, serde_json::to_string(&self.public)?.as_bytes())?;
        let f = Helpers::filename(dir, false);
        fs::write(f, serde_json::to_string(&self)?.as_bytes())?;
        Ok(())
    }
}

impl Deref for Helper {
    type Target = PublicHelper;
    fn deref(&self) -> &Self::Target {
        &self.public
    }
}

impl DerefMut for Helper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.public
    }
}
