use actix_web::{web,post,Responder, HttpResponse, http::StatusCode};
use crate::{api::proveedor::controllers::c_proveedor::{
    c_proveedores_a_sp,c_proveedores_l_sp,c_proveedores_m_sp,c_proveedores_b_sp
},errors::AppError}; 
use serde_json::{Value,json};
use crate::AppState;
use crate::extractors_token::AuthenticationToken;

pub async fn s_proveedores_l_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_proveedores_l_sp(
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
pub async fn s_proveedores_a_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"]

    });
    
    let result = c_proveedores_a_sp(
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
pub async fn s_proveedores_m_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
  
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_proveedores_m_sp(
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
pub async fn s_proveedores_b_sp( 
    app_data:web::Data<AppState>,
    auth_token: AuthenticationToken,
    data:web::Json<Value>
) -> Result<impl Responder,AppError> {
    //let v = serde_json::from_str().unwrap();
    let value_data:Value=json!({
        "datajson":data["datajson"],
    });
    
    let result = c_proveedores_b_sp(
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