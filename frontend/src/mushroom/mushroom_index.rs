use dioxus::prelude::*;

use crate::misc::{MarkdownComponent, TitleHeading};

static MUSHROOM_INDEX_MARKDOWN: &str = include_str!("mushroom_markdowns/mushroom_index_markdown.md");


#[component]
pub fn MushroomIndexPage() -> Element {
    rsx!{
        TitleHeading { text: "Mushroom Index Page"  }
        MarkdownComponent {text:  MUSHROOM_INDEX_MARKDOWN }
    }
}