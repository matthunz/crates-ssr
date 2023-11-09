use std::rc::Rc;

use api::CrateItemData;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::{use_signal, Signal};
use log::LevelFilter;
use reqwest::Error;
use serde::{Deserialize, Serialize};

mod api;

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

#[derive(Clone)]
enum ReadyState {
    Fetching,
    Result(Result<Vec<CrateItemData>, Rc<Error>>),
}

enum State {
    Loading,
    Ready {
        query: String,
        state: Option<ReadyState>,
    },
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    let state = use_signal(cx, || State::Loading);

    let query = use_signal(cx, || String::new());

    use_effect(cx, (), move |_| async move {
        state.set(State::Ready {
            query: String::new(),
            state: None,
        })
    });

    let state_ref = &*state.read();
    match state_ref {
        State::Loading => cx.render(rsx!( div { "Loading..." } )),
        State::Ready {
            query,
            state: ready_state,
        } => {
            let content = match &ready_state {
                Some(ReadyState::Result(Ok(crates))) => {
                    let elems = crates
                        .iter()
                        .map(|krate| cx.render(rsx!( div { "{krate.name}" } )));
                    cx.render(rsx! {
                        ul { elems }
                    })
                }
                Some(ReadyState::Result(Err(_))) => cx.render(rsx!( div { "Error" } )),
                Some(ReadyState::Fetching) => cx.render(rsx!( div { "Fetching" } )),
                None => cx.render(rsx!( div { "Search" } )),
            };

            let ready_state = ready_state.clone();
            let query = query.clone();

            render!(
                form {
                    onsubmit: move |_| {
                        let query = query.clone();
                        state
                            .set(State::Ready {
                                query: query.clone(),
                                state: Some(ReadyState::Fetching),
                            });
                        async move {
                            let res = api::get_crates(1, 10, &query).await.map_err(Rc::new);
                            state
                                .set(State::Ready {
                                    query: query.clone(),
                                    state: Some(ReadyState::Result(res)),
                                })
                        }
                    },
                    input { onchange: move |event| {
                            state
                                .set(State::Ready {
                                    query: event.value.clone(),
                                    state: ready_state.clone(),
                                })
                        } }
                }
                content
            )
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}
