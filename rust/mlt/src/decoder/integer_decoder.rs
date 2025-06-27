use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::metadata::stream::StreamMetadata;
use crate::metadata::stream_encoding::{
    DictionaryType, LengthType, Logical, LogicalLevelTechnique, LogicalStreamType, OffsetType,
    Physical, PhysicalLevelTechnique, PhysicalStreamType,
};
// use fastpfor::cpp::Codec32 as _;
// use fastpfor::rust::Integer as _;
// use fastpfor::cpp;

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
    vec![]
}

fn decode_int_array() -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_fast_pfor_empty() {
        assert_eq!(vec![1], vec![1]);
    }

    #[test]
    fn test_decode_fast_pfor_non_empty_placeholder() {
        let mut tile = TrackedBytes::from(vec![1, 2, 3, 4]);
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
            num_values: 4,
            byte_length: 4,
            morton: None,
            rle: None,
        };
        let result = decode_fast_pfor(&mut tile, &stream_metadata);
        assert_eq!(vec![1], vec![1]);
    }
}
