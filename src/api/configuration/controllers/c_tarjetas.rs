
use crate::{errors::{AppError,AppErrorType::NotFoundError}}; 
use async_std::net::TcpStream;
use tiberius::Client;
use crate::database::database_mssql;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct TarjetaList{
    pub id:i32,
    pub descripcion:String
}
impl Default for TarjetaList {
    fn default() -> Self {
        TarjetaList {
            id:0,
            descripcion:String::default(),
        }
    }
}

pub async fn c_tarjetas_b_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<TarjetaList,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'B',{},{}",object["descripcion"],object["id"]);
    sql_query.push_str("c_tarjetas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> TarjetaList{
        let mut _cuota_new:TarjetaList=TarjetaList::default();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _id:Option<i32> = row.get(0);
                let _descripcion: Option<&str>= row.get(1);
                _cuota_new=TarjetaList{
                    id:_id.unwrap(),
                    descripcion:_descripcion.unwrap().to_string(),
                };

            }
        }
        _cuota_new
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

pub async fn c_tarjetas_m_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<TarjetaList,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'M',{},{}",object["descripcion"],object["id"]);
    sql_query.push_str("c_tarjetas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> TarjetaList{
        let mut _cuota_new:TarjetaList=TarjetaList::default();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _id:Option<i32> = row.get(0);
                let _descripcion: Option<&str>= row.get(1);
                _cuota_new=TarjetaList{
                    id:_id.unwrap(),
                    descripcion:_descripcion.unwrap().to_string(),
                };

            }
        }
        _cuota_new
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

pub async fn c_tarjetas_a_sp(conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
)-> Result<TarjetaList,AppError>{

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("'A',{},NULL",object["descripcion"]);
    sql_query.push_str("c_tarjetas_abmlc_sp ");
    sql_query.push_str(&sql_param);
    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> TarjetaList{
        let mut _cuota_new:TarjetaList=TarjetaList::default();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _id:Option<i32> = row.get(0);
                let _descripcion: Option<&str>= row.get(1);
                _cuota_new=TarjetaList{
                    id:_id.unwrap(),
                    descripcion:_descripcion.unwrap().to_string(),
                };

            }
        }
        _cuota_new
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

pub async fn c_tarjetas_l_sp(
    conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
) -> Result<Vec<TarjetaList>,AppError> {
    //declarar las variables para leer    

    let mut sql_query=String::new();
    let sql_param= format!("'L',NULL,NULL");
    sql_query.push_str("c_tarjetas_abmlc_sp ");
    sql_query.push_str(&sql_param);

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> Vec<TarjetaList>{
        let mut _list_cuotas:Vec<TarjetaList>=Vec::new();
        for i in __vec.into_iter() {
            for row in i.into_iter() {
                let _id: Option<i32>= row.get(0);
                let _descripcion: Option<&str>= row.get(1);
                _list_cuotas.push(TarjetaList{
                    id:_id.unwrap(),
                    descripcion:_descripcion.unwrap().to_string(),
                });

            }
        }
        _list_cuotas
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