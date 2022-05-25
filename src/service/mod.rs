use crate::es::{get_record, insert_record};
use chrono;
use products::product_server::Product;
use products::{CreateProductMessage, GetProductMessage, ProductResponse};
use serde_json::json;
use tonic::{Request, Response, Status};
use utils::jwt;
use uuid::Uuid;

pub mod products {
    tonic::include_proto!("product");
}

#[derive(Debug, Default)]
pub struct ProductService {}

#[tonic::async_trait]
impl Product for ProductService {
    async fn create_product(
        &self,
        request: Request<CreateProductMessage>,
    ) -> Result<Response<ProductResponse>, Status> {
        let payload = request.extensions().get::<jwt::Payload>().unwrap();

        let data = json!({
            "id": Uuid::new_v4().to_string(),
            "name": request.get_ref().name,
            "description": request.get_ref().description,
            "quantity": request.get_ref().quantity,
            "stock": request.get_ref().stock,
            "created_at": chrono::Utc::now().to_rfc3339(),
            "seller": payload.email,
        });
        insert_record(data, "products").await;
        let res = ProductResponse {
            status: true,
            payload: "Product created successfully".to_string(),
        };

        Ok(Response::new(res))
    }

    async fn get_product(
        &self,
        request: Request<GetProductMessage>,
    ) -> Result<Response<ProductResponse>, Status> {
        let query = request.get_ref().query.clone();

        let data = json!({
                "query": {
                    "multi_match": {
                        "fields": ["name", "description"],
                        "query": query,
            }
        },
            });

        let res = get_record(data, "products").await;

        let res = ProductResponse {
            status: true,
            payload: serde_json::to_string(&res).unwrap(),
        };

        Ok(Response::new(res))
    }
}
