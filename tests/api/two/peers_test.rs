use *;
use serde_json::{from_str, Value};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("peers");
    {
        let client = mock_client_two();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let meta = actual.meta.unwrap();
        assert_eq!(
            meta.count,
            expected["meta"]["count"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.page_count,
            expected["meta"]["pageCount"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.total_count,
            expected["meta"]["totalCount"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.next.unwrap(),
            expected["meta"]["next"].as_str().unwrap()
        );
        assert_eq!(
            meta.previous.unwrap(),
            expected["meta"]["previous"].as_str().unwrap()
        );
        assert_eq!(
            meta.self_url,
            expected["meta"]["self"].as_str().unwrap()
        );
        assert_eq!(
            meta.first,
            expected["meta"]["first"].as_str().unwrap()
        );
        assert_eq!(
            meta.last.unwrap(),
            expected["meta"]["last"].as_str().unwrap()
        );

        assert_eq!(
            actual.data[0].ip,
            expected["data"][0]["ip"].as_str().unwrap()
        );
        assert_eq!(
            actual.data[0].port,
            expected["data"][0]["port"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data[0].version,
            expected["data"][0]["version"].as_str().unwrap()
        );
        assert_eq!(
            actual.data[0].height,
            expected["data"][0]["height"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data[0].status,
            expected["data"][0]["status"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data[0].os,
            expected["data"][0]["os"].as_str().unwrap()
        );
        assert_eq!(
            actual.data[0].latency,
            expected["data"][0]["latency"].as_u64().unwrap() as u32
        );
        assert_eq!(
            actual.data[0].hashid,
            expected["data"][0]["hashid"].as_str().unwrap()
        );
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
