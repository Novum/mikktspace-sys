extern crate cc;

fn main() {
    cc::Build::new()
        .file("c_code/mikktspace.c")
        .compile("mikktspace");
}
