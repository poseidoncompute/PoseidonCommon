use crate::PoseidonError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonError {
    pub code: i16,
    pub message: String,
}


#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum HttpError {
    MalformedChunkLength,
    MalformedChunkEnd,
    MalformedContentLength,
    HeadersOverflow,
    StatusLineOverflow,
    AddressNotFound,
    RedirectLocationMissing,
    InfiniteRedirectionLoop,
    TooManyRedirections,
    InvalidUtf8InResponse,
    PunycodeConversionFailed,
    HttpsFeatureNotEnabled,
    PunycodeFeatureNotEnabled,
    BadProxy,
    BadProxyCreds,
    ProxyConnect,
    InvalidProxyCreds,
    Other(String),
}

impl From<minreq::Error> for PoseidonError {
    fn from(error: minreq::Error) -> Self {
        use minreq::Error as MinreqError;

        match error {
            MinreqError::InvalidUtf8InBody(inner_error) => {
                PoseidonError::InvalidUtf8(format!("{:?}", inner_error))
            }
            MinreqError::RustlsCreateConnection(inner_error) => {
                PoseidonError::Rustls(inner_error.into())
            }
            MinreqError::IoError(inner_error) => PoseidonError::IoErr(inner_error.into()),
            MinreqError::MalformedChunkLength => {
                PoseidonError::Http(HttpError::MalformedChunkLength)
            }
            MinreqError::MalformedChunkEnd => PoseidonError::Http(HttpError::MalformedChunkEnd),
            MinreqError::MalformedContentLength => {
                PoseidonError::Http(HttpError::MalformedContentLength)
            }
            MinreqError::HeadersOverflow => PoseidonError::Http(HttpError::HeadersOverflow),
            MinreqError::StatusLineOverflow => PoseidonError::Http(HttpError::StatusLineOverflow),
            MinreqError::AddressNotFound => PoseidonError::Http(HttpError::AddressNotFound),
            MinreqError::RedirectLocationMissing => {
                PoseidonError::Http(HttpError::RedirectLocationMissing)
            }
            MinreqError::InfiniteRedirectionLoop => {
                PoseidonError::Http(HttpError::InfiniteRedirectionLoop)
            }
            MinreqError::TooManyRedirections => PoseidonError::Http(HttpError::TooManyRedirections),
            MinreqError::InvalidUtf8InResponse => {
                PoseidonError::Http(HttpError::InvalidUtf8InResponse)
            }
            MinreqError::PunycodeConversionFailed => {
                PoseidonError::Http(HttpError::PunycodeConversionFailed)
            }
            MinreqError::HttpsFeatureNotEnabled => {
                PoseidonError::Http(HttpError::HttpsFeatureNotEnabled)
            }
            MinreqError::PunycodeFeatureNotEnabled => {
                PoseidonError::Http(HttpError::PunycodeFeatureNotEnabled)
            }
            MinreqError::BadProxy => PoseidonError::Http(HttpError::BadProxy),
            MinreqError::BadProxyCreds => PoseidonError::Http(HttpError::BadProxyCreds),
            MinreqError::ProxyConnect => PoseidonError::Http(HttpError::ProxyConnect),
            MinreqError::InvalidProxyCreds => PoseidonError::Http(HttpError::InvalidProxyCreds),
            MinreqError::Other(other_error) => {
                PoseidonError::Http(HttpError::Other(other_error.to_owned()))
            }
        }
    }
}
