use std::{fmt::Debug, fmt::Display, marker::PhantomData};

use rand::Rng;
use serde::Serialize;

pub trait Identifiable: Sized {
    fn id(&self) -> UniqueId<Self>;
}

pub struct UniqueId<T: Identifiable>(u32, PhantomData<T>);
impl<T: Identifiable> UniqueId<T> {
    pub fn new(current: &[T]) -> Self {
        assert!(
            current.len() < u32::MAX as usize,
            "{} id list should not exceed u32::MAX. (got {})",
            std::any::type_name::<T>(),
            current.len()
        );
        let mut random = rand::thread_rng();
        loop {
            let candidate = Self(random.gen(), PhantomData);
            if !current.iter().any(|m| m.id() == candidate) {
                return candidate;
            }
        }
    }
    pub const fn new_unchecked(id: u32) -> Self {
        Self(id, PhantomData)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}
impl<T: Identifiable> Debug for UniqueId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UniqueId<{}>({})",
            std::any::type_name::<T>()
                .split("::")
                .last()
                .unwrap_or(std::any::type_name::<T>()),
            self.0
        )
    }
}
impl<T: Identifiable> Display for UniqueId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: Identifiable> PartialEq for UniqueId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Identifiable> Eq for UniqueId<T> {}
impl<T: Identifiable> Clone for UniqueId<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Identifiable> Copy for UniqueId<T> {}
impl<T: Identifiable> Serialize for UniqueId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}
