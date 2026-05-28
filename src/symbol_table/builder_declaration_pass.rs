use crate::syntax::{Node, NodeKind};

use super::SymbolTableError;
use super::builder::{SpanKey, SymbolTableBuilder};
use super::table::{FunctionKind, ScopeKind, SymbolKind};

impl SymbolTableBuilder {
    pub(super) fn visit_node(&mut self, node: &Node) {
        match &node.kind {
            NodeKind::ProgramNode(inner) => {
                self.visit_node(inner);
            }

            NodeKind::ElementsNode(elements) => {
                for elem in elements {
                    self.visit_node(elem);
                }
            }

            NodeKind::ElementNode(inner) => {
                self.visit_node(inner);
            }

            NodeKind::SetqNode(name_node, value_node) => {
                self.visit_setq(name_node, value_node);
            }

            NodeKind::FuncNode(name_node, args_node, body_node) => {
                self.visit_func(node, name_node, args_node, body_node);
            }

            NodeKind::LambdaNode(args_node, body_node) => {
                self.visit_lambda(node, args_node, body_node);
            }

            NodeKind::ProgNode(locals_node, body_node) => {
                self.visit_prog(node, locals_node, body_node);
            }

            NodeKind::CondNode(cond, then_node, else_node) => {
                self.visit_node(cond);
                self.visit_node(then_node);

                if let Some(else_node) = else_node {
                    self.visit_node(else_node);
                }
            }

            NodeKind::WhileNode(cond, body) => {
                self.visit_node(cond);
                self.visit_node(body);
            }

            NodeKind::ReturnNode(expr) => {
                self.visit_node(expr);
            }

            NodeKind::QuoteNode(_) => {
                // todo!("SymbolTable build doesn't resolve Quote node yet");
            }

            NodeKind::ListNode(inner) => {
                self.visit_node(inner);
            }

            NodeKind::NullNode
            | NodeKind::BoolNode(_)
            | NodeKind::IntNode(_)
            | NodeKind::RealNode(_)
            | NodeKind::Identifier(_)
            | NodeKind::BreakNode
            | NodeKind::ErrorNode => {}
        }
    }

