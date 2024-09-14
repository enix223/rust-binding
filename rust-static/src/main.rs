mod binding;
use binding::cpplib;

fn main() {
    unsafe {
        cpplib::foo();
        let res = cpplib::bar(1, 2);
        print!("res = {}", res);
    }
}
