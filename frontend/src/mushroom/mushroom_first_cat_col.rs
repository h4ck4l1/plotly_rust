use std::{any::Any, time::Duration};
use dioxus::prelude::*;
use dioxus_motion::{prelude::*, use_motion, AnimationManager};
use itertools::Itertools;
use plotly::{ common::{color::Rgb, Font, Marker, Mode, Pad}, layout::{themes::PLOTLY_DARK, update_menu::{Button, UpdateMenu}, Axis}, plot::Traces, Histogram, Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;
use crate::{table_callback::new_table, CubeSpinner, Markdown, MarkdownComponent, Route, CUSTOM_LAYOUT};
use super::get_histogram;

const MUSHROOM_FIRST_CAT_COL_MARKDOWN: &str = include_str!("mushroom_markdowns/mushroom_first_cat_col_markdown.md");

static MUSHROOM_CAP_DIA_IMAGE: Asset = asset!("src/mushroom/mushroom_assets/cap_diameter.png");

#[component]
pub fn MushroomFirstCategoricalColumn() -> Element {
    let mut is_hidden = use_signal(|| true);
    let div_id = use_signal(|| "mushroom-plot");
    let table_div_id = use_signal(|| "mushroom-cap-dia-table");
    let mut error_response = use_signal(|| "".to_string());
    let mut scale_value = use_motion(1f32);
    let table_rows = use_signal(|| vec![
        ("Sohail","Uddin",25),
        ("Shafi","Uddin",22),
        ("Akhtar", "Parveen",21)
    ]);


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

    let table_mount = move |_| {
        new_table(table_div_id())
    };


    let mouse_enter_scaleup = move |_| {
        scale_value.animate_to(1.2, AnimationConfig::new(
            AnimationMode::Spring(
                Spring::default()
            )
        ));
    };

    let mouse_leave_scaledown = move |_| {
        scale_value.animate_to(1.0, AnimationConfig::new(
            AnimationMode::Spring(
                Spring::default()
            )
        ));
    };
    

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
                onmouseenter: mouse_enter_scaleup,
                onmouseleave: mouse_leave_scaledown,
                style: "transform: scale({scale_value.get_value()})",
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
        MarkdownComponent { text: MUSHROOM_FIRST_CAT_COL_MARKDOWN}
        div {  
            table {
                onmounted: table_mount,
                id: "{table_div_id()}",
                class: "display",
                thead {  
                    tr {  
                        th { "Name Col" }
                        th { "Age Col" }
                        th { "Place Col" }
                    }
                }
                tbody {
                    id: "table-element",
                    for (i,(first_name, last_name, age)) in table_rows().iter().enumerate() {
                        tr {
                            td { border: "2px solid cyan","{first_name}" }
                            td { border: "2px solid cyan","{last_name}" }
                            td { border: "2px solid cyan","{age}" }
                        }
                    }
                }
            }
        }
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
        "Cap Diameter with Best Fit distributions",
        0f32
    ).await?;
    Ok(plot)
}
