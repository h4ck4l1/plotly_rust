```js

export function getClickCallback(divId, dataString) {
    console.log(`Type of divId: ${typeof divId} and object: ${divId}`);
    console.log(`Type of dataString: ${typeof dataString} and object: ${dataString}`);
    
    try {
        const parseJsondata = JSON.parse(dataString);
        console.log(`Type of parseJsondata: ${typeof parseJsondata} and keys: ${Object.keys(parseJsondata)}`);
        return parseJsondata;
    } catch (e) {
        console.error("Error parsing JSON:", e);
        throw e;
    }
}


#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn registerPlotlyClickHandler(div_id: &str, handler_id: &str) -> Result<bool, JsValue>;
    
    #[wasm_bindgen(catch)]
    pub fn getLastClickedPoint(handler_id: &str) -> Result<Option<PointData>, JsValue>;
}


#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct PointData {
    pub x: f64,
    pub y: f64,
    pub curve_number: usize,
}

#[wasm_bindgen]
impl PointData {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, curve_number: usize) -> Self {
        Self { x, y, curve_number }
    }
    
    // Add getters if needed
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }
    
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }
    
    #[wasm_bindgen(getter)]
    pub fn curve_number(&self) -> usize {
        self.curve_number
    }
    
    // Static method to create from JS object
    #[wasm_bindgen(js_name = fromJsValue)]
    pub fn from_js_value(val: JsValue) -> Result<PointData, JsValue> {
        serde_wasm_bindgen::from_value(val)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))
    }
    
    // Method to convert to JS object
    pub fn to_js_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(self)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}


```


```rust

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
    };

    if let Some(cell_el) = target.closest("tr").unwrap() {
        let cell = cell_el.dyn_into::<HtmlTableRowElement>().unwrap();
        let row = cell.row_index();
    }
};

let table_rows = use_signal(|| vec![
    ("Sohail","Uddin",25),
    ("Shafi","Uddin",22),
    ("Akhtar", "PArveen",21)
]);
let row_index = use_signal(|| 0usize);
let col_index = use_signal(|| 0usize);

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

```