/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

/*

CREATE TABLE `project` (
  `id` int NOT NULL,
  `name` varchar(100) COLLATE utf8mb4_general_ci NOT NULL,
  `client_id` int NOT NULL,
  `description` varchar(500) COLLATE utf8mb4_general_ci NOT NULL,
  `startpoint` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `endpoint` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `estimated_costs` float NOT NULL,
  `estimatedHours` float NOT NULL,
  `costs` float NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub client_id: i32,
    pub description: String,
    pub startpoint: String,
    pub endpoint: String,
    pub estimated_costs: f32,
    pub estimated_hours: f32,
    pub costs: f32,
}
