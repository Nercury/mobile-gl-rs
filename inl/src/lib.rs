pub use c_api::*;

mod c_api {
    #[no_mangle]
    pub extern fn rust_hello_world_7() -> i32 {
        println!("Hello, I'm in Rust code! I'm about to return 11.");
        12
    }
}

