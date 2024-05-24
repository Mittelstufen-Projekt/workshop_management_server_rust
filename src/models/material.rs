/*

    Author: Justin Kosten
    Description: This file contains the model for the Material object. This object is used to represent a material in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub amount: i32,
    pub costs: f32,
    pub threshold_value: i32,
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