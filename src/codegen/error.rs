#[derive(Debug, Clone)]
pub enum CodegenError {
    MissingSymbol { name: String },
    MissinfFunction { lebel: String },
    InternalError { message: String },
    InvalidNode { message: String },
    UnsupportedNode { message: String },
}
