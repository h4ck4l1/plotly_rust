use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::Route;
const INDEX_ICON: Asset = asset!("assets/index.png");
const DATASCIENCE_ICON: Asset = asset!("assets/datascience_icon.png");
const MUSHROOM_PNG: Asset = asset!("assets/mushroom.png");
const COVID_ICON: Asset = asset!("assets/covid.png");
const KFC_STOCK_ICON: Asset = asset!("assets/kfc_stock.png");

#[component]
pub fn CubeSpinner() -> Element {

    rsx!{
        div {
            class: "cube-spinner-container",
            div {  
                class: "cube-spinner-cube-container"
            }
            div {  
                class: "cube-spinner-cube",
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--front"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--back"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--right"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--left"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--top"
                }
                div {  
                    class: "cube-spinner-cube-side cube-spinner-cube-side--bottom"
                }
            }
        }
    }
}

#[component]
pub fn MarkdownComponent(text: ReadOnlySignal<&'static str>) -> Element {

    rsx!{
        div {  
            class: "container is-fluid",
            Markdown {
                src: text()
            }
        }
    }
}

#[component]
pub fn DropdownComponent() -> Element {

    let dropdown_options = vec![
        ("index", Route::HomePage {}, INDEX_ICON),
        ("mushroom", Route::MushroomIndexPage {}, MUSHROOM_PNG),
        ("covid", Route::CovidIndexPage {}, COVID_ICON),
        ("kfc_stock", Route::KfcIndexPage {}, KFC_STOCK_ICON),
    ];

    rsx! {
        div {
            class: "total-dropdown-container",  
            div {
                class: "total-dropdown",
                ul {
                    class: "dropdown-button",
                    li {
                        span {
                            h1 { class: "dropdown-button-title", "DATASET MENU" },
                            img { src: DATASCIENCE_ICON },
                        }
                    },
                },
                div {
                    class: "arrow-space",
                    span { class: "arrow" },
                },
                ul {
                    class: "dropdown-box",
                    for (ds_name, ds_route, ds_asset) in dropdown_options.into_iter() {
                        li {
                            span {
                                img { src: ds_asset },
                                Link { to: ds_route, class: "dropdown-link","{ds_name.to_uppercase().replace(\"_\",\" \")}" },
                            }
                        }
                    },
                }
            }
        }
        Outlet::<Route>{}
    }
}

