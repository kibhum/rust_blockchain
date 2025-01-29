mod blockchain;
mod wallet;
use blockchain::{block_and_blockchain::BlockChain, transaction};
use wallet::wallet::Wallet;
mod api_server;
use actix_web::middleware::Logger;
use api_server::api_server::ApiServer;

#[actix_web::main]
async fn main() {
    env_logger::init();
    let server = ApiServer::new(3000);
    server.run().await;
}
