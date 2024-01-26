mod ga;

use ga::{gene::*, generation::*};
use std::{ffi::*, process::*};

type Flag = &'static str;
type Code = Vec<Flag>;
type Value = u64;

const INITIAL_POPULATION_SIZE: usize = 10;
const ITERATION_COUNT: usize = 10;
const FLAGS: [Flag; 23] = [
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
    fn delete_vulkan_app(_: *mut c_void);
    fn create_vulkan_app(_: *const c_char) -> *mut c_void;
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
    /*
    let mut prev = 0;
    for _ in 0..100 {
        let org_shader_name = CString::new("./shader.frag.spv").unwrap();
        let vapp = unsafe { create_vulkan_app(org_shader_name.as_ptr()) };
        if vapp.is_null() {
            println!("[ error ] main(): failed to create a vulkan app.");
            return;
        }
        let mut time = 0;
        unsafe { render(vapp, &mut time) };
        unsafe { delete_vulkan_app(vapp) };
        println!("{time} {}", time as i64 - prev);
        prev = time as i64;
    }
    */

    // create a function to measure the rendering time.
    // it returns the time taken for rendering in microseconds.
    let measure = |shader_name| -> u64 {
        let vapp = unsafe { create_vulkan_app(shader_name) };
        if vapp.is_null() {
            println!("[ error ] main(): failed to create a vulkan app.");
            return u64::MAX;
        }
        let mut time = 0;
        unsafe { render(vapp, &mut time) };
        unsafe { delete_vulkan_app(vapp) };
        time
    };

    // measure the rendering time without optimization.
    // it's a base time for the evaluation of genes.
    let org_shader_name = CString::new("./shader.frag.spv").unwrap();
    let base_time = measure(org_shader_name.as_ptr());
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
                Err(format!(
                    "[ warning ] optimize: failed to create an optimized shader: code={:?}",
                    code
                ))
            }
            Err(e) => {
                Err(format!(
                "[ warning ] optimize: failed to run the 'spirv-opt' command: {} : flags = {:?}",
                e.to_string(),
                code
            ))
            }
            _ => Ok(()),
        }
    };

    // measure the rendering time with a "-O" flag.
    // it's just a reference for me.
    if let Err(e) = optimize(&Vec::from(["-O"])) {
        println!("{e}");
        return;
    }
    let o_shader_name = CString::new("./shader.opt.frag.spv").unwrap();
    let o_time = measure(o_shader_name.as_ptr());
    println!("[ info ] main(): -O time = {o_time}");

    // create an evaluation function.
    // it's called every time a gene is created.
    let eval = |code: &Code| -> Option<Value> {
        // create an optimized shader and recreate a pipeline with it.
        if let Err(e) = optimize(code) {
            println!("{}", e);
            return None;
        }
        // evaluate.
        let shader_name = CString::new("./shader.frag.spv").unwrap();
        let time = measure(shader_name.as_ptr());
        if time <= base_time {
            Some(base_time - time)
        } else {
            Some(1)
        }
    };

    // create the initial population.
    println!("[ debug ] main(): initial population creating:");
    let mut genes = Vec::new();
    for _ in 0..INITIAL_POPULATION_SIZE {
        loop {
            let code = FLAGS.into_iter().filter(|_| flip_coin()).collect::<Code>();
            let code = shuffle_code(code);
            if let Some(gene) = Gene::new(&code, &eval) {
                genes.push(gene);
                println!("[ debug ] main(): value = {} , code = {}", gene.value, format_code_as_idx(&code));
                break;
            }
            println!("[ debug ] main(): again");
        }
    }

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
        // revalue
        if let Some(n) = eval(&max_gene.code) {
            println!("[ debug ] main(): re_time = {n} : code= {}", format_code_as_idx(&max_gene.code));
        } else {
            println!("[ warning ] main(): re_time is none : code = {}", format_code_as_idx(&max_gene.code));
        }
        print_genes(&generation.genes);
    }

    // print the result.
    let time = base_time - max_gene.value;
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
}
