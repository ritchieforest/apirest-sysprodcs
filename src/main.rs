mod database;
mod  api;
mod errors;
mod app_state;
mod authentication;
mod extractors_token;
use crate::database::database_mssql::connect;
use crate::api::user::route_users;
use crate::api::configuration::route_configuration;
use crate::api::productos::route_productos;
use crate::api::proveedor::route_proveedor;


use actix_web::{
    http::header,
    web,App, HttpServer 
   
};
use actix_web::middleware::Logger;
use std::sync::{Arc, Mutex};
use env_logger::Env;
use actix_cors::Cors;

use crate::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool=connect().await.unwrap();
    let app_state=web::Data::new(
        AppState{
            db:Arc::new(Mutex::new(pool)),
            secret:"asdñlkjasdfdoi12342edsssvasdghjlññññ".to_string()
        });
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION, 
                header::ACCEPT,
                header::CONTENT_TYPE, 
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ORIGIN
            ]).supports_credentials()
            .max_age(3600);
        //let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
        .app_data(app_state.clone())
        .wrap(cors)
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        //.wrap(bearer_middleware)
        .configure(route_users::config)
        .configure(route_configuration::config)
        .configure(route_productos::config)
        .configure(route_proveedor::config)




    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
