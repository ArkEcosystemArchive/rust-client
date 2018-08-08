extern crate reqwest;

pub struct Endpoint {
    base_url: String
}

impl Endpoint {
    pub fn new(base_url: String) -> Endpoint {
        Endpoint {
            base_url
        }
    }

    pub fn all(&self) -> Result<String, reqwest::Error> {
        reqwest::get(&self.base_url)?.text()
    }
}
