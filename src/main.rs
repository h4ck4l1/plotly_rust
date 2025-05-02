use frontend::App;
use dioxus::launch;


fn main() {
    // tracing_wasm::set_as_global_default();
    launch(App);
}