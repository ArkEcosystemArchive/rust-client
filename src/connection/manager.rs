use crate::Connection;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct Manager {
    connections: HashMap<String, Rc<Connection>>,
    default_connection: String,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            connections: HashMap::new(),
            default_connection: String::from("main"),
        }
    }

    pub fn connect(&mut self, connection: Connection) -> Result<(), &str> {
        let default_connection = &self.get_default_connection();
        self.connect_as(connection, default_connection)
    }

    pub fn connect_as(&mut self, connection: Connection, name: &str) -> Result<(), &str> {
        if self.connections.contains_key(name) {
            return Err("Connection already exists.");
        }

        self.connections
            .insert(name.to_owned(), Rc::new(connection));
        Ok(())
    }

    pub fn disconnect(&mut self, name: &str) {
        if name.is_empty() {
            self.connections.remove(&self.default_connection);
        } else {
            self.connections.remove(name);
        }
    }

    pub fn connection(&self) -> Option<Rc<Connection>> {
        let connection_name = self.get_default_connection();
        if let Some(conn) = self.connections.get(&connection_name) {
            return Some(conn.clone());
        }

        None
    }

    pub fn connection_by_name(&self, name: &str) -> Option<Rc<Connection>> {
        if let Some(conn) = self.connections.get(name) {
            return Some(conn.clone());
        }

        None
    }

    pub fn get_default_connection(&self) -> String {
        self.default_connection.to_owned()
    }

    pub fn set_default_connection(&mut self, name: &str) {
        self.default_connection = name.to_owned();
    }

    pub fn connections(&self) -> Values<String, Rc<Connection>> {
        self.connections.values()
    }
}
