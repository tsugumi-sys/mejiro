use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
pub mod base_search;
use crate::base_search::{SearchEngine, SearchPostData};
pub mod word_search;
use crate::word_search::WordSearchEngine;

#[wasm_bindgen]
pub fn search(posts: JsValue, query: &str) -> JsValue {
    let parsed: Vec<SearchPostData> = from_value(posts).unwrap_or_default();
    let engine = WordSearchEngine::new(parsed);
    let results = SearchEngine::search(&engine, query);
    to_value(&results).unwrap()
}
