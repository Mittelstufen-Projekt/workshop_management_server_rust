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

}

impl Keycloak {
    pub const fn new() -> Keycloak {
        Keycloak {

        }
    }

    // Only needed for tests to be able to verify the token
    pub async fn login_user(&mut self, username: &str, password: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/realms/{}/protocol/openid-connect/token", API_URL, REALM))
            .form(&[
                ("client_id", CLIENT_ID),
                ("client_secret", CLIENT_SECRET),
                ("username", username),
                ("password", password),
                ("grant_type", "password"),
            ])
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        match response.error_for_status_ref().err() {
            Some(err) => Err(err),
            None => {
                let token: serde_json::Value = response.json().await?;
                Ok(token["access_token"].as_str().unwrap().to_string())
            }
        }
    }

    pub async fn get_groups(&mut self, token: String) -> Result<Vec<String>, Error> {
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
