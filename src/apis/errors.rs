use crate::models::message::ErrorContent as APIError;
use reqwest::Error as RError;
use serde::Serializer;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Error as SJError;
use std::error;
use std::error::Error;
use std::fmt::{self, write};

//#[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub enum STError {
    reqwesterror(RError),
    serdejerror(SJError),
    stapierror(APIError),
    stgeneralerror,
}

impl From<RError> for STError {
    fn from(e: RError) -> Self {
        STError::reqwesterror(e)
    }
}

impl From<SJError> for STError {
    fn from(e: SJError) -> Self {
        STError::serdejerror(e)
    }
}

impl fmt::Display for STError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            STError::reqwesterror(_) => write!(f, "This is a reqwest error"),
            STError::serdejerror(_) => write!(f, "This is a serde json error"),
            STError::stapierror(_) => write!(f, "This is a space trader api error"),
            STError::stgeneralerror => write!(f, "this is a space trader api error"),
        }
    }
}

impl error::Error for STError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            STError::stgeneralerror => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            STError::stapierror(_) => None,
            STError::reqwesterror(ref e) => Some(e),
            STError::serdejerror(ref e) => Some(e),
        }
    }
}
