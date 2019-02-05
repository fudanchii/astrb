//! Abstract syntax tree representation for Ruby programming language.

/// Represent ruby source code as a list of expressions.
pub struct Root {
    expressions: Vec<Expression>,
}

/// Expression in ruby can be splitted into some more specific types.
/// For example class definition is an expression. a single literal value
/// is also an expression. Even an assignment is also an expression.
pub enum Expression {
    Literal(ValueVariants),
    Access(AccessVariants),
    Assignment(AssignmentVariants),
    ClassDefinition(ClassDefinitionVariants),
    ModuleDefinition(ModuleDefinition),
    MethodDefinition(MethodDefinitionVariants),
    MethodUndefinition(MethodUndefinition),
    Aliasing(AliasingVariants),
    SendMethod(SendMethodVariants),
    Operation(OperationVariants),
    LogicalOperation(LogicalOperationVariants),
    Branching(BranchingVariants),
    TernaryBranching(TernaryBranching),
    CaseMatching(CaseMatching),
    Loop(LoopVariants),
    Return(Option<Expression>),
    ExceptionHandling(ExceptionHandlingVariants),
    BEGINBlock(Vec<Expression>),
    ENDBlock(Vec<Expression>),
    FlipFlop(FlipFlopVariants),
}

/// Represent variants of literal value.
pub enum ValueVariants {
    Singleton(SingletonVariants),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Complex(ComplexLiteral),
    Rational(RationalLiteral),
    String(StringLiteral),
    Symbol(StringLiteral),
    HereDocument(StringLiteral),
    ExecuteString(StringLiteral),
    RegularExpression(RegularExpression),
    Array(ArrayLiteral),
    Hash(HashLiteral),
    Range(RangeLiteral),
}

/// In ruby, true, false, and nil are categoried as singleton value.
pub enum SingletonVariants {
    True,
    False,
    Nil,
}

/// Literal representation for signed integer.
pub struct IntegerLiteral(i64);

/// Literal representation for float.
pub struct FloatLiteral(f64);

/// Literal representation for rational number.
pub struct RationalLiteral(f64);

/// Literal representation for complex number.
pub struct ComplexLiteral(f64);


/// String literal representation, without quotes.
/// Quotes will be determined by each variants.
pub enum StringLiteral {
    Static(String),
    WithInterpolation(Vec<Expressions>),
}


/// Literal representation for regular expression 
pub struct RegularExpression {
    expression: StringLiteral,
    option: Option<RegularExpressionFlag>,
}

/// Represent regular expression flag,
/// I is case-insensitive
/// M is multi-line
pub enum RegularExpressionFlag {
    I,
    M,
}

/// Literal representation for array.
pub enum ArrayLiteral {
    /// Plain array is a list of expression.
    Plain(Vec<Expression>),

    /// Splat can be either '*var' or '*[v1, v2]'
    /// represent them as its own expression, see: ArrayExpression.
    Splat(ArrayExpression),

    /// Array may be interpolated, that is an array can be constructed
    /// as a literal which contains splat in its declaration,
    /// in this case, we categorized this array literal as interpolated.
    WithInterpolation(Vec<ArrayInterpolation>),
}

/// Specific expression that returns array.
/// It can be an array literal, or an access.
/// See: AccessVariants.
pub enum ArrayExpression {
    Literal(Box<ArrayLiteral>),
    Access(AccessVariants),
}

/// Represent each element in interpolated array.
pub enum ArrayInterpolation {
    Expression(Expression),
    Splat(ArrayExpression),
}

/// Literal representation for hash (map).
pub enum HashLiteral {
    /// Plain hash is a list of HashElement.
    Plain(Vec<HashElement>),

    /// In hash, splat can be either '**var' or '**{ a: v1 }'
    /// represent these as its own expression, see: HashExpression.
    Splat(HashExpression),

    /// Hash may be interpolated if it's constructed as a literal which contains splat
    /// in its declaration.
    WithInterpolation(Vec<HashInterpolation>),
}

/// Each element of a hash is either
/// a pair:
///     "key" => :value
/// or
/// a labeled value:
///     key: :value
///
/// Each of this need to be represented separately.
pub enum HashElement {
    Pair {
        key: Expression,
        value: Expression
    },

    WithLabel {
        key: StringLiteral,
        value: Expression
    },
}

