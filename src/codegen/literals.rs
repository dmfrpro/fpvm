use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_null(
        &mut self,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        function.emit(Instruction::LoadNull);
        Ok(())
    }

    pub(crate) fn compile_bool(
        &mut self,
        value: bool,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        function.emit(Instruction::LoadBool(value));
        Ok(())
    }

    pub(crate) fn compile_int(
        &mut self,
        value: i64,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        function.emit(Instruction::LoadInt(value));
        Ok(())
    }

    pub(crate) fn compile_real(
        &mut self,
        value: f64,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        function.emit(Instruction::LoadReal(value));
        Ok(())
    }
}
