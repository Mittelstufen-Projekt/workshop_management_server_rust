/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::error::Error;

const API_URL: &str = "http://justinrauch.myftp.org:8480";
const REALM: &str = "WMS";
const CLIENT_ID: &str = "workshop_client";
const CLIENT_SECRET: &str = "Ip7GUqM8mRuIHMcq3tOuuHCaejSwSk3S";

#[derive(Clone)]
pub struct Keycloak {
    last_request: u64,
    number_of_frequent_requests: u64,
}

impl Keycloak {
    pub const fn new() -> Keycloak {
        Keycloak {
            last_request: 0,
            number_of_frequent_requests: 0,
        }
    }

    pub async fn get_groups(&mut self, token: String) -> Result<Vec<String>, Error> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if current_time - self.last_request < 3 {
            self.number_of_frequent_requests += 1;
            if self.number_of_frequent_requests > 10 {
                return Err(Error::new("Too many requests".to_string(), 429));
            }
        } else {
            self.last_request = current_time;
            self.number_of_frequent_requests = 0;
        }
        let response = reqwest::Client::new()
            .post(&format!(
                "{}/realms/{}/protocol/openid-connect/token/introspect",
                API_URL, REALM
            ))
            .form(&[
                ("client_id", CLIENT_ID),
                ("client_secret", CLIENT_SECRET),
                ("token", &token),
            ])
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await;
        let response = match response {
            Ok(response) => response,
            Err(_) => return Err(Error::new("Keycloak server error".to_string(), 500)),
        };
        match response.error_for_status_ref().err() {
            Some(err) => Err(Error::new(err.to_string(), 500)),
            None => {
                let json: serde_json::Value = response.json().await.unwrap();
                let groups = match json["realm_access"]["roles"].as_array() {
                    None => return Err(Error::new("Invalid token".to_string(), 401)),
                    Some(groups) => groups,
                };
                let mut result = Vec::new();
                for group in groups {
                    result.push(group.as_str().unwrap().to_string());
                }
                Ok(result)
            }
        }
    }
}
