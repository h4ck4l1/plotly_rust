use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name=DataTable)]
    type DataTable;

    #[wasm_bindgen(constructor)]
    fn new_table_(div_id:&str) -> DataTable;
}

pub fn new_table(div_id:&str) {

    let table = DataTable::new_table_(&format!("#{}",div_id));
}