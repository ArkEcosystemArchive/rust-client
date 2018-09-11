use *;
use serde_json::ser::to_string_pretty;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("transactions");
    {
        let client = mock_client_two();
        let response = client
            .transactions
            .all(Vec::<(String, String)>::new())
            .unwrap();
        let mut actual = to_string_pretty(&response).unwrap();
        actual.push('\n');

        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("transactions/dummy");
    {
        let client = mock_client_two();
        let response = client.transactions.show("dummy".to_owned()).unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show_unconfirmed() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request_two("transactions/unconfirmed/dummy");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.show_unconfirmed("dummy".to_owned());
    //
    //     //mock_assert_success_two(&_mock, "transactions/unconfirmed/dummy", response);
    // }
}

#[test]
fn test_all_unconfirmed() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request_two("transactions/unconfirmed");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.all_unconfirmed(Vec::<(String, String)>::new());
    //     //mock_assert_success_two(&_mock, "transactions/unconfirmed", response);
    // }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request_two("transactions/search");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.search(vec![("id", "dummy")]);
    //     //mock_assert_success_two(&_mock, "transactions/search", response);
    // }
}
