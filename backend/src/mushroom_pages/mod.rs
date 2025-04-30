use serde::{Deserialize, Serialize};
pub mod mushroom_cap_diameter;

#[derive(Debug,Serialize,Deserialize)]
pub struct MushroomData {
    col_data: Vec<f32>,
    first_fit: Vec<f32>,
    second_fit: Vec<f32>,
    third_fit: Vec<f32>,
    fourth_fit: Vec<f32>,
    fifth_fit: Vec<f32>,
}


impl MushroomData {
    pub fn new(
        col_data: Vec<f32>,
        first_fit: Vec<f32>,
        second_fit: Vec<f32>,
        third_fit: Vec<f32>,
        fourth_fit: Vec<f32>,
        fifth_fit: Vec<f32>
    ) -> Self {
        Self { 
            col_data, 
            first_fit, 
            second_fit, 
            third_fit, 
            fourth_fit, 
            fifth_fit 
        }
    }
}





