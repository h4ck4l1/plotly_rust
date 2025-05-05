use dioxus::prelude::*;

use crate::{misc::DropdownComponent, Route};


#[component]
pub fn HomePage() -> Element {
    rsx!{
        div {  
            class: "glass-bg",
            div {class: "shine"  }
            h1 {  
                class: "heading",
                "Index Page"
            }
        }
    }
}
