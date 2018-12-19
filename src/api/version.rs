#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    Two,
}

impl Version {
    pub fn to_string(&self) -> &'static str {
        match self {
            Version::Two => "2"
        }
    }
}
