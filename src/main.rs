use html::RenderError;
use yew::prelude::*;

use futures::{SinkExt, StreamExt};
use gloo_console::{assert, log};
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;

mod logic;
mod messages;
mod server;

#[function_component(App)]
fn app() -> Result<Html, RenderError> {
    Ok(html! {
        <h1>{ LocalStorage::get::<String>("adad").unwrap() }</h1>
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}
