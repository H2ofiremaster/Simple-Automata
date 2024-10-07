use std::marker::PhantomData;

use rand::Rng;
use serde::{Deserialize, Serialize};

pub trait Identifiable: Sized {
    fn id(&self) -> UniqueId<Self>;
}

#[derive(Debug, Serialize, Deserialize)]
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
