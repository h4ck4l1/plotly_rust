use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use gloo::utils::format::JsValueSerdeExt;
use ndarray::Array1;
use ndarray_stats::histogram::{Grid,Bins,Edges};
use plotly::{histogram::Bins as PlotlyBins, Histogram, Plot, Scatter};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use web_sys::{console, js_sys, window, CustomEvent, Event, HtmlElement, HtmlTableCellElement, HtmlTableRowElement};
use crate::{Route, CUSTOM_LAYOUT};

pub fn MushroomFirstCategoricalColumn() -> Element {

    let plot_id = use_signal(|| "mushroom-first-cat-col");
    let mut click_cb = use_signal(|| None::<ClickEvent>);
    let mut x = use_signal(|| 0f64);
    let mut y = use_signal(|| 0f64);
    let mut all_points = use_signal(|| Vec::new());
    let mut current_curve_number = use_signal(|| 0usize);
    let table_rows = use_signal(|| vec![
        ("Sohail","Uddin",25),
        ("Shafi","Uddin",22),
        ("Akhtar", "PArveen",21)
    ]);
    let mut mouse_data = use_signal(|| "".to_string());
    let nav = navigator();

    use_effect(move || {


        let rust_cb = {
            let mut click_cb  = click_cb.clone();

            Closure::wrap(Box::new(move |js_vec: JsValue| {
                let event: ClickEvent = serde_wasm_bindgen::from_value(js_vec).unwrap();
                click_cb.set(Some(event));
            }) as Box<dyn FnMut(JsValue)>)

        };


        match &*click_cb.read() {
            Some(click_event) => {
                x.set(click_event.points[0].x);
                y.set(click_event.points[0].y);
                all_points.set(click_event.points[0].point_numbers.clone());
                current_curve_number.set(click_event.points[0].curve_number);
            },
            None => ()
        }

        spawn(async move {
            
            let mushroom_plot = MushroomCapDiameter().await.unwrap_or_else(|e| Plot::default());

            let _ = plotly::bindings::new_plot(plot_id(), &mushroom_plot).await;
            
            bind_click(plot_id(),rust_cb.as_ref().unchecked_ref()).expect("bind click failed");

            rust_cb.forget();
        });
    });


    let table_click = move |e:MouseEvent | {
        e.prevent_default();
        let raw = e.as_web_event();
        let target = raw
            .target()
            .and_then(|t| t.dyn_into::<HtmlElement>().ok())
            .unwrap();

        if let Some(cell_el) = target.closest("td").unwrap() {
            let cell = cell_el.dyn_into::<HtmlTableCellElement>().unwrap();
            let col = cell.cell_index();
            tracing::info!("\n\n\n col: {}",col);
        };

        if let Some(cell_el) = target.closest("tr").unwrap() {
            let cell = cell_el.dyn_into::<HtmlTableRowElement>().unwrap();
            let row = cell.row_index();
            tracing::info!("row: {} \n\n\n",row);
        }
    };

    rsx!{
        div { 
            onclick: move |e| {
                nav.push(Route::HomePage {  });
            }, 
            h2 {  
                "HomePage"
            }
        }
        div {  
            h1 {  
                "Mushroom First Categorical Column"
            }
        }
        br {  }
        div {
            id: "{plot_id()}"
        }
        div {  
            div { "X Cordinate: {x()}" }
            div { "Y Cordinate: {y()}" }
            div { "Current Curve Number: {current_curve_number()}" }
            div { "{all_points():?}" }
        }
        table {  
            thead {  
                tr {  
                    th { "Name Col" }
                    th { "Age Col" }
                    th { "Place Col" }
                }
            }
            tbody {
                id: "table-element",
                onclick: table_click,
                for (i,(first_name, last_name, age)) in table_rows().iter().enumerate() {
                    tr {
                        td { border: "2px solid cyan","{first_name}" }
                        td { border: "2px solid cyan","{last_name}" }
                        td { border: "2px solid cyan","{age}" }
                    }
                }
            }
        }
        div {  
            "{table_rows():#?}"
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
        Histogram::new(x[..100].to_vec())
            .auto_bin_x(true)  
    );

    Ok(plot)
}

#[wasm_bindgen]
pub fn bind_click(div_id: &str, js_cb: &js_sys::Function) -> Result<(),JsValue> {

    let cb = js_cb.clone();
    let doc = window().unwrap().document().unwrap();
    let plot_div = doc
        .get_element_by_id(div_id)
        .unwrap()
        .dyn_into::<HtmlElement>()?;

    let onclick = Closure::wrap(Box::new(move |event: JsValue| {
        let click_event: ClickEvent = serde_wasm_bindgen::from_value(event)
            .expect("couldn't pase ClickEvent");

        let out = serde_wasm_bindgen::to_value(&click_event).unwrap();
        cb.call1(&JsValue::NULL, &out).unwrap();

    }) as Box<dyn FnMut(JsValue)>);

    let on_fn = js_sys::Reflect::get(&plot_div,&"on".into())?
        .dyn_into::<js_sys::Function>()?;

    on_fn.call2(&plot_div.into(), &"plotly_click".into(), onclick.as_ref().unchecked_ref())?;
    onclick.forget();
    Ok(())
}

#[derive(Debug,Deserialize,Serialize)]
struct ClickPoint {
    #[serde(rename="curveNumber")]
    pub curve_number: usize,
    #[serde(rename="pointNumbers")]
    pub point_numbers: Vec<usize>,
    pub x: f64,
    pub y: f64
}

#[derive(Debug,Deserialize,Serialize)]
struct ClickEvent {
    pub points: Vec<ClickPoint>
}