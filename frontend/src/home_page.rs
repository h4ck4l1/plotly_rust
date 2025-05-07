use dioxus::prelude::*;

use crate::{misc::{MarkdownComponent, TitleHeading}, Route};

const HOMEPAGE_MARKDOWN: &str = include_str!("mushroom/mushroom_markdowns/homepage_markdown.md");

#[component]
pub fn HomePage() -> Element {
    rsx!{
        TitleHeading {text: "Index Page"  }
        MarkdownComponent {text:  HOMEPAGE_MARKDOWN }
    }
}
