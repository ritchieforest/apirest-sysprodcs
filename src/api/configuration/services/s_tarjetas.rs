use actix_web::{web,post,Responder, HttpResponse, http::StatusCode};
use crate::{api::configuration::controllers::c_tarjetas::{
c_tarjetas_a_sp,c_tarjetas_l_sp,c_tarjetas_m_sp,c_tarjetas_b_sp
},errors::AppError}; 
use serde_json::{Value,json};
use crate::AppState;
use crate::extractors_token::AuthenticationToken;

pub async fn s_tarjetas_l_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "operation":'L',
    });
    
    let result = c_tarjetas_l_sp(
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
pub async fn s_tarjeta_a_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "descripcion":data["descripcion"]
    });
    
    let result = c_tarjetas_a_sp(
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
pub async fn s_tarjetas_m_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "descripcion":data["descripcion"],
        "id":data["id"],
    });
    
    let result = c_tarjetas_m_sp(
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
pub async fn s_tarjetas_b_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "descripcion":data["descripcion"],
        "id":data["id"],
    });
    
    let result = c_tarjetas_b_sp(
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