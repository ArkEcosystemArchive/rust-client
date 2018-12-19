mod manager;
pub use self::manager::Manager;

use api::Api;
use http::client::Client;
use std::ops::Deref;

pub struct Connection {
    pub client: Client,
    api: Api,
}

impl Connection {
    pub fn new(host: &str) -> Connection {
        let client = Client::new(host);
        let api = Api::new_with_client(&client);
        Connection { client, api }
    }
}

impl Deref for Connection {
    type Target = Api;

    fn deref(&self) -> &Api {
        &self.api
    }
}
