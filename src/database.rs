
pub mod database_mssql{
    use anyhow::Ok;
    use tiberius::error::Error;
    use tiberius::{Row, Query};
    use tiberius::{Client, Config, AuthMethod};
    use std::sync::{Arc, Mutex};
    use async_std::net::TcpStream;

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
}