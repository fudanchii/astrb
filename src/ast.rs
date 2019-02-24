//! Abstract syntax tree representation for Ruby programming language.

/// Represent ruby source code as a list of expressions.
pub struct Root {
    pub(crate) expressions: Vec<Expression>,
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
    Return(Option<Box<Expression>>),
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
    HereDocument(HereDocumentVariants),
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
pub struct IntegerLiteral(pub(crate) i64);

/// Literal representation for float.
pub struct FloatLiteral(pub(crate) f64);

/// Literal representation for rational number.
pub struct RationalLiteral(pub(crate) f64);

/// Literal representation for complex number.
pub struct ComplexLiteral(pub(crate) f64);

/// String literal representation, without quotes.
/// Quotes will be determined by each variants.
pub enum StringLiteral {
    Static(String),
    WithInterpolation(Vec<Expression>),
}

/// Here document representations, there are 2 kinds
/// of here document in ruby, plain heredoc, and squiggly heredoc
pub enum HereDocumentVariants {
    Plain(HereDocument),
    Dash(HereDocument),
    Squiggly(HereDocument),
}

pub struct HereDocument {
    pub(crate) enclosure: Constant,
    pub(crate) document: StringLiteral,
}

/// Literal representation for regular expression
pub struct RegularExpression {
    pub(crate) expression: StringLiteral,
    pub(crate) options: Vec<RegularExpressionFlag>,
}

/// Represent regular expression flag,
/// E is fixed-encofing
/// I is case-insensitive
/// M is multi-line
/// N is no-encoding
/// U is unicode
/// X is extended
pub enum RegularExpressionFlag {
    E,
    I,
    M,
    N,
    U,
    X,
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
    Pair(PairElement),
    WithLabel(LabeledElement),
}

pub struct PairElement {
    pub(crate) key: Expression,
    pub(crate) value: Expression,
}

pub struct LabeledElement {
    pub(crate) key: StringLiteral,
    pub(crate) value: Expression,
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
pub struct Variable(pub(crate) String);

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
pub struct Constant(pub(crate) String);

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

pub struct AccessAttributeVariants {
    pub(crate) receiver: AccessVariants,
    pub(crate) attribute: Variable,
}

/// Represent multiple left hand side in an assignment.
pub struct MultipleLeftHandSide(pub(crate) Vec<MultipleLeftHandSideElement>);

/// Represent right hand side expression in multiple left hand side assignment.
pub struct MultipleRightHandSide(pub(crate) Box<ArrayInterpolation>);

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
    pub(crate) name: ConstantVariants,

    /// Class may inherits another class,
    /// which make the other class its parent.
    pub(crate) parent: Option<ConstantVariants>,

    /// Class definition may contains method declarations and
    /// whatnot, list them as expressions.
    pub(crate) expressions: Vec<Expression>,
}

/// Singleton class definition
/// i.e. `class << self`
/// This class definition doesn't have any name,
/// so it only represented by its list of expressions.
pub struct SingletonClassDefinition {
    pub(crate) expressions: Vec<Expression>,
}

/// Represent module definition, it's similar to class
/// but has not inheritance.
pub struct ModuleDefinition {
    /// Module name, a constant.
    pub(crate) name: ConstantVariants,

    /// Similar as class body, module
    /// definition may have list of expressions.
    pub(crate) expressions: Vec<Expression>,
}

pub enum MethodDefinitionVariants {
    Instance(InstanceMethod),
    Singleton(SingletonMethod),
}

pub struct InstanceMethod {
    pub(crate) name: VariableOrIndex,
    pub(crate) args: FormalArgument,
    pub(crate) expressions: Vec<Expression>,
}

pub struct SingletonMethod {
    pub(crate) name: Variable,
    pub(crate) args: FormalArgument,
    pub(crate) expressions: Vec<Expression>,
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
    pub(crate) Vec<PlainArgumentVariants>,
    pub(crate) Option<SplatsAndBlockArgumentVariants<Variable>>,
);

pub struct DecomposedArgument(
    pub(crate) Vec<DecomposedArgumentVariants>,
    pub(crate) Option<SplatsAndBlockArgumentVariants<Variable>>,
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
    pub(crate) Vec<ProcArgumentVariants>,
    pub(crate) Option<SplatsAndBlockArgumentVariants<MultipleLeftHandSideElement>>,
);

pub enum ProcArgumentVariants {
    PlainArgument(PlainArgumentVariants),
    MultipleLeftHandSide(MultipleLeftHandSideElement),
}

pub enum DecomposedArgumentVariants {
    Plain(Variable),
    Nested(DecomposedArgument),
}

pub struct MethodUndefinition(pub(crate) Vec<StringLiteral>);

pub struct AliasingMethod {
    pub(crate) oldname: StringLiteral,
    pub(crate) newname: StringLiteral,
}

pub struct AliasingVariable {
    pub(crate) oldname: GlobalVariable,
    pub(crate) newname: GlobalVariable,
}

pub enum SendMethodVariants {
    Singleton(SendMethod),
    WithReceiver(Box<Expression>, SendMethod),
}

pub enum SendMethodAssignmentVariants {
    Plain(SendMethodAssignment),
    WithIndex(SendMethodAssignmentWithIndex),
}

pub struct SendMethodAssignment {
    pub(crate) receiver: Box<Expression>,
    pub(crate) method: SendMethod,
}

pub struct SendMethodAssignmentWithIndex {
    pub(crate) receiver: Box<Expression>,
    pub(crate) index: Box<Expression>,
    pub(crate) method: SendMethod,
}

pub struct SendMethod {
    pub(crate) name: Variable,
    pub(crate) args: SendMethodArgument,
}

pub struct SendMethodArgument(
    pub(crate) Vec<ArgumentVariants>,
    pub(crate) Option<BlockArgument>,
);

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
    Lambda(ProcArgument, Vec<Expression>),
    Stubby(ProcArgument, Vec<Expression>),
}

