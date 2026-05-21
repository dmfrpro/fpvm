use super::{BytecodeFunction, CodeGenerator, CodegenError};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_identifier(
        &mut self,
        name: &str,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let symbol_id = self.lookup_symbol(name)?;
        self.emit_load_symbol(symbol_id, function)
    }
}
