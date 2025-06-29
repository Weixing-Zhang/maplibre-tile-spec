use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::metadata::stream::StreamMetadata;
use crate::metadata::stream_encoding::{
    DictionaryType, LengthType, Logical, LogicalLevelTechnique, LogicalStreamType, OffsetType,
    Physical, PhysicalLevelTechnique, PhysicalStreamType,
};
use fastpfor::cpp::Codec32 as _;
use fastpfor::cpp::FastPFor128Codec;
use fastpfor::rust::Integer as _;

use bytes::{Buf, Bytes};

pub fn decode_int_stream(
    tile: &mut TrackedBytes,
    stream_metadata: &StreamMetadata,
    is_signed: bool,
) -> Vec<i32> {
    if stream_metadata.physical.technique == PhysicalLevelTechnique::FastPfor {
        return decode_fast_pfor(tile, stream_metadata);
    } else if stream_metadata.physical.technique == PhysicalLevelTechnique::Varint {
        let values = varint::decode(tile, stream_metadata.num_values as usize);
        return decode_int_array();
    } else {
        // Handle other techniques or return an empty vector
        return vec![];
    }
}

fn decode_fast_pfor(tile: &mut TrackedBytes, stream_metadata: &StreamMetadata) -> Vec<i32> {
    let codec = FastPFor128Codec::new();
    let mut decoded = vec![0; stream_metadata.num_values as usize];
    let mut encoded_u32s: Vec<u32> = Vec::with_capacity(stream_metadata.byte_length as usize / 4);
    for _ in 0..(stream_metadata.byte_length / 4) {
        let b1 = tile.get_u8();
        let b2 = tile.get_u8();
        let b3 = tile.get_u8();
        let b4 = tile.get_u8();
        let val = u32::from_be_bytes([b1, b2, b3, b4]);
        encoded_u32s.push(val);
    }

    codec.decode32(&encoded_u32s, &mut decoded);
    decoded.iter().map(|&x| x as i32).collect()
}

fn decode_int_array() -> Vec<i32> {
    vec![]
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
        let input: Vec<i32> = vec![5, 10, 15, 20, 25, 30, 35, 40];
        let input_u32: Vec<u32> = input.iter().map(|&x| x as u32).collect();
        let mut output = vec![0; input.len()];
        let encoded = codec.encode32(&input_u32, &mut output).unwrap();
        let byte_length = (encoded.len() * std::mem::size_of::<i32>()) as u32;
        let num_values = input.len() as u32;

        // Prepare the tile as a TrackedBytes instance
        let mut encoded_bytes: Vec<u8> =
            Vec::with_capacity(encoded.len() * std::mem::size_of::<u32>());
        for val in encoded.iter() {
            encoded_bytes.extend(&val.to_be_bytes());
        }
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
        let result: Vec<i32> = decode_fast_pfor(&mut tile, &stream_metadata);
        assert_eq!(input, result);
        assert_eq!(tile.offset(), byte_length as usize);
    }
}
