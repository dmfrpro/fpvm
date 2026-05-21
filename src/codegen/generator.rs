use crate::symbol_table::{FunctionInfo, SymbolKind, SymbolTable};

use super::{BytecodeFunction, BytecodeProgram, CodegenError, Instruction};

pub struct CodeGenerator<'a> {
    symbol_table: &'a SymbolTable,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn generate_program_skeleton(&self) -> Result<BytecodeProgram, CodegenError> {
        let entry_function = self
            .symbol_table
            .function(self.symbol_table.entry_function_id)
            .ok_or_else(|| CodegenError::InternalError {
                message: "missing entry function".to_string(),
            })?;

        let mut program = BytecodeProgram::new(entry_function.label.clone());

        self.generate_globals(&mut program)?;
        self.generate_function_headers(&mut program)?;

        Ok(program)
    }

    fn generate_globals(&self, program: &mut BytecodeProgram) -> Result<(), CodegenError> {
        for symbol in self.symbol_table.symbols() {
            if symbol.kind == SymbolKind::Global {
                program.add_global(symbol.label.clone());
            }
        }

        Ok(())
    }

    fn generate_function_headers(&self, program: &mut BytecodeProgram) -> Result<(), CodegenError> {
        for function in self.symbol_table.functions() {
            let bytecode_function = self.generate_function_header(function)?;
            program.add_function(bytecode_function);
        }

        Ok(())
    }

    fn generate_function_header(
        &self,
        function: &FunctionInfo,
    ) -> Result<BytecodeFunction, CodegenError> {
        let mut bytecode_function = BytecodeFunction::new(function.label.clone());

        for capture in &function.captures {
            let symbol = self
                .symbol_table
                .symbol(capture.symbol_id)
                .ok_or_else(|| CodegenError::InternalError {
                    message: format!("missing captured symbol #{}", capture.symbol_id),
                })?;

            bytecode_function.captures.push(symbol.label.clone());
        }

        for arg_id in &function.args {
            let symbol = self
                .symbol_table
                .symbol(*arg_id)
                .ok_or_else(|| CodegenError::InternalError {
                    message: format!("missing arg symbol #{}", arg_id),
                })?;

            bytecode_function.args.push(symbol.label.clone());
        }

        for local_id in &function.locals {
            let symbol = self
                .symbol_table
                .symbol(*local_id)
                .ok_or_else(|| CodegenError::InternalError {
                    message: format!("missing local symbol #{}", local_id),
                })?;

            bytecode_function.locals.push(symbol.label.clone());
        }

        // Temporary body. Later this will be replaced by real codegen.
        bytecode_function.emit(Instruction::LoadNull);
        bytecode_function.emit(Instruction::Ret);

        Ok(bytecode_function)
    }
}