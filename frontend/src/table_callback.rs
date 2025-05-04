use js_sys::{Object, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlScriptElement};

use crate::TABULATOR_JS;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name=Tabulator)]
    type Tabulator;

    #[wasm_bindgen(constructor)]
    fn new_table_(div_id: &str, obj: &Object) -> Tabulator;

}


pub async fn new_table(div_id: &str, data: Vec<TableData>) {
    let table_options = TableOptions {
        data,
        autoColumns: true,
        layout: "fitDataTable".to_string()
    };

    let table_obj = Object::from(serde_wasm_bindgen::to_value(&table_options).expect("Cannot be convert to Value"));

    JsFuture::from(load_script(TABULATOR_JS)).await.expect("Cannot load the script");

    Tabulator::new_table_(&format!("#{}",div_id), &table_obj);

}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TableData {
    #[serde(rename="DistributionName")]
    distribution_name: String,
    #[serde(rename="P-Value")]
    p_value: f32
}

impl TableData {
    pub fn new(distribution_name: &String, p_value: f32) -> Self {
        Self { distribution_name: distribution_name.clone(), p_value }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
struct TableOptions {
    data: Vec<TableData>,
    autoColumns: bool,
    layout: String
}


pub fn load_script(src: &str) -> Promise {
    let doc = window().unwrap().document().unwrap();
    let script = doc
        .create_element("script").unwrap()
        .dyn_into::<HtmlScriptElement>().unwrap();

    script.set_src(src);
    script.set_type("text/javascript");

    let promise = Promise::new(&mut |resolve, reject| {
        let onload = Closure::once_into_js(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        });
        let onerror = Closure::once_into_js(move || {
            reject.call0(&JsValue::NULL).unwrap();
        });
        script.set_onload(Some(onload.unchecked_ref()));
        script.set_onerror(Some(onerror.unchecked_ref()));
    });

    doc.head().unwrap()
        .append_child(&script).unwrap();
    promise
}