extern crate arkecosystem_client;

use arkecosystem_client::connection::Connection;
use arkecosystem_client::api::{Two};

const MOCK_HOST: &str = "http://127.0.0.1:1234/api/";

#[test]
fn test_connection() {
    let two = Connection::<Two>::new(MOCK_HOST);

    assert_eq!(two.client.host, MOCK_HOST);
}
