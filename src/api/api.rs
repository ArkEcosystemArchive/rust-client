use super::{Version};

pub trait Api {
    fn version() -> Version
    where
        Self: Sized;
}
