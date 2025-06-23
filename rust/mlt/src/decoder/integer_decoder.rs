use crate::decoder::tracked_bytes::TrackedBytes;
use crate::decoder::varint;
use crate::metadata::stream::StreamMetadata;
use crate::metadata::stream_encoding::PhysicalLevelTechnique;

pub fn decode_int_stream(
    tile: &mut TrackedBytes,
    stream_metadata: &StreamMetadata,
    is_signed: bool,
) -> Vec<i32> {
    if stream_metadata.physical.technique == PhysicalLevelTechnique::FastPfor {
        // Weixing: need to implement the FastPfor decoding logic
        return decode_fast_pfor();
    } else if stream_metadata.physical.technique == PhysicalLevelTechnique::Varint {
        let values = varint::decode(tile, stream_metadata.num_values as usize);
        return decode_int_array();
    } else {
        // Handle other techniques or return an empty vector
        return vec![];
    }
}

fn decode_fast_pfor() -> Vec<i32> {
    vec![] // Placeholder for FastPfor decoding logic
}

fn decode_int_array() -> Vec<i32> {
    vec![] // Placeholder for Varint decoding logic
}
