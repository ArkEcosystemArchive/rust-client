extern crate arkecosystem_client;

use arkecosystem_client::api::one::One;

#[test]
fn test_delegates_fee() {
    let one = One::new(String::from("https://node1.arknet.cloud/api/"));
    println!("{:?}", one.accounts.delegates_fee());
}
