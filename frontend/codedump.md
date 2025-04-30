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


```rust


pub struct UpdateMenu {
    active,
    background_color,
    border_color,
    border_width,
    buttons,
    direction,
    font,
    name,
    pad,
    show_active,
    template_item_name,
    ty,
    visible,
    x,
    x_anchor,
    y,
    y_anchor
}



```


```rust

use serde::Serialize;

/// Represents a Plotly configuration with update menus and sliders.
#[derive(Serialize, Debug)]
pub struct PlotlyConfig {
    #[serde(rename = "updatemenus")]
    pub update_menus: Vec<UpdateMenu>,

    #[serde(rename = "sliders")]
    pub sliders: Vec<Slider>,
}

/// Represents an update menu (buttons control).
#[derive(Serialize, Debug)]
pub struct UpdateMenu {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "yanchor")]
    pub y_anchor: Anchor,
    #[serde(rename = "xanchor")]
    pub x_anchor: Anchor,
    #[serde(rename = "showactive")]
    pub show_active: bool,
    pub direction: Direction,
    #[serde(rename = "type")]
    pub kind: MenuType,
    pub pad: Padding,
    pub buttons: Vec<Button>,
}

/// Represents a button in the update menu.
#[derive(Serialize, Debug)]
pub struct Button {
    pub method: Method,
    pub args: Vec<serde_json::Value>,
    pub label: String,
}

/// Represents a slider control.
#[derive(Serialize, Debug)]
pub struct Slider {
    pub pad: Padding,
    #[serde(rename = "currentvalue")]
    pub current_value: CurrentValue,
    pub steps: Vec<Step>,
}

/// Represents the appearance and layout of the current value display.
#[derive(Serialize, Debug)]
pub struct CurrentValue {
    pub visible: bool,
    pub prefix: String,
    #[serde(rename = "xanchor")]
    pub x_anchor: Anchor,
    pub font: Font,
}

/// Represents an individual step in the slider.
#[derive(Serialize, Debug)]
pub struct Step {
    pub method: Method,
    pub args: Vec<serde_json::Value>,
    pub label: String,
}

/// Padding values for layout elements.
#[derive(Serialize, Debug)]
pub struct Padding {
    pub t: i32,
    pub r: Option<i32>,
    pub l: Option<i32>,
}

/// Font settings for text elements.
#[derive(Serialize, Debug)]
pub struct Font {
    pub size: i32,
    pub color: String,
}

/// Enumerates possible anchors for positioning.
#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Anchor {
    Top,
    Left,
    Right,
    Bottom,
}

/// Enumerates directions for menu layout.
#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// Enumerates menu types.
#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MenuType {
    Buttons,
    Dropdown,
}

/// Enumerates animation/control methods.
#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Animate,
    Restyle,
    Relayout,
}


```

```rust
#[derive(Serialize)]
pub struct Args {
    mode: FrameMode,
    #[serde(skip_serializing_if="Option::is_none")]
    fromcurrent: Option<bool>,
    transition: FrameTransition,
    frame: FrameData
}

impl Args {
    pub fn new(mode: FrameMode, fromcurrent: Option<bool>,transition: FrameTransition, frame: FrameData) -> Self {
        Self { mode, fromcurrent, transition, frame }
    }
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub enum FrameMode {
    Immediate,
    Next,
    Afterall
}

#[derive(Serialize)]
pub struct FrameTransition {
    duration: u64
}

impl FrameTransition {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }
}


#[derive(Serialize)]
pub struct FrameData {
    duration: u64,
    redraw: bool
}

impl FrameData {
    pub fn new(duration: u64, redraw: bool) -> Self {
        Self { duration, redraw }
    }
}

```





