use crate::{
    api::{self},
    route::Query,
};
use dioxus::prelude::*;
use dioxus_signals::use_signal;

#[component]
pub fn Search(cx: Scope, query: Query) -> Element {
    let data = use_signal(cx, || None);

    use_effect(cx, &query.q, |q| async move {
        data.set(None);

        let res = api::get_crates(1, 10, &q).await;
        data.set(Some(res));
    });

    match &*data() {
        Some(Ok(crates)) => {
            let elems = crates.iter().map(|krate| render!( div { "{krate.name}" } ));
            render!(ul { elems })
        }
        Some(Err(_)) => render!("Error!"),
        None => render!("Fetching..."),
    }
}
