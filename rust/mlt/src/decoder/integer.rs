use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::encoder::integer::encoded_u32s_to_bytes;
use crate::metadata::stream::{Morton, Rle, StreamMetadata};
use crate::metadata::stream_encoding::{
    DictionaryType, LengthType, Logical, LogicalLevelTechnique, LogicalStreamType, OffsetType,
    Physical, PhysicalLevelTechnique, PhysicalStreamType,
};
use crate::MltError;
use fastpfor::cpp::Codec32 as _;
use fastpfor::cpp::FastPFor128Codec;
use fastpfor::rust::Integer as _;

use bytes::{Buf, Bytes};
use num_traits::PrimInt;
use serde_columnar::ColumnarError;
use std::io;
use zigzag::ZigZag;

/// decode_int_stream can handle multiple decoding techniques,
/// some of which do represent signed integers (like varint with ZigZag)
/// so returning Vec<i32>
pub fn decode_int_stream(
    tile: &mut TrackedBytes,
    stream_metadata: &StreamMetadata,
    is_signed: bool,
) -> Result<Vec<i32>, MltError> {
    // Byte-level decoding based on the physical technique and stream type
    let values = match stream_metadata.physical.technique {
        PhysicalLevelTechnique::FastPfor => decode_fast_pfor(tile, stream_metadata),
        PhysicalLevelTechnique::Varint => varint::decode(tile, stream_metadata.num_values as usize),
        _ => {
            return Err(MltError::UnsupportedIntStreamTechnique(format!(
                "{:?}",
                stream_metadata.physical.technique
            )));
        }
    };

    let result = values.into_iter().map(|x| x as i32).collect();

    Ok(result)

    // Conceptual decoding based on the logical technique and type
}

fn decode_fast_pfor(tile: &mut TrackedBytes, stream_metadata: &StreamMetadata) -> Vec<u32> {
    let codec = FastPFor128Codec::new();
    let mut decoded = vec![0; stream_metadata.num_values as usize];
    let mut encoded_u32s: Vec<u32> =
        bytes_to_encoded_u32s(tile, stream_metadata.byte_length as usize);
    codec.decode32(&encoded_u32s, &mut decoded);
    decoded
}

fn bytes_to_encoded_u32s(tile: &mut TrackedBytes, num_bytes: usize) -> Vec<u32> {
    let num_bytes = num_bytes / 4;
    let mut encoded_u32s: Vec<u32> = Vec::with_capacity(num_bytes);
    for _ in 0..num_bytes {
        let b1 = tile.get_u8();
        let b2 = tile.get_u8();
        let b3 = tile.get_u8();
        let b4 = tile.get_u8();
        let val = u32::from_be_bytes([b1, b2, b3, b4]);
        encoded_u32s.push(val);
    }
    encoded_u32s
}

/// Decode RLE (Run-Length Encoding) data
/// It serves the same purpose as the `decodeUnsignedRLE` and `decodeRLE` methods in the Java code.
fn decode_rle<T: PrimInt>(data: &[T], rle_meta: &Rle) -> Result<Vec<T>, MltError> {
    let runs = rle_meta.runs as usize;
    let total = rle_meta.num_rle_values as usize;

    if data.len() != runs * 2 {
        return Err(MltError::RleDecodeError(
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Unexpected RLE data length: got {}, expected {}",
                    data.len(),
                    runs * 2
                ),
            )
            .into(),
        ));
    }

    let (run_lens, values) = data.split_at(runs);
    let mut result = Vec::with_capacity(total);
    for (&run, &val) in run_lens.iter().zip(values.iter()) {
        result.extend(std::iter::repeat(val).take(run.to_usize().unwrap()));
    }
    Ok(result)
}

/// Decode ZigZag encoded a vector of unsigned integers.
fn decode_zigzag<T: ZigZag>(data: &[T::UInt]) -> Result<Vec<T>, MltError> {
    let decoded = data.iter().map(|&v| T::decode(v)).collect();
    Ok(decoded)
}

// ---------------- The followings need to be imiplemented by their order ----------------
fn decode_zigzag_delta() -> Result<Vec<i32>, MltError> {
    // Placeholder for ZigZag delt decoding logic which requires decode_zigzag first
    Ok(vec![])
}