pub enum OperationVariants {
    Paren(Vec<Expression>),
    BinaryExpression(BinaryExpressionOperation),
    Not(Box<Expression>),
}

pub struct BinaryExpressionOperation {
    pub(crate) operator: BinaryOperator,
    pub(crate) lefthand: Box<Expression>,
    pub(crate) righthand: Box<Expression>,
}

pub enum LogicalOperationVariants {
    Equal(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    LowerPrecedenceAnd(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    LowerPrecedenceOr(Box<Expression>, Box<Expression>),
    DoubleAmpersands(Box<Expression>, Box<Expression>),
    DoublePipes(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Match(RegularExpressionMatch),
}

pub enum BranchingVariants {
    If(BranchingIfVariants),
    Unless(BranchingUnlessVariants),
}

pub enum BranchingIfVariants {
    WithoutElse(BranchingIf),
    WithElse(TernaryBranching),
    WithElsif(WithElsifBranching),
}

pub struct BranchingIf {
    pub(crate) condition: Box<Expression>,
    pub(crate) iftrue: Box<Expression>,
}
pub struct WithElsifBranching {
    condition: Box<Expression>,
    iftrue: Box<Expression>,
    elsif: Box<BranchingIfVariants>,
}

pub enum BranchingUnlessVariants {
    WithoutElse(BranchingUnless),
    WithElse(TernaryBranching),
}

pub struct BranchingUnless {
    pub(crate) condition: Box<Expression>,
    pub(crate) iffalse: Box<Expression>,
}

pub struct TernaryBranching {
    pub(crate) condition: Box<Expression>,
    pub(crate) iftrue: Box<Expression>,
    pub(crate) iffalse: Box<Expression>,
}

pub struct CaseMatching {
    pub(crate) condition: Option<Box<Expression>>,
    pub(crate) when: Vec<WhenDefinitionVariants>,
    pub(crate) default: Option<Box<Expression>>,
}

pub struct WhenDefinitionVariants {
    pub(crate) condition: ArrayInterpolation,
    pub(crate) iftrue: Box<Expression>,
}

pub enum LoopVariants {
    PreCondition(LoopConditionVariants),
    PostCondition(LoopConditionVariants),
    ForIn(ForLoop),
}

pub struct ForLoop {
    pub(crate) assignee: MultipleLeftHandSideElement,
    pub(crate) iterator: ArrayExpression,
    pub(crate) expressions: Vec<Expression>,
}

pub enum LoopConditionVariants {
    While(LoopStruct),
    Until(LoopStruct),
}

pub struct LoopStruct {
    pub(crate) condition: Box<Expression>,
    pub(crate) expressions: Vec<InLoopExpression>,
}

pub enum InLoopExpression {
    Plain(Expression),
    Break(Option<Expression>),
    Next(Option<Expression>),
    Redo,
}

pub enum ExceptionHandlingVariants {
    InlineRescue(Box<Expression>, Box<Expression>),
    DefRescue(Vec<Expression>, RescueBodyVariants),
    BeginRescue(Vec<Expression>, RescueBodyVariants),
}

pub enum RescueBodyVariants {
    Rescue(Vec<RescueBody>, Option<RescueEnsureOrElse>),
    Ensure(Vec<Expression>),
}

pub struct RescueBody {
    pub(crate) exceptions: Vec<ConstantVariants>,
    pub(crate) assignment: Option<AccessVariants>,
    pub(crate) expressions: (Vec<Expression>, Option<Retry>),
}

pub enum RescueEnsureOrElse {
    Ensure(Vec<Expression>),
    Else(Vec<Expression>),
}

pub struct Retry;

pub struct BEGINBlock(pub(crate) Vec<Expression>);
pub struct ENDBlock(pub(crate) Vec<Expression>);

pub enum FlipFlopVariants {
    Inclusive(FlipFlop),
    Exclusive(FlipFlop),
}

pub struct FlipFlop {
    pub(crate) flip: Box<Expression>,
    pub(crate) flop: Box<Expression>,
    pub(crate) expressions: Vec<Expression>,
}

pub struct RegularExpressionMatch {
    pub(crate) regex: RegularExpression,
    pub(crate) expression: Box<Expression>,
}
