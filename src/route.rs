use crate::{Home, Layout, Search};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/search/?:query")]
    Search { query: Query },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub q: String,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "q={}", self.q)
    }
}

impl FromQuery for Query {
    fn from_query(query: &str) -> Self {
        let mut q = None;

        let pairs = form_urlencoded::parse(query.as_bytes());
        pairs.for_each(|(key, value)| {
            if key == "q" {
                q = Some(value.clone().into());
            }
        });
        Self { q: q.unwrap() }
    }
}
