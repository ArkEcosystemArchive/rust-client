use api::Api;
use api::one::One;
use api::two::Two;
use http::client::Client;
use std::ops::Deref;

pub struct Connection<T> where T: Api + ?Sized {
    pub client: Client,
    api: T
}


impl Connection<One> {
    pub fn new(host: &str) -> Connection<One> {
        let mut client = Client::new(host);
        let one = One::new(&mut client);
        Connection { client, api: one }
    }
}

impl Connection<Two> {
    pub fn new(host: &str) -> Connection<Two> {
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
