use bytes_varint::VarIntError as BvVarIntError;
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
    // Structured, allocation-light
    //---------------------------------------------------------

    //---------------------------------------------------------
    // Parsing / validation
    //---------------------------------------------------------
    #[error("Property parse error: field={field} at={at:?} kind={kind:?}")]
    PropertyParse {
        field: &'static str,
        at: Option<usize>,
        kind: PropertyParseKind,
    },

    #[error("Unsupported key type: code={code}")]
    UnsupportedKeyTypeCode { code: u8 },

    #[error("Unsupported geometry type: code={code}")]
    UnsupportedGeometryCode { code: u32 },

    #[error("Missing required field `{field}`")]
    MissingField { field: &'static str },

    #[error("Invalid numeric value for `{field}`: got={got}")]
    InvalidNumeric { field: &'static str, got: u64 },

    #[error("Input length must be a multiple of {multiple_of}, got {got}")]
    InvalidLength { multiple_of: usize, got: usize },

    #[error("Index {index} is out of bounds for length {len}")]
    OutOfBounds { index: usize, len: usize },

    //---------------------------------------------------------
    // Decoding
    //---------------------------------------------------------
    #[error("Varint decode error: {0:?}")]
    Varint(#[from] VarintError),

    #[error("Protobuf decode error at offset={offset} kind={kind:?}")]
    Protobuf { offset: usize, kind: ProtobufError },

    #[error("Metadata decode error: field={field} kind={kind:?}")]
    MetadataDecode {
        field: &'static str,
        kind: MetadataErrorKind,
    },

    #[error("Run-length decode would exceed target length: runs={runs} target={target}")]
    RleOverflow { runs: usize, target: usize },

    #[error("FastPFor decode failed: expected={expected} got={got}")]
    FastPforDecode { expected: usize, got: usize },

    //---------------------------------------------------------
    // Arithmetic / numeric
    //---------------------------------------------------------
    #[error("Coordinate {coordinate} too large for i32 (shift={shift})")]
    CoordinateOverflow { coordinate: u32, shift: u32 },

    #[error("Subtract overflow: {lhs} - {rhs}")]
    SubtractOverflow { lhs: i32, rhs: i32 },

    //---------------------------------------------------------
    // Technique selection
    //---------------------------------------------------------
    #[error("Unsupported technique at {level:?}: {technique:?}")]
    UnsupportedTechnique {
        level: ErrorLevel,
        technique: TechniqueDiscriminant,
    },

    //---------------------------------------------------------
    // Passthrough (external errors; creating these is zero-alloc in our code)
    //---------------------------------------------------------
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RleDecode(#[from] serde_columnar::ColumnarError),
}

/// Where the technique applies.
#[derive(Debug)]
pub enum ErrorLevel {
    Physical,
    Logical,
}

/// Technique discriminant for messages.
#[derive(Debug)]
pub enum TechniqueDiscriminant {
    Physical(PhysicalLevelTechnique),
    Logical(LogicalLevelTechnique),
}

/// Fine-grained parse kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyParseKind {
    InvalidUtf8,
    TypeMismatch,
    NumberParse,
    UnexpectedNull,
    UnexpectedEnd,
    Other,
}

/// Varint failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum VarintError {
    #[error("Unexpected end of input while reading varint")]
    Eof,
    #[error("Varint too long")]
    TooLong,
    #[error("Varint overflowed target integer type")]
    Overflow,
    #[error("Varint not in canonical form")]
    NonCanonical,
}

/// Coarse protobuf failures (no heap message).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtobufError {
    Truncated,
    Malformed,
    InvalidTag,
    UnexpectedWireType,
    Utf8,
    Other,
}

/// Metadata decoding failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataErrorKind {
    Missing,
    Malformed,
    TypeMismatch,
    OutOfRange,
    Other,
}

// Helper functions
impl MltError {
    #[cold]
    #[inline(never)]
    pub fn protobuf(offset: usize, kind: ProtobufError) -> Self {
        Self::Protobuf { offset, kind }
    }

    #[cold]
    #[inline(never)]
    pub fn unsupported_physical(tech: PhysicalLevelTechnique) -> Self {
        Self::UnsupportedTechnique {
            level: ErrorLevel::Physical,
            technique: TechniqueDiscriminant::Physical(tech),
        }
    }

    #[cold]
    #[inline(never)]
    pub fn unsupported_logical(tech: LogicalLevelTechnique) -> Self {
        Self::UnsupportedTechnique {
            level: ErrorLevel::Logical,
            technique: TechniqueDiscriminant::Logical(tech),
        }
    }

    #[cold]
    #[inline(never)]
    pub fn invalid_multiple(multiple_of: usize, got: usize) -> Self {
        Self::InvalidLength { multiple_of, got }
    }
}

//---------------------------------------------------------
// bytes-varint integration
//---------------------------------------------------------
impl From<BvVarIntError> for MltError {
    fn from(e: BvVarIntError) -> Self {
        match e {
            BvVarIntError::BufferUnderflow => MltError::Varint(VarintError::Eof),
            BvVarIntError::NumericOverflow => MltError::Varint(VarintError::Overflow),
        }
    }
}
