
use crate::{errors::{AppError,AppErrorType::NotFoundError}}; 
use async_std::net::TcpStream;
use tiberius::Client;
use crate::database::database_mssql;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

pub async fn c_modules_by_user(
    conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
) -> Result<Vec<Value>,AppError> {
    //declarar las variables para leer    

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("{},{}",object["user"],object["empresa"]);
    sql_query.push_str("c_modules_by_user ");
    sql_query.push_str(&sql_param);

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Vec<Value>{
        let mut _list_user:Vec<Value>=Vec::new();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _type_result: Option<&str> = row.get("TipeResult");
                    if _type_result.unwrap().to_string().contains("Exito") {

                        let _alta: Option<bool> = row.get(0);
                        let _baja: Option<bool> = row.get(1);
                        let _modi: Option<bool> = row.get(2);
                        let _listado: Option<bool> = row.get(3);
                        let _id_empresa: Option<i32>= row.get(4);
                        let _id: Option<i32>= row.get(5);
                        let _descripcion: Option<&str>= row.get(6);
                        let _url: Option<&str>= row.get(7);
                        let _icon: Option<&str>= row.get(8);
                        let _id_pattern:Option<i32>=row.get(9);
        
                        _list_user.push(json!({
                            "alta":_alta.unwrap(),
                            "baja":_baja.unwrap(),
                            "modi":_modi.unwrap(),
                            "listado":_listado.unwrap(),
                            "id":_id.unwrap(),
                            "descripcion":_descripcion.unwrap().to_string(),
                            "url":_url.unwrap().to_string(),
                            "icon":_icon.unwrap().to_string(),
                            "id_pattern":_id_pattern.unwrap(),
                            "id_empresa":_id_empresa.unwrap()
                        }));
                    } else {
                        _list_user.push(database_mssql::errno_resolved(row)); 
                    }

            }
        }
        _list_user
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