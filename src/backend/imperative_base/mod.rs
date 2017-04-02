pub mod size_of;
pub mod serialize;

pub mod utils;
pub mod container_utils;

mod tests;

#[derive(Debug)]
pub struct Block(pub Vec<Operation>);

#[derive(Debug)]
pub struct ResultBlock {
    block: Block,
    result_var: Var,
}

#[derive(Debug)]
pub enum Operation {
    Assign {
        name: Var,
        value: Expr,
    },
    AddCount(Expr),
    ForEachArray {
        array: Var,
        index: Var,
        typ: Var,
        block: Block,
    },
    MapValue {
        input: Var,
        output: Var,
        operation: MapOperation,
    },
    Block(Block),
}

#[derive(Debug)]
pub enum Literal {
    Number(String),
}

#[derive(Debug)]
pub enum Expr {
    InputData,
    Var(Var),
    Literal(Literal),
    ContainerField {
        value: Box<Expr>,
        field: String,
    },
    TypeCall {
        typ: CallType,
        type_name: String,
        input: Var,
    },
}

#[derive(Debug)]
pub enum MapOperation {
    ArrayLength,
    UnionTagToExpr(Vec<UnionTagCase>),
}

#[derive(Debug)]
pub struct UnionTagCase {
    pub variant_name: String,
    pub variant_var: Option<Var>,
    pub block: Block,
}

#[derive(Debug)]
pub enum CallType {
    SizeOf,
    Serialize,
    Deserialize,
}
impl CallType {
    pub fn short(&self) -> &str {
        match *self {
            CallType::SizeOf => "size_of",
            CallType::Serialize => "serialize",
            CallType::Deserialize => "deserialize",
        }
    }
}

#[derive(Debug)]
pub struct Var(pub String);
impl From<String> for Var {
    fn from(input: String) -> Var {
        Var(input)
    }
}

trait BaseCodegen : size_of::BaseSizeOf + serialize::BaseSerialize {}

impl BaseCodegen for ::ir::variant::SimpleScalarVariant {}
impl BaseCodegen for ::ir::variant::ContainerVariant {}
impl BaseCodegen for ::ir::variant::ArrayVariant {}
impl BaseCodegen for ::ir::variant::UnionVariant {}

fn codegen_for_type<'a>(typ: &'a ::ir::Type) -> &'a BaseCodegen {
    use ::ir::variant::Variant;
    match typ.variant {
        Variant::SimpleScalar(ref inner) => inner,
        Variant::Container(ref inner) => inner,
        Variant::Array(ref inner) => inner,
        Variant::Union(ref inner) => inner,
        _ => unimplemented!(),
    }
}