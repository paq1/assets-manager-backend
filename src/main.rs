mod settings;
mod assets;

use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
// use server::swagger::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::settings::Settings;
// use core_lib::authn::services::AuthService;
// use server::authn::services::Auth0Service;
// use server::metrics::routes::health;
// use server::settings::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let settings = Arc::new(Settings::new().unwrap());
    // let auth_service: Arc<dyn AuthService> = Arc::new(Auth0Service {settings: settings.clone()});

    HttpServer::new(move || {

        // let openapi = ApiDoc::openapi();
        let cors: Cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        App::new()
            .wrap(cors)
            /*
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url(
                        "/api-docs/openapi.json",
                        openapi
                    )
            )
             */
            // TODO : add app_data générique
            // .app_data(Data::new(auth_service.clone()))
            // metrics
            /*
            .service(
                web::scope("/metrics")
                    .service(health)
            )
             */
    })
        .workers(2)
        .bind((settings.api.address.clone().as_str(), settings.api.port.clone()))?
        .run()
        .await
}
