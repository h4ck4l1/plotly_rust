use std::collections::HashMap;
use plotly::{common::Line, Histogram, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{console::info, js_sys::Math};

use crate::CUSTOM_LAYOUT;

pub mod mushroom_first_cat_col;
pub mod callback;


pub async fn get_histogram(
    col_data: Vec<f32>,
    fit_data: Value,
    col_name: &str,
    bar_gap: f32,
    width: usize,
    height: usize
) -> Result<Plot,anyhow::Error> {
    
    let x = &fit_data["x"];
    let x = serde_json::from_value::<Vec<f32>>(x.to_owned())?;
    let mut names = Vec::new();
    if let Some(cap_dia) = fit_data.as_object() {
        for key in cap_dia.keys() {
            if key != "x" {
                names.push(key.clone());
            }
        }
    }
    let mut plot = Plot::new();
    plot.add_trace(
        Histogram::new(col_data)
            .auto_bin_x(true)
            .name(col_name.replace("_", " ").to_uppercase())
    );
    plot.set_layout(
        CUSTOM_LAYOUT.clone()
            .bar_gap(0f64)
            .height(height)
            .width(width)
    );
    for name in names {
        let y = serde_json::from_value::<Vec<f32>>((&fit_data[&name]["y"]).to_owned())?
            .iter().map(|n| *n * 750000f32)
            .collect::<Vec<f32>>();
        plot.add_trace(
            Scatter::new(x.clone(), y)
                .mode(plotly::common::Mode::Lines)
                .name(&name.to_uppercase().replace("_", " "))
        );
    }
    Ok(plot)
}