/// A specific expression that returns hash.
/// It can be an array literal, or an access.
/// See: AccessVariants.
pub enum HashExpression {
    Literal(Box<HashLiteral>),
    Access(AccessVariants),
}

/// Represent each possible value for interpolated hash element
pub enum HashInterpolation {
    Element(HashElement),
    Splat(HashExpression),
}

/// range literals, if second bound is None, then it's an infinite/endless range
pub enum RangeLiteral {
    Inclusive(IntegerLiteral, Option<IntegerLiteral>),
    Exclusive(IntegerLiteral, Option<IntegerLiteral>),
}

/// Variant for access. Access is an invocation for variables.
/// Including self, instance variables, global variables and even constants.
pub enum AccessVariants {
    _Self,
    LocalVariable(Variable),
    InstanceVariable(Variable),
    ClassVariable(Variable),
    GlobalVariable(GlobalVariable),
    Constant(ConstantVariants),
}

/// A variable only has its name, represented as string.
pub struct Variable(String);

/// global variable is prefixed by dollar '$'
pub enum GlobalVariable {
    Plain(Variable),
    NthReference(IntegerLiteral),
    Colon,
    Splat,
    QuestionMark,
    Dollar,
    Tilde,
    Ampersand,
    Plus,
    Backtick,
    Aposthrope,
    Bang,
    AtSymbol,
}

/// Variants for constants.
pub enum ConstantVariants {
    /// Top level constant is '::A' in Ruby.
    TopLevel(Constant),

    /// Constant can be scoped (namespaced) in which it represented as a list of its
    /// namespaces, ended by the constant itself.
    /// e.g.
    ///     'ConstantVariants::Scoped(vec![Constant("A"), constant("B")])'
    /// is equal to 'A::B' in Ruby.
    Scoped(Vec<Constant>),

    /// Unscoped constants is all constants which accessed without its namespace,
    /// or simply not namespaced.
    Unscoped(Constant),

    /// Represents special constant '__FILE__' in Ruby.
    File,

    /// Represents special constant '__LINE__' in Ruby.
    Line,

    /// Represents special constant '__ENCODING__' in Ruby.
    Encoding,
}

/// Constant name representation.
pub struct Constant(String);

/// Variants of assignment expression.
pub enum AssignmentVariants {
    /// Assign to local variable.
    ToLocalVariable(Variable, Box<Expression>),

    /// Assign tot instance variable.
    ToInstanceVariable(Variable, Box<Expression>),

    /// Assign to class variable.
    ToClassVariable(Variable, Box<Expression>),

    /// Assign to global variable.
    ToGlobalVariable(GlobalVariable, Box<Expression>),

    /// Assign to constant.
    ToConstant(ConstantVariants, Box<Expression>),

    /// Assign to object attribute.
    ToAttribute(SendMethodAssignmentVariants),

    /// Destructuring, multiple left hand assignment.
    MultipleAssignment(MultipleLeftHandSide, MultipleRightHandSide),

    /// Assignment with binary operation.
    BinaryOperator(BinaryOperator, AccessVariants, Box<Expression>),

    /// Assignment with logical operation.
    LogicalOperator(LogicalOperator, AccessVariants, Box<Expression>),
}

/// All possible form of multiple left hand side elements.
pub enum MultipleLeftHandSideElement {
    /// Plain access is variables/constants like symbols.
    PlainAccess(AccessVariants),

    /// Attribute access, for assignment to instance attributes.
    AttributeAccess(AccessAttributeVariants),

    /// Multiple left hand side can be nested.
    Nested(MultipleLeftHandSide),
}

/// Represent multiple left hand side in an assignment.
pub struct MultipleLeftHandSide(Vec<MultipleLeftHandSideElement>);

/// Represent right hand side expression in multiple left hand side assignment.
pub struct MultipleRightHandSide(ArrayInterpolation);

/// Assignment operator for binary operation
/// e.g. `And` is &=
pub enum BinaryOperator {
    /// Add operator: `+`,
    /// `+=` in assignment.
    Add,

    /// Substract operator: `-`
    /// `-=` in assignment.
    Sub,

    /// Binary or operator: `|`
    /// `|=` in assignment.
    Or,

    /// Binary xor operator: `^`
    /// `^=` in assignment.
    Xor,

    /// Binary and operator: `&`
    /// `&=` in assignment.
    And,

