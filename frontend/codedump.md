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