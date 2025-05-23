pub mod mushroom_cap_dia_cat_col;
pub mod mushroom_cap_shape_cat_col;
pub mod mushroom_gill_attachment_cat_col;
pub mod mushroom_gill_color_cat_col;
pub mod mushroom_stem_height_cat_col;
pub mod mushroom_stem_width_cat_col;
pub mod mushroom_class;
pub mod mushroom_season;
pub mod mushroom_stem_color;


use std::collections::{BTreeSet, HashMap};
use dioxus::html::g::format;
use plotly::{common::{Line, Title}, layout::{themes::PLOTLY_DARK, Axis}, Histogram, Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use web_sys::js_sys::Math;
use crate::{table_callback::TableData, CUSTOM_LAYOUT};

use super::get_col_data;


pub async fn single_col_histogram_request(
    col_name: &str,
    bar_gap: f32,
) -> Result<(Plot,Vec<TableData>),anyhow::Error> {
    

    let mut full_data: Map<String,Value> = get_col_data(col_name, None).await?;
    let col_data = serde_json::from_value::<Vec<f32>>(full_data.remove("col_data").unwrap())?;
    let fit_data = full_data.remove("col_json").unwrap();
    let type_of_col = serde_json::from_value::<String>(full_data.remove("type_of_col").unwrap())?;
    let mut fit_data_map = serde_json::from_value::<Map<String, Value>>(fit_data)?;
    let mut table_data = Vec::new();
    let x = serde_json::from_value::<Vec<f32>>(fit_data_map.remove("x").unwrap())?;
    let mut plot = Plot::new();
    let n_bins = col_data.iter().map(|s| format!("{}",s)).collect::<BTreeSet<String>>().len();
    match type_of_col.as_str() {
        "cont" => {
            plot.add_trace(
                Histogram::new(col_data)
                    .auto_bin_x(true)
                    .name(col_name)
            );
        },
        _ => {
            plot.add_trace(
                Histogram::new(col_data)
                    .n_bins_x(n_bins)
                    .auto_bin_x(true)
                    .name(col_name)
            );
        }
    }
    for (key,val) in fit_data_map.into_iter() {
        let mut val_map = serde_json::from_value::<Map<String,Value>>(val)?;
        let y = serde_json::from_value::<Vec<f32>>(val_map.remove("y").unwrap())?;
        let p_value = serde_json::from_value::<f32>(val_map.remove("p").unwrap())?;
        table_data.push(
            TableData::new(&key, p_value)
        );
        plot.add_trace(
            Scatter::new(x.clone(), y)
                .name(&key)
                .mode(plotly::common::Mode::Lines)
                .y_axis("y2")
        );
    }
    plot.set_layout(
        Layout::new()
            .template(&*PLOTLY_DARK)
            .title(Title::with_text(format!("{} Histogram Plot",col_name.replace("_", " ").to_uppercase())))
            .x_axis(Axis::new().show_grid(false).title(format!("{} values",col_name.replace("_", " ").to_uppercase())))
            .y_axis(Axis::new().show_grid(false).title("Counts").zero_line(true))
            .y_axis2(Axis::new().overlaying("y").side(plotly::common::AxisSide::Right).visible(false))
    );
    Ok((plot,table_data))
}