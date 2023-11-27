use std::sync::{Arc, Mutex};
use tiberius::Client;
use async_std::net::TcpStream;
pub struct AppState {
    pub db:Arc<Mutex<Client<TcpStream>>>,
    pub secret:String
}
