mod keywordz;
use keywordz::*;

use dekatron::Constant as TConstant;

pub enum ConstantVal {
    String(String),
    Character(char),
    // Numeric types
    Int(i32),
    LongInt(i64),
    LongLongInt(i128),
    UInt(u32),
    ULongInt(u64),
    ULongLongInt(u128),
    Float(f32),
    Double(f64),
    //Long double not currently supported, Long doubles will be reduced to 64 bits
    //LongDouble(f)
}

enum Types {
    String,
    Character,
    // Numeric types
    Int,
    LongInt,
    LongLongInt,
    UInt,
    ULongInt,
    ULongLongInt,
    Float,
    Double,
}


pub enum SymAstNode {
    // Keywords
    Type(Types),
    Control(String),
    StorageClass(String),
    Structure(String),
    Return,
    SizeOf,
    Const,
    Enum,
    TypeDef,
    Volatile,
    // Identifier
    FunctionDefinition {
        type_specifier: NChild,
        function_name: NChild,
        parameter_list: Vec<NChild>,
        body: NChild,
    },
    // Constant
    Constant(ConstantVal),
    // Operations
    BinaryExpression {
        // Must be identifier or constant
        operands: [NChild; 2],
        // Must not be unary or special operator
        operator: Operator,
    },
    UnaryExpression {
        // Must be identifier
        operand: NChild,
        // Must be unary operator
        operator: Operator,
    },
    /// Parenthesized expression
    PsExpression(NChild),
    Body(Vec<NChild>),
}





