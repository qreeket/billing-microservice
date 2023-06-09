use mongodb::bson::Document;

use crate::{config, proto};
use crate::proto::billing_service_server::BillingServiceServer;
use crate::server::BillingServiceImpl;

pub(crate) async fn init_server() -> Result<(), Box<dyn std::error::Error>> {
    // define the socket address from .env
    let port = std::env::var("PORT").expect("PORT must be set");
    // bind to address
    let host = "[::]";
    let addr = format!("{}:{}", &host, &port).parse().unwrap();

    let mongo_db = match config::db::init_database().await {
        Ok(db) => db,
        Err(e) => {
            log::error!("failed to connect to database: {}", e);
            return Err(e.into());
        }
    };

    let transactions_collection_name =
        std::env::var("TRANSACTION_COLLECTION").expect("TRANSACTION_COLLECTION must be set");

    // create collections based on proto
    let transactions_collection = mongo_db.collection::<Document>(&transactions_collection_name);

    // instantiate the services
    let billing_service = BillingServiceImpl::new(transactions_collection.clone());

    // reflection service
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::BILLING_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    log::info!("initiating billing grpc server on port {}", &port);
  
    // create a new instance of the grpc server
    tonic::transport::Server::builder()
        .add_service(service)
        .add_service(BillingServiceServer::new(billing_service))
        .serve(addr)
        .await?;

    Ok(())
}