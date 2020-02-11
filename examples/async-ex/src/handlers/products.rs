use kayrx::web::{web, Error, HttpResponse};

use crate::common::{Part, Product};

pub async fn get_products(
    query: web::Query<Option<Part>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_product(
    new_product: web::Json<Product>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_product_detail(id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_product(id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use crate::appconfig::config_app;
    use kayrx::service::Service;
    use kayrx::http::{header, StatusCode};
    use kayrx::web::{ test, App,};

    #[kayrx::test]
    async fn test_add_product() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let payload = r#"{"id":12345,"product_type":"fancy","name":"test"}"#.as_bytes();

        let req = test::TestRequest::post()
            .uri("/products")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
