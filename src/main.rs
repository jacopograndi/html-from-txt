use std::fs;
use std::path::Path;

fn embed(path : &Path, base : &String) {
    let source = fs::read_to_string(path).expect("Can't read file");
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let filepath = path.parent().unwrap().to_str().unwrap();
    let htmlname = format!("{}/{}.html", filepath, filename);
    let convertnewline = source.replace("\n", "<br>\n");
    let withtitle = base.replace("no page title", filename);
    let embedded = withtitle.replace("<p>no page source</p>", &convertnewline);
    fs::write(htmlname, embedded).unwrap();
}

fn traverse(fullpath : &str, base : &String) {
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
                _ => ()
            }
        }
    }
}

fn main() {
    let base : String= fs::read_to_string("base.html")
        .expect("File not found: base.html");

    traverse("./", &base);
}
