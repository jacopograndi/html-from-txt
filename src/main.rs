use std::fs;
use std::path::Path;

const PAGE_TITLE_MARKER: &str = "no page title";
const PAGE_SOURCE_MARKER: &str = "no page source";

fn embed(path: &Path, base: &String) {
    let source = fs::read_to_string(path).expect("Can't read file");
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let filepath = path.parent().unwrap().to_str().unwrap();
    let htmlname = format!("{}/{}.html", filepath, filename);
    let withtitle = base.replace(PAGE_TITLE_MARKER, filename);
    let embedded = withtitle.replace(PAGE_SOURCE_MARKER, &source);
    fs::write(htmlname, embedded).unwrap();
}

fn traverse(fullpath: &str, base: &String) {
    for entry in fs::read_dir(fullpath).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            traverse(path.to_str().unwrap(), base);
        } else {
            match path.extension() {
                Some(os_str) => {
                    if os_str == "txt" {
                        embed(&path, &base);
                    }
                }
                _ => (),
            }
        }
    }
}

fn main() {
    let base: String = fs::read_to_string("base.html").expect("File not found: base.html");
    traverse("./", &base);
}
