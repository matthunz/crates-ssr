use api::CrateItemData;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use log::LevelFilter;
use reqwest::Error;

use std::rc::Rc;

mod api;

mod layout;
use layout::Layout;

mod route;
use crate::route::Route;

mod search;
use search::Search;

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

#[component]
fn Home(cx: Scope) -> Element {
    render!( h4 { "Home" } )
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}
