use thiserror::Error;

use crate::metadata::stream_encoding::{LogicalLevelTechnique, PhysicalLevelTechnique};
pub type MltResult<T> = Result<T, MltError>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MltError {
    #[error("Unable to parse property: {0}")]
    PropertyParseError(String),
    #[error("Unsupported key value type: {0}")]
    UnsupportedKeyType(String),
    // #[error("Failed to read file: {0}")]
    // FileReadError(#[from] std::io::Error),
    #[error("Unsupported geometry type: {0}")]
    UnsupportedGeometryType(String),
    #[error("Failed to decode protobuf: {0}")]
    DecodeError(String),
    #[error("Failed to decode metadata: {0}")]
    MetadataDecodeError(String),
    // #[error("Failed to decode rle: {0}")]
    // RleDecodeError(#[from] serde_columnar::ColumnarError),
    #[error("Unsupported technique in decode_int_stream: {0:?}")]
    UnsupportedIntStreamTechnique(String),

    //---------------------------------------------------------
    // Refacotred to use `thiserror` for better error handling
    //---------------------------------------------------------
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RleDecode(#[from] serde_columnar::ColumnarError),

    // Till here: 2025-08-10
    #[error("Protobuf decode failed at offset={offset}")]
    Protobuf {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        offset: usize,
    },

    // ------------ Varint: the crate doesn't expose a concrete public error type
    // so wrap it as a source trait object (no String allocation on creation).
    #[error("Varint decode error")]
    Varint {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    // ------------ Typed, programmatically matchable variants
    #[error("FastPFor decode failed: expected={expected} got={got}")]
    FastPforDecode { expected: usize, got: usize },

    #[error("Missing required field `{field}`")]
    MissingField { field: &'static str },

    #[error("Invalid value for `{field}`: {value}")]
    InvalidFieldValue { field: &'static str, value: String },

    #[error("Unsupported technique at {level:?}: {technique:?}")]
    UnsupportedTechnique {
        level: ErrorLevel,
        technique: TechniqueDiscriminant,
    },

    #[error("Input length must be a multiple of {multiple_of}, got {got}")]
    InvalidLength { multiple_of: usize, got: usize },

    #[error("Run-length decode would exceed target length: runs={runs} target={target}")]
    RleOverflow { runs: usize, target: usize },

    #[error("Coordinate {coordinate} too large for i32 (shift={shift})")]
    CoordinateOverflow { coordinate: u32, shift: u32 },

    #[error("Subtract overflow: {lhs} - {rhs}")]
    SubtractOverflow { lhs: i32, rhs: i32 },
}

#[derive(Debug)]
pub enum ErrorLevel {
    Physical,
    Logical,
}

#[derive(Debug)]
pub enum TechniqueDiscriminant {
    Physical(PhysicalLevelTechnique),
    Logical(LogicalLevelTechnique),
}
