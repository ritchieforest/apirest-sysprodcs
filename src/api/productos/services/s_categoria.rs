use actix_web::{web,post,Responder, HttpResponse, http::StatusCode};
use crate::{api::productos::controllers::c_categoria::{
    c_categoria_a_sp,c_categoria_l_sp,c_categoria_b_sp,c_categoria_m_sp
},errors::AppError}; 
use serde_json::{Value,json};
use crate::AppState;
use crate::extractors_token::AuthenticationToken;

pub async fn s_categoria_a_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_categoria_a_sp(
        app_data.db.clone(),
        value_data
    ).await;
    result.map(|
        todos| HttpResponse::Ok().status(StatusCode::OK).json(todos)
    )
    .map_err(|err|{
        AppError::from(err)
    })
 
}
pub async fn s_categoria_m_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_categoria_m_sp(
        app_data.db.clone(),
        value_data
    ).await;
    result.map(|
        todos| HttpResponse::Ok().status(StatusCode::OK).json(todos)
    )
    .map_err(|err|{
        AppError::from(err)
    })
 
}
pub async fn s_categoria_b_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_categoria_b_sp(
        app_data.db.clone(),
        value_data
    ).await;
    result.map(|
        todos| HttpResponse::Ok().status(StatusCode::OK).json(todos)
    )
    .map_err(|err|{
        AppError::from(err)
    })
 
}
pub async fn s_categoria_l_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_categoria_l_sp(
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