mod ga;

use std::ffi::*;

#[link(name = "vulkan-wrapper", kind = "static")]
extern "C" {
    fn delete_vulkan_app(_: *mut c_void);
    fn create_vulkan_app() -> *mut c_void;
    fn create_pipeline(_: *mut c_void, _: *const c_char) -> c_int;
    fn render(_: *mut c_void) -> c_int;
    fn save_rendering_result(_: *mut c_void) -> c_int;
}

fn main() {
    let vapp = unsafe { create_vulkan_app() };
    if vapp.is_null() {
        println!("[ error ] main(): failed to create a vulkan app.");
        return;
    }

    let measure = || {
        let start = std::time::Instant::now();
        unsafe { render(vapp) };
        let end = start.elapsed();
        end.as_micros()
    };

    let base_frag_shader = CString::new("./shader.frag.spv").unwrap();
    if unsafe { create_pipeline(vapp, base_frag_shader.as_ptr()) == 0 } {
        println!("[ error ] main(): failed to create base pipeline.");
        return;
    }
    let base_time = measure();

    unsafe { save_rendering_result(vapp) };

    let eval = |_| {
        // TODO:
        let micros = measure();
        println!("[ info ] eval: {} micros", micros);
        // TODO:
        Some(0)
    };

    ga::run(15, eval);

    unsafe { delete_vulkan_app(vapp) };
}
