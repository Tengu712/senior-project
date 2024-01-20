use super::gene::*;

fn random(start: u64, end: u64) -> u64 {
    use rand::Rng;
    rand::thread_rng().gen_range(start..end)
}

fn shuffle_genes<C, V>(mut v: Vec<Gene<C, V>>) -> Vec<Gene<C, V>> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v
}

fn is_mutation_occur() -> bool {
    use rand::Rng;
    rand::thread_rng().gen::<f64>() < 0.5
}

pub struct Generation<C, V> {
    pub genes: Vec<Gene<C, V>>,
}

impl<C, V> Generation<C, V> {
    pub fn new(genes: Vec<Gene<C, V>>) -> Self
    where
        C: Clone,
        V: Clone + Ord,
    {
        Self { genes }
    }

    pub fn next<F>(mut self, genes_count: usize, items: &[C], eval: &F) -> Self
    where
        C: Clone,
        V: Clone + Ord + Into<u64>,
        F: Fn(&Vec<C>) -> Option<V>,
    {
        // shuffle genes.
        self.genes = shuffle_genes(self.genes);

        // CROSSOVER and MUTATION
        let parents_count = self.genes.len();
        let mut die_count = 0;
        for i in 0..parents_count {
            let code1 = &self.genes[i].code;
            let code2 = if i + 1 < parents_count {
                &self.genes[i + 1].code
            } else {
                &self.genes[0].code
            };
            // 1. crossover.
            let mut code = Vec::new();
            let mut count1 = 0;
            let mut count2 = 0;
            loop {
                if random(0, 2) == 0 {
                    if count1 < code1.len() && random(0, 2) == 0 {
                        code.push(code1[count1].clone());
                    }
                    count1 += 1;
                } else {
                    if count2 < code2.len() && random(0, 2) == 0 {
                        code.push(code2[count2].clone());
                    }
                    count2 += 1;
                }
                if count1 >= code1.len() && count2 >= code2.len() {
                    break;
                }
            }
            // 2. mutation.
            // 2-1. insert a new item.
            if is_mutation_occur() {
                let idx_code = random(0, code.len() as u64 + 1) as usize;
                let idx_items = random(0, items.len() as u64) as usize;
                code.insert(idx_code, items[idx_items].clone());
            }
            // 2-2. remove an item.
            if is_mutation_occur() {
                let idx = random(0, code.len() as u64) as usize;
                code.remove(idx);
            }
            // 2-3. exchange two items.
            if is_mutation_occur() {
                let idx1 = random(0, code.len() as u64) as usize;
                let idx2 = random(0, code.len() as u64) as usize;
                let code_idx1 = code[idx1].clone();
                let code_idx2 = code[idx2].clone();
                code[idx1] = code_idx2;
                code[idx2] = code_idx1;
            }
            // 2-4. alter an item.
            if is_mutation_occur() {
                let idx_code = random(0, code.len() as u64) as usize;
                let idx_items = random(0, items.len() as u64) as usize;
                code[idx_code] = items[idx_items].clone();
            }
            // 3. create a new indivisual.
            if let Some(gene) = Gene::new(&code, eval) {
                self.genes.push(gene);
            } else {
                die_count += 1;
            }
        }
        println!("[ debug ] Generation.next(): die_count = {die_count}");

        // SELECTING
        let mut genes = Vec::new();
        // 1. select a best genes.
        genes.push(self.get_max_gene());
        // 2. select others by roulette.
        let sum = self
            .genes
            .iter()
            .fold(0, |sum, n| sum + n.value.clone().into());
        for _ in 1..genes_count {
            let mut border = random(0, sum);
            for gene in &self.genes {
                let value = gene.value.clone().into();
                if border < value {
                    genes.push(gene.clone());
                    break;
                } else {
                    border -= value;
                }
            }
        }

        // to next generation.
        Self { genes }
    }

    pub fn get_max_gene(&self) -> Gene<C, V>
    where
        C: Clone,
        V: Clone + Ord,
    {
        self.genes.iter().max_by_key(|n| &n.value).unwrap().clone()
    }
}
