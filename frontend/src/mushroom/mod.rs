use serde_json::{Map, Value};
use web_sys::window;
pub mod mushroom_single_cat_col;
pub mod mushroom_double_cat_col;
pub mod mushroom_index;


pub async fn get_col_data(col_name: &str, fit_col_name: Option<String>) -> Result<Map<String,Value>,anyhow::Error> {
    
    // let base_api = &format!("https://sohailmd123.com/api/mushroom");

    let base_api = &format!("http://localhost:3000/api/mushroom");

    let url = if let Some(fit_col_name) = fit_col_name {
        format!("{}?col_name={}&fit_col_name={}",base_api,col_name,fit_col_name)
    } else {
        format!("{}?col_name={}",base_api,col_name)
    };

    Ok(
        reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json()
        .await?
    )
}