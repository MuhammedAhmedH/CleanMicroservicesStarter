#[tokio::main]
async fn main() {
    let server = Server::new(Configs::init());
    server.serve("0.0.0.0:3000").await;
}

mod user;

mod databases;

use crate::server::configs::Configs;

mod server;
use server::server::Server;
