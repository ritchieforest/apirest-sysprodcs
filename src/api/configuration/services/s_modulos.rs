use actix_web::{web,post,Responder, HttpResponse, http::StatusCode};
use crate::{api::configuration::controllers::c_modulos::{
    c_modules_by_user
},errors::AppError}; 
use serde_json::{Value,json};
use crate::AppState;
use crate::extractors_token::AuthenticationToken;

pub async fn s_modules_by_user( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "user":data["user"],
        "empresa":data["empresa"]
    });
    
    let result = c_modules_by_user(
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