/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::error::Error;

// Constants
const API_URL: &str = "http://justinrauch.myftp.org:8480";
const REALM: &str = "WMS";
const CLIENT_ID: &str = "workshop_client";
const CLIENT_SECRET: &str = "Ip7GUqM8mRuIHMcq3tOuuHCaejSwSk3S";

// Give keycloak the 'a so that it will hold on the constans for the lifetime of the main struct
#[derive(Clone)]
pub struct Keycloak<'a> {
    last_request: u64,
    number_of_frequent_requests: u64,
    // Constants (make public, cannot be changed anyways)
    pub api_url: &'a str,
    pub realm: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

// Implement fuctions to the Keycloak struct
impl<'a> Keycloak<'a> {
    // Default constructor
    pub const fn new() -> Keycloak<'a> {
        Keycloak {
            last_request: 0,
            number_of_frequent_requests: 0,
            api_url: API_URL,
            realm: REALM,
            client_id: CLIENT_ID,
            client_secret: CLIENT_SECRET,
        }
    }

    // Function to get the groups of a user
    pub async fn get_groups(&mut self, token: String) -> Result<Vec<String>, Error> {
        // Check if the last request was less than 3 seconds ago
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if current_time - self.last_request < 3 {
            self.number_of_frequent_requests += 1;
            if self.number_of_frequent_requests > 10 {
                // Too many requests
                return Err(Error::new("Too many requests".to_string(), 429));
            }
        } else {
            // Reset the number of frequent requests
            self.last_request = current_time;
            self.number_of_frequent_requests = 0;
        }
        // Send the request to keycloak
        let response = reqwest::Client::new()
            .post(&format!(
                "{}/realms/{}/protocol/openid-connect/token/introspect",
                &self.api_url, &self.realm
            ))
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("token", &token.as_ref()),
            ])
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await;
        // Check if the request was successful
        let response = match response {
            Ok(response) => response,
            Err(_) => return Err(Error::new("Keycloak server error".to_string(), 500)),
        };
        // Check if keycloak maybe returned an access error instead of an HTML error
        match response.error_for_status_ref().err() {
            Some(err) => Err(Error::new(err.to_string(), 500)),
            None => {
                // Parse the response
                let json: serde_json::Value = response.json().await.unwrap();
                // Check if the token is valid (Contains groups?)
                let groups = match json["realm_access"]["roles"].as_array() {
                    None => return Err(Error::new("Invalid token".to_string(), 401)),
                    Some(groups) => groups,
                };
                // Return the groups
                let mut result = Vec::new();
                for group in groups {
                    result.push(group.as_str().unwrap().to_string());
                }
                Ok(result)
            }
        }
    }
}
