
mod dir;
use std::fs::OpenOptions;


fn main() {
    let file = OpenOptions::new().read(true).open("foo.txt");
}
