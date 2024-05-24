/*

    Author: Justin Kosten
    Description: This is the main file for the actix-web server. It is responsible for starting the server and routing requests to the appropriate controller.

*/

// Declare the necessary modules
mod models;
mod utils;

// Import the necessary modules
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

// Import the controller module
use utils::controller;

#[actix_rt::main] // Macro to start the actix runtime
async fn main() -> std::io::Result<()> {
    let port = "8580".to_string();

    // Create a new HttpServer and bind it to the specified port
    HttpServer::new(|| {
        App::new()
            // Enable logger
            .wrap(middleware::Logger::default())
            // Enable CORS
            .wrap(Cors::permissive())
            // Define the routes for the server
            .service(controller::get_project)
            .service(controller::get_project_by_id)
            .service(controller::create_project)
            .service(controller::update_project)
            .service(controller::delete_project)
            .service(controller::get_material_types)
            .service(controller::get_material_type_by_id)
            .service(controller::create_material_type)
            .service(controller::update_material_type)
            .service(controller::delete_material_type)
            .service(controller::get_materials)
            .service(controller::get_material_by_id)
            .service(controller::create_material)
            .service(controller::update_material)
            .service(controller::delete_material)
            .service(controller::get_project_materials)
            .service(controller::get_project_material_by_id)
            .service(controller::create_project_material)
            .service(controller::update_project_material)
            .service(controller::delete_project_material)
            .service(controller::get_clients)
            .service(controller::get_client_by_id)
            .service(controller::create_client)
            .service(controller::update_client)
            .service(controller::delete_client)
    })
    // Bind the server to the specified port
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run()
    .await
}
