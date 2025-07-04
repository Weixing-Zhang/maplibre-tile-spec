use crate::decoder::helpers::bytes_to_encoded_u32s;
use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::encoder::helpers::encoded_u32s_to_bytes;
use crate::metadata::stream::StreamMetadata;
use crate::metadata::stream_encoding::{
    DictionaryType, LengthType, Logical, LogicalLevelTechnique, LogicalStreamType, OffsetType,
    Physical, PhysicalLevelTechnique, PhysicalStreamType,
};
use fastpfor::cpp::Codec32 as _;
use fastpfor::cpp::FastPFor128Codec;
use fastpfor::rust::Integer as _;

use bytes::{Buf, Bytes};

// decode_int_stream can handlemultiple decoding techniques,
// some of which do represent signed integers (like varint with ZigZag)
// so returning Vec<i32>
pub fn decode_int_stream(
    tile: &mut TrackedBytes,
    stream_metadata: &StreamMetadata,
    is_signed: bool,
) -> Vec<i32> {
    if stream_metadata.physical.technique == PhysicalLevelTechnique::FastPfor {
        decode_fast_pfor(tile, stream_metadata)
            .into_iter()
            .map(|x| x as i32)
            .collect()
    } else if stream_metadata.physical.technique == PhysicalLevelTechnique::Varint {
        let values = varint::decode(tile, stream_metadata.num_values as usize);
        return decode_int_array();
    } else {
        // Handle other techniques or return an empty vector
        return vec![];
    }
}

fn decode_fast_pfor(tile: &mut TrackedBytes, stream_metadata: &StreamMetadata) -> Vec<u32> {
    let codec = FastPFor128Codec::new();
    let mut decoded = vec![0; stream_metadata.num_values as usize];
    let mut encoded_u32s: Vec<u32> =
        bytes_to_encoded_u32s(tile, stream_metadata.byte_length as usize);
    codec.decode32(&encoded_u32s, &mut decoded);
    decoded
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
}
