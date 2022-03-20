use std::fs;

fn embed(filename : &String, base : &String, source : &String) {
    let htmlname = format!("{}.html", filename);
    let convertnewline = source.replace("\n", "<br>\n");
    let withtitle = base.replace("no page title", filename);
    let embedded = withtitle.replace("<p>no page source</p>", &convertnewline);
    fs::write(htmlname, embedded);
}

fn main() {
    let base : String= fs::read_to_string("base.html")
        .expect("File not found: base.html");

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        match path.extension() {
            Some(os_str) => {
                if os_str == "txt" {
                    let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
                    let source = fs::read_to_string(path).expect("Can't read file");
                    embed(&filename, &base, &source);
                }
            }
            _ => ()
        }
    }
}
