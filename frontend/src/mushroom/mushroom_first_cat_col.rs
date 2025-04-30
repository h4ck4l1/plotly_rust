use std::{any::Any, time::Duration};
use dioxus::prelude::*;
use itertools::Itertools;
use plotly::{ common::{color::Rgb, Font, Marker, Mode, Pad}, layout::{themes::PLOTLY_DARK, update_menu::{Button, UpdateMenu}, Axis}, plot::Traces, Histogram, Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;
use crate::{mushroom::callback, CubeSpinner, Route, CUSTOM_LAYOUT};

use super::get_histogram;

#[component]
pub fn MushroomFirstCategoricalColumn() -> Element {
    let div_id = use_signal(|| "mushroom-plot");
    let mut error_response = use_signal(|| "".to_string());
    // let is_hidden = use_signal(|| true);

    use_effect(move || {
        // let mut is_hidden = is_hidden.clone();
        spawn(async move {
            match mushroom_first_cat_col_data_request().await {
                Ok(plot) => {
                    // is_hidden.set(false);
                    async_std::task::sleep(Duration::from_secs(500)).await;
                    let _ = plotly::bindings::new_plot(div_id(), &plot).await;
                },
                Err(err) => {
                    // is_hidden.set(true);
                    error_response.set(err.to_string());
                }
            }
        });
    });

    rsx!{
        div {  
            h1 {  
                "Mushroom First Animated Plot"
            }
        }
        div {  
            id: "{div_id()}"
        }
        div {  
            "{error_response()}"
        }
    }
}


#[component]
pub fn MushroomCatColComponent() -> Element {
    rsx!{

    }
}


async fn mushroom_first_cat_col_data_request() -> Result<Plot,anyhow::Error> {
    
    let full_data = reqwest::Client::new()
        .get("http://localhost:3000/mushroom_cap_diameter")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let col_data = &full_data["cap_diameter"];
    let fit_data = &full_data["cap_dia_json"];
    let col_data = serde_json::from_value::<Vec<f32>>(col_data.to_owned())?;
    let fit_data = serde_json::from_value(fit_data.to_owned())?;
    let mut plot = get_histogram(col_data, fit_data, "cap_diameter", 0f32).await?;
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 1000);
    Ok(plot)
}
