use std::{format, time::Duration};
use dioxus::prelude::*;
use dioxus_motion::{prelude::*, use_motion, AnimationManager};
use crate::{misc::{CubeSpinner, MarkdownComponent, TitleHeading}, mushroom::single_col_histogram_request, plotly_callback, table_callback::{new_table}};

const MUSHROOM_STEM_WIDTH_MARKDOWN: &str = include_str!("../mushroom_markdowns/mushroom_stem_width_markdown.md");
const MUSHROOM_STEM_WIDTH_IMAGE: Asset = asset!("src/mushroom/mushroom_assets/stem_width.png");

#[component]
pub fn MushroomStemWidthColumn() -> Element {
    let mut is_hidden = use_signal(|| true);
    let mut is_plot_mounted = use_signal(|| false);
    let mut is_loaded = use_signal(|| false);
    let plot_div_id = use_signal(|| "mushroom-stem-width-plot");
    let table_div_id = use_signal(|| "mushroom-stem-width-table");
    let mut error_response = use_signal(|| "".to_string());
    let mut scale_value = use_motion(1f32);
    let mut table_rows = use_signal(|| Vec::new());

    use_effect(move || {
        let mut is_hidden = is_hidden.clone();
        let is_plot_mounted = is_plot_mounted.clone();
        spawn(async move {
            match single_col_histogram_request("stem_width",0f32).await {
                Ok((plot,trows)) => {
                    is_hidden.set(false);
                    async_std::task::sleep(Duration::from_millis(50)).await;
                    if !is_hidden() & is_plot_mounted() {
                        let _ = plotly_callback::new_plot(plot_div_id(), &plot).await;
                    
                    }
                    table_rows.set(trows);
                },
                Err(err) => {
                    error_response.set(err.to_string());
                }
            }
        });
    });

    use_effect(move || {
        let mut is_loaded = is_loaded.clone();
        let table_rows = table_rows.read().clone();
        spawn(async move {
            is_loaded.set(true);
            async_std::task::sleep(Duration::from_millis(50)).await;
            if is_loaded() {
                let _ = new_table(&table_div_id(), table_rows).await;
            }
        });
    });

        

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
                "Mushroom Stem Width Plot"
            }
        }
        div {
            class: "asset-image-container",  
            img {
                class:"asset-image",
                onmouseenter: mouse_enter_scaleup,
                onmouseleave: mouse_leave_scaledown,
                style: "transform: scale({scale_value.get_value()})",
                src: MUSHROOM_STEM_WIDTH_IMAGE
            }
        }
        if is_hidden() {
            div {
                class: "cube-spinner",
                CubeSpinner {  }
            }
        } else {
            div {
                onmounted: move |e| {
                    e.prevent_default();
                    is_plot_mounted.set(true);
                },
                class: "graph-container",
                div {
                    id: "{plot_div_id()}",
                    class: "graph-class"
                }
            }
        }
        div {  
            "{error_response()}"
        }
        div {  
            class: "fade-in-wrapper",
            div {  
                class: "glass-markdown",
                h1 {  
                    "Mushroom Stem Width Observations"
                }
                MarkdownComponent { text: MUSHROOM_STEM_WIDTH_MARKDOWN}
            }
        }
        div {
            class: "table-container",  
            div {
                onmounted: move |e| {
                    e.prevent_default();
                    is_loaded.set(true);
                },
                id: "{table_div_id()}",
            }
        }
    }
}