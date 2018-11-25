#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    One,
    Two,
}

impl Version {
    pub fn to_string(self) -> &'static str {
        match self {
            Version::One => "1",
            Version::Two => "2"
        }
    }
}

pub trait Api {
    fn version() -> Version
    where
        Self: Sized;
}
