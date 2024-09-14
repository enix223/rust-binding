mod binding;

fn main() {
    unsafe {
        binding::foo();
        let res = binding::bar(1, 2);
        print!("res = {}", res);
    }
    println!("Hello, world!");
}
