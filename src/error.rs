use reqwest;
use serde_json;
use std::fmt;
use std::error;
use api::models::RequestError;

#[derive(Debug)]
pub enum Error {
    Api(RequestError), // node response for statusCode != 200
    ReqwestHttp(reqwest::Error),
    ReqwestUrl(reqwest::UrlError),
    Serde(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Api(ref err) => write!(f, "{:?}", err),
            Error::ReqwestHttp(ref err) => write!(f, "Reqwest Http Error: {}", err),
            Error::ReqwestUrl(ref err) => write!(f, "Reqwest Url Error: {}", err),
            Error::Serde(ref err) => write!(f, "Serde Error: {}", err)
        }
    }
}

impl error::Error for Error {

    fn description(&self) -> &str {
        match *self {
            Error::Api(ref _err) => "API request error.",
            Error::ReqwestHttp(ref err) => err.description(),
            Error::ReqwestUrl(ref err) => err.description(),
            Error::Serde(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Api(ref _err) => None,
            Error::ReqwestHttp(ref err) => Some(err),
            Error::ReqwestUrl(ref err) => Some(err),
            Error::Serde(ref err) => Some(err)
        }
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Api(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestHttp(err)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Error {
        Error::ReqwestUrl(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}