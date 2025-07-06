use std::fs;
use std::path::Path;

pub fn new_project(dir_name: &Path) {
    fs::create_dir(&dir_name).expect("Cannot create directory (invalid name)");
    fs::create_dir(&dir_name.join("src")).expect("'src/' directory already exists");

    let _ = fs::write(
        &dir_name.join("main.glang"),
        "func main() {\n    bark(\"Hello world!\");\n}\n\nmain();",
    );
    let _ = fs::write(
        &dir_name.join("README.md"),
        "# Welcome to GLang!\nTo get started, see our documentation [here](https://sites.google.com/view/george-lang/documentation).",
    );
}
