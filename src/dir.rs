use std::fs;
use std::process::Command;

//get home directory
pub fn get_home() -> String {
    let user = Command::new("whoami").output().expect("failed to execute process").stdout;
    let user_string = String::from_utf8_lossy(&user).replace("\n","");

    "/home/".to_owned() + &user_string

}


//list files and folders in dir
pub fn list(path: &str, show_hidden: bool) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut result: Vec<String> = vec![];

    for path in paths {
        if show_hidden == true {
            result.push(path.unwrap().path().display().to_string());
        }

        else if path.as_ref().unwrap().path().display().to_string().contains("./.") == false {
            result.push(path.unwrap().path().display().to_string());
        }

    }

    return result;
}
