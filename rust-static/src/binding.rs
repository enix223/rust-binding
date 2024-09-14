pub mod cpplib {
    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn foo();

        pub fn bar(a: i32, b: i32) -> i32;
    }
}