// Todo do decode_componentwise_delta_vec2 in vectorized/helpers.rs
fn decode_componentwise_delta_vec2() -> Result<Vec<i32>, MltError> {
    // Placeholder for componentwise_delta_vec2 decoding logic
    Ok(vec![])
}

fn decode_morton() -> Result<Vec<i32>, MltError> {
    // Placeholder for Morton decoding logic
    Ok(vec![])
}

fn decode_morton_code() -> Result<Vec<i32>, MltError> {
    // Placeholder for Morton decoding logic
    Ok(vec![])
}

fn decode_morton_codes() -> Result<Vec<i32>, MltError> {
    // Placeholder for Morton decoding logic
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use core::num;

    use super::*;

    #[test]
    fn test_decode_fast_pfor() {
        // Encode a sample input using FastPFor128Codec
        let codec = FastPFor128Codec::new();
        let input = vec![5, 10, 15, 20, 25, 30, 35];
        let mut output = vec![0; input.len()];
        let encoded = codec.encode32(&input, &mut output).unwrap();
        let byte_length = (encoded.len() * std::mem::size_of::<u32>()) as u32;
        let num_values = input.len() as u32;

        // Prepare the tile as a TrackedBytes instance
        let mut encoded_bytes: Vec<u8> = encoded_u32s_to_bytes(&encoded);
        let mut tile: TrackedBytes = encoded_bytes.into();

        // Create a StreamMetadata instance
        let stream_metadata = StreamMetadata {
            logical: Logical::new(
                Some(LogicalStreamType::Dictionary(None)),
                LogicalLevelTechnique::None,
                LogicalLevelTechnique::None,
            ),
            physical: Physical::new(
                PhysicalStreamType::Present,
                PhysicalLevelTechnique::FastPfor,
            ),
            num_values,
            byte_length,
            morton: None,
            rle: None,
        };
        let result = decode_fast_pfor(&mut tile, &stream_metadata);
        assert_eq!(input, result);
        assert_eq!(tile.offset(), byte_length as usize);
    }

    #[test]
    fn test_bytes_to_encoded_u32s() {
        let mut tile: TrackedBytes = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]
            .as_slice()
            .into();
        let result = bytes_to_encoded_u32s(&mut tile, 8);
        assert_eq!(result, vec![0x12345678, 0x90abcdef]);
    }

    #[test]
    fn test_decode_rle() {
        let rle_meta = Rle {
            runs: 3,
            num_rle_values: 6,
        };

        let data_u32: Vec<u32> = vec![3, 2, 1, 10, 20, 30];
        let decoded_u32 = decode_rle::<u32>(&data_u32, &rle_meta).unwrap();
        assert_eq!(decoded_u32, vec![10, 10, 10, 20, 20, 30]);

        let data_u64: Vec<u64> = vec![3, 2, 1, 10, 20, 30];
        let decoded_u64 = decode_rle::<u64>(&data_u64, &rle_meta).unwrap();
        assert_eq!(decoded_u64, vec![10, 10, 10, 20, 20, 30]);

        let data_i32: Vec<i32> = vec![3, 2, 1, -10, 20, 30];
        let decoded_i32 = decode_rle::<i32>(&data_i32, &rle_meta).unwrap();
        assert_eq!(decoded_i32, vec![-10, -10, -10, 20, 20, 30]);

        let data_i64: Vec<i64> = vec![3, 2, 1, -10, 20, 30];
        let decoded_i64 = decode_rle::<i64>(&data_i64, &rle_meta).unwrap();
        assert_eq!(decoded_i64, vec![-10, -10, -10, 20, 20, 30]);
    }

    #[test]
    fn test_decode_zigzag() {
        let encoded_u32 = vec![0u32, 1, 2, 3, 4, 5];
        let expected_i32 = vec![0i32, -1, 1, -2, 2, -3];
        let decoded_i32 = decode_zigzag::<i32>(&encoded_u32).unwrap();
        assert_eq!(decoded_i32, expected_i32);

        let encoded_u64 = vec![0u64, 1, 2, 3, 4, 5];
        let expected_i64 = vec![0i64, -1, 1, -2, 2, -3];
        let decoded_i64 = decode_zigzag::<i64>(&encoded_u64).unwrap();
        assert_eq!(decoded_i64, expected_i64);
    }
}
