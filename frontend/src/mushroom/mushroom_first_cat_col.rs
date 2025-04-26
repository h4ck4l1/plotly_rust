use std::time::Duration;
use dioxus::prelude::*;
use plotly::{ histogram::Histogram, Plot};
use serde::{Deserialize, Serialize};
use crate::{Route, CubeSpinner,CUSTOM_LAYOUT};

#[component]
pub fn MushroomFirstCategoricalColumn() -> Element {

    let is_hidden = use_signal(|| true);

    use_effect(move || {
        let mut is_hidden = is_hidden.clone();
        spawn(async move {
            let plot = MushroomCapDiameter().await.unwrap_or(Plot::default());
            is_hidden.set(false);
            async_std::task::sleep(Duration::from_millis(500)).await;
            if !is_hidden() {
                let _ = plotly::bindings::new_plot("mushroom-plot", &plot).await;
            }
        });
    });

    rsx!{
        div { h1 {  
            "Mushroom First Cat col"
        } }
        div {  
            if !is_hidden() {
                div {  
                    id: "mushroom-plot"
                }
            } else {
                div {  
                    CubeSpinner {  }
                }
            }
        }
    }
}

pub async fn MushroomCapDiameter() -> Result<Plot,anyhow::Error> {
    
    let x = reqwest::Client::new()
        .get("http://localhost:3000/mushroom_cap_diameter")
        .send()
        .await?
        .json::<Vec<i16>>()
        .await?;
    let mut plot = Plot::new();
    plot.set_layout(CUSTOM_LAYOUT.clone());
    plot.add_trace(
        Histogram::new(x)
            .auto_bin_x(true)
    );
    Ok(plot)
}