use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_identifier(
        &mut self,
        name: &str,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        // Builtins may be used as identifiers (e.g., returned from functions)
        let is_builtin = matches!(
            name,
            "plus"
                | "minus"
                | "times"
                | "divide"
                | "mod"
                | "equal"
                | "nonequal"
                | "less"
                | "lesseq"
                | "greater"
                | "greatereq"
                | "head"
                | "tail"
                | "cons"
                | "isnull"
                | "length"
                | "or"
                | "not"
                | "islist"
                | "eval"
                | "print"
                | "+"
                | "-"
                | "*"
                | "/"
        );

        if is_builtin {
            function.emit(Instruction::LoadAtom(name.to_string()));
            return Ok(());
        }

        let symbol_id = self.lookup_symbol(name)?;
        self.emit_load_symbol(symbol_id, function)
    }
}
