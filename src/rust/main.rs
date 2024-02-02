mod ga;

use std::process::*;

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
    fn ff_delete_vulkan_app(_: *mut std::ffi::c_void);
    fn ff_create_vulkan_app() -> *mut std::ffi::c_void;
    fn ff_render(_: *mut std::ffi::c_void, _: *mut u64) -> std::ffi::c_int;
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

fn print_gene(gene: &ga::Gene<usize, u64>) {
    for n in &gene.code {
        print!("{n} ");
    }
    println!("");
    println!("{}", gene.value);
}

fn measure() -> Option<u64> {
    // create a vulkan app
    let vapp = unsafe { ff_create_vulkan_app() };
    if vapp.is_null() {
        println!("[ warning ] measure(): failed to create a vulkan app.");
        return None;
    }
    // increase the rendering frequency to 20 times
    // sum up the measurement values for the latter 10 iterations
    let mut total = 0;
    for i in 0..20 {
        let mut time = 0;
        if unsafe { ff_render(vapp, &mut time) } == 0 {
            println!("[ warning ] measure(): failed to render.");
            return None;
        }
        if i >= 10 {
            total += time;
        }
    }
    // finish
    unsafe { ff_delete_vulkan_app(vapp) };
    Some((total as f64 / 10.0) as u64)
}

fn eval(indices: &Vec<usize>) -> Option<u64> {
    // get flags from indices
    let mut flags = Vec::new();
    for n in indices {
        flags.push(FLAGS[*n]);
    }
    // run spirv-opt with flags
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
    // measure
    measure()
}

/// A function to run Genetic Algorithm.
///
/// The size of population is 10.
/// When an invalid result is obtained, the fitness (value) becomes 99999999999.
///
/// When you want to terminate it, send a SIGINT, as it runs indefinitely.
/// To cool the GPU, the thread pauses for 10 seconds after each evaluation.
/// Due to extensive standard output, it is advisable to redirect it to some text file.
fn run_run() {
    let size = 10;
    let interval = 10;
    let invalid_value = 99999999999;
    let items = (0..FLAGS.len()).collect::<Vec<usize>>();

    // create the initial population
    let mut genes = Vec::new();
    for _ in 0..size {
        let indices = items
            .clone()
            .into_iter()
            .filter(|_| flip_coin())
            .collect::<Vec<usize>>();
        let code = shuffle(indices);
        let value = eval(&code).unwrap_or(invalid_value);
        let gene = ga::Gene { code, value };
        print_gene(&gene);
        genes.push(gene);
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }

    // run Genetic Algorithm
    for i in 1.. {
        println!(
            "================================================================================"
        );
        println!("    {i} TH GENERATION");
        println!(
            "================================================================================"
        );

        println!("// CROSSOVER");
        let children_indicess = ga::generation::crossover(&genes, &items);
        for code in children_indicess {
            let value = eval(&code).unwrap_or(invalid_value);
            let gene = ga::Gene { code, value };
            print_gene(&gene);
            genes.push(gene);
            std::thread::sleep(std::time::Duration::from_secs(interval));
        }

        println!("// SELECT");
        genes = ga::generation::select(&genes, size);
        for n in &genes {
            print_gene(n);
        }
    }
}

/// A function to measure the rendering time.
///
/// Apply the shader 'shader.frag.spv'.
fn run_measure() {
    if let Some(value) = measure() {
        println!("{value}");
    } else {
        println!("[ warning ] run_measure(): measure() returned None.");
    }
}

/// A function to evaluate a gene.
///
/// Provide the genetic code as a command-line argument.
fn run_eval(args: Vec<String>) {
    let mut indices = Vec::new();
    for n in &args[2..] {
        if let Ok(n) = n.parse::<usize>() {
            indices.push(n);
        } else {
            println!("[ error ] run_eval(): failed to parse an index: {n}");
            return;
        }
    }
    if let Some(value) = eval(&indices) {
        println!("{value}");
    } else {
        println!("[ warning ] run_eval(): eval() returned None.");
    }
}

/// A function to verify GPU measurement errors.
///
/// Apply the shader 'shader.frag.spv'.
/// Specify the number of seconds to pause the thread as the 3rd command-line argument.
/// When you want to terminate it, send a SIGINT, as it runs indefinitely.
fn run_test(args: Vec<String>) {
    // create a vulkan app
    let vapp = unsafe { ff_create_vulkan_app() };
    if vapp.is_null() {
        println!("[ error ] run_test(): failed to create a vulkan app.");
        return;
    }
    // measure
    let interval = args[2].parse::<u64>().unwrap();
    for _ in 0.. {
        let mut time = 0;
        if unsafe { ff_render(vapp, &mut time) } == 0 {
            println!("[ warning ] run_eval(): failed to render.");
            continue;
        }
        println!("{time}");
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}

/// Entry point.
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        run_run();
    } else if args[1] == "run" {
        run_run();
    } else if args[1] == "measure" {
        run_measure();
    } else if args[1] == "eval" {
        run_eval(args);
    } else if args[1] == "test" {
        run_test(args);
    }
}
