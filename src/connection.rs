use api::Api;
use api::one::One;
use api::two::Two;
use http::client::Client;
use std::ops::Deref;

#[allow(dead_code)]
pub struct Connection<T> where T: Api + ?Sized {
    client: Client,
    pub api: T
}

impl<T: Api + ?Sized> Connection<T> {
    pub fn one(host: &str) -> Connection<One> {
        let mut client = Client::new(host);
        let one = One::new(&mut client);
        Connection { client, api: one }
    }

    pub fn two(host: &str) -> Connection<Two> {
        let mut client = Client::new(host);
        let two = Two::new(&mut client);
        Connection { client, api: two }
    }
}

impl<T: Api> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.api
    }
}
