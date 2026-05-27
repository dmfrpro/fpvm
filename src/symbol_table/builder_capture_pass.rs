use super::builder::{SpanKey, SymbolTableBuilder};

use crate::syntax::{Node, NodeKind};

use super::table::{FunctionId, SymbolKind};

impl SymbolTableBuilder {
    pub(super) fn collect_captures(&mut self, node: &Node) {
        match &node.kind {
            NodeKind::ProgramNode(inner) => {
                self.collect_captures(inner);
            }

            NodeKind::ElementsNode(elements) => {
                for elem in elements {
                    self.collect_captures(elem);
                }
            }

            NodeKind::ElementNode(inner) => {
                self.collect_captures(inner);
            }

            NodeKind::SetqNode(_name_node, value_node) => {
                // name_node is declaration/assignment target, not variable usage.
                self.collect_captures(value_node);
                self.collect_assignment_target(_name_node);
            }

            NodeKind::FuncNode(_name_node, _args_node, body_node) => {
                self.collect_captures_in_function(node, body_node);
            }

            NodeKind::LambdaNode(_args_node, body_node) => {
                self.collect_captures_in_function(node, body_node);
            }

            NodeKind::ProgNode(_locals_node, body_node) => {
                self.collect_captures_in_prog(node, body_node);
            }

            NodeKind::CondNode(cond, then_node, else_node) => {
                self.collect_captures(cond);
                self.collect_captures(then_node);

                if let Some(else_node) = else_node {
                    self.collect_captures(else_node);
                }
            }

            NodeKind::WhileNode(cond, body) => {
                self.collect_captures(cond);
                self.collect_captures(body);
            }

            NodeKind::ReturnNode(expr) => {
                self.collect_captures(expr);
            }

            NodeKind::QuoteNode(_) => {
                // Quote expression is data not executable code.
            }

            NodeKind::ListNode(inner) => {
                self.collect_captures(inner);
            }

            NodeKind::Identifier(name) => {
                self.collect_identifier_usage(name);
            }

            NodeKind::NullNode
            | NodeKind::BoolNode(_)
            | NodeKind::IntNode(_)
            | NodeKind::RealNode(_)
            | NodeKind::BreakNode
            | NodeKind::ErrorNode => {}
        }
    }

    fn collect_captures_in_function(&mut self, function_node: &Node, body_node: &Node) {
        let key = SpanKey::from(&function_node.span);

        let Some(scope_id) = self.scope_by_span.get(&key).copied() else {
            return;
        };

        let Some(function_id) = self.function_by_span.get(&key).copied() else {
            return;
        };

        self.scope_stack.push(scope_id);
        self.function_stack.push(function_id);

        self.collect_captures(body_node);

        self.function_stack.pop();
        self.scope_stack.pop();
    }

    fn collect_captures_in_prog(&mut self, prog_node: &Node, body_node: &Node) {
        let key = SpanKey::from(&prog_node.span);

        let Some(scope_id) = self.scope_by_span.get(&key).copied() else {
            return;
        };

        self.scope_stack.push(scope_id);

        self.collect_captures(body_node);

        self.scope_stack.pop();
    }

    fn collect_assignment_target(&mut self, node: &Node) {
        let NodeKind::Identifier(name) = &node.kind else {
            return;
        };

        self.collect_identifier_usage(name);
    }

    fn collect_identifier_usage(&mut self, name: &str) {
        let Some(symbol_id) = self.table.lookup(self.current_scope(), name) else {
            // Builtins like plus/less/head/tail are not stored in SymbolTable.
            // Existing SemanticAnalyzer should report real unknown names.
            return;
        };

        let Some(symbol) = self.table.symbol(symbol_id) else {
            return;
        };

        let symbol_kind = symbol.kind;
        let symbol_scope_id = symbol.scope_id;
        let symbol_function_id = symbol.function_id;

        // Globals are accessed through loadglobal, no capture needed.
        if symbol_kind == SymbolKind::Global || symbol_scope_id == self.table.global_scope_id {
            return;
        }

        // Function symbols are addressed by function label.
        // They do not need runtime captures.
        if symbol_kind == SymbolKind::Function {
            return;
        }

        let Some(owner_function_id) = symbol_function_id else {
            return;
        };

        let current_function_id = self.current_function();

        // Symbol belongs to current function: local/arg access, no capture.
        if owner_function_id == current_function_id {
            return;
        }

        self.add_capture_chain(current_function_id, owner_function_id, symbol_id);
    }

    fn add_capture_chain(
        &mut self,
        current_function_id: FunctionId,
        owner_function_id: FunctionId,
        symbol_id: super::SymbolId,
    ) {
        let mut function_id = current_function_id;

        loop {
            if function_id == owner_function_id {
                break;
            }

            if let Err(error) = self.table.add_capture(function_id, symbol_id) {
                self.push_error(error);
                return;
            }

            let parent_function_id = self
                .table
                .function(function_id)
                .and_then(|function| function.parent_function_id);

            match parent_function_id {
                Some(parent_function_id) => {
                    function_id = parent_function_id;
                }
                None => break,
            }
        }
    }
}
