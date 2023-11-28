pub mod services;
pub mod controllers;
pub mod route_configuration;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct ModulesByUser{
    pub alta:bool,
    pub baja:bool,
    pub modi:bool,
    pub listado:bool,
    pub id:i32,
    pub descripcion:String,
    pub url:String,
    pub icon:String,
    pub id_pattern:i32,
    pub id_empresa:i32
}
impl Default for ModulesByUser {
    fn default() -> Self {
        ModulesByUser {
            alta:false,
            baja:false,
            modi:false,
            listado:false,
            id:0,
            descripcion:String::default(),
            url:String::default(),
            icon:String::default(),
            id_pattern:0,
            id_empresa:0

        }
    }
}