use crate::PoseidonError;

impl From<sled::Error> for PoseidonError {
    fn from(sled_error: sled::Error) -> Self {
        use sled::Error as SledError;

        match sled_error {
            SledError::CollectionNotFound(byte_name) => match String::from_utf8(byte_name.to_vec())
            {
                Ok(collection_name) => PoseidonError::SledCollectionNotFound(collection_name),
                Err(_) => PoseidonError::InvalidByteToUtf8StringConversion,
            },
            SledError::Corruption { at, bt: _ } => {
                PoseidonError::SledCorruption(format!("{:?}", at))
            }
            SledError::ReportableBug(bug) => PoseidonError::SledReportableBug(bug),
            SledError::Unsupported(message) => PoseidonError::SledUnsupported(message),
            SledError::Io(io_error) => PoseidonError::IoErr(io_error.into()),
        }
    }
}
