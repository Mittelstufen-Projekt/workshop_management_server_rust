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
  `startpoint` float NOT NULL,
  `endpoint` float NOT NULL,
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
    pub startpoint: f32,
    pub endpoint: f32,
    pub estimated_costs: f32,
    pub estimated_hours: f32,
    pub costs: f32,
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