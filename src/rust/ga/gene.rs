use super::*;

#[derive(Clone)]
pub(super) struct Gene {
    pub(super) code: Code,
    pub(super) value: Value,
}

impl Gene {
    pub(super) fn new<F>(code: u32, eval: F) -> Option<Self>
    where
        F: Fn(Code) -> Option<Value>,
    {
        if let Some(value) = eval(code) {
            Some(Self { code, value })
        } else {
            None
        }
    }
}
