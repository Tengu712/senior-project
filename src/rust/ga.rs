mod gene;
mod generation;

pub type Code = u32;
pub type Value = u32;

const ITERATION_COUNT: usize = 30;

pub fn run<F>(code_length: usize, eval: F)
where
    F: Fn(Code) -> Option<Value>,
{
    let mut generation = generation::Generation::new(code_length, &eval);
    for i in 0..ITERATION_COUNT {
        generation = generation.next();
        println!("[ info ] run(): {} th generation", i);
    }
}
