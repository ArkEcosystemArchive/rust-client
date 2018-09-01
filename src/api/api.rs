#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    One,
    Two,
}

pub trait Api {
    fn version() -> Version
    where
        Self: Sized;
}
