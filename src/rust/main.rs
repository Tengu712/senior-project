mod ga;

use ga::{gene::*, generation::*};
use std::{ffi::*, process::*};

type Flag = &'static str;
type Code = Vec<Flag>;
type Value = u64;

const INITIAL_POPULATION_SIZE: usize = 30;
const ITERATION_COUNT: usize = 20;
const FLAGS: [Flag; 44] = [
    "--wrap-opkill",
    "--eliminate-dead-branches",
    "--merge-return",
    "--inline-entry-points-exhaustive",
    "--eliminate-dead-functions",
    "--eliminate-dead-code-aggressive",
    "--private-to-local",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--eliminate-dead-code-aggressive",
    "--scalar-replacement=100",
    "--convert-local-access-chains",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--eliminate-dead-code-aggressive",
    "--ssa-rewrite",
    "--eliminate-dead-code-aggressive",
    "--ccp",
    "--eliminate-dead-code-aggressive",
    "--loop-unroll",
    "--eliminate-dead-branches",
    "--redundancy-elimination",
    "--combine-access-chains",
    "--simplify-instructions",
    "--scalar-replacement=100",
    "--convert-local-access-chains",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--eliminate-dead-code-aggressive",
    "--ssa-rewrite",
    "--eliminate-dead-code-aggressive",
    "--vector-dce",
    "--eliminate-dead-inserts",
    "--eliminate-dead-branches",
    "--simplify-instructions",
    "--if-conversion",
    "--copy-propagate-arrays",
    "--reduce-load-size",
    "--eliminate-dead-code-aggressive",
    "--merge-blocks",
    "--redundancy-elimination",
    "--eliminate-dead-branches",
    "--merge-blocks",
    "--simplify-instructions",
];

#[link(name = "vulkan-wrapper", kind = "static")]
extern "C" {
    fn delete_vulkan_app(_: *mut c_void);
    fn create_vulkan_app() -> *mut c_void;
    fn create_pipeline(_: *mut c_void, _: *const c_char) -> c_int;
    fn render(_: *mut c_void, _: *mut u64) -> c_int;
    // fn save_rendering_result(_: *mut c_void) -> c_int;
}

fn flip_coin() -> bool {
    use rand::Rng;
    rand::thread_rng().gen()
}

fn shuffle_code(mut v: Code) -> Code {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v
}

fn format_code_as_idx(code: &Code) -> String {
    let mut s = String::new();
    for flag in code {
        let idx = FLAGS.iter().position(|n| &n == &flag).unwrap();
        s.push_str(&format!("{idx} "));
    }
    s.pop();
    s
}

fn print_genes(genes: &Vec<Gene<Flag, Value>>) {
    for n in genes {
        println!(
            "[ debug ] print_genes(): value = {} , code = {}",
            n.value,
            format_code_as_idx(&n.code)
        );
    }
}

fn main() {
    // create a Vulkan app.
    let vapp = unsafe { create_vulkan_app() };
    if vapp.is_null() {
        println!("[ error ] main(): failed to create a vulkan app.");
        return;
    }

    // create a function to measure the rendering time.
    // it returns the time taken for rendering in microseconds.
    let measure = || -> u64 {
        let mut time = 0;
        unsafe { render(vapp, &mut time) };
        time
    };

    // measure the rendering time without optimization.
    // it's a base time for the evaluation of genes.
    let org_shader_name = CString::new("./shader.frag.spv").unwrap();
    if unsafe { create_pipeline(vapp, org_shader_name.as_ptr()) == 0 } {
        println!("[ error ] main(): failed to create a base pipeline.");
        return;
    }
    let base_time = measure();
    println!("[ info ] main(): base_time = {} us", base_time);

    // create a function to create an optimized shader based on flags and recreate a pipeline with it.
    let optimize = |code: &Code| -> Result<(), String> {
        let output = Command::new("spirv-opt")
            .args(code)
            .args(["shader.frag.spv", "-o", "shader.opt.frag.spv"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();
        match output {
            Ok(n) if !n.success() => {
                return Err(format!(
                    "[ warning ] optimize: failed to create an optimized shader: code={:?}",
                    code
                ))
            }
            Err(e) => {
                return Err(format!(
                "[ warning ] optimize: failed to run the 'spirv-opt' command: {} : flags = {:?}",
                e.to_string(),
                code
            ))
            }
            _ => (),
        }
        let opt_shader_name = CString::new("./shader.opt.frag.spv").unwrap();
        if unsafe { create_pipeline(vapp, opt_shader_name.as_ptr()) == 0 } {
            return Err(format!(
                "[ error ] main(): failed to create a optimized pipeline: flags = {:?}",
                code
            ));
        }
        Ok(())
    };

    // measure the rendering time with a "-O" flag.
    // it's just a reference for me.
    if let Err(e) = optimize(&Vec::from(["-O"])) {
        println!("{}", e);
        return;
    }
    let o_time = measure();
    println!("[ info ] main(): -O time = {} us", o_time);

    // create an evaluation function.
    // it's called every time a gene is created.
    let eval = |code: &Code| -> Option<Value> {
        // create an optimized shader and recreate a pipeline with it.
        if let Err(e) = optimize(code) {
            println!("{}", e);
            return None;
        }
        // evaluate.
        let time = measure();
        if time <= base_time {
            Some(base_time - time)
        } else {
            None
        }
    };

    // create the initial population.
    let mut genes = Vec::new();
    for _ in 0..INITIAL_POPULATION_SIZE {
        loop {
            let code = FLAGS.into_iter().filter(|_| flip_coin()).collect::<Code>();
            let code = shuffle_code(code);
            if let Some(gene) = Gene::new(&code, &eval) {
                genes.push(gene);
                break;
            }
        }
    }
    println!("[ debug ] main(): initial population created:");
    print_genes(&genes);

    // start Genetic Algorithm.
    let mut generation = Generation::new(genes);
    let mut max_gene = generation.get_max_gene();
    for i in 0..ITERATION_COUNT {
        println!("[ info ] main(): {} th generation:", i + 1);
        generation = generation.next(INITIAL_POPULATION_SIZE, &FLAGS, &eval);
        let this_max_gene = generation.get_max_gene();
        if this_max_gene.value > max_gene.value {
            max_gene = this_max_gene;
        }
        println!(
            "[ info ] main(): end: max_gene.value = {}",
            max_gene.value
        );
        print_genes(&generation.genes);
    }

    // print the result.
    let time = base_time - generation.genes[0].value;
    println!(
        "[ info ] main(): max gene rendering time {} us , {} % of no optimization , {} % of -O",
        time,
        time as f32 / base_time as f32,
        time as f32 / o_time as f32
    );
    println!(
        "[ info ] main(): max_gene.value = {} , max_gene.code = {:?}",
        max_gene.value,
        max_gene.code
    );

    // exit.
    unsafe { delete_vulkan_app(vapp) };
}
