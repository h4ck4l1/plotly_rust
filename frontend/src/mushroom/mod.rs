use std::collections::HashMap;

use plotly::{common::Line, Histogram, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;

use crate::CUSTOM_LAYOUT;

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
    pub fn first_fit(&self) -> &Vec<f32> {
        &self.first_fit
    }

    pub fn second_fit(&self) -> &Vec<f32> {
        &self.second_fit
    }

    pub fn third_fit(&self) -> &Vec<f32> {
        &self.third_fit
    }

    pub fn fourth_fit(&self) -> &Vec<f32> {
        &self.fourth_fit
    }

    pub fn fifth_fit(&self) -> &Vec<f32> {
        &self.fifth_fit
    }
    
}



pub async fn get_histogram(col_data: Vec<f32>,data: Value,col_name: &str,bar_gap: f32) -> Result<Plot,anyhow::Error> {
    
    let mut names = Vec::new();
    let x = &data[col_name]["x"];
    let x = serde_json::from_value::<Vec<f32>>(x.to_owned())?;
    if let Some(cap_dia) = data[col_name].as_object() {
        for key in cap_dia.keys() {
            names.push(key.clone());
        }
    }
    let x_mean = x.iter().sum::<f32>()/x.len() as f32;
    let mut plot = Plot::new();
    plot.add_trace(
        Histogram::new(col_data)
            .auto_bin_x(true)
    );
    plot.set_layout(
        CUSTOM_LAYOUT.clone()
            .bar_gap(bar_gap as f64)
    );
    for k in names {
        let y = &data[col_name][&k];
        let y = serde_json::from_value::<Vec<f32>>(y.to_owned())?;
        let y_mean = y.iter().sum::<f32>()/(y.len() as f32);
        plot.add_trace(
            Scatter::new(x.clone(),y)
                .mode(plotly::common::Mode::Lines)
                .name(&format!("Mean of {}",&k))
                .hover_template(&format!("Mean: {:?}",y_mean))
                .line(Line::new().dash(plotly::common::DashType::DashDot))
                .show_legend(false)
        );
    }
    Ok(plot)
}
