use crate::syntax::node::{Node, NodeKind};

use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_quote(
        &mut self,
        quoted: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        self.compile_quoted_value(quoted, function)
    }

    fn compile_quoted_value(
        &mut self,
        node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &node.kind {
            NodeKind::NullNode => {
                function.emit(Instruction::LoadNull);
                Ok(())
            }

            NodeKind::BoolNode(value) => {
                function.emit(Instruction::LoadBool(*value));
                Ok(())
            }

            NodeKind::IntNode(value) => {
                function.emit(Instruction::LoadInt(*value));
                Ok(())
            }

            NodeKind::RealNode(value) => {
                function.emit(Instruction::LoadReal(*value));
                Ok(())
            }

            NodeKind::Identifier(name) => {
                // If the identifier refers to a function, emit LoadFunc so that
                // eval can call it later.
                if let Ok(symbol_id) = self.lookup_symbol(name) {
                    if let Some(symbol) = self.symbol_table.symbol(symbol_id) {
                        if symbol.kind == crate::symbol_table::SymbolKind::Function {
                            function.emit(Instruction::LoadFunc(symbol.label.clone()));
                            return Ok(());
                        }
                    }
                }
                function.emit(Instruction::LoadAtom(name.clone()));
                Ok(())
            }

            NodeKind::ElementNode(inner) => self.compile_quoted_value(inner, function),

            NodeKind::ListNode(inner) => self.compile_quoted_list(inner, function),

            NodeKind::ElementsNode(elements) => self.compile_quoted_elements(elements, function),

            NodeKind::QuoteNode(inner) => {
                // Nested quote is treated as data too.
                // If the language wants quote itself to appear as an atom,
                // this can later be changed to: loadatom quote; <inner>; makelist 2.
                self.compile_quoted_value(inner, function)
            }

            NodeKind::LambdaNode(args, body) => {
                function.emit(Instruction::LoadAtom("lambda".to_string()));
                self.compile_quoted_value(args, function)?;
                self.compile_quoted_value(body, function)?;
                function.emit(Instruction::MakeList(3));
                Ok(())
            }

            NodeKind::ProgramNode(_)
            | NodeKind::SetqNode(_, _)
            | NodeKind::FuncNode(_, _, _)
            | NodeKind::ProgNode(_, _)
            | NodeKind::CondNode(_, _, _)
            | NodeKind::WhileNode(_, _)
            | NodeKind::ReturnNode(_)
            | NodeKind::BreakNode
            | NodeKind::ErrorNode => Err(CodegenError::UnsupportedNode {
                message: format!("unsupported quoted node: {:?}", node.kind),
            }),
        }
    }

    fn compile_quoted_list(
        &mut self,
        inner: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &inner.kind {
            NodeKind::ElementsNode(elements) => self.compile_quoted_elements(elements, function),

            NodeKind::ElementNode(element) => {
                self.compile_quoted_value(element, function)?;
                function.emit(Instruction::MakeList(1));
                Ok(())
            }

            _ => {
                self.compile_quoted_value(inner, function)?;
                function.emit(Instruction::MakeList(1));
                Ok(())
            }
        }
    }

    fn compile_quoted_elements(
        &mut self,
        elements: &[Box<Node>],
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        for element in elements {
            self.compile_quoted_value(element, function)?;
        }

        function.emit(Instruction::MakeList(elements.len()));

        Ok(())
    }
}
