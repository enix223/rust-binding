use std::env;

fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Join the current directory with the relative path
    let lib_path = current_dir.join("cpp-lib").join("Release-static");

    // Specify the path to your .lib file's directory
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Specify the name of the .lib file (without the 'lib' prefix and '.lib' extension)
    println!("cargo:rustc-link-lib=static=binding");
}
