/*

    Author: Justin Kosten
    Description: This is the main file for the actix-web server. It is responsible for starting the server and routing requests to the appropriate controller.

*/

use std::sync::Mutex;

use actix_web::{delete, get, post, put, HttpResponse, web::{Json, Path}};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::utils::repository::Repository;

use crate::models::client::Client;
use crate::models::material::Material;
use crate::models::project::Project;
use crate::models::material_type::MaterialType;
use crate::models::project_material::ProjectMaterial;

static REPO: Mutex<Repository> = Mutex::new(Repository::new());

#[get("/Projects")]
pub async fn get_project(auth: BearerAuth) -> HttpResponse {
    let result = REPO.lock().unwrap().get_all_projects(&auth.token().to_string()).await;
    let projects = match result {
        Ok(projects) => projects,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(projects)
}

#[get("/Projects/{id}")]
pub async fn get_project_by_id(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().get_project(id.into_inner(), &auth.token().to_string()).await;
    let project = match result {
        Ok(project) => project,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(project)
}

#[post("/Projects")]
pub async fn create_project(auth: BearerAuth, project: Json<Project>) -> HttpResponse {
    let result = REPO.lock().unwrap().add_project(project.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[put("/Projects/{id}")]
pub async fn update_project(auth: BearerAuth, project: Json<Project>) -> HttpResponse {
    let result = REPO.lock().unwrap().update_project(project.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[delete("/Projects/{id}")]
pub async fn delete_project(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().remove_project(id.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[get("/MaterialTypes")]
pub async fn get_material_types(auth: BearerAuth) -> HttpResponse {
    let result = REPO.lock().unwrap().get_all_material_types(&auth.token().to_string()).await;
    let material_types = match result {
        Ok(material_types) => material_types,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(material_types)
}

#[get("/MaterialTypes/{id}")]
pub async fn get_material_type_by_id(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().get_material_type(id.into_inner(), &auth.token().to_string()).await;
    let material_type = match result {
        Ok(material_type) => material_type,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(material_type)
}

#[post("/MaterialTypes")]
pub async fn create_material_type(auth: BearerAuth, material_type: Json<MaterialType>) -> HttpResponse {
    let result = REPO.lock().unwrap().add_material_type(material_type.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[put("/MaterialTypes/{id}")]
pub async fn update_material_type(auth: BearerAuth, material_type: Json<MaterialType>) -> HttpResponse {
    let result = REPO.lock().unwrap().update_material_type(material_type.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[delete("/MaterialTypes/{id}")]
pub async fn delete_material_type(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().remove_material_type(id.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[get("/Materials")]
pub async fn get_materials(auth: BearerAuth) -> HttpResponse {
    let result = REPO.lock().unwrap().get_all_materials(&auth.token().to_string()).await;
    let materials = match result {
        Ok(materials) => materials,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(materials)
}

#[get("/Materials/{id}")]
pub async fn get_material_by_id(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().get_material(id.into_inner(), &auth.token().to_string()).await;
    let material = match result {
        Ok(material) => material,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(material)
}

#[post("/Materials")]
pub async fn create_material(auth: BearerAuth, material: Json<Material>) -> HttpResponse {
    let result = REPO.lock().unwrap().add_material(material.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[put("/Materials/{id}")]
pub async fn update_material(auth: BearerAuth, material: Json<Material>) -> HttpResponse {
    let result = REPO.lock().unwrap().update_material(material.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[delete("/Materials/{id}")]
pub async fn delete_material(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().remove_material(id.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[get("/ProjectMaterials")]
pub async fn get_project_materials(auth: BearerAuth) -> HttpResponse {
    let result = REPO.lock().unwrap().get_all_project_materials(&auth.token().to_string()).await;
    let project_materials = match result {
        Ok(project_materials) => project_materials,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(project_materials)
}

#[get("/ProjectMaterials/{id}")]
pub async fn get_project_material_by_id(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().get_project_material(id.into_inner(), &auth.token().to_string()).await;
    let project_material = match result {
        Ok(project_material) => project_material,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(project_material)
}

#[post("/ProjectMaterials")]
pub async fn create_project_material(auth: BearerAuth, project_material: Json<ProjectMaterial>) -> HttpResponse {
    let result = REPO.lock().unwrap().add_project_material(project_material.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[put("/ProjectMaterials/{id}")]
pub async fn update_project_material(auth: BearerAuth, project_material: Json<ProjectMaterial>) -> HttpResponse {
    let result = REPO.lock().unwrap().update_project_material(project_material.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[delete("/ProjectMaterials/{id}")]
pub async fn delete_project_material(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().remove_project_material(id.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[get("/Clients")]
pub async fn get_clients(auth: BearerAuth) -> HttpResponse {
    let result = REPO.lock().unwrap().get_all_clients(&auth.token().to_string()).await;
    let clients = match result {
        Ok(clients) => clients,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(clients)
}

#[get("/Clients/{id}")]
pub async fn get_client_by_id(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().get_client(id.into_inner(), &auth.token().to_string()).await;
    let client = match result {
        Ok(client) => client,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    HttpResponse::Ok().json(client)
}

#[post("/Clients")]
pub async fn create_client(auth: BearerAuth, client: Json<Client>) -> HttpResponse {
    let result = REPO.lock().unwrap().add_client(client.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[put("/Clients/{id}")]
pub async fn update_client(auth: BearerAuth, client: Json<Client>) -> HttpResponse {
    let result = REPO.lock().unwrap().update_client(client.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[delete("/Clients/{id}")]
pub async fn delete_client(auth: BearerAuth, id: Path<i32>) -> HttpResponse {
    let result = REPO.lock().unwrap().remove_client(id.into_inner(), &auth.token().to_string()).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}