    fn identifier_name<'a>(&mut self, node: &'a Node) -> Option<&'a str> {
        match &node.kind {
            NodeKind::Identifier(name) => Some(name.as_str()),
            _ => {
                self.push_error(SymbolTableError::UnknownSymbol {
                    name: "expected identifier".to_string(),
                    span: Some(node.span.clone()),
                });
                None
            }
        }
    }

    // if current scope Global -> identifier is global
    // if current scope isn't global -> identifier is local
    fn visit_setq(&mut self, name_node: &Node, value_node: &Node) {
        self.visit_node(value_node);

        let Some(name) = self.identifier_name(name_node) else {
            return;
        };

        // find in stacked scopes
        if self.table.lookup(self.current_scope(), name).is_some() {
            return;
        }

        let kind = if self.current_scope() == self.table.global_scope_id {
            SymbolKind::Global
        } else {
            SymbolKind::Local
        };

        let function_id = match kind {
            SymbolKind::Global => None,
            _ => Some(self.current_function()),
        };

        if let Err(error) = self.table.declare_symbol(
            name,
            kind,
            self.current_scope(),
            function_id,
            Some(name_node.span.clone()),
        ) {
            self.push_error(error);
        }
    }

    // declare function name in current scope
    // create scope for this function
    // creat FunctionInfo
    // declaring function arguments
    // visit function body
    fn visit_func(
        &mut self,
        func_node: &Node,
        name_node: &Node,
        args_node: &Node,
        body_node: &Node,
    ) {
        let Some(name) = self.identifier_name(name_node) else {
            return;
        };

        let function_symbol_id = self.table.declare_symbol_overwrite(
            name,
            SymbolKind::Function,
            self.current_scope(),
            Some(self.current_function()),
            Some(name_node.span.clone()),
        );

        let function_label = self
            .table
            .symbol(function_symbol_id)
            .expect("declared symbol must exist")
            .label
            .clone();

        let function_scope = self.table.create_scope(
            ScopeKind::Function,
            Some(self.current_scope()),
            None,
            Some(func_node.span.clone()),
        );

        let function_id = self.table.create_function(
            FunctionKind::Named,
            Some(name.to_string()),
            function_label,
            Some(self.current_function()),
            function_scope,
            Some(func_node.span.clone()),
        );

        self.table.scopes[function_scope].function_id = Some(function_id);

        // collecting function scopes into table
        let func_key = SpanKey::from(&func_node.span);
        self.scope_by_span.insert(func_key, function_scope);
        self.function_by_span.insert(func_key, function_id);

        self.scope_stack.push(function_scope);
        self.function_stack.push(function_id);

        self.declare_arguments(args_node);
        self.visit_node(body_node);

        self.function_stack.pop();
        self.scope_stack.pop();
    }

    fn visit_lambda(&mut self, lambda_node: &Node, args_node: &Node, body_node: &Node) {
        let lambda_scope = self.table.create_scope(
            ScopeKind::Lambda,
            Some(self.current_scope()),
            None,
            Some(lambda_node.span.clone()),
        );

        let function_id = self.table.create_function(
            FunctionKind::Lambda,
            None,
            format!("lambda_{}", self.table.functions.len()),
            Some(self.current_function()),
            lambda_scope,
            Some(lambda_node.span.clone()),
        );

        self.table.scopes[lambda_scope].function_id = Some(function_id);

        // collecting function scopes into table
        let lambda_key = SpanKey::from(&lambda_node.span);
        self.scope_by_span.insert(lambda_key, lambda_scope);
        self.function_by_span.insert(lambda_key, function_id);

        self.scope_stack.push(lambda_scope);
        self.function_stack.push(function_id);

        self.declare_arguments(args_node);
        self.visit_node(body_node);

        self.function_stack.pop();
        self.scope_stack.pop();
    }

    // creating scope but not creating a function
    fn visit_prog(&mut self, prog_node: &Node, locals_node: &Node, body_node: &Node) {
        let prog_scope = self.table.create_scope(
            ScopeKind::Prog,
            Some(self.current_scope()),
            Some(self.current_function()),
            Some(prog_node.span.clone()),
        );

        // collecting prog scope for table
        let prog_key = SpanKey::from(&prog_node.span);
        self.scope_by_span.insert(prog_key, prog_scope);

        self.scope_stack.push(prog_scope);

        self.declare_locals(locals_node);
        self.visit_node(body_node);

        self.scope_stack.pop();
    }

    // returning all Identifiers from `node`
    fn collect_list_identifiers<'a>(&mut self, node: &'a Node) -> Vec<&'a Node> {
        match &node.kind {
            NodeKind::ListNode(inner) => self.collect_list_identifiers(inner),

            NodeKind::ElementsNode(elements) => elements
                .iter()
                .filter_map(|elem| match &elem.kind {
                    NodeKind::Identifier(_) => Some(elem.as_ref()),
                    NodeKind::ElementNode(inner) => match &inner.kind {
                        NodeKind::Identifier(_) => Some(inner.as_ref()),
                        _ => None,
                    },
                    _ => None,
                })
                .collect(),

            _ => Vec::new(),
        }
    }

    // declaring function arguments
    fn declare_arguments(&mut self, args_node: &Node) {
        let args = self.collect_list_identifiers(args_node);

        for arg_node in args {
            let Some(name) = self.identifier_name(arg_node) else {
                continue;
            };

            if let Err(error) = self.table.declare_symbol(
                name,
                SymbolKind::Argument,
                self.current_scope(),
                Some(self.current_function()),
                Some(arg_node.span.clone()),
            ) {
                self.push_error(error);
            }
        }
    }

    // declaring local symbols for ProgNode
    fn declare_locals(&mut self, locals_node: &Node) {
        let locals = self.collect_list_identifiers(locals_node);

        for local_node in locals {
            let Some(name) = self.identifier_name(local_node) else {
                continue;
            };

            if let Err(error) = self.table.declare_symbol(
                name,
                SymbolKind::Local,
                self.current_scope(),
                Some(self.current_function()),
                Some(local_node.span.clone()),
            ) {
                self.push_error(error);
            }
        }
    }
}
