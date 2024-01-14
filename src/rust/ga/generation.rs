use super::{gene::Gene, *};

const GENES_COUNT_IN_ONE_GENERATION: usize = 20;
const MUTATION_PROBABILITY: u32 = 10;

fn random(max: u32) -> u32 {
    use rand::Rng;
    (rand::thread_rng().gen::<f32>() * max as f32) as u32
}

pub(super) struct Generation {
    pub(super) max_gene: Gene,
    pub(super) genes: Vec<Gene>,
}

impl Generation {
    pub(super) fn new<F>(code_length: usize, eval: F) -> Self
    where
        F: Fn(Code) -> Option<Value>,
    {
        let mut genes = Vec::new();
        for _ in 0..GENES_COUNT_IN_ONE_GENERATION {
            loop {
                let code = random(1 << code_length);
                if let Some(gene) = Gene::new(code, &eval) {
                    genes.push(gene);
                    break;
                }
            }
        }
        genes.sort_by(|a, b| b.value.cmp(&a.value));
        Self {
            max_gene: genes[0].clone(),
            genes,
        }
    }

    pub(super) fn next(self) -> Self {
        /*
                let mut selected = Vec::<Gene>::new();
                // select 2 good genes
                selected.push(self.genes[0].clone());
                selected.push(self.genes[1].clone());
                // roulette
                let sum = self.genes.iter().fold(0, |sum, n| sum + n.value);
                for _ in 2..(GENES_COUNT_IN_ONE_GENERATION + 1) {
                    let border = random(sum);
                    let mut tmp_sum = 0;
                    for n in &self.genes {
                        tmp_sum += n.value;
                        if tmp_sum > border {
                            selected.push(n.clone());
                            break;
                        }
                    }
                }
                assert!(selected.len() == GENES_COUNT_IN_ONE_GENERATION + 1);

                let mut genes = Vec::<Gene>::new();
                for i in 0..GENES_COUNT_IN_ONE_GENERATION {
                    let mut code = 0;
                    for b in 0..ITEMS.len() {
                        if random(2) == 1 {
                            code |= selected[i].code & (1 << b)
                        } else {
                            code |= selected[i + 1].code & (1 << b)
                        }
                    }
                    if let Some(gene) = Gene::new(code) {
                        genes.push(gene);
                    }
                }

                let mut genes = genes.into_iter().filter_map(|n| {
                    let p = random(101);
                    if p <= MUTATION_PROBABILITY {
                        let b = random(ITEMS.len() as u32);
                        let code = n.code ^ (1 << b);
                        Gene::new(code)
                    } else {
                        Some(n)
                    }
                }).collect::<Vec<Gene>>();

                genes.sort_by(|a, b| b.value.cmp(&a.value));

                let max_gene = if genes[0].value > self.max_gene.value {
                    genes[0].clone()
                } else {
                    self.max_gene
                };

                Self { max_gene, genes }
        */
        self
    }
}
