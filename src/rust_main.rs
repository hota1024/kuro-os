pub fn rust_main() -> ! {
    crate::console::console_init();
    crate::console::println!("hello world");

    loop {}
}
