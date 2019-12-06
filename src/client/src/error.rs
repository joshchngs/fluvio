
use std::fmt;
use std::io::Error as IoError;

use kf_socket::KfSocketError;


#[derive(Debug)]
pub enum ClientError {
    IoError(IoError),
    KfSocketError(KfSocketError)
}

impl From<IoError> for ClientError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl From<KfSocketError> for ClientError {
    fn from(error: KfSocketError) -> Self {
        Self::KfSocketError(error)
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::KfSocketError(err) => write!(f,"{:#?}",err)
        }
    }
}