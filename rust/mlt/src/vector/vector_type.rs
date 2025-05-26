#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorType {
    Flat,
    Const,
    Sequence,
    Dictionary,
    FsstDictionary,
}