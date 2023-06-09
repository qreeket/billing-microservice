use log::info;
use mongodb::bson::Document;
use prost_types::Duration;
use tonic::{Request, Response, Status};
use tonic::metadata::MetadataMap;

use crate::config::locale;
use crate::proto::{ChargeRequest, ChargeResponse, ListTransactionsResponse, Transaction};
use crate::proto::billing_service_server::BillingService;
use crate::utils;

rust_i18n::i18n!("locales");

#[derive(Debug)]
pub struct BillingServiceImpl {
    pub transaction_col: mongodb::Collection<Document>,
}

impl BillingServiceImpl {
    pub fn new(transaction_col: mongodb::Collection<Document>) -> Self {
        Self { transaction_col }
    }
}

#[tonic::async_trait]
impl BillingService for BillingServiceImpl {
    async fn charge(&self, request: Request<ChargeRequest>) -> Result<Response<ChargeResponse>, Status> {
        // validate language id
        let language_id = match _validate_language_id_from_request(request.metadata()) {
            Ok(language_id) => language_id,
            Err(e) => {
                return Err(e);
            }
        };

        // parse request
        let req = request.into_inner();
        let transaction_id = utils::generators::generate_random_string(24);
        info!("request sent as {} -> id({}) => {:?}", language_id,transaction_id ,format!("{:?}", req));

        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_transaction(&self, _: Request<String>) -> Result<Response<Transaction>, Status> {
        // TODO -> implement get_transaction
        Err(Status::unimplemented("Not implemented"))
    }

    async fn list_transactions(&self, _: Request<Duration>) -> Result<Response<ListTransactionsResponse>, Status> {
        // TODO -> implement list_transactions
        Err(Status::unimplemented("Not implemented"))
    }

    async fn delete_transaction(&self, _: Request<String>) -> Result<Response<()>, Status> {
        // TODO -> implement delete_transaction
        Err(Status::unimplemented("Not implemented"))
    }
}

// validate language id
fn _validate_language_id_from_request(md: &MetadataMap) -> Result<String, Status> {
    let language_id = match md.get("x-language-id") {
        Some(result) => result.to_str().unwrap().to_string(),
        None => {
            return Err(Status::invalid_argument(t!("invalid_language_code")));
        }
    };

    // validate language id from request
    match locale::validate_language_id(&language_id) {
        Ok(_) => {
            rust_i18n::set_locale(&language_id);
            Ok(language_id)
        }
        Err(_) => {
            return Err(Status::invalid_argument(t!("invalid_language_code")));
        }
    }
}