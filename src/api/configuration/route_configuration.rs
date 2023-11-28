use actix_web::web;
use crate::api::configuration::services::s_modulos::{s_modules_by_user};


pub fn config(cfg:&mut web::ServiceConfig){
    cfg.route("api/configuration/modules_by_user",web::post().to(s_modules_by_user));


    //eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MiwiZXhwIjoxNzMyNTc5MTg1fQ.HLJfh17fuQNWzkdI1jE-oNrPgljEFfiAgLvGOSS4E2c
 
}

