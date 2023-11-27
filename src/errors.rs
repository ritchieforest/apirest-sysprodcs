use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use tiberius::error::{Error as TiberiusError,TokenError};
use anyhow::Error as AnyError;
#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,

}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub debug_stores:Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
    pub fn store_params(&self)->String{
        match &*self {
            AppError{
                debug_stores:Some(debug_stores),
                ..
            }=>debug_stores.clone(),
            AppError{
                debug_stores:None,
                error_type:AppErrorType::NotFoundError,
                ..
            }=>"Problemas con el store".to_string(),
            _=>"Sin Store".to_string()
            
        }
    }
}

impl From<TiberiusError> for AppError {
    fn from(error: TiberiusError) -> AppError {
        AppError {
            message: Some(error.to_string()), 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
            debug_stores:None
        }
    }
}

impl From<TokenError> for AppError {
    fn from(error: TokenError) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
            debug_stores:None
        }
    }
}

impl From<AnyError> for AppError {
    fn from(error: AnyError) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
            debug_stores:None

        }
    }
}



impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub status:bool,
    pub error: String,
    pub store_params:String
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(
        AppErrorResponse {
                status:false,
                error: self.message(),
                store_params:self.store_params()
            }
        )
    }
}
 