    /// Multiplication operator: `*`
    /// `*=` in assignment.
    Multiply,

    /// Division operator: `/`
    /// `/=` in assignment.
    Divide,

    /// Modulus operator: `%`
    /// `%=` in assignment.
    Mod,

    /// Shift bits to the left: `<<`
    /// `<<=` in assignment.
    LeftShift,

    /// Shift bits to the right: `>>`
    /// `>>=` in assignment.
    RightShift,
}

/// Assignment operator for logical operation
/// e.g.
///     `And` is &&=
///     `Or` id ||=
pub enum LogicalOperator {
    Or,
    And,
}

/// Variants for class definition.
pub enum ClassDefinitionVariants {
    /// Plain class definition
    /// see: ClassDefinition.
    Class(ClassDefinition),

    /// Singleton class definition
    /// see: SingletonClassDefinition.
    Singleton(SingletonClassDefinition),
}

/// Represent class definition that is not a singleton (class << self)
pub struct ClassDefinition {
    /// Class has a name, and it's a constant.
    name: ConstantVariants,

    /// Class may inherits another class,
    /// which make the other class its parent.
    parent: Option<ConstantVariants>,

    /// Class definition may contains method declarations and
    /// whatnot, list them as expressions.
    expressions: Vec<Expression>,
}

/// Singleton class definition
/// i.e. `class << self`
/// This class definition doesn't have any name,
/// so it only represented by its list of expressions.
pub struct SingletonClassDefinition {
    expressions: Vec<Expression>,
}

/// Represent module definition, it's similar to class
/// but has not inheritance.
pub struct ModuleDefinition {
    /// Module name, a constant.
    name: ConstantVariants,

    /// Similar as class body, module
    /// definition may have list of expressions.
    expressions: Vec<Expression>,
}

pub enum MethodDefinitionVariants {
    Instance(InstanceMethod),
    Singleton(SingletonMethod),
}

pub struct InstanceMethod {
    name: VariableOrIndex,
    args: FormalArgument,
    expressions: Vec<Expression>,
}

pub struct SingletonMethod {
    name: Variable,
    args: FormalArgument,
    expression: Vec<Expression>,
}

pub enum VariableOrIndex {
    Variable(Variable),
    Index,
}

pub enum AliasingVariants {
    Method(StringLiteral, StringLiteral),
    GlobalVariable(GlobalVariable),
}

pub struct FormalArgument(
    Vec<PlainArgumentVariants>,
    Option<SplatsAndBlockArgumentVariants<Variable>>
);

pub struct DecomposedArgument(
    Vec<DecomposedArgumentVariants>,
    Option<SplatsAndBlockArgumentVariants<Variable>>
);

pub enum PlainArgumentVariants {
    Required(Variable),
    KeywordRequired(Variable),
    KeywordOptional(Variable, Expression),
    Optional(Variable, Expression),
    Decomposition(DecomposedArgument),
}

pub enum SplatsAndBlockArgumentVariants<VarArg> {
    Splat(VarArg),
    UnnamedSplat,
    KeyWordSplat(VarArg),
    UnnamedKeywordSplat,
    SplatThenKeywordSplat(VarArg, VarArg),
    Block(VarArg),
    SplatThenBlock(VarArg, VarArg),
    SplatThenKeywordSplatThenBlock(VarArg, VarArg, VarArg),
}

pub struct ProcArgument(
    Vec<ProcArgumentVariants>,
    Option<SplatsAndBlockArgumentVariants<MultipleLeftHandSideElement>>,
);

pub enum ProcArgumentVariants {
    PlainArgument(PlainArgumentVariants),
    MultipleLeftHandSide(MultipleLeftHandSideElement),
}

pub enum DecomposedArgumentVariants {
    Plain(Variable),
    Nested(DecomposedArgument),
}

pub struct MethodUndefinition(Vec<StringLiteral>);

pub enum AliasingVariants {
    Method {
        oldname: StringLiteral,
        newname: StringLiteral,
    },

    GlobalVariable {
        oldname: GlobalVariable,
        newname: GlobalVariable,
    },
}

pub enum SendMethodVariants {
    Singleton(SendMethod),
    WithReceiver(Expression, SendMethod),
}

pub enum SendMethodAssignmentVariants {
    Plain {
        receiver: Expression,
        method: SendMethod,
    },

