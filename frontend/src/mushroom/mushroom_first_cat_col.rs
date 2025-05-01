use std::{any::Any, time::Duration};
use dioxus::prelude::*;
use itertools::Itertools;
use plotly::{ common::{color::Rgb, Font, Marker, Mode, Pad}, layout::{themes::PLOTLY_DARK, update_menu::{Button, UpdateMenu}, Axis}, plot::Traces, Histogram, Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;
use crate::{mushroom::callback, CubeSpinner, Markdown, MarkdownComponent, Route, CUSTOM_LAYOUT};
use super::get_histogram;

const MUSHROOM_FIRST_CAT_COL_MARKDOWN: &str = include_str!("mushroom_markdowns/mushroom_first_cat_col_markdown.md");

static MUSHROOM_CAP_DIA_IMAGE: Asset = asset!("src/mushroom/mushroom_assets/cap_diameter.png");

#[component]
pub fn MushroomFirstCategoricalColumn() -> Element {
    let mut is_hidden = use_signal(|| true);
    let div_id = use_signal(|| "mushroom-plot");
    let mut error_response = use_signal(|| "".to_string());


    use_effect(move || {
        let mut is_hidden = is_hidden.clone();
        spawn(async move {
            match mushroom_first_cat_col_data_request().await {
                Ok(plot) => {
                    is_hidden.set(false);
                    async_std::task::sleep(Duration::from_millis(500)).await;
                    if !is_hidden() {
                        let _ = plotly::bindings::new_plot(div_id(), &plot).await;
                    }
                },
                Err(err) => {
                    error_response.set(err.to_string());
                }
            }
        });
    });

    

    rsx!{
        div {
            class: "heading-container", 
            h1 {
                class: "heading",
                color: "cyan", 
                "Mushroom Cap Diameter Plot"
            }
        }
        div {
            class: "asset-image-container",  
            img {
                class:"asset-image",
                src: MUSHROOM_CAP_DIA_IMAGE
            }
        }
        if is_hidden() {
            div {
                class: "cube-spinner",
                CubeSpinner {  }
            }
        } else {
            div {  
                class: "graph-container",
                div {  
                    id: "{div_id()}",
                    class: "graph-class"
                }
            }
        }
        div {  
            "{error_response()}"
        }
        div {
            class: "general-heading-container",
            h1 {  
                class: "general-heading",
                "mushroom cap diameter observations"
            }
        }
        MarkdownComponent { text: MUSHROOM_FIRST_CAT_COL_MARKDOWN }
    }
}


async fn mushroom_first_cat_col_data_request() -> Result<Plot,anyhow::Error> {
    
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
        0f32
    ).await?;
    Ok(plot)
}
