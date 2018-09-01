use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::boxed::Box;
use api::Api;

use connection::Connection;

pub struct ConnectionManager {
    connections: HashMap<String, Box<Connection<Api>>>,
    default_connection: String,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            connections: HashMap::<String, Box<Connection<Api>>>::new(),
            default_connection: String::from("main"),
        }
    }

    pub fn connect<T: Api + 'static>(&mut self, connection: Connection<T>) -> Result<(), &str> {
        let default_connection = &self.get_default_connection();
        self.connect_as(connection, default_connection)
    }

    pub fn connect_as<T: Api + 'static>(
        &mut self,
        connection: Connection<T>,
        name: &str,
    ) -> Result<(), &str> {
        if self.connections.contains_key(name) {
            return Err("Connection already exists.");
        }

        self.connections
            .insert(name.to_owned(), Box::new(connection));
        Ok(())
    }

    pub fn disconnect(&mut self, name: &str) {
        if name.is_empty() {
            self.connections.remove(&self.default_connection);
        } else {
            self.connections.remove(name);
        }
    }

    // TODO: return T
    pub fn connection_default(&self) -> Option<&Box<Connection<Api>>> {
        self.connections.get(&self.get_default_connection())
    }

    // TODO: return T
    pub fn connection(&self, name: &str) -> Option<&Box<Connection<Api>>> {
        self.connections.get(name)
    }

    pub fn get_default_connection(&self) -> String {
        self.default_connection.to_owned()
    }

    pub fn set_default_connection(&mut self, name: &str) {
        self.default_connection = name.to_owned();
    }

    pub fn connections(&self) -> Values<String, Box<Connection<Api>>> {
        self.connections.values()
    }
}
