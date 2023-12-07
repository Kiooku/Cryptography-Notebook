use crate::communication;
use std::collections::HashMap;
use communication::key_collection::ServerKeyCollection;
use std::fmt;

use super::message::Message;

#[derive(Debug)]
pub enum ServerError {
    UserDoesNotExist,
}

pub struct Server {
    users: HashMap<String, (ServerKeyCollection, Vec<Message>)>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            users: HashMap::new()
        }
    }

    pub fn add_user(&mut self, username: String, keys: ServerKeyCollection) -> () {
        self.users.insert(username.to_string(), (keys, Vec::new()));
    }

    pub fn add_message_to(&mut self, username: &String, message: Message) -> Result<(), ServerError> {
        let user_information = self.users.get_mut(username).ok_or(ServerError::UserDoesNotExist)?;
        user_information.1.push(message);
        Ok(())
    }

    pub fn get_user_keys(&self, username: &String) -> Result<&ServerKeyCollection, ServerError> {
        let user_information = self.users.get(username).ok_or(ServerError::UserDoesNotExist)?;
        Ok(&user_information.0)
    }

    pub fn get_user_messages(&mut self, username: &String) -> Result<Vec<Message>, ServerError> {
        let user_information = self.users.get_mut(username).ok_or(ServerError::UserDoesNotExist)?;
        Ok(user_information.1.drain(..).collect::<Vec<Message>>())
    }

    pub fn get_users(&self, requester_username: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for username in self.users.keys() {
            if username != &requester_username {
                res.push(username.clone());
            }
        }
        res
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::UserDoesNotExist => write!(f, "User does not exist on the server"),
        }
    }
}