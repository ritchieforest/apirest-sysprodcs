use actix_web::web;
use crate::api::user::services::s_login::{s_user_inf_c_sp,protected_route};


pub fn config(cfg:&mut web::ServiceConfig){
    cfg.route("api/user/verifyLogin",web::post().to(s_user_inf_c_sp))
    .route("api/user/protected",web::get().to(protected_route))
    
    ;

    //eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MiwiZXhwIjoxNzMyNTc5MTg1fQ.HLJfh17fuQNWzkdI1jE-oNrPgljEFfiAgLvGOSS4E2c
 
}

