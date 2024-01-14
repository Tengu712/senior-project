mod ga;

use std::ffi::*;

#[link(name = "vulkan-wrapper", kind = "static")]
extern "C" {
    fn delete_vulkan_app(_: *mut c_void);
    fn create_vulkan_app() -> *mut c_void;
    fn render(_: *mut c_void) -> c_int;
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

    let base_time = measure();

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
