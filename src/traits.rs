//! A collection of traits that allow for reading and writing values in the binary format.

use crate::{error::Error, private};

/// A trait for reading values from a binary exolvl file.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented for types outside of this crate.
pub trait Read: private::Sealed {
    /// Reads a value from the given input.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying reader returns an error.
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A trait to allow reading values from a binary exolvl file while adhering to a specific version of the format.
///
/// This allows reading old files while also supporting new features.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented for types outside of this crate.
pub trait ReadVersioned: private::Sealed {
    /// Reads a value from a given output. This method takes an additional parameter of type [`i32`] which describes the level format version.
    ///
    /// This method should almost never be called by outside users of the library. Use [`Read`] instead. If you *need* to use this method, take special care to pass the correct version for the specific level file you're using.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying reader returns an error.
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A trait for reading values from a binary exolvl file with additional context.
///
/// This trait is similar to [`Read`], but takes an additional generic parameter. This is sometimes necessary for reading enums.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented for types outside of this crate.
pub trait ReadContext: private::Sealed {
    /// Additional context to pass to `read_with`.
    type Context;

    /// Reads a value from a given output. This method takes an additional parameter as context.
    ///
    /// This method should almost never be called by outside users of the library. Use [`Read`] instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying reader returns an error.
    fn read_ctx(input: &mut impl std::io::Read, with: Self::Context) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A trait for writing values to a binary exolvl file.
///
/// # Sealed
///
/// This trait is sealed and cannot be implemented for types outside of this crate.
pub trait Write: private::Sealed {
    /// Writes a value to a given output.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying writer returns an error.
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error>;
}
