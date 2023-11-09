use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::{use_signal, Signal};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    #[cfg(feature = "ssr")]
    let config = config.incremental(
        IncrementalRendererConfig::default().invalidate_after(std::time::Duration::from_secs(120)),
    );

    config.launch();
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[inline_props]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let query = use_state(cx, || String::new());
    let crates: Signal<Option<Vec<CrateItemData>>> = use_signal(cx, || None);

    let crates_list = if let Some(crates) = &*crates() {
        log::info!("WAT");
        let elems = crates.iter().map(|krate| render!( div { "{krate.name}" } ));
        render!(elems)
    } else {
        render!( div { "Loading..." } )
    };

    cx.render(rsx! {
        div {
            onmounted: |_| {
                log::info!("loaded");
            }
        }
        form { onsubmit: move |_| {
                log::info!("here");
                async move {
                    let data = get_server_data().await.unwrap();
                    crates.set(Some(data));
                }
            },
            input { onchange: |event| query.set(event.value.clone()) }
        }
        crates_list
    })
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct CrateItemData {
    pub name: String,
    pub description: Option<String>,
    pub newest_version: String,
    pub downloads: u32,
    pub recent_downloads: u32,
    pub updated_at: String,
}

#[server(GetServerData)]
async fn get_server_data() -> Result<Vec<CrateItemData>, ServerFnError> {
    Ok(vec![
        CrateItemData {
            name: String::from("dioxus"),
            description: Some(String::from("Da bomb")),
            newest_version: String::from("v1.0.0"),
            downloads: 100,
            recent_downloads: 20,
            updated_at: String::from("4h")
        }
    ])
}
