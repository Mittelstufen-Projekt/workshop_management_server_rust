/*

    Author: Justin
    Description: This file contains the tests for the repository functions.
    How to run: cargo test
    Important: All the code will be optimized away with --release flag

*/

#[cfg(test)]
mod tests {
    use crate::models::client::Client;
    use crate::models::material::Material;
    use crate::models::material_type::MaterialType;
    use crate::models::project::Project;
    use crate::models::project_material::ProjectMaterial;
    use crate::utils::keycloak::Keycloak;
    use crate::utils::repository::Repository;

    /*

        Add a function to keycloak to get a token (Usually only needed in frontend)

    */
    impl<'a> Keycloak<'a> {
        pub async fn login_user(
            &mut self,
            username: &str,
            password: &str,
        ) -> Result<String, reqwest::Error> {
            // Send a request to keycloak to get a token
            let response = reqwest::Client::new()
                .post(&format!(
                    "{}/realms/{}/protocol/openid-connect/token",
                    &self.api_url, &self.realm
                ))
                .form(&[
                    ("client_id", &self.client_id),
                    ("client_secret", &self.client_secret),
                    ("username", &username),
                    ("password", &password),
                    ("grant_type", &"password"),
                ])
                .timeout(std::time::Duration::from_secs(5))
                .send()
                .await?;
            // Check if the request was successful
            match response.error_for_status_ref().err() {
                Some(err) => Err(err),
                None => {
                    let token: serde_json::Value = response.json().await?;
                    Ok(token["access_token"].as_str().unwrap().to_string())
                }
            }
        }
    }

    /*

       Test Values for all data types

    */

    impl Client {
        pub fn test_data() -> Client {
            Client {
                id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                phone: "123456789".to_string(),
            }
        }

        pub fn test_update_data() -> Client {
            Client {
                id: 1,
                firstname: "Jane".to_string(),
                lastname: "Doe".to_string(),
                phone: "987654321".to_string(),
            }
        }
    }

    impl MaterialType {
        pub fn test_data() -> MaterialType {
            MaterialType {
                id: 1,
                name: "Wood".to_string(),
                description: "A material that comes from trees.".to_string(),
            }
        }

        pub fn test_update_data() -> MaterialType {
            MaterialType {
                id: 1,
                name: "Metal".to_string(),
                description: "A material that comes from the ground.".to_string(),
            }
        }
    }

    impl Material {
        pub fn test_data() -> Material {
            Material {
                id: 1,
                name: "Wood".to_string(),
                description: "A material that comes from trees.".to_string(),
                type_id: 1,
                amount: 100,
                costs: 10.0,
                threshold_value: 50,
            }
        }

        pub fn test_update_data() -> Material {
            Material {
                id: 1,
                name: "Metal".to_string(),
                description: "A material that comes from the ground.".to_string(),
                type_id: 1,
                amount: 100,
                costs: 10.0,
                threshold_value: 50,
            }
        }
    }

    impl ProjectMaterial {
        pub fn test_data() -> ProjectMaterial {
            ProjectMaterial {
                id: 1,
                project_id: 1,
                material_id: 1,
                amount: 10,
            }
        }

        pub fn test_update_data() -> ProjectMaterial {
            ProjectMaterial {
                id: 1,
                project_id: 1,
                material_id: 1,
                amount: 20,
            }
        }
    }

    impl Project {
        pub fn test_data() -> Project {
            Project {
                id: 1,
                name: "Table".to_string(),
                client_id: 1,
                description: "A table made out of wood.".to_string(),
                startpoint: 1000000000.0,
                endpoint: 1000000000.0,
                estimated_costs: 100.0,
                estimated_hours: 10.0,
                costs: 100.0,
            }
        }

        pub fn test_update_data() -> Project {
            Project {
                id: 1,
                name: "Chair".to_string(),
                client_id: 1,
                description: "A chair made out of wood.".to_string(),
                startpoint: 1000000000.0,
                endpoint: 1000000000.0,
                estimated_costs: 100.0,
                estimated_hours: 10.0,
                costs: 100.0,
            }
        }
    }

    /*

       Test Functions

    */

    // Testing add methods (Need to do it in one function because of the specific order and async would randomize it)
    // Only commented the first one because the other test are mostly the same but with different function calls
    #[actix_rt::test]
    async fn test_add_values() {
        // Create a new repository and keycloak instance
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        // Get a token from keycloak
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        // Create test data
        let client = Client::test_data();
        let material_type = MaterialType::test_data();
        let material = Material::test_data();
        let project = Project::test_data();
        let project_material = ProjectMaterial::test_data();
        // Add the test data to the repository
        let result = repository.add_client(client, &token).await;
        // Check if the result is ok
        assert!(result.is_ok());
        let result = repository.add_material_type(material_type, &token).await;
        assert!(result.is_ok());
        let result = repository.add_material(material, &token).await;
        assert!(result.is_ok());
        let result = repository.add_project(project, &token).await;
        assert!(result.is_ok());
        let result = repository
            .add_project_material(project_material, &token)
            .await;
        assert!(result.is_ok());
    }

    // Testing get all methods
    #[actix_rt::test]
    async fn test_get_all_projects() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_projects(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_clients() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_clients(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_materials() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_materials(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_material_types() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_material_types(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_project_materials() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_project_materials(&token).await;
        assert!(result.is_ok());
    }

    // Testing single get methods
    #[actix_rt::test]
    async fn test_get_project() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_project(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_client() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_client(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_material(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_material_type(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_project_material(1, &token).await;
        assert!(result.is_ok());
    }

    // Testing Update Methods
    #[actix_rt::test]
    async fn test_update_project() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let project = Project::test_update_data();
        let result = repository.update_project(project, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_client() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let client = Client::test_update_data();
        let result = repository.update_client(client, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let material = Material::test_update_data();
        let result = repository.update_material(material, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let material_type = MaterialType::test_update_data();
        let result = repository.update_material_type(material_type, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let project_material = ProjectMaterial::test_update_data();
        let result = repository
            .update_project_material(project_material, &token)
            .await;
        assert!(result.is_ok());
    }

    // Testing remove methods
    #[actix_rt::test]
    async fn test_remove_project() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_project(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_client() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_client(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_material(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_material_type(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_project_material(1, &token).await;
        assert!(result.is_ok());
    }
}
