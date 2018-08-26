extern crate arkecosystem_client;

use arkecosystem_client::connection::Connection;
use arkecosystem_client::api::one::One;
use arkecosystem_client::api::two::Two;

const MOCK_HOST: &'static str = "http://127.0.0.1:1234/api/";

#[test]
fn test_connection() {
    let one = Connection::<One>::new(MOCK_HOST);
    let two = Connection::<Two>::new(MOCK_HOST);

    assert_eq!(one.client.host, MOCK_HOST);
    assert_eq!(two.client.host, MOCK_HOST);
}
