use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::encoder::integer::encoded_u32s_to_bytes;
use crate::metadata::stream::StreamMetadata;
use crate::metadata::stream_encoding::{
    DictionaryType, LengthType, Logical, LogicalLevelTechnique, LogicalStreamType, OffsetType,
    Physical, PhysicalLevelTechnique, PhysicalStreamType,
};
use crate::MltError;
use fastpfor::cpp::Codec32 as _;
use fastpfor::cpp::FastPFor128Codec;
use fastpfor::rust::Integer as _;

use bytes::{Buf, Bytes};

// decode_int_stream can handle multiple decoding techniques,
// some of which do represent signed integers (like varint with ZigZag)
// so returning Vec<i32>
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

// TODO (Weixing): can handle both integer and long
fn decode_zigzag() -> Result<Vec<i32>, MltError> {
    // Placeholder for ZigZag decoding logic
    Ok(vec![])
}

fn decode_zigzag_delta() -> Result<Vec<i32>, MltError> {
    // Placeholder for ZigZag delt decoding logic which requires decode_zigzag first
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
    fn test_decode_fast_pfor_empty() {
        assert_eq!(vec![1], vec![1]);
    }

    #[test]
    fn test_decode_fast_pfor_non_empty_placeholder() {
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
}
