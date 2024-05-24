/*

    Author: Justin Kosten
    Description: This file contains the model for the Error object. This object is used to represent an error in the actix-web server.

*/

use serde::{Deserialize, Serialize};

// Define the Error struct
#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub code: i32,
}

// Implement new for the Error struct so that we can create a new error
impl Error {
    pub fn new(message: String, code: i32) -> Error {
        Error {
            message,
            code
        }
    }
}

// Implement the Display trait for the Error struct so that we can print the error message
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

// Implement the Default behavior for the Error struct
impl Default for Error {
    fn default() -> Error {
        Error {
            message: "An error occurred".to_string(),
            code: 500
        }
    }
}