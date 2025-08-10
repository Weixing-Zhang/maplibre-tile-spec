use thiserror::Error;

pub type MltResult<T> = Result<T, MltError>;

#[derive(Error, Debug)]
pub enum MltError {
    #[error("Unable to parse property: {0}")]
    PropertyParseError(String),
    #[error("Unsupported key value type: {0}")]
    UnsupportedKeyType(String),
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Unsupported geometry type: {0}")]
    UnsupportedGeometryType(String),
    #[error("Failed to decode protobuf: {0}")]
    DecodeError(String),
    #[error("Failed to decode metadata: {0}")]
    MetadataDecodeError(String),
    #[error("Failed to decode rle: {0}")]
    RleDecodeError(#[from] serde_columnar::ColumnarError),
    #[error("Unsupported technique in decode_int_stream: {0:?}")]
    UnsupportedIntStreamTechnique(String),

    // --- Plumbing / external sources ---
    #[error(transparent)]
    Io(#[from] std::io::Error),

    // If you use prost or quick-protobuf, pick the right type here.
    #[error("Protobuf decode failed at offset={offset}")]
    Protobuf {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        offset: usize,
    },

    #[error(transparent)]
    RleDecode(#[from] serde_columnar::ColumnarError),

    // Varint & FastPFor (wrap concrete error types if available; otherwise make typed variants)
    #[error(transparent)]
    Varint(#[from] bytes_varint::Error), // allocation-free conversion

    #[error("FastPFor decode failed: expected={expected} got={got}")]
    FastPforDecode { expected: usize, got: usize },

    // --- Metadata & schema ---
    #[error("Missing required field `{field}`")]
    MissingField { field: &'static str },

    #[error("Invalid value for `{field}`: {value}")]
    InvalidFieldValue { field: &'static str, value: String }, // if you truly need text; prefer typed numbers/enums

    #[error("Unsupported technique at {level:?}: {technique:?}")]
    UnsupportedTechnique {
        level: ErrorLevel,                // Physical or Logical
        technique: TechniqueDiscriminant, // see below
    },

    // --- Decoding invariants / bounds ---
    #[error("Input length must be a multiple of {multiple_of}, got {got}")]
    InvalidLength { multiple_of: usize, got: usize },

    #[error("Run-length decode would exceed target length: runs={runs} target={target}")]
    RleOverflow { runs: usize, target: usize },

    // Morton specifics
    #[error("Coordinate {coordinate} too large for i32 (shift={shift})")]
    CoordinateOverflow { coordinate: u32, shift: u32 },

    #[error("Subtract overflow: {lhs} - {rhs}")]
    SubtractOverflow { lhs: i32, rhs: i32 },
}
