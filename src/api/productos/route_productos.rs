use actix_web::web;
use crate::api::productos::services::s_categoria::{
    s_categoria_a_sp,s_categoria_l_sp,s_categoria_b_sp,s_categoria_m_sp
};
use crate::api::productos::services::s_productos::{
    s_productos_a_sp,s_productos_b_sp,s_productos_l_sp,s_productos_m_sp
};

pub fn config(cfg:&mut web::ServiceConfig){
    cfg.route("api/productos/categoria/add",web::post().to(s_categoria_a_sp))
    .route("api/productos/categoria/edit",web::post().to(s_categoria_m_sp))
    .route("api/productos/categoria/delete",web::post().to(s_categoria_b_sp))
    .route("api/productos/categoria/list",web::post().to(s_categoria_l_sp))

    .route("api/productos/productos/add",web::post().to(s_productos_a_sp))
    .route("api/productos/productos/edit",web::post().to(s_productos_m_sp))
    .route("api/productos/productos/delete",web::post().to(s_productos_b_sp))
    .route("api/productos/productos/list",web::post().to(s_productos_l_sp));
    //eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MiwiZXhwIjoxNzMyNTc5MTg1fQ.HLJfh17fuQNWzkdI1jE-oNrPgljEFfiAgLvGOSS4E2c
 
}

