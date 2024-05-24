/*

    Author: Justin Kosten
    Description: This file contains the repository for the actix-web server. It is responsible for handling all database interactions.

*/

use mysql::*;
use mysql::prelude::*;

use super::keycloak::Keycloak;
use crate::models::client::Client;
use crate::models::error::Error;
use crate::models::material::Material;
use crate::models::project::Project;
use crate::models::material_type::MaterialType;
use crate::models::project_material::ProjectMaterial;

const DB_URL: &str = "mysql://localdev:Jokerlll3@localhost:3306/workshopmanagement";

pub struct Repository {
    keycloak: Keycloak,
}

impl Repository {
    pub const fn new() -> Repository {
        Repository {
            keycloak: Keycloak::new(),
        }
    }

    pub fn connect(&self) -> Pool {
        let opts = Opts::from_url(DB_URL).unwrap();
        Pool::new(opts).unwrap()
    }

    pub async fn get_all_projects(&mut self, token: &String) -> Result<Vec<Project>, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let mut projects = Vec::new();
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "SELECT * FROM project";
        let result = conn.query_map(query, |(id, name, client_id, description, startpoint, endpoint, estimated_costs, estimated_hours, costs)| {
            Project {
                id,
                name,
                client_id,
                description,
                startpoint,
                endpoint,
                estimated_costs,
                estimated_hours,
                costs,
            }
        });
        match result {
            Ok(projects_res) => {
                for project in projects_res {
                    projects.push(project);
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Ok(projects)
    }

    pub async fn get_project(&mut self, id: i32, token: &String) -> Result<Project, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM project WHERE id = {}", id);
        let result = conn.query_map(query, |(id, name, client_id, description, startpoint, endpoint, estimated_costs, estimated_hours, costs)| {
            Project {
                id,
                name,
                client_id,
                description,
                startpoint,
                endpoint,
                estimated_costs,
                estimated_hours,
                costs,
            }
        });
        match result {
            Ok(mut projects) => {
                if projects.len() == 1 {
                    return Ok(projects.remove(0));
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Err(Error::new("Project not found".to_string(), 404))
    }

    pub async fn add_project(&mut self, project: Project, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "INSERT INTO project (id, name, client_id, description, startpoint, endpoint, estimated_costs, estimated_hours, costs) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = conn.exec_drop(query, (project.id, project.name, project.client_id, project.description, project.startpoint, project.endpoint, project.estimated_costs, project.estimated_hours, project.costs));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn update_project(&mut self, project: Project, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool: &Pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "UPDATE project SET name = ?, client_id = ?, description = ?, startpoint = ?, endpoint = ?, estimated_costs = ?, estimated_hours = ?, costs = ? WHERE id = ?";
        let result = conn.exec_drop(query, (project.name, project.client_id, project.description, project.startpoint, project.endpoint, project.estimated_costs, project.estimated_hours, project.costs, project.id));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn remove_project(&mut self, id: i32, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "DELETE FROM project WHERE id = ?";
        let result = conn.exec_drop(query, (id,));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn get_all_clients(&mut self, token: &String) -> Result<Vec<Client>, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let mut clients = Vec::new();
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "SELECT * FROM client";
        let result = conn.query_map(query, |(id, firstname, lastname, phone)| {
            Client {
                id,
                firstname,
                lastname,
                phone,
            }
        });
        match result {
            Ok(clients_res) => {
                for client in clients_res {
                    clients.push(client);
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Ok(clients)
    }

    pub async fn get_client(&mut self, id: i32, token: &String) -> Result<Client, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM client WHERE id = {}", id);
        let result = conn.query_map(query, |(id, firstname, lastname, phone)| {
            Client {
                id,
                firstname,
                lastname,
                phone,
            }
        });
        match result {
            Ok(mut clients) => {
                if clients.len() == 1 {
                    return Ok(clients.remove(0));
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Err(Error::new("Client not found".to_string(), 404))
    }

    pub async fn add_client(&mut self, client: Client, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool: &Pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "INSERT INTO client (id, firstname, lastname, phone) VALUES (?, ?, ?, ?)";
        let result = conn.exec_drop(query, (client.id, client.firstname, client.lastname, client.phone));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn update_client(&mut self, client: Client, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "UPDATE client SET firstname = ?, lastname = ?, phone = ? WHERE id = ?";
        let result = conn.exec_drop(query, (client.firstname, client.lastname, client.phone, client.id));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn remove_client(&mut self, id: i32, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "DELETE FROM client WHERE id = ?";
        let result = conn.exec_drop(query, (id,));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn get_all_materials(&mut self, token: &String) -> Result<Vec<Material>, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let mut materials = Vec::new();
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "SELECT * FROM material";
        let result = conn.query_map(query, |(id, name, description, type_id, amount, costs, threshold_value)| {
            Material {
                id,
                name,
                description,
                type_id,
                amount,
                costs,
                threshold_value,
            }
        });
        match result {
            Ok(materials_res) => {
                for material in materials_res {
                    materials.push(material);
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Ok(materials)
    }

    pub async fn get_material(&mut self, id: i32, token: &String) -> Result<Material, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM material WHERE id = {}", id);
        let result = conn.query_map(query, |(id, name, description, type_id, amount, costs, threshold_value)| {
            Material {
                id,
                name,
                description,
                type_id,
                amount,
                costs,
                threshold_value,
            }
        });
        match result {
            Ok(mut materials) => {
                if materials.len() == 1 {
                    return Ok(materials.remove(0));
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Err(Error::new("Material not found".to_string(), 404))
    }

    pub async fn add_material(&mut self, material: Material, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "INSERT INTO material (id, name, description, type_id, amount, costs, threshold_value) VALUES (?, ?, ?, ?, ?, ?, ?)";
        let result = conn.exec_drop(query, (material.id, material.name, material.description, material.type_id, material.amount, material.costs, material.threshold_value));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn update_material(&mut self, material: Material, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "UPDATE material SET name = ?, description = ?, type_id = ?, amount = ?, costs = ?, threshold_value = ? WHERE id = ?";
        let result = conn.exec_drop(query, (material.name, material.description, material.type_id, material.amount, material.costs, material.threshold_value, material.id));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn remove_material(&mut self, id: i32, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "DELETE FROM material WHERE id = ?";
        let result = conn.exec_drop(query, (id,));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn get_all_material_types(&mut self, token: &String) -> Result<Vec<MaterialType>, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let mut material_types = Vec::new();
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "SELECT * FROM material_type";
        let result = conn.query_map(query, |(id, name, description)| {
            MaterialType {
                id,
                name,
                description,
            }
        });
        match result {
            Ok(material_types_res) => {
                for material_type in material_types_res {
                    material_types.push(material_type);
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Ok(material_types)
    }

    pub async fn get_material_type(&mut self, id: i32, token: &String) -> Result<MaterialType, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM material_type WHERE id = {}", id);
        let result = conn.query_map(query, |(id, name, description)| {
            MaterialType {
                id,
                name,
                description,
            }
        });
        match result {
            Ok(mut material_types) => {
                if material_types.len() == 1 {
                    return Ok(material_types.remove(0));
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Err(Error::new("Material type not found".to_string(), 404))
    }

    pub async fn add_material_type(&mut self, material_type: MaterialType, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "INSERT INTO material_type (id, name, description) VALUES (?, ?, ?)";
        let result = conn.exec_drop(query, (material_type.id, material_type.name, material_type.description));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn update_material_type(&mut self, material_type: MaterialType, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "UPDATE material_type SET name = ?, description = ? WHERE id = ?";
        let result = conn.exec_drop(query, (material_type.name, material_type.description, material_type.id));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn remove_material_type(&mut self, id: i32, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "DELETE FROM material_type WHERE id = ?";
        let result = conn.exec_drop(query, (id,));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn get_all_project_materials(&mut self, token: &String) -> Result<Vec<ProjectMaterial>, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let mut project_materials = Vec::new();
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "SELECT * FROM project_material";
        let result = conn.query_map(query, |(id, project_id, material_id, amount)| {
            ProjectMaterial {
                id,
                project_id,
                material_id,
                amount,
            }
        });
        match result {
            Ok(project_materials_res) => {
                for project_material in project_materials_res {
                    project_materials.push(project_material);
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Ok(project_materials)
    }

    pub async fn get_project_material(&mut self, id: i32, token: &String) -> Result<ProjectMaterial, Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = format!("SELECT * FROM project_material WHERE id = {}", id);
        let result = conn.query_map(query, |(id, project_id, material_id, amount)| {
            ProjectMaterial {
                id,
                project_id,
                material_id,
                amount,
            }
        });
        match result {
            Ok(mut project_materials) => {
                if project_materials.len() == 1 {
                    return Ok(project_materials.remove(0));
                }
            }
            Err(err) => {
                return Err(Error::new(err.to_string(), 500));
            }
        }
        Err(Error::new("Project material not found".to_string(), 404))
    }

    pub async fn add_project_material(&mut self, project_material: ProjectMaterial, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "INSERT INTO project_material (id, project_id, material_id, amount) VALUES (?, ?, ?, ?)";
        let result = conn.exec_drop(query, (project_material.id, project_material.project_id, project_material.material_id, project_material.amount));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn update_project_material(&mut self, project_material: ProjectMaterial, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "UPDATE project_material SET project_id = ?, material_id = ?, amount = ? WHERE id = ?";
        let result = conn.exec_drop(query, (project_material.project_id, project_material.material_id, project_material.amount, project_material.id));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }

    pub async fn remove_project_material(&mut self, id: i32, token: &String) -> Result<(), Error> {
        self.keycloak.get_groups(token.to_string()).await?;
        let pool = &self.connect();
        let mut conn = pool.get_conn().unwrap();
        let query = "DELETE FROM project_material WHERE id = ?";
        let result = conn.exec_drop(query, (id,));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(err.to_string(), 500)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::utils::keycloak;

    use super::*;

    // Testing add methods (Need to do it in one function because of the specific order and async would randomize it)
    #[actix_rt::test]
    async fn test_add_values() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let client = Client::test_data();
        let material_type = MaterialType::test_data();
        let material = Material::test_data();
        let project = Project::test_data();
        let project_material = ProjectMaterial::test_data();
        let result = repository.add_client(client, &token).await;
        assert!(result.is_ok());
        let result = repository.add_material_type(material_type, &token).await;
        assert!(result.is_ok());
        let result = repository.add_material(material, &token).await;
        assert!(result.is_ok());
        let result = repository.add_project(project, &token).await;
        assert!(result.is_ok());
        let result = repository.add_project_material(project_material, &token).await;
        assert!(result.is_ok());
    }

    // Testing get all methods
    #[actix_rt::test]
    async fn test_get_all_projects() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_projects(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_clients() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_clients(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_materials() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_materials(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_material_types() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_material_types(&token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_all_project_materials() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_all_project_materials(&token).await;
        assert!(result.is_ok());
    }

    // Testing single get methods
    #[actix_rt::test]
    async fn test_get_project() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_project(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_client() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_client(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_material(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_material_type(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.get_project_material(1, &token).await;
        assert!(result.is_ok());
    }

    // Testing Update Methods
    #[actix_rt::test]
    async fn test_update_project() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let project = Project::test_update_data();
        let result = repository.update_project(project, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_client() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let client = Client::test_update_data();
        let result = repository.update_client(client, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let material = Material::test_update_data();
        let result = repository.update_material(material, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let material_type = MaterialType::test_update_data();
        let result = repository.update_material_type(material_type, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let project_material = ProjectMaterial::test_update_data();
        let result = repository.update_project_material(project_material, &token).await;
        assert!(result.is_ok());
    }

    // Testing remove methods
    #[actix_rt::test]
    async fn test_remove_project() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_project(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_client() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_client(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_material(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_material_type() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_material_type(1, &token).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_remove_project_material() {
        let mut repository = Repository::new();
        let mut keycloak = keycloak::Keycloak::new();
        let token = keycloak.login_user("test_user", "test_user").await.unwrap();
        let result = repository.remove_project_material(1, &token).await;
        assert!(result.is_ok());
    }    
}