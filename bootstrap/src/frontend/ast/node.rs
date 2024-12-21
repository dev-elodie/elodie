use std::hash::Hash;
use std::rc::Rc;

use tree::CalculateNode;

use crate::common::{Column, Index, PackagePath, Position, Row, Span, StringTableId, tree};
use crate::common::tree::{AccessVariableNode, AccessVariableOfObjectNode, AccessVariableOfSelfNode, BlockNode, BreakLoopNode, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, ContinueLoopNode, DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareTypeNode, DeclareVariableNode, DefineTypeNode, ExportPackageNode, IfNode, InstantiateTypeNode, InterpolateStringNode, LiteralBooleanNode, LiteralNumberNode, LiteralStringNode, LoopNode, ReturnFromFunctionNode, Source, TreeNode, Variant};
use crate::frontend::lex::token::Token;
use crate::frontend::modifier::Modifiers;

#[derive(Clone, Debug, PartialEq)]
pub struct AstVariant {}

impl Variant for AstVariant {}

pub static SPAN_NOT_IMPLEMENTED: Span = Span {
    start: Position {
        row: Row(0),
        column: Column(0),
        index: Index(0),
    },
    end: Position {
        row: Row(0),
        column: Column(0),
        index: Index(0),
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct AstAccessVariableNode {
    pub variable: AstIdentifier,
}

impl AccessVariableNode<AstVariant> for AstAccessVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstAccessVariableOfObjectNode {
    pub object: AstIdentifier,
    pub variable: AstIdentifier,
}

impl AccessVariableOfObjectNode<AstVariant> for AstAccessVariableOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstAccessVariableOfSelfNode {
    pub variable: AstIdentifier,
}

impl AccessVariableOfSelfNode<AstVariant> for AstAccessVariableOfSelfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBlockNode {
    pub nodes: Vec<TreeNode<AstVariant>>,
}

impl BlockNode<AstVariant> for AstBlockNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBreakLoopNode {
    pub node: Option<Rc<TreeNode<AstVariant>>>,
}

impl BreakLoopNode<AstVariant> for AstBreakLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCalculateNode {
    pub left: Rc<TreeNode<AstVariant>>,
    pub operator: tree::CalculationOperator,
    pub right: Rc<TreeNode<AstVariant>>,
}

impl CalculateNode<AstVariant> for AstCalculateNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AStCallFunctionNode {
    pub function: AstIdentifier,
    pub arguments: Vec<TreeNode<AstVariant>>,
}

impl CallFunctionNode<AstVariant> for AStCallFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCallFunctionWithLambdaNode {
    pub function: AstIdentifier,
    pub arguments: Vec<TreeNode<AstVariant>>,
    pub lambda: Rc<AstBlockNode>,
}

impl CallFunctionWithLambdaNode<AstVariant> for AstCallFunctionWithLambdaNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCallFunctionOfObjectNode {
    pub object: AstIdentifier,
    pub function: AstIdentifier,
    pub arguments: Vec<TreeNode<AstVariant>>,
}

impl CallFunctionOfObjectNode<AstVariant> for AstCallFunctionOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCallFunctionOfPackageNode {
    pub package: PackagePath,
    pub function: AstIdentifier,
    pub arguments: Vec<TreeNode<AstVariant>>,
}

impl CallFunctionOfPackageNode<AstVariant> for AstCallFunctionOfPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCompareNode {
    pub left: Rc<TreeNode<AstVariant>>,
    pub operator: CompareOperator,
    pub right: Rc<TreeNode<AstVariant>>,
}

impl CompareNode<AstVariant> for AstCompareNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstContinueLoopNode {}

impl ContinueLoopNode<AstVariant> for AstContinueLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclareExternalFunctionNode {
    pub function: AstIdentifier,
    pub arguments: Vec<AstFunctionArgument>,
    pub return_type: Option<AstType>,
}

impl DeclareExternalFunctionNode<AstVariant> for AstDeclareExternalFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclareFunctionNode {
    pub function: AstIdentifier,
    pub arguments: Vec<AstFunctionArgument>,
    pub return_type: Option<AstType>,
    pub nodes: Rc<AstBlockNode>,
}

impl DeclareFunctionNode<AstVariant> for AstDeclareFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclarePackageNode {
    pub package: AstIdentifier,
    pub modifiers: Modifiers,
    pub external_functions: Vec<AstDeclareExternalFunctionNode>,
    pub functions: Vec<AstDeclareFunctionNode>,
    pub packages: Vec<AstDeclarePackageNode>,
    pub definitions: Vec<AstDefineTypeNode>,
}

impl DeclarePackageNode<AstVariant> for AstDeclarePackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclareTypeNode {
    pub r#type: AstIdentifier,
    pub modifiers: Modifiers,
    pub variables: Vec<TypeVariable>,
}

impl DeclareTypeNode<AstVariant> for AstDeclareTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefineTypeNode {
    pub r#type: AstIdentifier,
    pub modifiers: Modifiers,
    pub functions: Vec<AstDeclareFunctionNode>,
}

impl DefineTypeNode<AstVariant> for AstDefineTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclareVariableNode {
    pub variable: AstIdentifier,
    pub value: Rc<TreeNode<AstVariant>>,
    pub value_type: Option<AstType>,
}

impl DeclareVariableNode<AstVariant> for AstDeclareVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstExportPackageNode {
    pub package: AstIdentifier,
    pub source: Source,
}

impl ExportPackageNode<AstVariant> for AstExportPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstIfNode {
    pub condition: Rc<TreeNode<AstVariant>>,
    pub then: Rc<AstBlockNode>,
    pub otherwise: Option<Rc<AstBlockNode>>,
}

impl IfNode<AstVariant> for AstIfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstInterpolateStringNode {
    pub nodes: Vec<TreeNode<AstVariant>>,
}

impl InterpolateStringNode<AstVariant> for AstInterpolateStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstInstantiateTypeNode {
    pub r#type: AstIdentifier,
    pub arguments: Vec<AstNamedArgument>,
}

impl InstantiateTypeNode<AstVariant> for AstInstantiateTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLiteralBooleanNode(pub Token);

impl LiteralBooleanNode<AstVariant> for AstLiteralBooleanNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLiteralNumberNode(pub Token);

impl LiteralNumberNode<AstVariant> for AstLiteralNumberNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLiteralStringNode(pub Token);

impl LiteralStringNode<AstVariant> for AstLiteralStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLoopNode {
    pub nodes: Vec<TreeNode<AstVariant>>,
}

impl LoopNode<AstVariant> for AstLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstReturnFromFunctionNode {
    pub node: Option<Rc<TreeNode<AstVariant>>>,
}

impl ReturnFromFunctionNode<AstVariant> for AstReturnFromFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AstFunctionArgument {
    pub argument: AstIdentifier,
    pub argument_type: Option<AstType>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct AstIdentifier(pub StringTableId);

#[derive(Clone, Debug, PartialEq)]
pub struct AstNamedArgument {
    pub identifier: AstIdentifier,
    pub value: TreeNode<AstVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Boolean,
    Object,
    Number,
    String,
    Function { arguments: Vec<Box<AstType>>, return_type: Option<Box<AstType>> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVariable {
    pub variable: AstIdentifier,
    pub r#type: AstType,
}