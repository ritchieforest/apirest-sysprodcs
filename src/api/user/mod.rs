pub mod route_users;
pub mod services;
pub mod controllers;


use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct UserLogging{
    pub id_user:i32,
    pub id_empresa:i32,
    pub user_active:bool,
    pub usuario:String,
    pub token:String
}
impl Default for UserLogging {
    fn default() -> Self {
        UserLogging {
            id_user: 0,               // Valor predeterminado para id_user
            id_empresa: 0,           // Valor predeterminado para id_employed
            user_active: false,           // Valor predeterminado para user_active
            usuario: String::default(),  // Valor predeterminado para usuario (cadena vacía)
            token: String::default(),  // Valor predeterminado para usuario (cadena vacía)

        }
    }
}