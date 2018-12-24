extern crate arkecosystem_client;

use arkecosystem_client::connection::Connection;

const MOCK_HOST: &str = "http://127.0.0.1:1234/api/";

#[test]
fn test_connection() {
    let connection = Connection::new(MOCK_HOST);

    assert_eq!(connection.client.host, MOCK_HOST);
}
