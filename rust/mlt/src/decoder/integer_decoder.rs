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
    // tile pos needs to be advanced by bytelength
    vec![]
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
        let codec = FastPFor128Codec::new();
        let input = vec![5, 10, 15, 20, 25, 30, 35, 40];
        let mut output = vec![0; input.len()];
        let encoded = codec.encode32(&input, &mut output).unwrap();
        let byte_length = encoded.len() * std::mem::size_of::<i32>();
        let num_values = input.len();
        println!(
            "Encoded: {:?}, Byte Length: {}, Num Values: {}",
            encoded, byte_length, num_values
        );

        // Comparing the jave encoded output with the expected output
        // let mut byte_vec: Vec<u8> = Vec::with_capacity(encoded.len() * 4);
        // for val in encoded {
        //     byte_vec.extend(&val.to_be_bytes());
        // }
        // let signed_bytes: Vec<i8> = byte_vec.iter().map(|b| *b as i8).collect();
        // println!("{:?}", signed_bytes);

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

        let mut decoded = vec![0; 10];
        let decoded = codec.decode32(encoded, &mut decoded).unwrap();
        assert_eq!(input, decoded);
    }
}
