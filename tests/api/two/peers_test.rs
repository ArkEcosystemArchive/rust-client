use serde_json::ser::to_string_pretty;
use test_helper::{mock_client_two, mock_http_request_two};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("peers");
    {
        let client = mock_client_two();
        let response = client.peers.all(Vec::<(String, String)>::new()).unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("peers/dummy");
    // {
    //     let client = mock_client_two();
    //     let response = client.blocks.show("dummy".to_owned());
    //
    //     //mock_assert_success_two(&_mock, "peers/dummy", response);
    // }
}
