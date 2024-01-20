#[derive(Clone)]
pub struct Gene<C, V> {
    pub code: Vec<C>,
    pub value: V,
}

impl<C, V> Gene<C, V> {
    pub fn new<F>(code: &Vec<C>, eval: &F) -> Option<Self>
    where
        C: Clone,
        V: Clone,
        F: Fn(&Vec<C>) -> Option<V>,
    {
        if let Some(value) = eval(code) {
            Some(Self {
                code: code.clone(),
                value: value.clone(),
            })
        } else {
            None
        }
    }
}