    WithIndex {
        receiver: Expression,
        index: Expression,
        method: SendMethod,
    }
}

pub struct SendMethod {
    name: Variable,
    args: SendMethodArgument,
}

pub struct SendMethodArgument(Vec<ArgumentVariants>, Option<BlockArgument>);

pub enum ArgumentVariants {
    Expression(Expression),
    Splat(ArrayExpression),
    Keyword(HashElement),
    KeywordSplat(HashExpression),
}

pub enum BlockArgument {
    Pass(ProcAsArgumentVariants),
    BeginBlock(ProcArgument, Vec<Expression>),
}

pub enum ProcAsArgumentVariants {
    Variable(Variable),
    Expression(ProcExpressionVariants),
}

pub enum ProcExpressionVariants {
    Proc(ProcArgument, Vec<Expression>),
    Lambda(ProcArgument, Vec<Eexpression>),
    Stubby(ProcArgument, Vec<Expression>),
}

pub enum OperationVariants {
    Paren(Vec<Expression>),
    BinaryExpression {
        operator: BinaryOperator,
        lefthand: Expression,
        righthand: Expression,
    },
    Not(Expression),
}

pub enum LogicalOperationVariants {
    Equal(Expression, Expression),
    And(Expression, Expression),
    LowerPrecedenceAnd(Expression, Expression),
    Or(Expression, Expression),
    LowerPrecedenceOr(Expression, Expression),
    DoubleAmpersands(Expression, Expression),
    DoublePipes(Expression, Expression),
    Not(Expression),
    Match(RegularExpressionMatch),
}

pub enum BranchingVariants {
    If(BranchingIfVariants),
    Unless(BranchingUnlessVariants),
}

pub enum BranchingIfVariants {
    WithoutElse {
        condition: Expression,
        iftrue: Expression
    },

    WithElse(TernaryBranching),
    WithElsif(WithElsifBranching),
}

pub struct WithElsifBranching {
    condition: Expression,
    iftrue: Expression,
    elsif: BranchingIfVariants,
}

pub enum BranchingUnlessVariants {
    WithoutElse {
        condition: Expression,
        iffalse: Expression,
    },

    WithElse(TernaryBranching),
}

pub struct TernaryBranching {
    condition: Expression,
    iftrue: Expression,
    iffalse: Expression,
}

pub struct CaseMatching {
    condition: Option<Expression>,
    when: Vec<WhenDefinitionVariants>,
    default: Option<Expression>,
}

pub struct WhenDefinitionVariants {
    condition: ArrayInterpolation,
    iftrue: Expression,
}

pub enum LoopVariants {
    PreCondition(LoopConditionVariants),
    PostCondition(LooptConditionVariants),
    ForIn {
        assignee: MultipleLeftHandSideElement,
        iterator: ArrayExpression,
        expressions: Vec<Expression>,
    }
}

pub enum LoopConditionVariants {
    While(LoopStruct),
    Until(LoopStruct),
}

pub struct LoopStruct {
    condition: Expresssion,
    expressions: Vec<InLoopExpression>,
}

pub enum InLoopExpression {
    Plain(Expression),
    Break(Option<Expression>),
    Next(Option<Expression>),
    Redo,
}

pub enum ExceptionHandlingVariants {
    InlineRescue(Expression, Expression),
    DefRescue(Vec<Expression>, RescueBodyVariants),
    BeginRescue(Vec<Expression>, RescueBodyVariants),
}

pub enum RescueBodyVariants {
    Rescue(Vec<RescueBody>, Option<RescueEnsureOrElse>),
    Ensure(Vec<Expression>),
}

pub struct RescueBody {
    exceptions: Vec<ConstantVariants>,
    assignment: Option<AccessVariants>,
    expressions: (Vec<Expression>, Option<Retry>),
}

pub enum RescueEnsureOrElse {
    Ensure(Vec<Expression>),
    Else(Vec<Expression>),
}

pub struct Retry;

pub struct BEGINBlock(Vec<Expression>);
pub struct ENDBlock(Vec<Expression>);

pub enum FlipFlopVariants {
    Inclusive(FlipFLop),
    Exclusive(FlipFlop),
}

pub struct FlipFlop {
    flip: Expression,
    flop: Expression,
    expressions: Vec<Expression>,
} 

pub struct RegularExpressionMatch {
    regex: RegularExpression,
    expression: Expression,
}
