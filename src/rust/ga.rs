pub mod generation;

/// A data structure representing genes.
/// However, genes are defined as combinations of chromosomes (code) and fitness (value).
///
/// Chromosomes are variable-length arrays.
#[derive(Clone, Debug)]
pub struct Gene<C, V>
where
    C: Clone,
    V: Clone,
{
    pub code: Vec<C>,
    pub value: V,
}
