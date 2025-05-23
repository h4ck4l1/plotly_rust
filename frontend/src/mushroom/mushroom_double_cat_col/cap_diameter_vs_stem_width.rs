use std::time::Duration;
use dioxus::prelude::*;
use plotly::{layout::{themes::PLOTLY_DARK, Axis}, Layout, Plot, Scatter};
use reqwest::Client;
use serde_json::{Map, Value};

use crate::{misc::{CubeSpinner, TitleHeading}, mushroom::get_col_data, plotly_callback};

#[component]
pub fn CapDiameterVsStemWidth() -> Element {

    let plot_div = use_signal(|| "cap-dia-stem-width");
    let mut is_hidden = use_signal(|| true);
    let mut error_response = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            match ContcolvsContcol().await {
                Ok(plot) => {
                    is_hidden.set(false);
                    async_std::task::sleep(Duration::from_millis(100)).await;
                    if !is_hidden() {
                        let _ = plotly_callback::new_plot(plot_div(), &plot).await;
                    }
                },
                Err(error) => {
                    error_response.set(error.to_string());
                }
            }
        });
    });

    rsx! {
        TitleHeading {
            text: "Cap Diameter VS Stem Width Scatter Plot"
        }
        if is_hidden() {
            div {  
                CubeSpinner {  }
            }
        } else {
            div {  
                id: "{plot_div()}"
            }
        }
        div {  
            "{error_response()}"
        }
    }
}


pub async fn ContcolvsContcol() -> Result<Plot,anyhow::Error> {

    let first_col = "cap_diameter";
    let second_col = "stem_width";

    let mut full_data = get_col_data(&first_col, Some(second_col.to_string())).await?;

    let x = serde_json::from_value::<Vec<f32>>(full_data.remove("col_data").unwrap())?;
    let y = serde_json::from_value::<Vec<f32>>(full_data.remove("second_col_data").unwrap())?;
    let y_hat = serde_json::from_value::<Vec<f32>>(full_data.remove("fit_data").unwrap())?;

    let mut plot = Plot::new();

    plot.set_layout(
        Layout::new()
            .template(&*PLOTLY_DARK)  
            .x_axis(Axis::new().zero_line(true).show_grid(false))
            .y_axis(Axis::new().zero_line(false).show_grid(false))
    );

    plot.add_trace(
        Scatter::new(x.clone(), y)
            .mode(plotly::common::Mode::Markers)
            .web_gl_mode(true)
    );

    plot.add_trace(
        Scatter::new(x.clone(), y_hat)
            .mode(plotly::common::Mode::Lines)
            .web_gl_mode(true)
    );

    Ok(plot)
}