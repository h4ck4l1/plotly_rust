use serde_json::{Map, Value};

pub mod mushroom_single_cat_col;
pub mod mushroom_double_cat_col;
pub mod mushroom_index;



pub async fn get_col_data(col_name: &str) -> Result<Map<String,Value>,anyhow::Error> {
    if json_data {
        Ok(
            reqwest::Client::new()
                .get(&format!("http://localhost:3000/api/mushroom?col_name={}",col_name))   
                .send()
                .await?
                .json()
                .await?
        )
    } else {
        Ok(
            reqwest::Client::new()
                .get(&format!("http://localhost:3000/api/mushroom?col_name={}",col_name))   
                .send()
                .await?
                .json()
                .await?
        )
    }
}