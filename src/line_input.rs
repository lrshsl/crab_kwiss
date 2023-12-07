use std::ops::{Generator, GeneratorState};



pub struct StringLineInput<'a> {
    source: &'a str,
}

impl<'a> Generator<&'a str> for StringLineInput<'a> {
    type Yield = &'a str;
    type Return = ();

    fn resume(&mut self, _: &'a str) -> GeneratorState<Self::Yield, Self::Return> {
        for line in self.source.lines() {
            yield line;
        }
    }
}

