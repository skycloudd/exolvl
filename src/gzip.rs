use crate::error::Error;
use flate2::read::GzDecoder;
use std::io::{Cursor, Read, Write as _};

/// Extracts the compressed data from a gzip file.
///
/// This function is useful because regular .exolvl files are compressed by the game.
///
/// # Errors
///
/// This function will return an error if the bytes are not valid gzip data.
pub fn extract(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut reader = Cursor::new(bytes);
    let mut decoder = GzDecoder::new(&mut reader);

    let mut writer = Vec::new();
    decoder.read_to_end(&mut writer)?;

    Ok(writer)
}

/// Compresses the data using gzip.
///
/// This function is useful because regular .exolvl files are compressed by the game.
///
/// # Errors
///
/// This function will return an error if the data cannot be written.
pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let writer = Vec::new();
    let mut encoder = flate2::write::GzEncoder::new(writer, flate2::Compression::default());

    encoder.write_all(bytes)?;

    Ok(encoder.finish()?)
}
