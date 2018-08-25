extern crate arkecosystem_client;
extern crate mockito;
extern crate serde_json;
extern crate failure;

use arkecosystem_client::api::one::One;
use mockito::{mock, Mock, Matcher};
use self::serde_json::{Value};

const MOCK_HOST: &'static str = "http://127.0.0.1:1234/api/";

pub fn mock_http_server(endpoint: &str) -> Mock {
    let url = Matcher::Regex(endpoint.to_owned());
    println!("{:?}", url);
    mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{\"success\": true}")
        .create()
}

pub fn mock_client_one() -> One {
    One::new(&MOCK_HOST.to_owned())
}

pub fn mock_assert_success(mock: &Mock, response: Result<String, failure::Error>) {
    mock.assert();
    assert!(response.is_ok());

    let v: Value = serde_json::from_str(&response.unwrap()).unwrap();
    assert!(v["success"] == true);
}
