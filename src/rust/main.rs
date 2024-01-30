mod ga;

use std::process::*;

type Gene = ga::Gene<usize, u64>;
type Genes = Vec<Gene>;

const FLAGS: [&'static str; 23] = [
    "--wrap-opkill",
    "--eliminate-dead-branches",
    "--merge-return",
    "--inline-entry-points-exhaustive",
    "--eliminate-dead-functions",
    "--eliminate-dead-code-aggressive",
    "--private-to-local",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--scalar-replacement=100",
    "--convert-local-access-chains",
    "--ssa-rewrite",
    "--ccp",
    "--loop-unroll",
    "--redundancy-elimination",
    "--combine-access-chains",
    "--simplify-instructions",
    "--vector-dce",
    "--eliminate-dead-inserts",
    "--if-conversion",
    "--copy-propagate-arrays",
    "--reduce-load-size",
    "--merge-blocks",
];

#[link(name = "vulkan-wrapper", kind = "static")]
extern "C" {
    fn run_vulkan_app(_: *mut u64) -> std::ffi::c_int;
}

fn flip_coin() -> bool {
    use rand::Rng;
    rand::thread_rng().gen()
}

fn shuffle<T>(mut v: Vec<T>) -> Vec<T> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v
}

fn println_all_with_space<T>(v: &Vec<T>)
where
    T: std::fmt::Display,
{
    for n in v {
        print!("{n} ");
    }
    println!("");
}

fn eval(indices: &Vec<usize>) -> Option<u64> {
    //
    let mut flags = Vec::new();
    for n in indices {
        flags.push(FLAGS[*n]);
    }
    //
    let output = Command::new("spirv-opt")
        .args(flags)
        .args(["shader.org.frag.spv", "-o", "shader.frag.spv"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match output {
        Ok(n) if !n.success() => {
            println!(
                "[ warning ] eval(): failed to create an optimized shader: {}",
                n.code().unwrap()
            );
            return None;
        }
        Err(e) => {
            println!(
                "[ warning ] eval(): failed to run the 'spirv-opt' command: {}",
                e.to_string(),
            );
            return None;
        }
        _ => (),
    }
    //
    let mut res = 0;
    for _ in 0..3 {
        let mut tmp = 0;
        if unsafe { run_vulkan_app(&mut tmp) } == 0 {
            println!("[ warning ] eval(): failed to run vulkan app.");
            return None;
        }
        res += tmp;
        std::thread::sleep(std::time::Duration::from_secs(20));
    }
    Some((res as f64 / 3.0) as u64)
}

///
fn init(size: usize) -> Genes {
    let mut genes = Vec::new();
    for _ in 0..size {
        let indices = (0..FLAGS.len())
            .filter(|_| flip_coin())
            .collect::<Vec<usize>>();
        let indices = shuffle(indices);
        println_all_with_space(&indices);
        if let Some(value) = eval(&indices) {
            println!("{value}");
            genes.push(ga::Gene {
                code: indices,
                value,
            });
        } else {
            println!("99999999999");
            genes.push(ga::Gene {
                code: indices,
                value: 99999999999,
            });
        }
    }
    genes
}

///
fn crossover(parents: &Genes) -> Genes {
    let items = (0..FLAGS.len()).collect::<Vec<usize>>();
    let children = ga::generation::crossover(parents, &items);
    let mut genes = Vec::new();
    for n in children {
        println_all_with_space(&n);
        if let Some(value) = eval(&n) {
            println!("{value}");
            genes.push(ga::Gene { code: n, value });
        } else {
            println!("99999999999");
            genes.push(ga::Gene {
                code: n,
                value: 99999999999,
            });
        }
    }
    genes
}

///
fn select(genes: &Genes, size: usize) -> Genes {
    let genes = ga::generation::select(genes, size);
    for n in &genes {
        println_all_with_space(&n.code);
        println!("{}", n.value);
    }
    genes
}

///
fn run() {
    let mut genes = init(10);
    for i in 1.. {
        println!(
            "================================================================================"
        );
        println!("    {i} th generation");
        println!(
            "================================================================================"
        );
        println!("// crossover");
        let mut children = crossover(&genes);
        genes.append(&mut children);
        println!("// select");
        genes = select(&genes, 10);
    }
}

fn main() {
    run();
}

/*
fn get_genes_from_stdin() -> Option<Vec<ga::Gene<usize, u64>>> {
    let mut genes = Vec::new();
    let mut line = 1;
    loop {
        let mut indices_str = String::new();
        if std::io::stdin().read_line(&mut indices_str).is_err() {
            break;
        }
        let indices_str = indices_str.trim();
        if indices_str == "" {
            break;
        }
        let indices_str = indices_str.split(' ').collect::<Vec<&str>>();
        let mut indices = Vec::new();
        for n in &indices_str {
            if let Ok(idx) = n.parse::<usize>() {
                indices.push(idx);
            } else {
                println!(
                    "[ error ] get_genes_from_stdin(): failed to parse a index: {line} line : {n}"
                );
                return None;
            }
        }
        line += 1;
        let mut value_str = String::new();
        if std::io::stdin().read_line(&mut value_str).is_err() {
            println!("[ error ] get_genes_from_stdin(): value not found: {line} line");
            return None;
        }
        let value = if let Ok(n) = value_str.trim().parse::<u64>() {
            n
        } else {
            println!("[ error ] get_genes_from_stdin(): failed to parse a value: {line} line : {value_str}");
            return None;
        };
        genes.push(ga::Gene {
            code: indices,
            value: value,
        });
        line += 1;
    }
    Some(genes)
}
*/
