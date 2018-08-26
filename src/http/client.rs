use failure;
use std::borrow::Borrow;
use reqwest::header::{Headers, ContentType};
use reqwest::{RequestBuilder, Url};
use serde_json::{from_str, to_string, Value};
use api::Version;
use utils;

#[derive(Clone, Debug)]
pub struct Client {
    host: String,
    client: ::reqwest::Client,
    headers: Headers,
}

impl Client {
    pub fn new(host: &str) -> Client {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        Client {
            host: host.to_owned(),
            client: ::reqwest::Client::new(),
            headers: headers
        }
    }

    pub fn set_version(&mut self, version: Version) {
        self.headers.set_raw("API-Version", match version {
            Version::One => "1",
            Version::Two => "2",
        });
    }

    pub fn get(&self, endpoint: &str) -> Result<Value, failure::Error> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_get(&url)
    }

    pub fn get_with_params<I, K, V>(&self, endpoint: &str, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
                     I::Item: Borrow<(K, V)>,
                     K: AsRef<str>,
                     V: AsRef<str>
    {
        let url = Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters)?;
        self.internal_get(&url)
    }

    pub fn post<I, K, V>(&self, endpoint: &str, payload: Option<I>) -> Result<Value, failure::Error>
        where I: IntoIterator,
                     I::Item: Borrow<(K, V)>,
                     K: AsRef<str>,
                     V: AsRef<str>
    {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_post(&url, payload)
    }


    fn internal_get(&self, url: &Url) -> Result<Value, failure::Error> {
        let mut builder = self.client.get(url.as_str());
        self.send(&mut builder)
    }

    fn internal_post<I, K, V>(&self, url: &Url, payload: Option<I>) -> Result<Value, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let mut builder = self.client
            .post(url.as_str());

        let mut body = String::new();
        if payload.is_some() {
            let map = utils::to_map(payload.unwrap());
            body = to_string(&map)?;
        }

        self.send(builder.body(body))
    }

    fn send(&self, builder: &mut RequestBuilder) -> Result<Value, failure::Error> {
        let response = builder.headers(self.headers.clone())
            .send()?
            .text()?;

        Ok(from_str(&response)?)
    }
}