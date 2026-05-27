use super::error::{SemanticError, SemanticErrorKind};
use super::scope::{SymbolInfo, SymbolKind, SymbolTable};
use crate::syntax::{MultilinePosition, Node, NodeKind};

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
    in_function: Vec<bool>,
    in_loop: Vec<bool>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            in_function: vec![false],
            in_loop: vec![false],
        };

        for builtin in [
            "+",
            "-",
            "*",
            "/",
            "lesseq",
            "greater",
            "equal",
            "times",
            "minus",
            "plus",
            "mod",
            "print",
            "eval",
            "less",
            "nonequal",
            "greatereq",
            "head",
            "tail",
            "cons",
            "isnull",
            "length",
            "divide",
            "or",
            "not",
            "islist",
        ] {
            analyzer
                .symbol_table
                .insert(
                    builtin.to_string(),
                    SymbolInfo {
                        defined_at: MultilinePosition::default(),
                        kind: SymbolKind::Function,
                        is_prog_local: false,
                    },
                )
                .ok();
        }

        analyzer
    }

    pub fn analyze(mut self, root: &Node) -> Vec<SemanticError> {
        self.visit_node(root);
        self.errors
    }

    fn error(&mut self, kind: SemanticErrorKind, span: MultilinePosition, msg: Option<String>) {
        self.errors.push(SemanticError::new(kind, span, msg));
    }

    fn push_context(&mut self, function: bool, loop_: bool) {
        self.in_function.push(function);
        self.in_loop.push(loop_);
    }

    fn pop_context(&mut self) {
        self.in_function.pop();
        self.in_loop.pop();
    }

    fn is_inside_function(&self) -> bool {
        *self.in_function.last().unwrap_or(&false)
    }

    fn is_inside_loop(&self) -> bool {
        *self.in_loop.last().unwrap_or(&false)
    }

    fn declare_locals(&mut self, locals_node: &Node) {
        if let NodeKind::ListNode(elems) = &locals_node.kind {
            if let NodeKind::ElementsNode(local_nodes) = &elems.kind {
                for local in local_nodes {
                    if let NodeKind::Identifier(name) = &local.kind {
                        let info = SymbolInfo {
                            defined_at: local.span.clone(),
                            kind: SymbolKind::Variable,
                            is_prog_local: true,
                        };
                        if let Err(existing) = self.symbol_table.insert(name.clone(), info) {
                            self.error(
                                SemanticErrorKind::DuplicateDefinition(name.clone()),
                                local.span.clone(),
                                Some(format!(
                                    "local variable '{}' already defined at {}",
                                    name, existing.defined_at
                                )),
                            );
                        }
                    }
                }
            }
        }
    }

    fn add_parameters_to_scope(&mut self, params_node: &Node) {
        if let NodeKind::ListNode(elems) = &params_node.kind {
            if let NodeKind::ElementsNode(param_nodes) = &elems.kind {
                for param in param_nodes {
                    if let NodeKind::Identifier(name) = &param.kind {
                        let info = SymbolInfo {
                            defined_at: param.span.clone(),
                            kind: SymbolKind::Parameter,
                            is_prog_local: false,
                        };
                        if let Err(existing) = self.symbol_table.insert(name.clone(), info) {
                            self.error(
                                SemanticErrorKind::DuplicateDefinition(name.clone()),
                                param.span.clone(),
                                Some(format!(
                                    "parameter '{}' already defined at {}",
                                    name, existing.defined_at
                                )),
                            );
                        }
                    } else {
                        self.error(
                            SemanticErrorKind::InvalidLambdaParams,
                            param.span.clone(),
                            Some("lambda parameter must be an identifier".to_string()),
                        );
                    }
                }
            }
        } else {
            self.error(
                SemanticErrorKind::InvalidLambdaParams,
                params_node.span.clone(),
                Some("function parameters must be a list".to_string()),
            );
        }
    }

    fn visit_node(&mut self, node: &Node) {
        match &node.kind {
            NodeKind::ProgramNode(expr) => {
                self.visit_node(expr);
            }
            NodeKind::ElementsNode(exprs) => {
                for e in exprs {
                    self.visit_node(e);
                }
            }
            NodeKind::ListNode(elems) => {
                self.visit_node(elems);
            }
            NodeKind::Identifier(name) => {
                if self.symbol_table.lookup(name).is_none() {
                    self.error(
                        SemanticErrorKind::UndefinedVariable(name.clone()),
                        node.span.clone(),
                        Some(format!("undefined identifier '{}'", name)),
                    );
                }
            }
            NodeKind::QuoteNode(_sub) => {
                self.visit_node(_sub);
            }
            NodeKind::SetqNode(id_node, expr) => {
                if let NodeKind::Identifier(name) = &id_node.kind {
                    if let Some(existing) = self.symbol_table.current_scope().get(name) {
                        if !existing.is_prog_local {
                            self.error(
                                SemanticErrorKind::DuplicateDefinition(name.clone()),
                                id_node.span.clone(),
                                Some(format!(
                                    "variable '{}' already defined at {}",
                                    name, existing.defined_at
                                )),
                            );
                        }
                    } else {
                        let info = SymbolInfo {
                            defined_at: id_node.span.clone(),
                            kind: SymbolKind::Variable,
                            is_prog_local: false,
                        };
                        self.symbol_table.insert(name.clone(), info).ok();
                    }
                    self.visit_node(expr);
                } else {
                    self.error(
                        SemanticErrorKind::InvalidSetqTarget,
                        id_node.span.clone(),
                        Some("setq target must be an identifier".to_string()),
                    );
                    self.visit_node(expr);
                }
            }
            NodeKind::FuncNode(name_node, params_node, body_node) => {
                if let NodeKind::Identifier(fname) = &name_node.kind {
                    let info = SymbolInfo {
                        defined_at: name_node.span.clone(),
                        kind: SymbolKind::Function,
                        is_prog_local: false,
                    };
                    if let Err(existing) = self.symbol_table.insert(fname.clone(), info) {
                        self.error(
                            SemanticErrorKind::DuplicateDefinition(fname.clone()),
                            name_node.span.clone(),
                            Some(format!(
                                "function '{}' already defined at {}",
                                fname, existing.defined_at
                            )),
                        );
                    }
                } else {
                    self.error(
                        SemanticErrorKind::InvalidFuncName,
                        name_node.span.clone(),
                        Some("function name must be an identifier".to_string()),
                    );
                }

                self.symbol_table.enter_scope();

                self.add_parameters_to_scope(params_node);

                self.push_context(true, false);
                self.visit_node(body_node);
                self.pop_context();

                self.symbol_table.exit_scope();
            }
            NodeKind::LambdaNode(params_node, body_node) => {
                self.symbol_table.enter_scope();
                self.add_parameters_to_scope(params_node);
                self.push_context(true, false);
                self.visit_node(body_node);
                self.pop_context();
                self.symbol_table.exit_scope();
            }
            NodeKind::ProgNode(vars, body_node) => {
                self.symbol_table.enter_scope();
                self.push_context(true, self.is_inside_loop());
                self.declare_locals(vars);
                self.visit_node(body_node);
                self.pop_context();
                self.symbol_table.exit_scope();
            }
            NodeKind::CondNode(cond, then_expr, else_opt) => {
                if let NodeKind::ListNode(t) = &cond.kind {
                    if let NodeKind::ElementsNode(a) = &t.kind {
                        if a.len() == 0 {
                            self.error(
                                SemanticErrorKind::EmptyCond,
                                cond.span.clone(),
                                Some("Cond expression should not be empty".to_string()),
                            );
                        }
                    }
                }
                self.visit_node(cond);
                self.visit_node(then_expr);
                if let Some(else_node) = else_opt {
                    self.visit_node(else_node);
                }
            }
            NodeKind::WhileNode(cond, body) => {
                if let NodeKind::ListNode(t) = &cond.kind {
                    if let NodeKind::ElementsNode(a) = &t.kind {
                        if a.len() == 0 {
                            self.error(
                                SemanticErrorKind::EmptyCond,
                                cond.span.clone(),
                                Some("While condition should not be empty".to_string()),
                            );
                        }
                    }
                }

                self.visit_node(cond);
                self.push_context(self.is_inside_function(), true);
                self.visit_node(body);
                self.pop_context();
            }
            NodeKind::ReturnNode(expr) => {
                if !self.is_inside_function() {
                    self.error(
                        SemanticErrorKind::ReturnOutsideFunction,
                        node.span.clone(),
                        Some("return used outside of function".to_string()),
                    );
                }
                self.visit_node(expr);
            }
            NodeKind::BreakNode => {
                if !self.is_inside_loop() {
                    self.error(
                        SemanticErrorKind::BreakOutsideLoop,
                        node.span.clone(),
                        Some("break used outside of while loop".to_string()),
                    );
                }
            }
            NodeKind::NullNode
            | NodeKind::BoolNode(_)
            | NodeKind::IntNode(_)
            | NodeKind::RealNode(_)
            | NodeKind::ErrorNode => {}
            _ => {}
        }
    }
}
