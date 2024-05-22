/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

/*

CREATE TABLE `project_material` (
  `id` int NOT NULL,
  `project_id` int DEFAULT NULL,
  `material_id` int NOT NULL,
  `amount` int NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

*/


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectMaterial {
    pub id: i32,
    pub project_id: i32,
    pub material_id: i32,
    pub amount: i32,
}
