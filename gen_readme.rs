use std::{
    collections::BTreeMap,
    fs::{read_dir, read_to_string, File},
    io::Write,
    path::Path,
};

/*
fn format_dir(fullpath: Vec<&str>) -> String {
    let s = &fullpath[1..];
    let folder = s.last().unwrap();

    if s.len() == 1 {
        format!("[{folder}](/{folder})")
    } else {
        let fullpath = s.join("/");
        format!("[{folder}](/{fullpath})")
    }
}
*/

fn walkdir(dir: &Path, indent: usize, data: &mut BTreeMap<String, Vec<BTreeMap<String, String>>>) {
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let splited: Vec<&str> = path.to_str().unwrap().split("/").collect();

        if path.is_dir() && ![".git", "target", "src"].contains(splited.last().unwrap()) {
            //lines.push(format!("{}+ {}", " ".repeat(indent), format_dir(splited)));
            walkdir(&path, indent + 2, data);
        } else if splited.len() > 2 && splited.contains(&"README.md") {
            let readme = read_to_string(&path).unwrap();
            let mut desc: Vec<&str> = readme
                .split("\n")
                .filter(|x| x.starts_with("+"))
                .map(|x| x.strip_prefix("+ ").unwrap())
                .collect();
            if desc.len() > 0 {
                desc[1] = desc[1].strip_prefix("Tanggal dibuat: ").unwrap();
                if !data.contains_key(desc[1]) {
                    data.insert(desc[1].to_string(), Vec::new());
                }
                let mut subdata = BTreeMap::new();
                let dirname = path.parent().unwrap();
                subdata.insert(String::from("desc"), desc[0].to_string());
                subdata.insert(
                    String::from("folder"),
                    String::from(format!(
                        "[{dirname}](/{dirname})",
                        dirname = dirname.strip_prefix("./").unwrap().display()
                    )),
                );

                data.get_mut(desc[1]).map(|val| val.push(subdata));
            }
        }
    }
}

fn main() {
    let start = Path::new("./");
    //let mut lines = vec![];
    let mut data = BTreeMap::new();
    walkdir(&start, 0, &mut data);

    let mut readme_file = File::create("./README.md").unwrap();

    write!(readme_file, "<div align=\"center\">\n\n# Rust 🦀\n\n").unwrap();
    write!(readme_file, "Repo ini berisi semua program yang saya buat dalam rangka belajar bahasa pemrograman Rust\n\n</div>\n").unwrap();

    write!(readme_file, "\n").unwrap();
    write!(readme_file, "#### Table\n").unwrap();
    write!(readme_file, "\n").unwrap();
    write!(readme_file, "|Hari ke|Tanggal|Folder|Deskripsi|\n").unwrap();
    write!(readme_file, "|:--:|:--:|:--:|:--:|\n").unwrap();
    for (index, (tanggal, projects)) in data.iter().enumerate() {
        let first_project = &projects[0];

        write!(
            readme_file,
            "|{}|{}|{}|{}|\n",
            index + 1,
            tanggal,
            first_project.get("folder").unwrap(),
            first_project.get("desc").unwrap()
        )
        .unwrap();
        for project in projects[1..].iter() {
            write!(
                readme_file,
                "|||{}|{}|",
                project.get("folder").unwrap(),
                project.get("desc").unwrap()
            )
            .unwrap();
        }
    }
    println!("Done table created");

    /*
    write!(readme_file, "\n\n").unwrap();
    write!(readme_file, "#### Directory\n").unwrap();
    write!(readme_file, "\n").unwrap();
    for line in lines {
        write!(readme_file, "{}\n", line).unwrap();
    }
    println!("Done directory created");
    */

    write!(
        readme_file,
        "\n\n_dibuat menggunakan [gen_readme.rs](/gen_readme.rs)_\n"
    )
    .unwrap()
}
