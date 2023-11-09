use crate::route::{Query, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;

#[component]
pub fn Layout(cx: Scope) -> Element {
    render!(
        div {
            SearchBar {}
            Outlet::<Route> {}
        }
    )
}

enum State {
    Loading,
    Ready { q: Option<String> },
}

#[component]
fn SearchBar(cx: Scope) -> Element {
    let state = use_signal(cx, || State::Loading);
    let route = use_route(cx);
    let navigator = use_navigator(cx);

    use_effect(cx, &route, |route| async move {
        let q = if let Some(Route::Search { query }) = route {
            Some(query.q)
        } else {
            None
        };
        state.set(State::Ready { q });
    });

    if let State::Ready { q } = &*state() {
        render!(
            div { class: "header",
                Link { to: Route::Home {  }, "Crates" }
                form {
                    class: "search",
                    onsubmit: move |_| {
                        if let State::Ready { q, .. } = &*state() {
                            navigator
                                .push(Route::Search {
                                    query: Query {
                                        q: q.clone().unwrap_or_default(),
                                    },
                                });
                        }
                    },
                    input {
                        r#type: "text",
                        value: "{q.as_deref().unwrap_or_default()}",
                        onchange: move |event| {
                            let mut state_ref = state.write();
                            if let State::Ready { q: _ } = &mut *state_ref {
                                *state_ref = State::Ready {
                                    q: Some(event.value.clone()),
                                };
                            }
                        }
                    }
                    input {
                        r#type: "submit",
                        value: "Search"
                    }
                }
            }
        )
    } else {
        render!("Loading..")
    }
}
