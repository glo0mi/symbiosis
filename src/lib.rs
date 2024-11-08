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
        /// Must be identifier, constant or BinaryExpression (for nested operations)
        operands: [NChild; 2],
        // Must not be unary or special operator
        operator: Operator,
        parenthesized: bool,
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
    Assignment {},
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
        let mut li = block_start;
        while li < tlines.len() {
            let line = &tlines[li];
            if line.len() == 0 {
                panic!("Do ur job lexer!!!");
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
                            let pclose = line.len() - 1;
                            let mut plist = Vec::new();
                            for i in (3..pclose).step_by(3) {
                                let ptype = get_type(&line[i].raw).unwrap();
                                let pid = &line[i + 1].raw;
                                plist.push(Box::new(SymAstNode::Identifier {
                                    identifier_type: Identifier::Variable(ptype),
                                    name: pid.to_string(),
                                }));
                                if line[i + 2].raw != ",".to_string() {
                                    if line[i + 2].raw == ")".to_string() {
                                        break;
                                    } else {
                                        panic!();
                                    }
                                }
                            }
                            // Now define function
                            let bodyc = self.read_block(li + 1, tlines);
                            li += bodyc.len();
                            let func = SymAstNode::FunctionDefinition {
                                type_specifier: Box::new(SymAstNode::Type(t)),
                                function_name: line[1].raw.clone(),
                                parameter_list: plist,
                                body: Box::new(SymAstNode::Body(bodyc)),
                            };
                            block_lines.push(Box::new(func));
                        }
                    } else {
                        match line[0].raw.as_str() {
                            // Inner body containing nodes
                            "while" => {}
                            "for" => {}
                            "if" => {}
                            "else" => {}
                            // Other
                            "auto" => {}
                            "break" => {}
                            "case" => {}
                            "const" => {}
                            "continue" => {}
                            "default" => {}
                            "do" => {}
                            "enum" => {}
                            "extern" => {}
                            "goto" => {}
                            "register" => {}
                            "return" => {}
                            "sizeof" => {}
                            "static" => {}
                            "struct" => {}
                            "switch" => {}
                            "typedef" => {}
                            "union" => {}
                            "volatile" => {}
                            _ => panic!(),
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
            li += 1;
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

// This is only for binary expressions, unary to be handled seperately
fn handle_arithmetic(line: &Vec<Token>, op_start: usize, op_end: usize) -> (NChild, usize) {
    let mut operation = &line[op_start..op_end + 1].to_vec();
    // First, handle unnesssary parenthesis (int a = (1 + 2) should be handled as if
    // no parenthesis)
    // The root node for the operation will always be an operator
    // Of course remember order of operations, refer to table 2-1 in the c book
    //
    // Get left and right operands
    let mut left = None;
    let mut right = None;
    let mut c_operator;
    let top_opi;
    (c_operator, top_opi) = get_highest_op(&operation);
    let mut cur_operation = None;
    let mut i = 0;
    while i < operation.len() {
        let token = &operation[i];
        if token.raw.as_str() == "(" {
            // Find matching closing paren, then recursively call function to
            // get ParenExpr
            let mut paren_id = 1;
            let mut p_match = 0;
            for t in operation.iter().skip(1) {
                match t.raw.as_str() {
                    "(" => paren_id += 1,
                    ")" => paren_id -= 1,
                    _ => {}
                }
                if paren_id == 0 {
                    break;
                } else {
                    p_match += 1;
                }
            }
            cur_operation = Some(handle_arithmetic(line, i + 1, p_match).0);
            i += p_match;
        } else {
            // need to process single vals and operators
        }
        let is_right = i > top_opi;
        if left.is_none() {
            left = cur_operation.take();
        } else if right.is_none() && is_right {
            right = cur_operation.take();
        } else {
            // Handle multi expression left / right operands
            if !is_right {
                let mut lop = left.take().unwrap();
                match lop.as_mut() {
                    SymAstNode::BinaryExpression {
                        operands,
                        operator,
                        parenthesized,
                    } => {
                        if *parenthesized {
                            left = Some(Box::new(SymAstNode::BinaryExpression {
                                operands: [lop, cur_operation.take().unwrap()],
                                operator: string_to_operator(&c_operator.take().unwrap()),
                                parenthesized: false,
                            }));
                        }
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    todo!();
}

fn get_highest_op(operation: &Vec<Token>) -> (Option<String>, usize) {
    todo!();
}

fn string_to_operator(raw: &String) -> Operator {
    todo!();
}

fn get_operator_presidence(op: &String) -> usize {
    return match op.as_str() {
        "*" | "/" | "%" => 2,
        "+" | "-" => 3,
        "<<" | ">>" => 4,
        "<" | "<=" | ">" | ">=" => 5,
        _ => panic!(),
    };
}

fn get_type(raw: &String) -> Option<Type> {
    let raw = raw.as_str();
    return match raw {
        "int" => Some(Type::Int),
        // Todo, too tired rn lol, just want hello world to work
        _ => None,
    };
}
