use super::*;

/// The code generator may output either a single program,
/// ora  collection of modules, depending on earlier stages.
pub enum Compiled {
    Program(Box<masm::Program>),
    Modules(Vec<Box<masm::Module>>),
}

/// Perform code generation on the possibly-linked output of previous stages
pub struct CodegenStage;
impl Stage for CodegenStage {
    type Input = MaybeLinked;
    type Output = Compiled;

    fn enabled(&self, session: &Session) -> bool {
        session.should_codegen()
    }

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> DriverResult<Self::Output> {
        match input {
            MaybeLinked::Linked(program) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Program>::default();
                Ok(convert_to_masm
                    .convert(program, analyses, session)
                    .map(Compiled::Program)?)
            }
            MaybeLinked::Unlinked(modules) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Module>::default();
                let mut masm_modules = Vec::with_capacity(modules.len());
                for module in modules.into_iter() {
                    let masm_module = convert_to_masm.convert(module, analyses, session)?;
                    masm_modules.push(masm_module);
                }
                Ok(Compiled::Modules(masm_modules))
            }
        }
    }
}