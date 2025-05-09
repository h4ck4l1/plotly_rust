use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[wasm_bindgen(start)]
pub fn make_visible() -> Result<(),JsValue> {
    

    let win = window().ok_or("No window in the DOM")?;
    let doc = win.document().ok_or("No Document in the window")?;

    let node_list = doc.query_selector_all(".fade-in-wrapper")?;


    let callback = Closure::wrap(Box::new(move |entries: Array, observer: IntersectionObserver| {
        for entry in entries.iter() {
            let entry = entry.unchecked_into::<IntersectionObserverEntry>();
            if entry.is_intersecting() {
                let target = entry.target().unchecked_into::<HtmlElement>();
                let _ = target.class_list().add_1("is-visible");
                observer.unobserve(&target);
            }
        }
    }) as Box<dyn FnMut(Array,IntersectionObserver)>);

    let mut opts = IntersectionObserverInit::new();
    opts.set_threshold(&JsValue::from_f64(0.1));

    let observer = IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &opts
    )?;

    callback.forget();

    for i in 0..node_list.length() {
        let element = node_list
            .get(i)
            .unwrap()
            .unchecked_into::<Element>();

        observer.observe(&element);
    }

    Ok(())
}