use std::env;

fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Join the current directory with the relative path
    let lib_path = current_dir
        .parent()
        .unwrap()
        .join("cpp-lib")
        .join("x64")
        .join("Release-dylib");

    // Specify the path to your .dll file's directory
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Specify the name of the .lib file (without the 'dll' prefix and '.dll' extension)
    println!("cargo:rustc-link-lib=dylib=binding");
}
