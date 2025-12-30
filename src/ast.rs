// AST node types for Duck language

/// Binary operators for arithmetic, comparison, and logical operations
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Pow,      // **

    // Comparison
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    LtEq,     // <=
    Gt,       // >
    GtEq,     // >=

    // Logical
    And,      // and, &&
    Or,       // or, ||

    // String
    Concat,   // ++
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,      // -
    Not,      // not, !
}

/// Parts of an interpolated string
#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    /// Literal text portion
    Literal(String),
    /// Expression to be evaluated and converted to string
    Expr(Expr),
}

/// Assignment targets - where values can be assigned
#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    /// Simple variable: x
    Variable(String),
    /// Field access: object.field
    Field { object: Box<Expr>, field: String },
    /// Index access: list[index]
    Index { object: Box<Expr>, index: Box<Expr> },
}

/// Pattern for match expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Match a literal value
    Literal(Literal),
    /// Bind to a variable (catches anything)
    Variable(String),
    /// Wildcard - _ (catch-all, don't bind)
    Wildcard,
    /// Match a list structure
    List(Vec<Pattern>),
    /// Match a struct
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
}

/// Literal values in the source code
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Nil,
}

/// A match arm contains a pattern and the code/expression to execute if matched
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    /// Expression result (for expression-form match)
    pub expression: Option<Expr>,
    /// Block body (for statement-form match)
    pub body: Option<Vec<Statement>>,
}

/// Expressions - anything that produces a value
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A literal value (number, string, boolean, null)
    Literal(Literal),

    /// A variable reference
    Identifier(String),

    /// Binary operation: left op right
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    /// Unary operation: op operand
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    /// Function/method call: callee(args)
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },

    /// Field access: object.field
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },

    /// List/string indexing: list[index]
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    /// List literal: [1, 2, 3]
    List(Vec<Expr>),

    /// Lambda expression: [params] -> body
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },

    /// Struct instantiation: StructName { field: value, ... }
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
    },

    /// Ternary/conditional expression: if cond then a else b
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },

    /// Range expression: start..end or start..=end
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },

    /// String interpolation: "hello {name}!"
    StringInterpolation(Vec<StringPart>),

    /// Match expression (returns a value)
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
    },
}

/// Statements - things that do something but may not produce a value
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Variable declaration: let name = value
    Let {
        name: String,
        value: Expr,
    },

    /// Assignment: target = value (variable, field, or index)
    Assign {
        target: AssignTarget,
        value: Expr,
    },

    /// Expression as a statement (for side effects)
    Expression(Expr),

    /// Print statement: print expr
    Print(Expr),

    /// Block of statements
    Block(Vec<Statement>),

    /// Function definition: define name taking [params] as body
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// If statement: if condition then ... otherwise ...
    If {
        condition: Expr,
        then_block: Vec<Statement>,
        otherwise_block: Option<Vec<Statement>>,
    },

    /// Match statement
    Match {
        value: Expr,
        arms: Vec<MatchArm>,
    },

    /// Repeat loop: repeat count times ...
    Repeat {
        count: Expr,
        body: Vec<Statement>,
    },

    /// While loop: while condition do ...
    While {
        condition: Expr,
        body: Vec<Statement>,
    },

    /// For-each loop: for var in iterable do ...
    ForEach {
        variable: String,
        iterable: Expr,
        body: Vec<Statement>,
    },

    /// Struct definition: struct Name with [fields]
    StructDef {
        name: String,
        fields: Vec<String>,
    },

    /// Return statement: return value
    Return(Option<Expr>),

    /// Break statement: break
    Break,

    /// Continue statement: continue
    Continue,

    /// Assertion: honk condition [message]
    Honk {
        condition: Expr,
        message: Option<Expr>,
    },

    /// Push to list: list push value
    Push {
        list: Expr,
        value: Expr,
    },
}

/// A block is a statement with metadata about parsing
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The statement in this block
    pub statement: Statement,
    /// Whether this statement was preceded by "quack" (for duck-themed syntax)
    pub was_quacked: bool,
    /// Source line number for error reporting
    pub line: usize,
}

impl Block {
    /// Create a new block with the given statement
    pub fn new(statement: Statement, line: usize) -> Self {
        Block {
            statement,
            was_quacked: false,
            line,
        }
    }

    /// Create a new block that was quacked
    pub fn quacked(statement: Statement, line: usize) -> Self {
        Block {
            statement,
            was_quacked: true,
            line,
        }
    }
}

/// A complete Duck program is a list of blocks
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub blocks: Vec<Block>,
}

impl Program {
    pub fn new(blocks: Vec<Block>) -> Self {
        Program { blocks }
    }
}

// Display implementations for better error messages and debugging

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Pow => write!(f, "**"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::NotEq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::LtEq => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::GtEq => write!(f, ">="),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::Or => write!(f, "or"),
            BinaryOp::Concat => write!(f, "++"),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "not"),
        }
    }
}
