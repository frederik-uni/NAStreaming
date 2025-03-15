use std::{path::Path, sync::Arc};

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::redirect, App, HttpServer};
use apistos::{
    app::{BuildConfig, OpenApiWrapper},
    info::Info,
    spec::Spec,
    RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig,
};
use app_data::app_data_scope;
use config::get_config;
use log::LevelFilter;

mod app_data;
mod config;
mod error;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Arc::new(get_config(Path::new("./Config.toml")).unwrap());
    let log_level = config.logging.level.as_str();
    let level_filter = match log_level {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new().filter_level(level_filter).init();
    log::debug!("Start NASstreaming with logger: {log_level}");
    let config_ = config.clone();
    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "NAStreaming".to_string(),
                version: "0.1.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(redirect(
                "/github",
                "https://github.com/frederik-uni/NAStreaming",
            ))
            .service(redirect(
                "/source-code",
                "https://github.com/frederik-uni/NAStreaming",
            ))
            .service(redirect(
                "/releases",
                "https://github.com/frederik-uni/NAStreaming/releases",
            ))
            .service(redirect(
                "/changelog",
                "https://github.com/frederik-uni/NAStreaming/blob/main/CHANGELOG.md",
            ))
            .service(redirect(
                "/license",
                "https://github.com/frederik-uni/NAStreaming/blob/main/LICENSE",
            ))
            .service(redirect(
                "/discord",
                "https://github.com/frederik-uni/NAStreaming",
            ))
            .service(redirect(
                "/support",
                "https://github.com/frederik-uni/NAStreaming/issues",
            ))
            .service(redirect(
                "/community",
                "https://github.com/frederik-uni/NAStreaming/discussions",
            ))
            .document(spec)
            .service(app_data_scope(config.clone()).service(routes::register()))
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(RapidocConfig::new(&"/rapidoc"))
                    .with(RedocConfig::new(&"/redoc"))
                    .with(ScalarConfig::new(&"/scalar"))
                    .with(SwaggerUIConfig::new(&"/swagger"))
                    .with(SwaggerUIConfig::new(&"/docs")),
            )
    })
    .bind((config_.server.host.as_str(), config_.server.port))?
    .run()
    .await
}
