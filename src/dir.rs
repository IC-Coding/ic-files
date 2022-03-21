use std::fs;

//list files and folders in dir
pub fn list(path: &str, show_hidden: bool) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut result: Vec<String> = vec![];

    for path in paths {
        if show_hidden == true {
            result.push(path.unwrap().path().display().to_string());
        }

    }

    return result;
}
