use crate::common::SymbolId;
use crate::frontend::parse;
use crate::ir::infer::{DeclareVariableNode, Node};
use crate::ir::infer::Inference;

impl<'a> Inference<'a> {
    pub(crate) fn infer_declare_variable(&self, parsed_node: &'a parse::DeclareVariableNode) -> crate::ir::infer::Result<Node<'a>> {
        let node = self.infer_node(&parsed_node.node)?;
        let inferred_type = if let Some(type_node) = &parsed_node.r#type {
            self.type_from_type_node(type_node)?
        } else {
            node.inferred_type()
        };

        Ok(Node::DeclareVariable(DeclareVariableNode {
            parsed_node,
            symbol: SymbolId(1),
            node: Box::new(node),
            inferred_type,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::frontend::parse_str;
    use crate::ir::infer::{infer, InferredType};
    use crate::ir::infer::node::Node::DeclareVariable;

    #[test]
    fn declare_number_variable() {
        let mut ctx = Context::new();
        let mut parsed = parse_str(&mut ctx, "let value = 23").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Number)
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = Context::new();
        let mut parsed = parse_str(&mut ctx, "let value: Number = 23").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Number)
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = Context::new();
        let mut parsed = parse_str(&mut ctx, "let value: String = 'Elo'").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::String)
    }

    #[test]
    fn declare_boolean_variable_with_explicit_type() {
        let mut ctx = Context::new();
        let mut parsed = parse_str(&mut ctx, "let value = true").unwrap();
        let inferred = infer(&mut ctx, &mut parsed).unwrap();
        assert_eq!(inferred.nodes.len(), 1);

        let DeclareVariable(node) = &inferred[0] else { panic!() };
        assert_eq!(node.inferred_type, InferredType::Boolean)
    }
}