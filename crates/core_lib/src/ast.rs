use serde::{Deserialize, Serialize};

pub struct Program {
    pub using_block: Option<UsingBlock>,
    pub request_blocks: Vec<RequestBlock>,
    pub response_blocks: Vec<ResponseBlock>,
}

// == Variables Block ==
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsingBlock {
    pub var_declarations: Vec<VarDeclaration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VarDeclaration {
    pub name: String,
    pub value: String,
}

// == Request Block ==
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpMethods {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CacheDuration {
    None,
    DurationInSeconds(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBlock {
    pub name: String,
    pub method: HttpMethods,
    pub url: String,
    pub headers: Vec<Header>,
    pub cache: CacheDuration,
}

// == Response Block ==

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBlock {
    pub query: QueryBlock,
}

// TODO: Add Order By, Group By, Joins etc
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryBlock {
    pub select_clause: SelectClause,
    pub from_clause: FromClause,
    pub where_clause: Option<Expression>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SelectClause {
    Fields(Vec<String>),
    Objects(Vec<SelectField>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField {
    pub alias: String,
    pub expression: Option<Expression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FromClause {
    pub from_type: FromType,
    pub path: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FromType {
    Body,
    Response,
}

//      == Where Clause ==
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expression {
    LiteralExpr(Literal),
    FieldPathExpr(FieldPath),
    BinaryOpExpr {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
    RegexMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPath {
    pub path: Vec<String>,
}
