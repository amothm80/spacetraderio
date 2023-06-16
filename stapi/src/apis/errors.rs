use crate::models::message::ErrorContent as APIError;
use reqwest::Error as RError;
use serde_json::Error as SJError;
use std::error;
use std::fmt;

//#[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
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
        match self {
            STError::reqwesterror(e) => write!(f, "{}", e),
            STError::serdejerror(e) => write!(f, "{}", e),
            STError::stapierror(e) => writeln!(f, "{}", e),
            STError::stgeneralerror => write!(f, "General API Error"),
            //STError::stderror(_) => write!(f, "this is a space trader standard error"),
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
            //STError::stderror(ref e) => None,
        }
    }
}
