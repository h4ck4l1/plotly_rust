use std::{any::Any, time::Duration};
use dioxus::prelude::*;
use itertools::Itertools;
use plotly::{ common::{color::Rgb, Font, Marker, Mode, Pad}, layout::{themes::PLOTLY_DARK, update_menu::{Button, UpdateMenu}, Axis}, plot::Traces, Histogram, Layout, Plot, Scatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::js_sys::Math;
use crate::{mushroom::callback, CubeSpinner, Route, CUSTOM_LAYOUT};

use super::{get_histogram, MushroomData};

#[component]
pub fn MushroomFirstCategoricalColumn() -> Element {
    let div_id = use_signal(|| "mushroom-plot");



    rsx!{
        div {  
            h1 {  
                "Mushroom First Animated Plot"
            }
        }
        div {
            id: "{div_id()}"
        }
        marker {  }
    }
}

async fn mushroom_first_cat_col_data_request() -> Result<Plot,anyhow::Error> {
    
    let x = reqwest::Client::new()
        .get("http://localhost:3000/mushroom_first_cat_col")
        .send()
        .await?
        .json::<MushroomData>()
        .await?;


    let plot = get_histogram(MushroomData::default(), 0f32).await?;

    Ok(Plot::default())
}
