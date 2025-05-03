#![allow(unused,non_snake_case)]

pub mod mushroom;
pub mod plotly_callback;
pub mod table_callback;

use std::{borrow::Cow, cell::LazyCell};
use dioxus::document;
use dioxus::prelude::*;
use gloo::utils::format::JsValueSerdeExt;
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use tracing::Value;
use wasm_bindgen::{convert::{FromWasmAbi, OptionFromWasmAbi}, describe::WasmDescribe};
use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi};
use wasm_bindgen::prelude::*;
use plotly::{common::Font, layout::{Axis, Template, themes::PLOTLY_DARK}, Layout, Plot, Scatter};
use dioxus_markdown::Markdown;
use mushroom::{
    mushroom_first_cat_col::MushroomFirstCategoricalColumn
};

static MAIN_CSS: Asset = asset!("assets/main.css");
static TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

pub const CUSTOM_LAYOUT: LazyCell<Layout> = LazyCell::new(|| {
    Layout::new()
        .font(Font::new().family("Monaco").size(15))    
        .x_axis(Axis::new().show_grid(false).show_line(true))
        .y_axis(Axis::new().show_grid(false).show_line(true))
        .template(&*PLOTLY_DARK)
});

#[derive(Debug,Clone,Routable)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/mushroom_first_cat_col")]
    MushroomFirstCategoricalColumn {}
}

#[component]
pub fn App() -> Element {
    rsx!{
        head {  
            document::Meta {name: "viewport", content: "width=device-width, initial-scale=1.0"}
            document::Stylesheet {href: MAIN_CSS}
            document::Stylesheet{href: "https://cdn.datatables.net/2.3.0/css/dataTables.dataTables.css"}
            document::Script {src: "https://cdn.plot.ly/plotly-3.0.1.min.js"}
            document::Script {src: "https://cdn.jsdelivr.net/npm/flatpickr"}
            document::Script {src: "https://code.jquery.com/jquery-3.7.1.js"}
            document::Script {src: "https://cdn.datatables.net/2.3.0/js/dataTables.js"}
        }
        Router::<Route>{}
    }    
}


#[component]
pub fn HomePage() -> Element {
    let nav = navigator();
    rsx!{
        div {
            class: "homepage-contanier",
            h1 {
                class: "homepage-heading",
                "This is HomePage"
            }
        }
        div {
            onclick: move |e| {
                nav.push(Route::MushroomFirstCategoricalColumn {  });
            },
            h2 {
                "Scatter"
            }
        }
    }
}

#[component]
pub fn CubeSpinner() -> Element {

    rsx!{
        div {
            class: "cube-spinner-container",
            div {  
                class: "cube-spinner-cube-container"
            }
            div {  
                class: "cube-spinner-cube",
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--front"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--back"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--right"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--left"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--top"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--bottom"
                }
            }
        }
    }
}




#[component]
pub fn MarkdownComponent(text: ReadOnlySignal<&'static str>) -> Element {


    rsx!{
        div {  
            class: "container is-fluid",
            Markdown {
                src: text()
            }
        }
    }
}