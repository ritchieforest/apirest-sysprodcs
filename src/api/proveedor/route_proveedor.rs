use actix_web::web;
use crate::api::proveedor::services::s_proveedores::{s_proveedores_a_sp,s_proveedores_b_sp,s_proveedores_m_sp,s_proveedores_l_sp};
pub fn config(cfg:&mut web::ServiceConfig){
    cfg.route("api/proveedores/add",web::post().to(s_proveedores_a_sp))
    .route("api/proveedores/delete",web::get().to(s_proveedores_b_sp))
    .route("api/proveedores/edit",web::get().to(s_proveedores_m_sp))
    .route("api/proveedores/list",web::get().to(s_proveedores_l_sp));

}

