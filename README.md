# workshop_management_server_rust
A version of our workshop management server but just written in Rust :)

## How to run
1. Install Rust
2. Clone the repository
3. Run `cargo run` in the root directory of the project (Only on server)
4. The server should now be running on `localhost:8580`

## How to test
1. Run `cargo test` in the root directory of the project (Only on server)

## Endpoints
- `GET /Projects` - get all projects
- `GET /Projects/{id}` - get project by id
- `POST /Projects` - create project
- `PUT /Projects/{id}` - update project
- `DELETE /Projects/{id}` - delete project
- `GET /Materials` - get all materials
- `GET /Materials/{id}` - get material by id
- `POST /Materials` - create material
- `PUT /Materials/{id}` - update material
- `DELETE /Materials/{id}` - delete material
- `GET /ProjectMaterials` - get all project materials
- `GET /ProjectMaterials/{id}` - get project material by id
- `POST /ProjectMaterials` - create project material
- `PUT /ProjectMaterials/{id}` - update project material
- `DELETE /ProjectMaterials/{id}` - delete project material
- `GET /Clients` - get all clients
- `GET /Clients/{id}` - get client by id
- `POST /Clients` - create client
- `PUT /Clients/{id}` - update client
- `DELETE /Clients/{id}` - delete client
