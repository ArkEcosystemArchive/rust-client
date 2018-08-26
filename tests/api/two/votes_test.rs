extern crate arkecosystem_client;

use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_all() {
    let _mock = mock_http_request_two("votes");
    {
        let client = mock_client_two();
        let response = client.votes.all(Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "votes", response);
    }
}

#[test]
fn test_show() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("votes/dummy");
    // {
    //     let client = mock_client_two();
    //     let response = client.votes.show("dummy".to_owned());
    //
    //     mock_assert_success_two(&_mock, "votes/dummy", response);
    // }
}
