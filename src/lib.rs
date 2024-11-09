mod keywordz;
use core::panic;
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

#[derive(Clone)]
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
        symast
    }
}

// This is only for binary expressions, unary to be handled seperately
fn handle_arithmetic(
    line: &Vec<Token>,
    op_start: usize,
    op_end: usize,
    ast: &SymAst,
) -> (NChild, usize) {
    // Gonna retry this because v1 was melting my brain
    // I think a good way to do this is to first get the operator with the highest
    // presidence in the paren scope and create 2 slices with the op as the middle,
    // even if the operands are singular. If get_highest_op returns none, and the operand
    // if not parenthisezed just instantly return the value.
    // If get_highest_op returns Some, you can be certain it in not parenthesized
    // in the current scope
    let (top_op_option, op_index) = get_highest_op(line);
    if let Some(top_op) = top_op_option {
        let left = line[op_start..op_index].to_vec();
        let right = line[op_index + 1..op_end].to_vec();
        let op = parse_operator(top_op.as_str());
        let operation = SymAstNode::BinaryExpression {
            operands: [
                handle_arithmetic(&left, 0, left.len(), ast).0,
                handle_arithmetic(&right, 0, right.len(), ast).0,
            ],
            operator: op,
            parenthesized: false,
        };
        (Box::new(operation), op_end + 1)
    } else {
        // No operator, check if operand is parenthesized
        let operand = &line[op_start];
        if operand.raw.as_str() == "(" {
            // If parenthesized, recursively call fn on the inner expression
            handle_arithmetic(line, op_start + 1, op_end - 1, ast)
        } else {
            // If not parenthesized, operand is unary
            match operand.token_type {
                TokenType::Identifier => {
                    let idt = ast.identifier_map.get(&operand.raw);
                    // Check if identifier is in the identifier map, if not,
                    // variable does not exist
                    if let Some(identifier) = idt {
                        (
                            Box::new(SymAstNode::Identifier {
                                identifier_type: identifier.clone(),
                                name: operand.raw.clone(),
                            }),
                            op_end,
                        )
                    } else {
                        println!("{} is not a variable!", operand.raw);
                        panic!()
                    }
                }
                TokenType::Constant(cv) => {
                    (Box::new(SymAstNode::Constant(get_constant_val(cv))), op_end)
                }
                _ => panic!(),
            }
        }
    }
}

fn get_constant_val(value: Constant) -> ConstantVal {
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
    match raw {
        "int" => Some(Type::Int),
        // Todo, too tired rn lol, just want hello world to work
        _ => None,
    }
}
