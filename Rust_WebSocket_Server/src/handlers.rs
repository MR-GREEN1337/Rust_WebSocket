use crate::{Ws, Clients, Result};
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| Ws::client_connection(socket, clients)))
}