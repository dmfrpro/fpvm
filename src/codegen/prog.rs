use crate::symbol_table::ScopeKind;
use crate::syntax::node::Node;

use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

use super::brancher::BranchType;
use super::generator::ReturnTarget;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_prog(
        &mut self,
        prog_node: &Node,
        _locals_node: &Node,
        body_node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let previous_scope = self.current_scope_id;

        let prog_scope = self.find_scope_by_owner_span(prog_node, ScopeKind::Prog)?;

        self.current_scope_id = Some(prog_scope);

        let brancher = self.loop_context.new_brancher(BranchType::Prog);
        let end_label = brancher.get_label("end".to_string());

        self.return_targets
            .push(ReturnTarget::Prog(end_label.clone()));

        let result = self.compile_prog_script(body_node, function);

        self.return_targets.pop();

        if result.is_ok() {
            function.emit(Instruction::Label(end_label));
        }

        self.current_scope_id = previous_scope;

        result
    }
}
