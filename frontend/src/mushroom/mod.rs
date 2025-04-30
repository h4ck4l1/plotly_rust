use plotly::Plot;
use serde::{Deserialize, Serialize};

pub mod mushroom_first_cat_col;
pub mod callback;



#[derive(Debug,Serialize,Deserialize,Default)]
pub struct MushroomData {
    col_data: Vec<f32>,
    first_fit: Vec<f32>,
    second_fit: Vec<f32>,
    third_fit: Vec<f32>,
    fourth_fit: Vec<f32>,
    fifth_fit: Vec<f32>,
}

impl MushroomData {
    pub fn col_data(&self) -> &Vec<f32> {
        &self.col_data
    }
    
}



pub async fn get_histogram(data: MushroomData,bar_gap: f32) -> Result<Plot,anyhow::Error> {

    
    Ok(Plot::default())
}