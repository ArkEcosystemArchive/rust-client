use *;
use serde_json::ser::to_string_pretty;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("votes");
    {
        let client = mock_client_two();
        let response = client.votes.all().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
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
    //     //mock_assert_success_two(&_mock, "votes/dummy", response);
    // }
}
