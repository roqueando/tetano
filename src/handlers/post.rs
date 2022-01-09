use actix_web::{HttpRequest, HttpResponse};

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Opa")
}

#[cfg(test)]
mod tests {
    use actix_web::{http, test};
    use super::*;

    #[actix_rt::test]
    async fn should_index_ok() {
        let req = test::TestRequest::get()
            .uri("/")
            .to_http_request();
        let response = index(req).await;
        //let body = test::read_body(response).await;
        
        assert_eq!(response.status(), http::StatusCode::OK);
        //assert_eq!(&body, "Opa");
    }
}