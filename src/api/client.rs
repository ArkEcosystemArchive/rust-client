use api::endpoint;

pub struct Client {
    pub anything: endpoint::Endpoint,
}

impl Client {
    pub fn new() -> Client {
        Client {
            anything: endpoint::Endpoint::new(String::from("http://91.134.115.30:4002/api/transactions")),
        }
    }
}
