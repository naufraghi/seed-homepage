//! The Seed homepage - hosting the guide, and acting as an example. Includes
//! simple interactions, markdown elements, basic routing, and lots of view markup.

mod book;

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Copy, Clone)]
enum Page {
    Guide,
    Changelog
}

impl Page {
    fn to_string(self) -> String {
        // Eg for url routing
        match self {
            Page::Guide => "guide".into(),
            Page::Changelog => "changelog".into(),
        }
    }
}

#[derive(Clone)]
struct GuideSection {
    title: String,
    element: El<Msg>
}

#[derive(Clone)]
struct Model {
    page: Page,
    guide_page: usize,  // Index of our guide sections.
    guide_sections: Vec<GuideSection>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let mut guide_sections = Vec::new();
        let md_texts = vec![
            ("Quickstart", crate::book::quickstart::text()),
            ("Prereqs", crate::book::prereqs::text()),
            ("Structure", crate::book::structure::text()),
            ("Events", crate::book::events::text()),
            ("Components", crate::book::components::text()),
            ("Http requests", crate::book::fetch::text()),
            ("Lifecycle hooks", crate::book::lifecycle::text()),
            ("Routing", crate::book::routing::text()),
            ("Misc features", crate::book::misc::text()),
            ("Release and debugging", crate::book::release_and_debugging::text()),
            ("Element Deep-dive", crate::book::element_deepdive::text()),
            ("About", crate::book::about::text()),
        ];

        for (title, md_text) in md_texts {
            let element = El::from_markdown(&md_text);
            guide_sections.push(GuideSection{title: title.to_string(), element});
        }

        Self {
            page: Page::Guide,
            guide_page: 0,
            guide_sections,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    ChangePage(Page),
    ChangeGuidePage(usize),
    RoutePage(Page),
    RouteGuidePage(usize),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ChangePage(page) => {
            seed::push_route(&page.to_string());
            update(Msg::RoutePage(page), model)
        },
        Msg::ChangeGuidePage(guide_page) => {
            seed::push_route(&format!("guide/{}", guide_page));
            update(Msg::RouteGuidePage(guide_page), model)
        },

        // This is separate, because nagivating the route triggers state updates, which would
        // trigger an additional push state.
        Msg::RoutePage(page) => Model {page, ..model},
        Msg::RouteGuidePage(guide_page) => Model {guide_page, page: Page::Guide, ..model},
    }
}


// View

fn header(version: &str) -> El<Msg> {
    let link_style = style!{
        "margin-left" => 20;
        "margin-right" => 20;
        "font-weight" => "bold";
        "font-size" => "1.2em";
        "color" => "black";
        "cursor" => "pointer";
    };

    header![ style!{"display" => "flex"; "justify-content" => "flex-end"},
        ul![
            a![ &link_style, "Guide", simple_ev("click", Msg::ChangePage(Page::Guide)) ],
            a![ &link_style, "Changelog", simple_ev("click", Msg::ChangePage(Page::Changelog)) ],
            a![ &link_style, "Repo", attrs!{"href" => "https://github.com/David-OConnor/seed"} ],
            a![ &link_style, "Quickstart repo", attrs!{"href" => "https://github.com/David-OConnor/seed-quickstart"} ],
            a![ &link_style, "Crate", attrs!{"href" => "https://crates.io/crates/seed"} ],
            a![ &link_style, "API docs", attrs!{"href" => "https://docs.rs/seed"} ]
        ]
    ]
}

fn title() -> El<Msg> {
    div![ style!{
            // todo look up areas
            "display" => "grid";
            "grid-template-rows" => "auto 160px";
            "grid-template-columns" => "1fr 1fr 1fr";
            "text-align" => "center";
            "align-items" => "center";
//            "justify-items" => "center";
            },
        div![ style!{"grid-row" => "1/2"; "grid-column" => "1 / 4"},
            h1![ style!{"font-size" => "2em"},"Seed" ],
            h2![ "A Rust framework for creating web apps" ],
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "1 / 2"},
            h2![ "Expressive view syntax"]
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "2 / 3"},
            h2![ "Compile-time error checking" ]
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "3 / 4"},
            h2![ "Clean architecture" ]
        ],

    ]
}