```rust

pub async fn mushroom_data_request() -> Result<(Plot,ExtendedLayout,Vec<Frame>),anyhow::Error> {
    

    let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10) = (0..100)
        .map(|_| Math::random() * 10f64)
        .chunks(10)
        .into_iter()
        .map(|v| v.collect_vec())
        .collect_tuple::<(Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>)>().unwrap();

    let (y1,y2,y3,y4,y5,y6,y7,y8,y9,y10) = (0..100)
        .map(|_| Math::random() * 10f64)
        .chunks(10)
        .into_iter()
        .map(|v| v.collect_vec())
        .collect_tuple::<(Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>)>().unwrap();

    
    let (m1,m2,m3,m4,m5) = (0..50)
        .map(|_| (Math::random() * 100f64) as usize)
        .chunks(10)
        .into_iter()
        .map(|v| v.collect_vec())
        .collect_tuple::<(Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>)>().unwrap();

    let id = (0..10)
        .map(|n| format!("Point {}",n+1))
        .collect::<Vec<String>>();

    let mut plot = Plot::new();

    plot.add_trace(
        Scatter::new(x1.clone(), y1.clone())
            .mode(Mode::Markers)  
            .ids(id.clone())
            .text_array(id.clone())
            .marker(Marker::new().size_array(m1.clone()))
    );

    let button_args1 = Value::Array(vec![
        Value::Null,
        serde_json::json!({
            "mode": "immediate",
            "fromcurrent": true,
            "transition": serde_json::json!({
                "duration": 1000,
                "easing": "quadratic-in-out"
            }),
            "frame": serde_json::json!({
                "duration": 1000,
                "redraw": false
            })
        })
    ]);

    let button_args2 = Value::Array(vec![
        Value::Array(vec![Value::Null]),
        serde_json::json!({
            "mode": "immediate",
            "transition": serde_json::json!({
                "duration": 0
            }),
            "frame": serde_json::json!({
                "duration": 0,
                "redraw": false
            })
        })
    ]);


    let mut slider_steps = Vec::new();

    for i in 1..11 {
        slider_steps.push(
            SliderSteps::new(
            animation::SliderStepMethod::Animate,
            format!("Year: {}",i),
            serde_json::json!(vec![
                    Value::Array(vec![Value::String(format!("Year: {}",i))]),
                    serde_json::json!({
                        "mode": "immediate",
                        "transition": serde_json::json!({
                            "duration": 1000
                        }),
                        "frame": serde_json::json!({
                            "duration": 1000,
                            "redraw": false
                        })
                    })
                ])    
            )
        );
    }


    let layout = ExtendedLayout::new(
        Axis::new().title("Life Expectancy").range(vec![0,15]).show_grid(false).show_line(true),
        Axis::new().title("GDP PerCapita").range(vec![0,15]).show_grid(false).show_line(true),
        plotly::layout::HoverMode::Closest,
        vec![
            UpdateMenu::new()
                .x(0.1)
                .y(0.1)
                .y_anchor(plotly::common::Anchor::Top)
                .x_anchor(plotly::common::Anchor::Left)
                .show_active(false)
                .direction(plotly::layout::update_menu::UpdateMenuDirection::Left)
                .ty(plotly::layout::update_menu::UpdateMenuType::Buttons)
                .pad(Pad::new(57, 0, 0))
                .buttons(
                    vec![
                        Button::new()
                            .method(plotly::layout::update_menu::ButtonMethod::Animate)   
                            .label("Play")                        
                            .args(button_args1),
                        Button::new()
                            .method(plotly::layout::update_menu::ButtonMethod::Animate)
                            .label("Pause")
                            .args(button_args2)
                    ]
                )
        ],
        (*PLOTLY_DARK).clone(),
        Font::new().family("Monaco").size(15),
        vec![
            Slider::new(Pad::new(55, 0, 130),
            CurrentValue::new(true, "", plotly::common::Anchor::Right, Font::new().size(20).color(Rgb::new(102, 102, 102))),
            slider_steps
        )]
    );

    let frames = vec![
        Frame::new(
            Scatter::new(x1, y1)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m1.clone())),
            format!("Year: 1")),
        Frame::new(
            Scatter::new(x2, y2)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m2.clone())), 
            format!("Year: 2")),
        Frame::new(
        Scatter::new(x3, y3)
            .mode(Mode::Markers)  
            .ids(id.clone())
            .text_array(id.clone())
            .marker(Marker::new().size_array(m3.clone())), 
        format!("Year: 3")),
        Frame::new(
            Scatter::new(x4, y4)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m4.clone())), 
            format!("Year: 4")),
        Frame::new(
            Scatter::new(x5, y5)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m5.clone())),
            format!("Year: 5")),
        Frame::new(
            Scatter::new(x6, y6)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m1)),
            format!("Year: 6")),
        Frame::new(
            Scatter::new(x7, y7)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m2)),
            format!("Year: 7")),
        Frame::new(
            Scatter::new(x8, y8)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m3)),
            format!("Year: 8")),
        Frame::new(
            Scatter::new(x9, y9)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m4)),
            format!("Year: 9")),
        Frame::new(
            Scatter::new(x10, y10)
                .mode(Mode::Markers)  
                .ids(id.clone())
                .text_array(id.clone())
                .marker(Marker::new().size_array(m5)),
            format!("Year: 10")),
    ];

    Ok((plot,layout,frames))
}


use plotly::{common::{Anchor, Font, Pad}, layout::{update_menu::{ButtonMethod, UpdateMenu}, Axis, HoverMode, Template}, plot::Traces, Layout, Plot, Scatter, Trace};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{console, js_sys::{Array, JsString, Object, JSON}};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch,js_namespace = Plotly, js_name = newPlot)]
    async fn new_plot_(div_id: &str, plot: &Object) -> Result<JsValue,JsValue>;

}

pub async fn new_plot(div_id: &str, data: &Traces,layout: ExtendedLayout ,frames: Vec<Frame>) {

    let full_data = FullPlot {
        data: data.to_owned(),
        layout,
        frames
    };
    let full_data_serialized = serde_wasm_bindgen::to_value(&full_data)
        .expect("Full data cannot be serialized");
    let full_data = Object::from(full_data_serialized);

    console::log_1(&full_data);

    new_plot_(div_id, &full_data).await.expect("Couldn't load graph");

}

#[derive(Serialize)]
pub struct FullPlot {
    data: Traces,
    layout: ExtendedLayout,
    frames: Vec<Frame>
}

#[derive(Serialize)]
pub struct Frame {
    data: Vec<Box<dyn Trace>>,
    name: String
}

impl Frame {
    pub fn new(data:Box<dyn Trace>,name: String) -> Self {
        Self { data: vec![data], name }
    }
}


#[derive(Serialize)]
pub struct ExtendedLayout {
    xaxis: Axis,
    yaxis: Axis,
    hovermode: HoverMode,
    updatemenus: Vec<UpdateMenu>,
    template: Template,
    font: Font,
    sliders: Vec<Slider>
}

impl ExtendedLayout {
    pub fn new(
        xaxis: Axis,
        yaxis: Axis,
        hovermode: HoverMode,
        updatemenus: Vec<UpdateMenu>,
        template: Template,
        font: Font,
        sliders: Vec<Slider>
    ) -> Self {
        Self { xaxis, yaxis, hovermode, updatemenus, template, font, sliders }
    }
}

#[derive(Serialize)]
pub struct Slider {
    pad: Pad,
    currentvalue: CurrentValue,
    steps: Vec<SliderSteps>
}


impl Slider {
    pub fn new(pad: Pad, currentvalue: CurrentValue, steps: Vec<SliderSteps>) -> Self {
        Self { pad, currentvalue, steps }
    }
}

#[derive(Serialize)]
pub struct SliderSteps {
    method: SliderStepMethod,
    label: String,
    args: Value
}

impl SliderSteps {
    pub fn new(method: SliderStepMethod, label: String, args: Value) -> Self {
        Self { method, label, args }
    }
}

#[derive(Serialize)]
pub struct CurrentValue {
    visible: bool,
    prefix: String,
    xanchor: Anchor,
    font: Font
}

impl CurrentValue {
    pub fn new(visible: bool, prefix: &str, xanchor: Anchor, font: Font) -> Self {
        Self { visible, prefix: prefix.to_string(), xanchor, font }
    }
}



#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SliderStepMethod {
    Animate,
    Restyle,
    Update,
    Skip,
    Relayout
}


```


```git

cp -a .git .git-backup


find . -type f -empty -delete -print


git fsck --full


git fetch -p


git status

```