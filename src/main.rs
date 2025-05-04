use frontend::App;
use dioxus::{launch, logger::tracing::Level};


fn main() {
    dioxus::logger::init(Level::DEBUG).expect("fialed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}