pub struct Root {
    expressions: Vec<Expression>,
}

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
    MatchWithAssign(MatchWithAssign),
}

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

pub enum SingletonVariants {
    True,
    False,
    Nil,
}

pub struct IntegerLiteral(i64);
pub struct FloatLiteral(f64);
pub struct RationalLiteral(f64);
pub struct ComplexLiteral(f64);


/// String literal without quotes
pub enum StringLiteral {
    Static(String),
    WithInterpolation(Vec<Expressions>),
};

pub struct RegularExpression {
    expression: StringLiteral,
    option: Option<RegularExpressionFlag>,
}

pub enum RegularExpressionFlag {
    I,
    M,
}

pub enum ArrayLiteral {
    Plain(Vec<Expression>),
    Splat(ArrayExpression),
    WithInterpolation(Vec<ArrayInterpolation>),
}

pub enum ArrayExpression {
    Literal(ArrayLiteral),
    Variable(Variable),
}

pub enum ArrayInterpolation {
    Expression(Expression),
    Splat(ArrayExpression),
}

pub enum HashLiteral {
    Plain(Vec<HashElement>),
    Splat(HashExpression),
    WithInterpolation(Vec<HashInterpolation>),
}

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

pub enum HashExpression {
    Literal(HashLiteral),
    Variable(Variable),
}

pub enum HashInterpolation {
    Element(HashElement),
    Splat(HashExpression),
}

/// range literals, if second bound is None, then it's an infinite/endless range
pub enum RangeLiteral {
    Inclusive(IntegerLiteral, Option<IntegerLiteral>),
    Exclusive(IntegerLiteral, Option<IntegerLiteral>),
}

pub enum AccessVariants {
    _Self,
    LocalVariable(Variable),
    InstanceVariable(Variable),
    ClassVariable(Variable),
    GlobalVariable(GlobalVariable),
    Constant(ConstantVariants),
}

pub struct Variable(String);

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

pub enum ConstantVariants {
    TopLevel(Constant),
    Scoped(Vec<Constant>),
    Unscoped(Constant),
    File,
    Line,
    Encoding,
}

pub struct Constant(String);

pub enum AssignmentVariants {
    ToLocalVariable(Variable, Expression),
    ToInstanceVariable(Variable, Expression),
    ToClassVariable(Variable, Expression),
    ToGlobalVariable(GlobalVariable, Expression),
    ToConstant(ConstantVariants, Expression),
    ToAttribute(SendMethodAssignmentVariants),
    MultipleAssignment(MultipleLeftHandSide, MultipleRightHandSide),
    BinaryOperator(BinaryOperator, AccessVariants, Expression),
    LogicalOperator(LogicalOperator, AccessVariants, Expression),
}

pub enum MultipleLeftHandSideElement {
    PlainAccess(AccessVariants),
    AttributeAccess(AccessAttributeVariants),
    Nested(MultipleLeftHandSide),
}

pub struct MultipleLeftHandSide(Vec<MultipleLeftHandSideElement>);

pub struct MultipleRightHandSide(ArrayInterpolation);

pub enum BinaryOperator {
    Add,
    Sub,
    Or,
    And,
    Multiply,
    Divide,
}

pub enum LogicalOperator {
    Or,
    And,
    LeftShift,
    RightShift,
}

pub enum ClassDefinitionVariants {
    Class(ClassDefinition),
    Singleton(SingletonClassDefinition),
}

pub struct ClassDefinition {
    name: ConstantVariants,
    parent: Option<ConstantVariants>,
    expressions: Vec<Expression>,
}

pub struct SingletonClassDefinition {
    expressions: Vec<Expression>,
}

pub struct ModuleDefinition {
    name: ConstantVariants,
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

pub struct BlockArgument {
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

pub enum LogicalOperationVariants {
    And(Expression, Expression),
    Or(Expression, Expression),
    DoubleAmpersands(Expression, Expression),
    DoublePipes(Expression, Expression),
    Not(Expression),
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

pub struct MatchWithAssign {
    regex: RegularExpression,
    expression: Expression,
}
