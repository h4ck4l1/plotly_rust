use std::time::Duration;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use plotly::{plot::Traces, Plot};
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::{self, Function, Object}, window, HtmlElement};
use crate::table_callback::load_script;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends= HtmlElement, js_name=HTMLElement)]
    type PlotlyDiv;

    #[wasm_bindgen(catch,js_namespace=Plotly,js_name=newPlot)]
    async fn new_plot_(div_id: &str, obj: &Object) -> Result<JsValue,JsValue>;

    #[wasm_bindgen(method,structural,js_name=on)]
    fn on(this: &PlotlyDiv, event: &str, cb: &Function);

}

pub async fn new_plot(div_id: &str, plot: &Plot) {

    let plot = plot.to_js_object();

    JsFuture::from(load_script("https://cdn.plot.ly/plotly-3.0.1.min.js")).await
        .expect("Unable to load plotly script");

    new_plot_(div_id, &plot).await.expect("Unable to new_ plot_");

}


pub fn bind_click<F>(div_id: &str, mut cb: F)
where 
    F: 'static + FnMut(ClickEvent)
{

    let plot_div: PlotlyDiv = window().unwrap()
        .document().unwrap()
        .get_element_by_id(div_id).unwrap()
        .unchecked_into();
    let closure = Closure::wrap(Box::new(move |event: JsValue| {
        let event: ClickEvent = serde_wasm_bindgen::from_value(event)
            .expect("\n Couldn't serialize the event \n");
        cb(event);
    }) as Box<dyn FnMut(JsValue)>);
    plot_div.on("plotly_click", closure.as_ref().unchecked_ref());
    closure.forget();
}

#[derive(Debug,Deserialize,Serialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct ClickPoint {
    pub curve_number: usize,
    pub point_numbers: Option<Vec<usize>>,
    pub point_number: Option<usize>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}

fn default_click_event() -> Vec<ClickPoint> {vec![ClickPoint::default()]}

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="camelCase",default)]
pub struct ClickEvent {
    #[serde(default="default_click_event")]
    pub points: Vec<ClickPoint>
}
impl Default for ClickEvent {
    fn default() -> Self {
        ClickEvent { points: vec![] }
    }
}