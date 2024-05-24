/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

/*

CREATE TABLE `material_type` (
  `id` int NOT NULL,
  `name` varchar(100) COLLATE utf8mb4_general_ci NOT NULL,
  `description` varchar(500) COLLATE utf8mb4_general_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MaterialType {
    pub id: i32,
    pub name: String,
    pub description: String,
}
