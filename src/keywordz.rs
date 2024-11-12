use crate::SymAstNode;

/// Box<SymAstNode>
/// Child node in AST
pub type NChild = Box<SymAstNode>;

pub enum ArithmeticOp {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
}

pub enum RelationalOp {
    /// ==
    Eq,
    /// !=
    Ne,
    /// <<
    Lt,
    /// >>
    Gt,
    /// <=
    Le,
    /// >=
    Ge,
}

pub enum LogicalOp {
    /// &&
    And,
    /// ||
    Or,
}

pub enum BitwiseOp {
    /// &
    BitAnd,
    /// |
    BitOr,
    /// ^
    BitXor,
    /// ~
    BitNot,
    /// <<
    Shl,
    /// >>
    Shr,
}

pub enum AssignmentOp {
    /// =
    Assign,
    /// +=
    AddAssign,
    /// -=
    SubAssign,
    /// *=
    MulAssign,
    /// /=
    DivAssign,
    /// %=
    ModAssign,
    /// &=
    AndAssign,
    /// |=
    OrAssign,
    /// ^=
    XorAssign,
    /// <<=
    ShlAssign,
    /// >>=
    ShrAssign,
}

pub enum Operator {
    Arithmetic(ArithmeticOp),
    Relational(RelationalOp),
    Logical(LogicalOp),
    Bitwise(BitwiseOp),
    Assignment(AssignmentOp),
}

pub fn parse_operator(op_raw: &str) -> Operator {
    todo!();
}

pub enum ControlFlow {
    // Child must be parenthesized expression
    If {
        expression: NChild,
        body: NChild,
    },
    Else {
        expression: Option<NChild>,
        body: NChild,
    },
    Switch {
        expression: NChild,
        body: NChild,
    },
    Case {
        value: NChild,
        body: NChild,
    },
    Default,
    While {
        expression: NChild,
        body: NChild,
    },
    Do {
        expression: NChild,
        body: NChild,
    },
    For {
        initialization: NChild,
        condition: NChild,
        update: NChild,
    },
    Break,
    Continue,
    GoTo(NChild),
    Return,
}
