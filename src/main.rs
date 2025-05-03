use frontend::App;
use dioxus::launch;


fn main() {
    // tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    launch(App);
}