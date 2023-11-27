use actix_web::{web,post,Responder, HttpResponse, http::StatusCode};
use crate::{api::user::controllers::c_login::{c_user_inf_c_sp},errors::AppError}; 
use serde_json::{Value,json};
use crate::AppState;
use crate::extractors_token::{ AuthenticationToken };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

pub async fn s_user_inf_c_sp( 
    app_data:web::Data<AppState>,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "user":data["user"],
        "pass":data["pass"],
    });
    

    let result = c_user_inf_c_sp(
        app_data.db.clone(),
        value_data
    ).await;
    result.map(|
        todos| HttpResponse::Ok().status(StatusCode::OK).json(todos)
    )
    .map_err(|err|{
        AppError::from(err)
    })
    
    //println!("{:?}", r);
}
// Example on how it probably should be handled
pub async fn verifyToken(auth_token: AuthenticationToken) -> HttpResponse {
    HttpResponse::Ok().json(Response { 
        message: "Authorized".to_owned()
    })
}