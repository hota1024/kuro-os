use crate::console;

#[no_mangle]
pub fn kernel_trap() {
    console::println!("kernel_trap");
}
