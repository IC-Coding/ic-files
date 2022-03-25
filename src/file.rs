use std::fs;
use std::process::Command;

//create file
pub fn mk(file_path: &str){
    fs::File::create(file_path).expect("Failed to create file");

}

//delete file
pub fn rm(file_path: &str) -> String {
    let err = Command::new("rm").arg(file_path).output().expect("Failed to delete file").stderr;
    String::from_utf8_lossy(&err).to_string()
}

//copy file
pub fn cp(orginal_path: &str, copy_path: &str) -> String {
    let err = Command::new("cp").args([orginal_path,copy_path]).output().expect("Failed to copy file").stderr;
    String::from_utf8_lossy(&err).to_string()
}
