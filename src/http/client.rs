use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Url};
use serde::de::{DeserializeOwned};
use serde_json::{from_str, to_string, Value, from_value};
use std::borrow::Borrow;
use utils;
use error::Error;

#[derive(Clone, Debug)]
pub struct Client {
    pub host: String,
    client: ::reqwest::Client,
    headers: HeaderMap,
}

impl Client {
    pub fn new(host: &str) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        Client {
            host: host.to_owned(),
            client: ::reqwest::Client::new(),
            headers,
        }
    }

    pub fn set_version(&mut self, version: &'static str) {
        self.headers
            .insert("API-Version", HeaderValue::from_static(version));
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_get(&url)
    }

    pub fn get_with_params<T, I, K, V>(
        &self,
        endpoint: &str,
        parameters: I,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters)?;
        self.internal_get(&url)
    }

    pub fn post<T, I, K, V>(&self, endpoint: &str, payload: Option<I>) -> Result<T, Error>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_post(&url, payload)
    }

    pub fn post_with_params<T, I, K, V>(
        &self,
        endpoint: &str,
        payload: Option<I>,
        parameters: I,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters)?;
        self.internal_post(&url, payload)
    }

    fn internal_get<T: DeserializeOwned>(&self, url: &Url) -> Result<T, Error> {
        let builder = self.client.get(url.as_str());
        self.send(builder).map(|v| from_value(v).unwrap())
    }

    fn internal_post<T, I, K, V>(&self, url: &Url, payload: Option<I>) -> Result<T, Error>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let builder = self.client.post(url.as_str());

        let body = if payload.is_some() {
            let map = utils::to_map(payload.unwrap());
            to_string(&map)?
        } else {
            String::new()
        };

        self.send(builder.body(body)).map(|v| from_value(v).unwrap())
    }

    fn send(&self, builder: RequestBuilder) -> Result<Value, Error> {
        let response = builder.headers(self.headers.clone()).send()?.text()?;
        Ok(from_str(&response)?)
    }
}
