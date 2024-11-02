mod keywordz;
use std::collections::HashMap;

use keywordz::*;

use dekatron::*;

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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
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
    Void,
}

pub enum SymAstNode {
    // Keywords
    Type(Type),
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
    FunctionCall {
        identifier: NChild,
        parameter_list: Vec<NChild>,
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
    Identifier(Identifier),
}

enum Identifier {
    Variable(Type),
    /// Contains the return type
    Function(Option<Type>),
    TypeName,
    EnumConstant,
    Label,
    Macro,
}

pub enum StdFnParam {
    Defined(Type),
    Multi,
}

pub struct StdFunction {
    identifier: String,
    parameters: Vec<StdFnParam>,
    return_type: Type,
}

pub struct SymAst {
    identifier_map: HashMap<String, Identifier>,
    std_map: HashMap<String, Vec<StdFunction>>,
    root_node: NChild,
}

impl SymAst {
    pub fn new() -> Self {
        return Self {
            identifier_map: HashMap::new(),
            std_map: HashMap::new(),
            root_node: NChild::new(SymAstNode::Body(Vec::new())),
        };
    }
    pub fn initialize_symast(tlines: Vec<Vec<Token>>) -> Self {
        let mut symast = SymAst::new();
        // Will make a file later containing stdlib functions, will just add printf now
        // manually for testing though (for hello world ofc)
        let printf = StdFunction {
            identifier: "printf".to_string(),
            parameters: vec![StdFnParam::Defined(Type::String), StdFnParam::Multi],
            return_type: Type::Int,
        };
        symast.std_map.insert("stdio.h".to_string(), vec![printf]);
        // Go through tokens and add to tree
        for line in tlines {
            if line.len() == 0 {
                continue;
            }
            match line[0].token_type {
                TokenType::Include => {
                    for stdfn in symast.std_map.get(&line[1].raw).expect("Unknown function!") {
                        let rt;
                        if stdfn.return_type == Type::Void {
                            rt = None;
                        } else {
                            rt = Some(stdfn.return_type.clone());
                        }
                        symast
                            .identifier_map
                            .insert(stdfn.identifier.clone(), Identifier::Function(rt))
                            .unwrap();
                    }
                }
                TokenType::Keyword => {
                    if let Some(t) = get_type(&line[0].raw) {
                        // This is either a function def or variable def
                        // Token 3 will be open paren if function
                        if line[2].raw == "(".to_string() {
                            // Function def
                        }
                    }
                }
                _ => {}
            }
        }
        return symast;
    }
}

fn get_type(raw: &String) -> Option<Type> {
    let raw = raw.as_str();
    return match raw {
        "int" => Some(Type::Int),
        // Todo, too tired rn lol, just want hello world to work
        _ => None,
    }
}



