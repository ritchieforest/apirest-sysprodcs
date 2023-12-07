
pub mod database_mssql{
    use anyhow::Ok;
    use tiberius::error::Error;
    use tiberius::{Row, Query};
    use tiberius::{Client, Config, AuthMethod};
    use std::sync::{Arc, Mutex};
    use async_std::net::TcpStream;
    use serde_json::{json, Value};

    use crate::errors::{AppError, AppErrorType};
    pub async fn connect()->anyhow::Result<Client<TcpStream>>{
        let mut _config = Config::new();
        _config.host("localhost");
        _config.port(1433);
        _config.database("PRODUCTS");
        _config.authentication(AuthMethod::sql_server("sa", "R1C4RD0."));
        _config.trust_cert();
        let _tcp = TcpStream::connect(_config.get_addr()).await?;
        _tcp.set_nodelay(true)?;
        let client = Client::connect(_config, _tcp).await?;
        return Ok(client);
    }
    pub async fn resolve_data_stored(
        store_procedure: &str,

        conexion: Arc<Mutex<Client<TcpStream>>>
        //params: &mut Vec<T>,
    ) -> Result<Vec<Vec<Row>>,Error>
    {
        let data=conexion.clone();
        let mut auxdata=data.lock();
        let con_mut=auxdata.as_deref_mut().unwrap();
        let stream = con_mut.query(store_procedure,&[]).
        await?;

        let response=stream.into_results().await;
        return response
    }
    pub fn errno_resolved(row:Row)->Value{
        let mut _errno: Value = json!({});
        let _error_number: Option<i32> = row.get(0);
        let _error_state: Option<i32> = row.get(1);
        let _error_severity: Option<i32> = row.get(2);
        let _error_procedure: Option<&str> = row.get(3);
        let _error_line: Option<i32> = row.get(4);
        let _error_msg: Option<&str> = row.get(5);
        let _error_store: Option<&str> = row.get(7);
        _errno = json!({
            "status":false,
            "error_number":_error_number.unwrap(),
            "error_severity":_error_severity.unwrap(),
            "error_procedure":_error_procedure.unwrap().to_string(),
            "error_line":_error_line.unwrap(),
            "error_msg":_error_msg.unwrap().to_string(),
            "store":_error_store.unwrap().to_string()
        });
        return _errno
        
    }
}