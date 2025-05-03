use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlScriptElement};
#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name=DataTable)]
    type DataTable;

    #[wasm_bindgen(constructor)]
    pub fn new_table_(div_id: &str) -> DataTable;
}

pub async fn new_table(div_id: &str) -> Result<(),JsValue> {

    JsFuture::from(load_script("https://code.jquery.com/jquery-3.7.1.js")).await?;
    JsFuture::from(load_script("https://cdn.datatables.net/2.3.0/js/dataTables.js")).await?;

    let _ = DataTable::new_table_(&format!("#{}",div_id));

    Ok(())

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