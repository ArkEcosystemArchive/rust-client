#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    One,
    Two
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub host: String,
    pub version: Version
}

impl Connection {

    pub fn new(host: String, version: Version) -> Connection {
        Connection {
            host, version
        }
    }

    pub fn one(host: String) -> Connection {
        Connection::new(host, Version::One)
    }

    pub fn two(host: String) -> Connection {
        Connection::new(host, Version::Two)
    }

}
