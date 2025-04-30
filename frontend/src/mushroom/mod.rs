use std::collections::HashMap;

use plotly::{common::Line, Histogram, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;

use crate::CUSTOM_LAYOUT;

pub mod mushroom_first_cat_col;
pub mod callback;


pub async fn get_histogram(col_data: Vec<f32>,fit_data: Value,col_name: &str,bar_gap: f32) -> Result<Plot,anyhow::Error> {
    
    let x = &fit_data["x"];
    let x = serde_json::from_value::<Vec<f32>>(x.to_owned())?;
    let mut names = Vec::new();
    if let Some(cap_dia) = fit_data[col_name].as_object() {
        for key in cap_dia.keys() {
            names.push(key.clone());
        }
    }
    let mut plot = Plot::new();
    plot.add_trace(
        Histogram::new(col_data)
            .auto_bin_x(true)
    );
    plot.set_layout(
        CUSTOM_LAYOUT.clone()
            .bar_gap(bar_gap as f64)
    );
    // for k in names {
    //     let y = &fit_data[col_name][&k];
    //     let y = serde_json::from_value::<Vec<f32>>(y.to_owned())?;
    //     let y_mean = y.iter().sum::<f32>()/(y.len() as f32);
    //     plot.add_trace(
    //         Scatter::new(x.clone(),y)
    //             .mode(plotly::common::Mode::Lines)
    //             .name(&format!("Mean of {}",&k))
    //             .hover_template(&format!("Mean: {:?}",y_mean))
    //             .line(Line::new().dash(plotly::common::DashType::DashDot))
    //             .show_legend(false)
    //     );
    // }
    Ok(plot)
}
