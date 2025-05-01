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
    let mut is_hidden = use_signal(|| true);
    let mut error_response = use_signal(|| "".to_string());

    use_effect(move || {
        let mut is_hidden = is_hidden.clone();
        spawn(async move {
            match mushroom_first_cat_col_data_request(1200,550).await {
                Ok(plot) => {
                    is_hidden.set(false);
                    async_std::task::sleep(Duration::from_millis(500)).await;
                    let _ = plotly::bindings::new_plot(div_id(), &plot).await;
                },
                Err(err) => {
                    is_hidden.set(true);
                    error_response.set(err.to_string());
                }
            }
        });
    });

    rsx!{
        div {  
            h1 {
                color: "cyan", 
                "Mushroom Cap Diameter Plot"
            }
        }
        div {
            class: "graph-container",  
            div {
                class: "graph-class",
                display: if is_hidden() {"inline-block"} else {"none"},
                CubeSpinner {  }
            }
            div {
                class: "graph-class",
                display: if is_hidden() {"none"} else {"inline-block"} ,
                id: "{div_id()}"
            }
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


async fn mushroom_first_cat_col_data_request(width: usize, height: usize) -> Result<Plot,anyhow::Error> {
    
    let full_data = reqwest::Client::new()
        .get("http://localhost:3000/mushroom_cap_diameter")
        .send()
        .await?
        .json::<Value>()
        .await?;
    let col_name = "cap_diameter";
    let col_data = &full_data[col_name];
    let fit_data = &full_data["cap_dia_json"];
    let col_data = serde_json::from_value::<Vec<f32>>(col_data.to_owned())?;
    let fit_data = serde_json::from_value::<Value>(fit_data.to_owned())?;
    let plot = get_histogram(
        col_data, 
        fit_data, 
        "cap_diameter", 
        0f32,
        width,
        height
    ).await?;
    Ok(plot)
}
