use actix_web::{web, App, HttpResponse, HttpServer};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::blockchain::block_and_blockchain::BlockChain;
use crate::wallet::wallet::Wallet;

#[derive(Clone, Debug)]
pub struct ApiServer {
    port: u16,
    cache: HashMap<String, BlockChain>,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        let mut api_server = Self {
            port,
            cache: HashMap::new(),
        };
        let miner_wallet = Wallet::new();
        api_server.cache.insert(
            "Blockchain".to_string(),
            BlockChain::new(miner_wallet.get_address()),
        );
        api_server
    }

    async fn get_index(&self) -> HttpResponse {
        let blockchain = self.cache.get("Blockchain").unwrap();
        let first_block = blockchain[0].clone();
        let block_json = serde_json::to_string(&first_block).unwrap();
        debug!("Block Json: {:?}", block_json);
        HttpResponse::Ok().json(block_json)
    }

    pub async fn get_index_handler(data: web::Data<Arc<ApiServer>>) -> HttpResponse {
        info!("Receiving request at /");
        debug!("Received ApiServer Data: {:?}", data);
        data.get_ref().get_index().await
    }

    pub async fn run(&self) {
        let api = Arc::new(self.clone());
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(api.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .route("/", web::get().to(Self::get_index_handler))
        });

        println!("Server running on port: {}", self.port);

        server
            .bind(("0.0.0.0", self.port))
            .expect("Error binding to the port")
            .run()
            .await
            .expect("Error running the server");
    }
}
