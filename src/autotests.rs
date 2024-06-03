#[cfg(test)]
mod tests {
    use actix_http::{header::HeaderValue, Request};
    use actix_web::{
        dev::{Service, ServiceResponse},
        test,
        web::Data,
        App, Error,
    };
    use bytes::Bytes;
    use sea_orm::Database;
    use serde::{Deserialize, Serialize};

    use crate::config::{self, AppConfig, AppState};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestBeer {
        pub beer_id: i32,
        pub name: String,
        pub untappd_style: String,
        pub category: String,
        pub subcategory: String,
        pub style: Option<String>,
        pub brewery: String,
        pub abv: f32,
        pub description: Option<String>,
        pub img_url: Option<String>,
        pub last_review: String,
        pub location: String,
    }

    async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        let config = AppConfig::from_env();
        let conn = Database::connect(config.database_url).await.unwrap();

        test::init_service(
            App::new()
                .app_data(Data::new(AppState { conn: conn.clone() }))
                .configure(config::config_handlers),
        )
        .await
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = create_app().await;
        let req = test::TestRequest::get().uri("/api").to_request();

        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, Bytes::from_static(b"Ok"));
    }

    #[actix_web::test]
    async fn test_get_all_beer() {
        let app = create_app().await;
        let payload = r#"{"location":"Шрум"}"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/all_beer")
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            .set_payload(payload)
            .to_request();

        let resp: Vec<TestBeer> = test::call_and_read_body_json(&app, req).await;

        for item in &resp {
            println!("{:?}", item);
        }
    }

    #[actix_web::test]
    async fn test_get_file() {
        let app = create_app().await;
        let req = test::TestRequest::get()
            .uri("/api/assets/images/beer-3739343_17bce_hd.jpeg")
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            .to_request();

        let resp: Bytes = test::call_and_read_body(&app, req).await;

        assert!(!resp.is_empty())
    }

    #[actix_web::test]
    async fn test_get_random_beer() {
        let app = create_app().await;
        let payload = r#"
            {
                "location":"Шрум"
            }
            "#
        .as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/random_beer")
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            .set_payload(payload)
            .to_request();

        let resp: TestBeer = test::call_and_read_body_json(&app, req).await;

        println!("{:?}", resp);
    }

    #[actix_web::test]
    async fn test_get_result_beer() {
        let app = create_app().await;
        let payload = r#"
            {
                "location":"Шрум",
                "category":"classic",
                "subcategory":"wheat",
                "style":"wheat_beer"
            }
            "#
        .as_bytes();

        let req = test::TestRequest::post()
            .uri("/api/result_beer")
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ))
            .set_payload(payload)
            .to_request();

        let resp: Vec<TestBeer> = test::call_and_read_body_json(&app, req).await;
        println!("{:?}", resp);
    }
}
