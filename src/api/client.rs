extern crate reqwest;
extern crate failure;

use self::reqwest::header::{Headers, ContentType};
use self::reqwest::Url;
use self::failure::Error;

#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    One,
    Two
}

#[derive(Clone)]
pub struct Client {
    host: String,
    client: reqwest::Client,
    header: reqwest::header::Headers,
}

impl Client {
    pub fn new(host: String, version: Version) -> Client {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        headers.set_raw("API-Version", match version {
            Version::One => "1",
            Version::Two => "2",
        });

        Client {
            host,
            client: reqwest::Client::new(),
            header: headers
        }
    }

    pub fn get(self, endpoint: &str) -> Result<String, Error> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        println!("{:?}", url);

        let response = self.client
            .get(url.as_str())
            .headers(self.header)
            .send()?
            .text()?;

        Ok(response)
    }
}
