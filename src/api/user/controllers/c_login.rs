
use crate::{errors::{AppError,AppErrorType::NotFoundError}, api::user::{self, UserLogging}}; 
use async_std::net::TcpStream;
use tiberius::Client;
use crate::database::database_mssql;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use jsonwebtoken::{
    encode,
    Header,
    EncodingKey,
};
use serde::{ Serialize, Deserialize };
use chrono::{ Utc, Duration };
use crate::extractors_token::Claims;




pub async fn c_user_inf_c_sp(
    conexion:Arc<Mutex<Client<TcpStream>>>,
    data: Value
) -> Result<UserLogging,AppError> {
    //declarar las variables para leer    

    let mut sql_query=String::new();
    let object=data.to_owned();
    let sql_param= format!("{},{}",object["user"],object["pass"]);
    sql_query.push_str("c_user_inf_login_c_sp ");
    sql_query.push_str(&sql_param);

    let mut _vec = 
    database_mssql::resolve_data_stored(&sql_query,conexion).
    await.map(|__vec| -> UserLogging{
        let mut _user_loggin:UserLogging=UserLogging::default();
        for i in __vec.into_iter() {

            for row in i.into_iter() {
                let _id_user: Option<i32> = row.get(2);
                let _id_empresa: Option<i32> = row.get(3);
                let _user_active: Option<bool> = row.get(0);
                let _usuario: Option<&str> = row.get(1);
                let _token=encode_token(_id_user.unwrap());

                _user_loggin=UserLogging{
                    id_user:_id_user.unwrap(),
                    id_empresa:_id_empresa.unwrap(),
                    user_active:_user_active.is_some(),
                    usuario:_usuario.unwrap().to_string(),
                    token:_token.to_string()
                };

            }
        }
        _user_loggin
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
#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

 
fn encode_token(id_user: i32) -> String {
    let id: i32 = id_user;
    let exp: usize = (Utc::now() + Duration::days(2)).timestamp() as usize;
    let claims: Claims = Claims { id:id.try_into().unwrap() , exp };
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret("asdñlkjasdfdoi12342edsssvasdghjlññññ".as_ref()),
    ).unwrap();
    return  token;
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    token: String
}

// Example for when the token is send through the body
// pub async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
//     let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
// 	&body.token,
// 	&DecodingKey::from_secret(secret.as_str().as_ref()),
// 	&Validation::new(Algorithm::HS256),
//     );

//     match token_result {
// 	Ok(token) => HttpResponse::Ok().json(DecodeResponse {
// 	    message: "Successfully logged in.".to_owned(),
// 	    id: token.claims.id,
// 	}),
// 	Err(e) => HttpResponse::Unauthorized().json(Response { message: e.to_string() }),
//     }
// }


