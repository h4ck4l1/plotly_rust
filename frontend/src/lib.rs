#![allow(unused,non_snake_case)]

pub mod mushroom;
pub mod covid;
pub mod kfc_stock;
pub mod plotly_callback;
pub mod table_callback;
pub mod home_page;
pub mod misc;

use std::{borrow::Cow, cell::LazyCell};
use dioxus::document;
use dioxus::prelude::*;
use gloo::utils::format::JsValueSerdeExt;
use misc::DropdownComponent;
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use tracing::Value;
use wasm_bindgen::{convert::{FromWasmAbi, OptionFromWasmAbi}, describe::WasmDescribe};
use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi};
use wasm_bindgen::prelude::*;
use plotly::{common::Font, layout::{Axis, Template, themes::PLOTLY_DARK}, Layout, Plot, Scatter};
use dioxus_markdown::Markdown;
use home_page::HomePage;

// Mushroom Imports


use mushroom::{
    mushrom_single_cat_col::MushroomSingleCategoricalColumn,
    mushroom_double_cat_col::MushroomDoubleCategoricalColumn,
    mushroom_index::MushroomIndexPage
};


// Covid Imports

use covid::{
    covid_index::CovidIndexPage
};


// Kfc-Stock Imports

use kfc_stock::{
    kfc_stock_index::KfcIndexPage
};

// APP wide Asests

static MAIN_CSS: Asset = asset!("assets/main.css");
static TAILWIND_CSS: Asset = asset!("assets/tailwind.css");



// PLOTLY

pub const CUSTOM_LAYOUT: LazyCell<Layout> = LazyCell::new(|| {
    Layout::new()
        .font(Font::new().family("Monaco").size(15))    
        .x_axis(Axis::new().show_grid(false).show_line(true))
        .y_axis(Axis::new().show_grid(false).show_line(true))
        .template(&*PLOTLY_DARK)
});



// JS SCRIPTS and CSS

pub const FLATPICKR_JS: &str = "https://cdn.jsdelivr.net/npm/flatpickr";
pub const PLOTLY_JS: &str = "https://cdn.plot.ly/plotly-3.0.1.min.js";
pub const TABULATOR_JS: &str = "https://unpkg.com/tabulator-tables@6.3.1/dist/js/tabulator.min.js";
pub const TABULATOR_CSS: &str = "https://unpkg.com/tabulator-tables@6.3.1/dist/css/tabulator_site_dark.css";




#[derive(Debug,Clone,Routable)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {

    #[layout(DropdownComponent)]
        #[route("/")]
        HomePage {},
        #[nest("/mushroom")]

            #[route("/single_variable/:col_name")]
            MushroomSingleCategoricalColumn {col_name: String},
            #[route("/double_variable/double?:first_col&:second_col")]
            MushroomDoubleCategoricalColumn {first_col: String, second_col: String},

            #[route("/", MushroomIndexPage)]
            MushroomIndexPage {},

        #[end_nest]
        #[nest("/covid")]

            #[route("/",CovidIndexPage)]
            CovidIndexPage {},

        #[end_nest]
        #[nest("/kfc_stock")]

            #[route("/",KfcIndexPage)]
            KfcIndexPage {},

        #[end_nest]
    #[end_layout]
    #[route("/..all_matches")]
    NotFound {all_matches: Vec<String>}
    
}

#[component]
pub fn App() -> Element {
    rsx!{
        head {  
            document::Meta {name: "viewport", content: "width=device-width, initial-scale=1.0"}
            document::Stylesheet {href: MAIN_CSS}
            document::Script {src: FLATPICKR_JS}
            document::Stylesheet {href: TABULATOR_CSS}
            document::Script {src: TABULATOR_JS}
            document::Script {src: PLOTLY_JS}
        }
        Router::<Route>{}
    }    
}



#[component]
fn NotFound(all_matches: Vec<String>) -> Element {
    rsx!{
        div {  
            "404 Error Page {all_matches[0]}  Not Found"
        }
        div {  
            h1 {  
                "The WebPage You are searching for is Not Found"
            }
        }
    }
}