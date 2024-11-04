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
    FunctionDefinition {
        type_specifier: NChild,
        function_name: String,
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
    Identifier {
        identifier_type: Identifier,
        name: String,
    },
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
    setup: bool,
}

impl SymAst {
    pub fn new() -> Self {
        return Self {
            identifier_map: HashMap::new(),
            std_map: HashMap::new(),
            root_node: NChild::new(SymAstNode::Body(Vec::new())),
            setup: true,
        };
    }
    /// Function which recursively collects nodes in a block and returns the block when reaching
    /// the block end (})
    pub fn read_block(&mut self, block_start: usize, tlines: &Vec<Vec<Token>>) -> Vec<NChild> {
        let mut block_lines = Vec::new();
        let mut block_end = false;
        for li in block_start..tlines.len() {
            let line = &tlines[li];
            if line.len() == 0 {
                continue;
            }
            if self.setup && line[0].token_type != TokenType::Include {
                self.setup = false;
            }
            match line[0].token_type {
                TokenType::Include => {
                    if !self.setup {
                        panic!("#include statements must be at top of file!!!");
                    }
                    for stdfn in self.std_map.get(&line[1].raw).expect("Unknown function!") {
                        let rt;
                        if stdfn.return_type == Type::Void {
                            rt = None;
                        } else {
                            rt = Some(stdfn.return_type.clone());
                        }
                        self.identifier_map
                            .insert(stdfn.identifier.clone(), Identifier::Function(rt))
                            .unwrap();
                    }
                }
                TokenType::Keyword => {
                    if let Some(t) = get_type(&line[0].raw) {
                        // This is either a function def or variable def
                        // Token 3 will be open paren if function and open bracket if array
                        if line[2].raw == "(".to_string() {
                            // Function def
                            // First get parameter list
                            let pclose = line.len() - 2;
                            let mut plist = Vec::new();
                            for i in (3..pclose).step_by(3) {
                                let ptype = get_type(&line[i].raw).unwrap();
                                let pid = &line[i + 1].raw;
                                plist.push(Box::new(SymAstNode::Identifier {
                                    identifier_type: Identifier::Variable(ptype),
                                    name: pid.to_string(),
                                }));
                                if line[i + 2].raw != ",".to_string() {
                                    break;
                                }
                            }
                            // Now define function
                            let func = SymAstNode::FunctionDefinition {
                                type_specifier: Box::new(SymAstNode::Type(t)),
                                function_name: line[1].raw.clone(),
                                parameter_list: plist,
                                body: Box::new(SymAstNode::Body(self.read_block(li + 2, tlines))),
                            };
                            block_lines.push(Box::new(func));
                        }
                    }
                }
                TokenType::SpecialChar(..) => match line[0].raw.as_str() {
                    "}" => {
                        block_end = true;
                    }
                    "{" => {}
                    _ => panic!("Do ur job lexer!!"),
                },
                _ => {}
            }
            if block_end {
                break;
            }
        }
        if !block_end {
            panic!();
        }
        return block_lines;
    }
    pub fn initialize_symast(tlines: Vec<Vec<Token>>) -> Self {
        let mut std_map = HashMap::new();
        // Will make a file later containing stdlib functions, will just add printf now
        // manually for testing though (for hello world ofc)
        let printf = StdFunction {
            identifier: "printf".to_string(),
            parameters: vec![StdFnParam::Defined(Type::String), StdFnParam::Multi],
            return_type: Type::Int,
        };
        std_map.insert("stdio.h".to_string(), vec![printf]);
        let mut symast = SymAst {
            std_map,
            identifier_map: HashMap::new(),
            root_node: Box::new(SymAstNode::Body(Vec::new())),
            setup: true,
        };
        let program_block = symast.read_block(0, &tlines);
        symast.root_node = Box::new(SymAstNode::Body(program_block));
        // Go through tokens and add to tree
        return symast;
    }
}

fn get_type(raw: &String) -> Option<Type> {
    let raw = raw.as_str();
    return match raw {
        "int" => Some(Type::Int),
        // Todo, too tired rn lol, just want hello world to work
        _ => None,
    };
}
