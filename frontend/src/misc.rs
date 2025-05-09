use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::Route;
const INDEX_ICON: Asset = asset!("assets/index.png");
const DATASCIENCE_ICON: Asset = asset!("assets/datascience_icon.png");
const MUSHROOM_ICON: Asset = asset!("assets/mushroom.png");
const COVID_ICON: Asset = asset!("assets/covid.png");
const KFC_STOCK_ICON: Asset = asset!("assets/kfc_stock.png");

// Mushroom pages icons
const MUSHROOM_INDEX_ICON : Asset = asset!("src/mushroom/mushroom_assets/mushroom_index_icon.png");

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
pub fn TitleHeading(text: ReadOnlySignal<&'static str>, border_radius: Option<&'static str>)  -> Element {
    rsx!{
        div {
            class: "fade-in-wrapper",
            position: "relative",
            top: "15vh",
            div {
                class: "glass-bg",
                border_radius,
                div {class: "shine"}
                h1 { 
                    class: "heading",
                    "{text()}"
                }
            }
        }
    }
}

#[component]
pub fn DropdownComponent() -> Element {

    let dropdown_options = vec![
        ("INDEX", Route::HomePage {}, INDEX_ICON),
        ("MUSHROOM", Route::MushroomIndexPage {}, MUSHROOM_ICON),
        ("COVID", Route::CovidIndexPage {}, COVID_ICON),
        ("KFC STOCK", Route::KfcIndexPage {}, KFC_STOCK_ICON),
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
                                Link { to: ds_route, class: "dropdown-link", "{ds_name}"},
                            }
                        }
                    },
                }
            }
        }
        Outlet::<Route>{}
    }
}

#[component]
pub fn MushroomDropdownComponent() -> Element {

    let dropdown_options = vec![
        ("INDEX",Route::MushroomIndexPage {  }),
        ("CAP DIAMETER",Route::MushroomCapDiaCatColumn {  }),
        ("CAP SHAPE",Route::MushroomCapShapeCatColumn {  }),
        ("GILL ATTACHMENT",Route::MushroomGillAttachmentColumn {  }),
        ("GILL COLOR",Route::MushroomGillColorColumn {  }),
        ("STEM HEIGHT",Route::MushroomStemHeightColumn {  }),
        ("STEM WIDTH",Route::MushroomStemWidthColumn {  }),
        ("CLASS",Route::MushroomClassCatColumn {  }),
    ];
    let nav = navigator();
    rsx! {
        div {
            class: "sub-dropdown-container",  
            div {
                class: "total-dropdown",
                ul {
                    class: "dropdown-button",
                    li {
                        span {
                            width: "300px",
                            h1 { class: "dropdown-button-title", "MUSHROOM EDA" },
                            img { src: MUSHROOM_INDEX_ICON },
                        }
                    },
                },
                div {
                    class: "arrow-space",
                    span { class: "arrow" },
                },
                ul {
                    class: "dropdown-box",
                    width: "320px",
                    max_height: "400px",
                    for (ds_name, ds_route) in dropdown_options.into_iter() {
                        li {
                            span {
                                left: "10px",
                                height: "60px",
                                width: "320px",
                                cursor: "pointer",
                                font_size: "2em",
                                font_weight: "bold",
                                letter_spacing: "1px",
                                onclick: move |e| {
                                    e.prevent_default();
                                    nav.push(ds_route.clone());
                                },
                                p {  
                                    margin: "10px",
                                    "{ds_name}"
                                }
                            }
                        }
                    },
                }
            }
        }
        Outlet::<Route>{}
    }

}

#[component]
pub fn CovidDropdownComponent() -> Element {

    let dropdown_options = vec![
        ("INDEX",Route::MushroomIndexPage {  }),
        ("CAP DIAMETER",Route::MushroomCapDiaCatColumn {  }),
        ("CAP SHAPE",Route::MushroomCapShapeCatColumn {  }),
        ("GILL ATTACHMENT",Route::MushroomGillAttachmentColumn {  }),
        ("GILL COLOR",Route::MushroomGillColorColumn {  }),
        ("STEM HEIGHT",Route::MushroomStemHeightColumn {  }),
        ("STEM WIDTH",Route::MushroomStemWidthColumn {  })
    ];
    let nav = navigator();
    rsx! {
        div {
            class: "sub-dropdown-container",  
            div {
                class: "total-dropdown",
                ul {
                    class: "dropdown-button",
                    li {
                        span {
                            width: "300px",
                            h1 { class: "dropdown-button-title", "MUSHROOM EDA" },
                            img { src: MUSHROOM_INDEX_ICON },
                        }
                    },
                },
                div {
                    class: "arrow-space",
                    span { class: "arrow" },
                },
                ul {
                    class: "dropdown-box",
                    width: "320px",
                    max_height: "400px",
                    for (ds_name, ds_route) in dropdown_options.into_iter() {
                        li {
                            span {
                                left: "10px",
                                height: "60px",
                                width: "320px",
                                cursor: "pointer",
                                font_size: "2em",
                                font_weight: "bold",
                                letter_spacing: "1px",
                                onclick: move |e| {
                                    e.prevent_default();
                                    nav.push(ds_route.clone());
                                },
                                p {  
                                    margin: "10px",
                                    "{ds_name}"
                                }
                            }
                        }
                    },
                }
            }
        }
        Outlet::<Route>{}
    }

}

#[component]
pub fn KfcStockDropdownComponent() -> Element {

    let dropdown_options = vec![
        ("INDEX",Route::MushroomIndexPage {  }),
        ("CAP DIAMETER",Route::MushroomCapDiaCatColumn {  }),
        ("CAP SHAPE",Route::MushroomCapShapeCatColumn {  }),
        ("GILL ATTACHMENT",Route::MushroomGillAttachmentColumn {  }),
        ("GILL COLOR",Route::MushroomGillColorColumn {  }),
        ("STEM HEIGHT",Route::MushroomStemHeightColumn {  }),
        ("STEM WIDTH",Route::MushroomStemWidthColumn {  })
    ];
    let nav = navigator();
    rsx! {
        div {
            class: "sub-dropdown-container",  
            div {
                class: "total-dropdown",
                ul {
                    class: "dropdown-button",
                    li {
                        span {
                            width: "300px",
                            h1 { class: "dropdown-button-title", "MUSHROOM EDA" },
                            img { src: MUSHROOM_INDEX_ICON },
                        }
                    },
                },
                div {
                    class: "arrow-space",
                    span { class: "arrow" },
                },
                ul {
                    class: "dropdown-box",
                    width: "320px",
                    max_height: "400px",
                    for (ds_name, ds_route) in dropdown_options.into_iter() {
                        li {
                            span {
                                left: "10px",
                                height: "60px",
                                width: "320px",
                                cursor: "pointer",
                                font_size: "2em",
                                font_weight: "bold",
                                letter_spacing: "1px",
                                onclick: move |e| {
                                    e.prevent_default();
                                    nav.push(ds_route.clone());
                                },
                                p {  
                                    margin: "10px",
                                    "{ds_name}"
                                }
                            }
                        }
                    },
                }
            }
        }
        Outlet::<Route>{}
    }

}


#[component]
pub fn SmallBreak() -> Element {
    rsx! {
        div {  
            class: "small-break"
        }
    }
}


#[component]
pub fn MediumBreak() -> Element {
    rsx! {
        div {  
            class: "medium-break"
        }
    }
}

#[component]
pub fn LargeBreak() -> Element {
    rsx!{
        div {  
            class: "large-break"
        }
    }
}