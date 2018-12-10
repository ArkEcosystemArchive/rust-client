use api::{Api, Two};
use http::client::Client;
use std::ops::Deref;

pub struct Connection<T>
where
    T: Api + ?Sized,
{
    pub client: Client,
    api: T,
}

impl Connection<Two> {
    pub fn new(host: &str) -> Connection<Two> {
        let mut client = Client::new(host);
        let two = Two::new_with_client(&mut client);
        Connection { client, api: two }
    }
}

impl<T: Api> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.api
    }
}
