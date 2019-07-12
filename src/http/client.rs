use api::models::{RequestError, Response};
use api::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::{from_str, from_value, to_string};
use std::borrow::Borrow;
use std::collections::HashMap;

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

    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_get(&url)
    }

    pub fn get_with_params<T, I, K, V>(&self, endpoint: &str, parameters: I) -> Result<T>
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

    pub fn post<T, V>(&self, endpoint: &str, payload: Option<HashMap<&str, V>>) -> Result<T>
    where
        T: DeserializeOwned,
        V: Serialize,
    {
        let url = Url::parse(&format!("{}{}", self.host, endpoint))?;
        self.internal_post(&url, payload)
    }

    pub fn post_with_params<T, H, I, K, V>(
        &self,
        endpoint: &str,
        payload: Option<HashMap<&str, H>>,
        parameters: I,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        H: Serialize,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters)?;
        self.internal_post(&url, payload)
    }

    fn internal_get<T: DeserializeOwned>(&self, url: &Url) -> Result<T> {
        let builder = self.client.get(url.as_str());
        self.send(builder)
    }

    fn internal_post<T, V>(&self, url: &Url, payload: Option<HashMap<&str, V>>) -> Result<T>
    where
        T: DeserializeOwned,
        V: Serialize,
    {
        let builder = self.client.post(url.as_str());

        let body = payload.map_or_else(|| Ok(String::default()), |v| to_string(&v));

        self.send(builder.body(body?))
    }

    fn send<T: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<T> {
        let response = builder.headers(self.headers.clone()).send()?.text()?;
        let parsed = from_str::<Value>(&response)?;

        if parsed.is_object() && parsed.as_object().unwrap().contains_key("statusCode") {
            let request_error = from_value::<RequestError>(parsed)?;
            Err(request_error.into())
        } else {
            match from_value::<Response<T>>(parsed) {
                Ok(response) => Ok(response),
                Err(err) => Err(err.into()),
            }
        }
    }
}
