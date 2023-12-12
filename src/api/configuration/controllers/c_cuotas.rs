use crate::database::database_mssql;
use crate::errors::{AppError, AppErrorType::NotFoundError};
use async_std::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tiberius::Client;

pub async fn c_cuotas_b_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'B',{},{}", object["descripcion"], object["id"]);
    sql_query.push_str("c_cuotas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _cuota_new: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        let _descripcion: Option<&str> = row.get(1);
                        _cuota_new = json!({
                            "status":true,
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                        });
                    } else { 
                        _cuota_new=database_mssql::errno_resolved(row);
                    }
                    
                }
            }
            _cuota_new
        })
        .map_err(|err| AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}", err.to_string())),
            message: Some(format!(
                "{:?}",
                err.to_string().replace('"', "")
            )),
            debug_stores: Some(format!("{}", sql_query.replace('"', "'"))),
        });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}

pub async fn c_cuotas_m_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'M',{},{}", object["descripcion"], object["id"]);
    sql_query.push_str("c_cuotas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _cuota_new: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        let _descripcion: Option<&str> = row.get(1);
                        _cuota_new = json!({
                            "status":true,
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                        });
                    } else { 
                        _cuota_new=database_mssql::errno_resolved(row);
                    } 
                    
                }
            }
            _cuota_new
        })
        .map_err(|err| AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}", err.to_string())),
            message: Some(format!(
                "Todo list {:?} not found.",
                err.to_string().replace('"', "")
            )),
            debug_stores: Some(format!("{}", sql_query.replace('"', "'"))),
        });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}

pub async fn c_cuotas_a_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'A',{},NULL", object["descripcion"]);
    sql_query.push_str("c_cuotas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _cuota_new: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        
                        let _id: Option<i32> = row.get(0);
                        let _descripcion: Option<&str> = row.get(1);
                        _cuota_new = json!({
                            "status":true,
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                        });
                    } else { 
                        _cuota_new=database_mssql::errno_resolved(row);
                    }
                }
            }
            _cuota_new
        })
        .map_err(|err| AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}", err.to_string())),
            message: Some(format!(
                "Todo list {:?} not found.",
                err.to_string().replace('"', "")
            )),
            debug_stores: Some(format!("{}", sql_query.replace('"', "'"))),
        });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}

pub async fn c_cuotas_l_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Vec<Value>, AppError> {
    //declarar las variables para leer

    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'L',NULL,NULL");
    sql_query.push_str("c_cuotas_abmlc_sp ");
    sql_query.push_str(&sql_param);

    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Vec<Value> {
            let mut _list_cuotas: Vec<Value> = Vec::new();
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                         
                        let _id: Option<i32> = row.get(0);
                        let _descripcion: Option<&str> = row.get(1);
                        _list_cuotas.push(json!({
                                "id":_id.unwrap(),
                                "descripcion":_descripcion.unwrap().to_string(),
                        }));
                    } else { 
                        _list_cuotas.push(database_mssql::errno_resolved(row));
                    }
                }
            }
            _list_cuotas
        })
        .map_err(|err| AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}", err.to_string())),
            message: Some(format!(
                "Todo list {:?} not found.",
                err.to_string().replace('"', "")
            )),
            debug_stores: Some(format!("{}", sql_query.replace('"', "'"))),
        });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}
