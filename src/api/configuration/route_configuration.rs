use actix_web::web;
use crate::api::configuration::services::s_modulos::{s_modules_by_user};
use crate::api::configuration::services::s_cuotas::{s_cuotas_l_sp,s_cuotas_a_sp};

pub fn config(cfg:&mut web::ServiceConfig){
    cfg.route("api/configuration/modules_by_user",web::post().to(s_modules_by_user))
    .route("api/configuration/listcuotas",web::get().to(s_cuotas_l_sp))
    .route("api/configuration/addcuotas",web::post().to(s_cuotas_a_sp));



    //eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MiwiZXhwIjoxNzMyNTc5MTg1fQ.HLJfh17fuQNWzkdI1jE-oNrPgljEFfiAgLvGOSS4E2c
 
}

