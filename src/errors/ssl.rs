use borsh::{BorshDeserialize, BorshSerialize};
use rustls::internal::msgs::enums::{
    AlertDescription as RustlsAlertDescription, ContentType as RustlsContentType,
    HandshakeType as RustlsHandshakeType,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum RustlsError {
    InappropriateMessage {
        expect_types: Vec<ContentType>,
        got_type: ContentType,
    },
    InappropriateHandshakeMessage {
        expect_types: Vec<HandshakeType>,
        got_type: HandshakeType,
    },
    CorruptMessage,
    CorruptMessagePayload(ContentType),
    NoCertificatesPresented,
    UnsupportedNameType,
    DecryptError,
    EncryptError,
    PeerIncompatibleError(String),
    PeerMisbehavedError(String),
    AlertReceived(AlertDescription),
    InvalidCertificateEncoding,
    InvalidCertificateSignatureType,
    InvalidCertificateSignature,
    InvalidCertificateData(String),
    InvalidSct(SctError),
    General(String),
    FailedToGetCurrentTime,
    FailedToGetRandomBytes,
    HandshakeNotComplete,
    PeerSentOversizedRecord,
    NoApplicationProtocol,
    BadMaxFragmentSize,
}

impl From<rustls::Error> for RustlsError {
    fn from(error: rustls::Error) -> Self {
        match error {
            rustls::Error::InappropriateMessage {
                expect_types,
                got_type,
            } => {
                let expect_types = expect_types
                    .into_iter()
                    .map(|item| item.into())
                    .collect::<Vec<ContentType>>();

                RustlsError::InappropriateMessage {
                    expect_types,
                    got_type: got_type.into(),
                }
            }
            rustls::Error::InappropriateHandshakeMessage {
                expect_types,
                got_type,
            } => {
                let expect_types = expect_types
                    .into_iter()
                    .map(|item| item.into())
                    .collect::<Vec<HandshakeType>>();

                RustlsError::InappropriateHandshakeMessage {
                    expect_types,
                    got_type: got_type.into(),
                }
            }
            rustls::Error::CorruptMessage => RustlsError::CorruptMessage,
            rustls::Error::CorruptMessagePayload(value) => {
                RustlsError::CorruptMessagePayload(value.into())
            }
            rustls::Error::NoCertificatesPresented => RustlsError::NoCertificatesPresented,
            rustls::Error::UnsupportedNameType => RustlsError::UnsupportedNameType,
            rustls::Error::DecryptError => RustlsError::DecryptError,
            rustls::Error::EncryptError => RustlsError::EncryptError,
            rustls::Error::PeerIncompatibleError(value) => {
                RustlsError::PeerIncompatibleError(value)
            }
            rustls::Error::PeerMisbehavedError(value) => RustlsError::PeerMisbehavedError(value),
            rustls::Error::AlertReceived(value) => RustlsError::AlertReceived(value.into()),
            rustls::Error::InvalidCertificateEncoding => RustlsError::InvalidCertificateEncoding,
            rustls::Error::InvalidCertificateSignatureType => {
                RustlsError::InvalidCertificateSignatureType
            }
            rustls::Error::InvalidCertificateSignature => RustlsError::InvalidCertificateSignature,
            rustls::Error::InvalidCertificateData(value) => {
                RustlsError::InvalidCertificateData(value)
            }
            rustls::Error::InvalidSct(value) => RustlsError::InvalidSct(value.into()),
            rustls::Error::General(value) => RustlsError::General(value),
            rustls::Error::FailedToGetCurrentTime => RustlsError::FailedToGetCurrentTime,
            rustls::Error::FailedToGetRandomBytes => RustlsError::FailedToGetRandomBytes,
            rustls::Error::HandshakeNotComplete => RustlsError::HandshakeNotComplete,
            rustls::Error::PeerSentOversizedRecord => RustlsError::PeerSentOversizedRecord,
            rustls::Error::NoApplicationProtocol => RustlsError::NoApplicationProtocol,
            rustls::Error::BadMaxFragmentSize => RustlsError::BadMaxFragmentSize,
        }
    }
}

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum SctError {
    MalformedSct,
    InvalidSignature,
    TimestampInFuture,
    UnsupportedSctVersion,
    UnknownLog,
}

impl From<sct::Error> for SctError {
    fn from(error: sct::Error) -> Self {
        match error {
            sct::Error::MalformedSct => SctError::MalformedSct,
            sct::Error::InvalidSignature => SctError::InvalidSignature,
            sct::Error::TimestampInFuture => SctError::TimestampInFuture,
            sct::Error::UnsupportedSctVersion => SctError::UnsupportedSctVersion,
            sct::Error::UnknownLog => SctError::UnknownLog,
        }
    }
}

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum AlertDescription {
    CloseNotify,
    UnexpectedMessage,
    BadRecordMac,
    DecryptionFailed,
    RecordOverflow,
    DecompressionFailure,
    HandshakeFailure,
    NoCertificate,
    BadCertificate,
    UnsupportedCertificate,
    CertificateRevoked,
    CertificateExpired,
    CertificateUnknown,
    IllegalParameter,
    UnknownCA,
    AccessDenied,
    DecodeError,
    DecryptError,
    ExportRestriction,
    ProtocolVersion,
    InsufficientSecurity,
    InternalError,
    InappropriateFallback,
    UserCanceled,
    NoRenegotiation,
    MissingExtension,
    UnsupportedExtension,
    CertificateUnobtainable,
    UnrecognisedName,
    BadCertificateStatusResponse,
    BadCertificateHashValue,
    UnknownPSKIdentity,
    CertificateRequired,
    NoApplicationProtocol,
    Unknown(u8),
}

impl From<RustlsAlertDescription> for AlertDescription {
    fn from(error: RustlsAlertDescription) -> Self {
        match error {
            RustlsAlertDescription::CloseNotify => AlertDescription::CloseNotify,
            RustlsAlertDescription::UnexpectedMessage => AlertDescription::UnexpectedMessage,
            RustlsAlertDescription::BadRecordMac => AlertDescription::BadRecordMac,
            RustlsAlertDescription::DecryptionFailed => AlertDescription::DecryptionFailed,
            RustlsAlertDescription::RecordOverflow => AlertDescription::RecordOverflow,
            RustlsAlertDescription::DecompressionFailure => AlertDescription::DecompressionFailure,
            RustlsAlertDescription::HandshakeFailure => AlertDescription::HandshakeFailure,
            RustlsAlertDescription::NoCertificate => AlertDescription::NoCertificate,
            RustlsAlertDescription::BadCertificate => AlertDescription::BadCertificate,
            RustlsAlertDescription::UnsupportedCertificate => {
                AlertDescription::UnsupportedCertificate
            }
            RustlsAlertDescription::CertificateRevoked => AlertDescription::CertificateRevoked,
            RustlsAlertDescription::CertificateExpired => AlertDescription::CertificateExpired,
            RustlsAlertDescription::CertificateUnknown => AlertDescription::CertificateUnknown,
            RustlsAlertDescription::IllegalParameter => AlertDescription::IllegalParameter,
            RustlsAlertDescription::UnknownCA => AlertDescription::UnknownCA,
            RustlsAlertDescription::AccessDenied => AlertDescription::AccessDenied,
            RustlsAlertDescription::DecodeError => AlertDescription::DecodeError,
            RustlsAlertDescription::DecryptError => AlertDescription::DecryptError,
            RustlsAlertDescription::ExportRestriction => AlertDescription::ExportRestriction,
            RustlsAlertDescription::ProtocolVersion => AlertDescription::ProtocolVersion,
            RustlsAlertDescription::InsufficientSecurity => AlertDescription::InsufficientSecurity,
            RustlsAlertDescription::InternalError => AlertDescription::InternalError,
            RustlsAlertDescription::InappropriateFallback => {
                AlertDescription::InappropriateFallback
            }
            RustlsAlertDescription::UserCanceled => AlertDescription::UserCanceled,
            RustlsAlertDescription::NoRenegotiation => AlertDescription::NoRenegotiation,
            RustlsAlertDescription::MissingExtension => AlertDescription::MissingExtension,
            RustlsAlertDescription::UnsupportedExtension => AlertDescription::UnsupportedExtension,
            RustlsAlertDescription::CertificateUnobtainable => {
                AlertDescription::CertificateUnobtainable
            }
            RustlsAlertDescription::UnrecognisedName => AlertDescription::UnrecognisedName,
            RustlsAlertDescription::BadCertificateStatusResponse => {
                AlertDescription::BadCertificateStatusResponse
            }
            RustlsAlertDescription::BadCertificateHashValue => {
                AlertDescription::BadCertificateHashValue
            }
            RustlsAlertDescription::UnknownPSKIdentity => AlertDescription::UnknownPSKIdentity,
            RustlsAlertDescription::CertificateRequired => AlertDescription::CertificateRequired,
            RustlsAlertDescription::NoApplicationProtocol => {
                AlertDescription::NoApplicationProtocol
            }
            RustlsAlertDescription::Unknown(value) => AlertDescription::Unknown(value),
        }
    }
}

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum ContentType {
    ChangeCipherSpec,
    Alert,
    Handshake,
    ApplicationData,
    Heartbeat,
    Unknown(u8),
}

impl From<RustlsContentType> for ContentType {
    fn from(error: RustlsContentType) -> Self {
        match error {
            RustlsContentType::ChangeCipherSpec => ContentType::ChangeCipherSpec,
            RustlsContentType::Alert => ContentType::Alert,
            RustlsContentType::Handshake => ContentType::Handshake,
            RustlsContentType::ApplicationData => ContentType::ApplicationData,
            RustlsContentType::Heartbeat => ContentType::Heartbeat,
            RustlsContentType::Unknown(value) => ContentType::Unknown(value),
        }
    }
}

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum HandshakeType {
    HelloRequest,
    ClientHello,
    ServerHello,
    NewSessionTicket,
    EndOfEarlyData,
    HelloRetryRequest,
    EncryptedExtensions,
    Certificate,
    ServerKeyExchange,
    CertificateRequest,
    ServerHelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
    CertificateURL,
    CertificateStatus,
    KeyUpdate,
    MessageHash,
    Unknown(u8),
}

impl From<RustlsHandshakeType> for HandshakeType {
    fn from(error: RustlsHandshakeType) -> Self {
        match error {
            RustlsHandshakeType::HelloRequest => HandshakeType::HelloRequest,
            RustlsHandshakeType::ClientHello => HandshakeType::ClientHello,
            RustlsHandshakeType::ServerHello => HandshakeType::ServerHello,
            RustlsHandshakeType::NewSessionTicket => HandshakeType::NewSessionTicket,
            RustlsHandshakeType::EndOfEarlyData => HandshakeType::EndOfEarlyData,
            RustlsHandshakeType::HelloRetryRequest => HandshakeType::HelloRetryRequest,
            RustlsHandshakeType::EncryptedExtensions => HandshakeType::EncryptedExtensions,
            RustlsHandshakeType::Certificate => HandshakeType::Certificate,
            RustlsHandshakeType::ServerKeyExchange => HandshakeType::ServerKeyExchange,
            RustlsHandshakeType::CertificateRequest => HandshakeType::CertificateRequest,
            RustlsHandshakeType::ServerHelloDone => HandshakeType::ServerHelloDone,
            RustlsHandshakeType::CertificateVerify => HandshakeType::CertificateVerify,
            RustlsHandshakeType::ClientKeyExchange => HandshakeType::ClientKeyExchange,
            RustlsHandshakeType::Finished => HandshakeType::Finished,
            RustlsHandshakeType::CertificateURL => HandshakeType::CertificateURL,
            RustlsHandshakeType::CertificateStatus => HandshakeType::CertificateStatus,
            RustlsHandshakeType::KeyUpdate => HandshakeType::KeyUpdate,
            RustlsHandshakeType::MessageHash => HandshakeType::MessageHash,
            RustlsHandshakeType::Unknown(value) => HandshakeType::Unknown(value),
        }
    }
}
