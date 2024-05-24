/*

    Author: Justin Kosten
    Description: This file contains the model for the Message object. This object is used to represent a message in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
}

impl Client {
    pub fn test_data() -> Client {
        Client {
            id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            phone: "123-456-7890".to_string(),
        }
    }

    pub fn test_update_data() -> Client {
        Client {
            id: 1,
            firstname: "Jane".to_string(),
            lastname: "Doe".to_string(),
            phone: "123-456-7890".to_string(),
        }
    }
}