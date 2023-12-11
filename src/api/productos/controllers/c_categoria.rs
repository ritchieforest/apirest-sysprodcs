use crate::{errors::{AppError,AppErrorType::NotFoundError}}; 
use async_std::net::TcpStream;
use tiberius::Client;
use crate::database::database_mssql;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

pub async fn c_categoria_a_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<Value,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'{}'",object["datajson"]);
    sql_query.push_str("c_categoria_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query=sql_query.replace("\\", "").replace("\"'", "'").replace("'\"", "'");

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Value{
        let mut _categoria_new:Value=json!({});
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id:Option<i32> = row.get(0);
                        let _descripcion: Option<&str>= row.get(1);
                        let _id_padre: Option<i32>= row.get(2);
                        let _cantidad_sub:Option<i32>=row.get(3);
                        _categoria_new=json!({
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                            "pattern_id":_id_padre.unwrap(),
                            "limit_subcategorias":_cantidad_sub.unwrap()
                        });
                    } else { 
                        _categoria_new=database_mssql::errno_resolved(row);
                    }

            }
        }
        _categoria_new
    })
    .map_err(|err|{
        AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}",err.to_string())),
            message: Some(format!("Todo list {:?} not found.", err.to_string().replace('"', ""))),
            debug_stores:Some(format!("{}",sql_query.replace('"', "'")))
        }
    });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}
pub async fn c_categoria_m_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<Value,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'{}'",object["datajson"]);
    sql_query.push_str("c_categoria_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query=sql_query.replace("\\", "").replace("\"'", "'").replace("'\"", "'");

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Value{
        let mut _categoria_new:Value=json!({});
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        
                        let _id:Option<i32> = row.get(0);
                        let _descripcion: Option<&str>= row.get(1);
                        let _id_padre: Option<i32>= row.get(2);
                        let _cantidad_sub:Option<i32>=row.get(3);
                        _categoria_new=json!({
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                            "pattern_id":_id_padre.unwrap(),
                            "limit_subcategorias":_cantidad_sub.unwrap()
                        });
                    } else { 
                        _categoria_new=database_mssql::errno_resolved(row);
                    }

            }
        }
        _categoria_new
    })
    .map_err(|err|{
        AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}",err.to_string())),
            message: Some(format!("Todo list {:?} not found.", err.to_string().replace('"', ""))),
            debug_stores:Some(format!("{}",sql_query.replace('"', "'")))
        }
    });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}
pub async fn c_categoria_b_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<Value,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'{}'",object["datajson"]);
    sql_query.push_str("c_categoria_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query=sql_query.replace("\\", "").replace("\"'", "'").replace("'\"", "'");

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Value{
        let mut _categoria_new:Value=json!({});
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {
                        let _id:Option<i32> = row.get(0);
                        let _descripcion: Option<&str>= row.get(1);
                        let _id_padre: Option<i32>= row.get(2);
                        let _cantidad_sub:Option<i32>=row.get(3);
                        _categoria_new=json!({
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                            "pattern_id":_id_padre.unwrap(),
                            "limit_subcategorias":_cantidad_sub.unwrap()
                        });
                    } else { 
                        _categoria_new=database_mssql::errno_resolved(row);
                    }

            }
        }
        _categoria_new
    })
    .map_err(|err|{
        AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}",err.to_string())),
            message: Some(format!("Todo list {:?} not found.", err.to_string().replace('"', ""))),
            debug_stores:Some(format!("{}",sql_query.replace('"', "'")))
        }
    });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}
pub async fn c_categoria_l_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<Vec<Value>,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'{}'",object["datajson"]);
    sql_query.push_str("c_categoria_abmlc_sp ");
    sql_query.push_str(&sql_param);
    sql_query=sql_query.replace("\\", "").replace("\"'", "'").replace("'\"", "'");

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Vec<Value>{
        let mut _categoria_list:Vec<Value>=Vec::new();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                
                let _id:Option<i32> = row.get(0);
                let _descripcion: Option<&str>= row.get(1);
                let _id_padre: Option<i32>= row.get(2);
                let _cantidad_sub:Option<i32>=row.get(3);
                _categoria_list.push(
                    json!({
                    "id":_id.unwrap(),
                    "descripcion":_descripcion.unwrap().to_string(),
                    "pattern_id":_id_padre.unwrap(),
                    "limit_subcategorias":_cantidad_sub.unwrap()
                }));

            }
        }
        _categoria_list
    })
    .map_err(|err|{
        AppError {
            error_type: NotFoundError,
            cause: Some(format!("{}",err.to_string())),
            message: Some(format!("Todo list {:?} not found.", err.to_string().replace('"', ""))),
            debug_stores:Some(format!("{}",sql_query.replace('"', "'")))
        }
    });
    match _vec {
        Ok(t) => Ok(t),
        Err(t) => Err(t),
    }
}