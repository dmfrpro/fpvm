use std::marker::PhantomData;

use crate::pipeline::stage::*;

pub struct Pipeline<Input, Output, Func> {
    f: Func,
    _marker: PhantomData<(Input, Output)>,
}

impl<Input, Output, Func> Pipeline<Input, Output, Func>
where
    Func: FnOnce(Input) -> StageOutput<Output>,
{
    pub fn new(f: Func) -> Self {
        Self {
            f,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn then<NextOutput, NextStage>(
        self,
        next: NextStage,
    ) -> Pipeline<Input, NextOutput, impl FnOnce(Input) -> StageOutput<NextOutput>>
    where
        NextStage: FnOnce(Output) -> StageOutput<NextOutput>,
    {
        Pipeline::new(move |input| {
            let first = (self.f)(input);

            let Some(value) = first.value else {
                return StageOutput {
                    value: None,
                    diagnostics: first.diagnostics,
                };
            };

            let second = next(value);

            let mut diagnostics = first.diagnostics;
            diagnostics.extend(second.diagnostics);

            StageOutput {
                value: second.value,
                diagnostics,
            }
        })
    }

    pub fn run(self, input: Input) -> StageOutput<Output> {
        (self.f)(input)
    }
}