fn guide(sections: &[GuideSection], guide_page: usize) -> El<Msg> {
    let menu_item_style = style!{
        "display" => "flex";  // So we can vertically center
        "align-items" => "center";
        "padding" => 4;
        "cursor" => "pointer";
        "height" => 40;
        "margin-bottom" => 0;
        "width" => "100%";
        "color" => "black";
        "font-size" => "1.2em";
    };

    let menu_items: Vec<El<Msg>> = sections
        .iter()
        .enumerate()
        .map(|(i, s)|
        h4![
            &menu_item_style,
            &attrs!{"class" => if i == guide_page {"guide-menu-selected"} else {"guide-menu"}},
            simple_ev("click", Msg::ChangeGuidePage(i)),
            s.title
        ]
    ).collect();

    div![ style! {
        "display" => "grid";
        "grid-template-columns" => "200px auto";
//        "grid-template-rows" => "1fr";
        "color" => "black";
        "grid-auto-rows" => "1fr";
        "align-items" => "start";

//        "padding" => 20;
    },
        div![ style!{"display" => "flex"; "flex-direction" => "column";
                     "grid-column" => "1 / 2";
//                      "grid-row" => "1 / 2";
                      "justify-content" => "flex-start";
                     "padding" => 10;},
            menu_items
        ],

        div![ attrs!{"class" => "guide"},
            style!{"display" => "flex"; "grid-column" => "2 / 3";
//                     "grid-row" => "1 / 2";
            "padding" => 80;},
            sections[guide_page].clone().element
        ]
    ]
}

fn changelog_entry(version: &str, changes: &[&str]) -> El<Msg> {
    let changes: Vec<El<Msg>> = changes.iter().map(|c| li![ c ]).collect();
    div![
        h2![ version ],
        ul![ changes ]
    ]
}

fn changelog(entries: Vec<El<Msg>>) -> El<Msg> {
    div![
        attrs!{"class" => "guide"},
        style!{
            "display" => "flex";
            "flex-direction" => "column";
            "align-items" => "flex-start";
            "padding" => 50;
            "color" => "black";
        },
        entries
    ]
}

fn footer() -> El<Msg> {
    footer![ style!{"display" => "flex"; "justify-content" => "center"},
        h4![ "© 2019 David O'Connor"]
    ]
}



fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    let version = "0.1.12";
    let changelog_entries = vec![
        changelog_entry("v0.2.0", &[
            "Added high-level fetch api",
            "Added routing",
            "Added element lifecycles (did_mount, did_update, will_unmount)",
            "Added support for updating state outside events",
            "Added server_interaction, and homepage (this site) examples"
        ]),
        changelog_entry("v0.1.0", &["Initial release"]),
    ];

    div![
        style!{
            "display" => "flex";
            "flex-direction" => "column";
        },

        section![ style!{"grid-row" => "1 / 2"; "grid-column" => "1 / 2"},
            header(version)
        ],
        section![ style!{"grid-row" => "2 / 3"; "grid-column" => "1 / 2"},
            title()
        ],
        section![ style!{"grid-row" => "3 / 4"; "grid-column" => "1 / 2"},
            match model.page {
                Page::Guide => guide(&model.guide_sections, model.guide_page),
                Page::Changelog => changelog(changelog_entries),
            }
        ],
        section![ style!{"grid-row" => "4 / 5"; "grid-column" => "1 / 2"},
            footer()
        ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    let mut routes = routes!{
        "guide" => Msg::RoutePage(Page::Guide),
        "changelog" => Msg::RoutePage(Page::Changelog),
    };

    for guide_page in 0..12 {
        routes.insert(
            "guide/".to_string() + &guide_page.to_string(),
            Msg::RouteGuidePage(guide_page)
        );
    }

    seed::run(Model::default(), update, view, "main", Some(routes));
}