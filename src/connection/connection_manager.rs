use api::Api;
use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Values;
use super::connection::Connection;

pub struct ConnectionManager<'a> {
    connections: HashMap<String, &'a Any>,
    default_connection: String,
}

impl<'a> ConnectionManager<'a> {
    pub fn new() -> ConnectionManager<'a> {
        ConnectionManager {
            connections: HashMap::<String, &'a Any>::new(),
            default_connection: String::from("main"),
        }
    }

    pub fn connect<T: Any + Api + 'static>(&mut self, connection: &'a Connection<T>) -> Result<(), &str> {
        let default_connection = &self.get_default_connection();
        self.connect_as(connection, default_connection)
    }

    pub fn connect_as<T: Any + Api + 'static> (
        &mut self,
        connection: &'a Connection<T>,
        name: &str,
    ) -> Result<(), &str> {
        if self.connections.contains_key(name) {
            return Err("Connection already exists.");
        }

        self.connections
            .insert(name.to_owned(), connection);
        Ok(())
    }

    pub fn disconnect(&mut self, name: &str) {
        if name.is_empty() {
            self.connections.remove(&self.default_connection);
        } else {
            self.connections.remove(name);
        }
    }

    pub fn connection<T: Any + Api + 'static>(&self) -> Option<&'a Connection<T>> {
        let connection_name = self.get_default_connection();
        if let Some(conn) = self.connections.get(&connection_name) {
            return conn.downcast_ref()
        }

        None
    }

    pub fn connection_by_name<T: Any + Api + 'static>(&self, name: &str) -> Option<&'a Connection<T>> {
        if let Some(conn) = self.connections.get(name) {
            return conn.downcast_ref()
        }

        None
    }

    pub fn get_default_connection(&self) -> String {
        self.default_connection.to_owned()
    }

    pub fn set_default_connection(&mut self, name: &str) {
        self.default_connection = name.to_owned();
    }

    pub fn connections(&self) -> Values<String, &'a Any> {
        self.connections.values()
    }
}
