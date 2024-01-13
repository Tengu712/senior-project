// Reference: https://qiita.com/k_mori/items/94f06b2d003c902788cb

const CAPACITY: u32 = 300;
const ITEMS: [[u32; 2]; 15] = [
    [6, 71],
    [4, 29],
    [78, 44],
    [63, 73],
    [23, 42],
    [60, 72],
    [14, 60],
    [19, 79],
    [16, 6],
    [9, 72],
    [8, 32],
    [74, 23],
    [42, 23],
    [2, 92],
    [85, 36],
];

const GENES_COUNT_IN_ONE_GENERATION: usize = 20;
const ITERATION_COUNT: usize = 30;
const MUTATION_PROBABILITY: u32 = 10;

fn random(max: u32) -> u32 {
    use rand::Rng;
    (rand::thread_rng().gen::<f32>() * max as f32) as u32
}

#[derive(Clone)]
struct Gene {
    code: u32,
    value: u32,
}

impl Gene {
    fn new(code: u32) -> Option<Self> {
        let mut weight = 0;
        let mut value = 0;
        for b in 0..ITEMS.len() {
            if (1 << b) & code > 0 {
                weight += ITEMS[b][0];
                value += ITEMS[b][1];
            }
        }
        if weight <= CAPACITY {
            Some(Self { code, value })
        } else {
            None
        }
    }

    fn print(&self) {
        println!("[ info ] Gene.print(): code={:015b} value={}", self.code, self.value);
    }
}

struct Generation {
    max_gene: Gene,
    genes: Vec<Gene>,
}

impl Generation {
    fn new() -> Self {
        let mut genes = Vec::new();
        for _ in 0..GENES_COUNT_IN_ONE_GENERATION {
            loop {
                let code = random(1 << ITEMS.len());
                if let Some(gene) = Gene::new(code) {
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

    fn next(self) -> Self {
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
    }
}

fn main() {
    let mut generation = Generation::new();
    for i in 0..ITERATION_COUNT {
        println!("[ info ] main(): {} th generation", i);
        generation = generation.next();
        generation.max_gene.print();
    }
}
