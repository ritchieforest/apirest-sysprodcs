use crate::database::database_mssql;
use crate::errors::{AppError, AppErrorType::NotFoundError};
use async_std::net::TcpStream;
use log::kv::value;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tiberius::{numeric::Numeric, Client};

pub async fn c_proveedores_b_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'{}'", object["datajson"]);
    sql_query.push_str("c_proveedor_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query = sql_query
        .replace("\\", "")
        .replace("\"'", "'")
        .replace("'\"", "'");
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _config_cuotas: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        _config_cuotas = json!({
                            "status":true,
                            "id":_id.unwrap(),
                        });
                    } else {
                        
                        _config_cuotas=database_mssql::errno_resolved(row);
                    }
                }
            }
            _config_cuotas
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

pub async fn c_proveedores_m_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'{}'", object["datajson"]);
    sql_query.push_str("c_proveedor_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query = sql_query
        .replace("\\", "")
        .replace("\"'", "'")
        .replace("'\"", "'");

    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _config_cuotas: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        _config_cuotas = json!({
                            "status":true,
                            "id":_id.unwrap(),
                        });
                    } else {
                        
                        _config_cuotas=database_mssql::errno_resolved(row);
                    }
                }
            }
            _config_cuotas
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

pub async fn c_proveedores_a_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Value, AppError> {
    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'{}'", object["datajson"]);
    sql_query.push_str("c_proveedor_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query = sql_query
        .replace("\\", "")
        .replace("\"'", "'")
        .replace("'\"", "'");
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Value {
            let mut _config_cuotas: Value = json!({});
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        _config_cuotas = json!({
                            "status":true,
                            "id":_id.unwrap(),
                        });
                    } else { 
                        _config_cuotas=database_mssql::errno_resolved(row);
                    }
                }
            }
            _config_cuotas
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

pub async fn c_proveedores_l_sp(
    conexion: Arc<Mutex<Client<TcpStream>>>,
    data: Value,
) -> Result<Vec<Value>, AppError> {
    //declarar las variables para leer

    let mut sql_query = String::new();
    let object = data.to_owned();
    let sql_param = format!("'{}'", object["datajson"]);
    sql_query.push_str("c_proveedor_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query = sql_query
        .replace("\\", "")
        .replace("\"'", "'")
        .replace("'\"", "'");
    println!("{}", sql_query);
    let mut _vec = database_mssql::resolve_data_stored(&sql_query, conexion)
        .await
        .map(|__vec| -> Vec<Value> {
            let mut _list_config_cuotas: Vec<Value> = Vec::new();
            for i in __vec.into_iter() {
                for row in i.into_iter() {
                    let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id: Option<i32> = row.get(0);
                        let _porcentaje: Option<&str> = row.get(1);
                        let _id_tarjeta: Option<i32> = row.get(2);
                        let _id_cuota: Option<i32> = row.get(4);
                        let _cuota: Option<&str> = row.get(5);
                        let _tarjeta: Option<&str> = row.get(3);
    
                        _list_config_cuotas.push(json!({
                            "id":_id.unwrap(),
                            "porcentaje":_porcentaje.unwrap().to_string(),
                            "id_cuota":_id_cuota.unwrap(),
                            "id_tarjeta":_id_tarjeta.unwrap(),
                            "cuota":_cuota.unwrap().to_string(),
                            "tarjeta":_tarjeta.unwrap().to_string()
                        }));
                    } else { 
                        _list_config_cuotas.push(database_mssql::errno_resolved(row));
                        break;
                    }
                    
                }
            }
            _list_config_cuotas
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
