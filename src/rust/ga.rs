pub mod generation;

#[derive(Clone, Debug)]
pub struct Gene<C, V>
where
    C: Clone,
    V: Clone,
{
    pub code: Vec<C>,
    pub value: V,
}
