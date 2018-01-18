// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
extern crate dest;

fn main() {
    dest::function();
}
