extern crate reqwest;
extern crate failure;

use self::reqwest::header::{Headers, ContentType};
use self::reqwest::Url;
use self::failure::Error;
use std::borrow::Borrow;

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
    pub fn new(host: &String, version: Version) -> Client {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        headers.set_raw("API-Version", match version {
            Version::One => "1",
            Version::Two => "2",
        });

        Client {
            host: host.to_owned(),
            client: reqwest::Client::new(),
            header: headers
        }
    }

    pub fn get(self, endpoint: &str) -> Result<String, Error> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.get_url(&url)
    }

    pub fn get_with_params<I, K, V>(self, endpoint: &str, iter: I) -> Result<String, Error>
        where I: IntoIterator,
                     I::Item: Borrow<(K, V)>,
                     K: AsRef<str>,
                     V: AsRef<str>
    {
        let url = Url::parse_with_params(&format!("{}{}", self.host, endpoint), iter)?;
        self.get_url(&url)
    }

    fn get_url(self, url: &Url) -> Result<String, Error> {
        println!("{:?}", url);
        let response = self.client
            .get(url.as_str())
            .headers(self.header)
            .send()?
            .text()?;

        Ok(response)
    }
}
