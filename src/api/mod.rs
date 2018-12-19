pub mod version;
pub mod two;

pub use self::version::{Version};
pub use self::two::Two;

pub trait Api {
    fn version() -> Version
    where
        Self: Sized;
}
