use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::utils::Uuid;

pub trait HashToUuidExt: Hash {
    fn to_uuid(&self) -> Uuid {
        let mut default_hasher = DefaultHasher::new();
        self.hash(&mut default_hasher);
        let hash = default_hasher.finish();
        Uuid::from_u128(hash as u128)
    }
}