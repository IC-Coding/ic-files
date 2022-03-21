
mod dir;
fn main() {
    println!("{:?}", dir::mkdir(format!("{}{}",dir::get_home(), "/test").as_str()));
}
