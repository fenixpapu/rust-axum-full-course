#![allow(unused)] // For beginning only


use std::net::SocketAddr;


use axum::Router;
use axum::routing::get;
use axum::response::Html;

#[tokio::main]
async fn main() {
  // println!("hello, world");
  let routes_hello = Router::new().route(
    "/hello",
    get(|| async {Html("Hello <strong>World!!!</strong>"
    )}));


  // region: -- Start Server
  let addr = SocketAddr::from(([127,0,0,1], 8080));
  println!("--> LISTENING on {addr}\n");
  axum::Server::bind(&addr).serve(routes_hello.into_make_service())
  .await
  .unwrap();

  // endregion: -- End server
}