use super::*;

fn random(start: u64, end: u64) -> u64 {
    use rand::Rng;
    rand::thread_rng().gen_range(start..end)
}

fn shuffle<T>(mut v: Vec<T>) -> Vec<T> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v
}

fn remove_consecutive_code<C>(v: Vec<C>) -> Vec<C>
where
    C: Clone + std::cmp::PartialEq,
{
    if v.is_empty() {
        return v;
    }
    let mut new = Vec::new();
    let mut prev = &v[0];
    for i in 1..v.len() {
        if &v[i] == prev {
            continue;
        }
        prev = &v[i];
        new.push(v[i].clone());
    }
    new
}

fn is_crossover_occur() -> bool {
    use rand::Rng;
    rand::thread_rng().gen::<f64>() < 0.75
}

fn is_mutation_occur() -> bool {
    use rand::Rng;
    rand::thread_rng().gen::<f64>() < 0.4
}

fn get_min_gene<C, V>(genes: &Vec<Gene<C, V>>) -> Gene<C, V>
where
    C: Clone,
    V: Clone + Ord,
{
    genes.iter().min_by_key(|n| &n.value).unwrap().clone()
}

fn get_max_gene<C, V>(genes: &Vec<Gene<C, V>>) -> Gene<C, V>
where
    C: Clone,
    V: Clone + Ord,
{
    genes.iter().max_by_key(|n| &n.value).unwrap().clone()
}

///
pub fn crossover<C, V>(genes: &Vec<Gene<C, V>>, items: &[C]) -> Vec<Vec<C>>
where
    C: Clone + std::cmp::PartialEq + std::fmt::Debug,
    V: Clone,
{
    // shuffle genes.
    let genes = shuffle(genes.clone());

    // crossover and mutation.
    let mut children = Vec::new();
    for i in 0..genes.len() {
        let code1 = &genes[i].code;
        let code2 = if i + 1 < genes.len() {
            &genes[i + 1].code
        } else {
            &genes[0].code
        };
        // 1. crossover.
        let (child1, child2) = if is_crossover_occur() {
            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            let mut count1 = 0;
            let mut count2 = 0;
            loop {
                if count1 >= code1.len() && count2 >= code2.len() {
                    break;
                }
                match random(0, 4) {
                    0 if count1 < code1.len() => {
                        child1.push(code1[count1].clone());
                        count1 += 1;
                    }
                    1 if count1 < code1.len() => {
                        child2.push(code1[count1].clone());
                        count1 += 1;
                    }
                    2 if count2 < code2.len() => {
                        child1.push(code2[count2].clone());
                        count2 += 1;
                    }
                    3 if count2 < code2.len() => {
                        child2.push(code2[count2].clone());
                        count2 += 1;
                    }
                    _ => (),
                }
            }
            (child1, child2)
        } else {
            (code1.clone(), code2.clone())
        };
        //
        for code in [child1, child2] {
            //
            let mut code = remove_consecutive_code(code);
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
            children.push(code);
        }
    }

    children
}

///
pub fn select<C, V>(genes: &Vec<Gene<C, V>>, size: usize) -> Vec<Gene<C, V>>
where
    C: Clone,
    V: Clone + Into<u64> + std::cmp::Ord,
{
    // selecting.
    let mut selected = Vec::new();
    // 1. select a best genes.
    selected.push(get_min_gene(genes));
    // 2. select others by roulette.
    let max: u64 = get_max_gene(genes).value.clone().into();
    let values = genes
        .iter()
        .map(|n| max - n.value.clone().into())
        .collect::<Vec<u64>>();
    let sum = values.iter().fold(0, |sum, n| sum + n);
    for _ in 1..size {
        let mut border = random(0, sum);
        for i in 0..values.len() {
            let value = values[i];
            if border < value {
                selected.push(genes[i].clone());
                break;
            } else {
                border -= value;
            }
        }
    }
    selected
}
