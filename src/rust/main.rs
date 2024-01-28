mod ga;

use std::process::*;

const FLAGS: [&'static str; 57] = [
    "--ccp",
    "--cfg-cleanup",
    "--code-sink",
    "--combine-access-chains",
    "--compact-ids",
    "--convert-local-access-chains",
    "--convert-relaxed-to-half",
    "--copy-propagate-arrays",
    "--descriptor-scalar-replacement",
    "--eliminate-dead-branches",
    "--eliminate-dead-code-aggressive",
    "--eliminate-dead-const",
    "--eliminate-dead-functions",
    "--eliminate-dead-input-components",
    "--eliminate-dead-inserts",
    "--eliminate-dead-members",
    "--eliminate-dead-variables",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--fix-func-call-param",
    "--fix-storage-class",
    "--flatten-decorations",
    "--fold-spec-const-op-composite",
    "--freeze-spec-const",
    "--if-conversion",
    "--inline-entry-points-exhaustive",
    "--inline-entry-points-opaque",
    "--interpolate-fixup",
    "--inst-buff-addr-check",
    "--inst-debug-printf",
    "--local-redundancy-elimination",
    "--loop-invariant-code-motion",
    "--loop-peeling",
    "--loop-unroll",
    "--loop-unswitch",
    "--merge-blocks",
    "--merge-return",
    "--private-to-local",
    "--redundancy-elimination",
    "--relax-float-ops",
    "--remove-dont-inline",
    "--remove-duplicates",
    "--remove-unused-interface-variables",
    "--replace-desc-array-access-using-var-index",
    "--replace-invalid-opcode",
    "--scalar-replacement=100",
    "--simplify-instructions",
    "--spread-volatile-semantics",
    "--ssa-rewrite",
    "--strength-reduction",
    "--strip-debug",
    "--strip-nonsemantic",
    "--unify-const",
    "--upgrade-memory-model",
    "--vector-dce",
    "--workaround-1209",
    "--wrap-opkill",
];

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

fn print_all_with_space<T>(v: &Vec<T>)
where
    T: std::fmt::Display,
{
    for n in v {
        print!("{n} ");
    }
}

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

///
fn init(args: Vec<String>) {
    if args.len() < 3 {
        println!("[ error ] init(): the initial population size not found.");
        return;
    }
    let size = match args[2].parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!(
                "[ error ] init(): failed to parse the initial population size: {}",
                e.to_string()
            );
            return;
        }
    };
    for _ in 0..size {
        let indices = (0..FLAGS.len())
            .filter(|_| flip_coin())
            .collect::<Vec<usize>>();
        let indices = shuffle(indices);
        print_all_with_space(&indices);
        println!("");
        println!("");
    }
}

///
fn opt(args: Vec<String>) {
    let mut flags = Vec::new();
    for n in &args[2..] {
        match n.parse::<usize>() {
            Ok(idx) => flags.push(FLAGS[idx]),
            Err(e) => {
                println!("[ error ] opt(): {}", e.to_string());
                return;
            }
        }
    }
    print_all_with_space(&flags);
    println!("");
    let output = Command::new("spirv-opt")
        .args(flags)
        .args(["shader.org.frag.spv", "-o", "shader.frag.spv"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match output {
        Ok(n) if !n.success() => {
            println!("[ error ] opt(): failed to create an optimized shader: {}", n.code().unwrap());
        }
        Err(e) => {
            println!(
                "[ error ] opt(): failed to run the 'spirv-opt' command: {}",
                e.to_string(),
            );
        }
        _ => (),
    }
}

///
fn crossover() {
    let genes = if let Some(n) = get_genes_from_stdin() {
        n
    } else {
        return;
    };
    let items = (0..FLAGS.len()).collect::<Vec<usize>>();
    let children = ga::generation::crossover(&genes, &items);
    for n in children {
        print_all_with_space(&n);
        println!("");
        println!("");
    }
}

///
fn select(args: Vec<String>) {
    if args.len() < 3 {
        println!("[ error ] select(): the next generation genes count not found.");
        return;
    }
    let size = if let Ok(n) = args[2].parse::<usize>() {
        n
    } else {
        println!(
            "[ error ] select(): failed to parse the next generation genes count: {}",
            args[2]
        );
        return;
    };
    let genes = if let Some(n) = get_genes_from_stdin() {
        n
    } else {
        return;
    };
    let genes = ga::generation::select(&genes, size);
    for n in genes {
        print_all_with_space(&n.code);
        println!("");
        println!("{}", n.value);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("[ error ] main(): usage: ga <mode> [flag indices]");
        return;
    }

    if args[1] == "init" {
        init(args);
    } else if args[1] == "opt" {
        opt(args);
    } else if args[1] == "crossover" {
        crossover();
    } else if args[1] == "select" {
        select(args);
    } else {
        println!("[ error ] main(): undefined mode: {}", args[1]);
    }
}
