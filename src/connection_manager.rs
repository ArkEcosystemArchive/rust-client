use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::boxed::Box;
use api::Api;

use connection::Connection;

pub struct ConnectionManager {
    connections: HashMap<String, Box<Connection<Api>>>,
    default_name: String,
}

impl ConnectionManager {
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
        self.connections.remove(name);
    }

    pub fn default_connection(&self) -> Option<&Box<Connection<Api>>> {
        self.connections.get(&self.get_default_connection())
    }

    pub fn connection(&self, name: &str) -> Option<&Box<Connection<Api>>> {
        self.connections.get(name)
    }

    pub fn get_default_connection(&self) -> String {
        if self.default_name.is_empty() {
            String::from("main")
        } else {
            self.default_name.to_owned()
        }
    }

    pub fn set_default_connection(&mut self, name: &str) {
        self.default_name = name.to_owned();
    }

    pub fn connections(&self) -> Values<String, Box<Connection<Api>>> {
        self.connections.values()
    }
}
