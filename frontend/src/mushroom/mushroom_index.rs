use dioxus::prelude::*;

use crate::misc::{MarkdownComponent, SmallBreak, TitleHeading};

static MUSHROOM_INDEX_MARKDOWN: &str = include_str!("mushroom_markdowns/mushroom_index_markdown.md");


#[component]
pub fn MushroomIndexPage() -> Element {
    rsx!{
        TitleHeading { text: "Mushroom Index Page"  }
        div {  
            class: "fade-in-wrapper",
            div {  
                class: "glass-markdown",
                MarkdownComponent {text:  MUSHROOM_INDEX_MARKDOWN }
            }
        }
    }
}