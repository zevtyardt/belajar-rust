use std::{
    collections::BTreeMap,
    fs::{read_dir, read_to_string, File},
    io::Write,
    path::Path,
};

fn walkdir(dir: &Path, indent: usize, data: &mut BTreeMap<String, Vec<BTreeMap<String, String>>>) {
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let splited: Vec<&str> = path.to_str().unwrap().split("/").collect();

        if path.is_dir() && ![".git", "target", "src"].contains(splited.last().unwrap()) {
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
                        "[{folder}](/{dirname})",
                        folder = splited[splited.len() - 2],
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
    let mut data = BTreeMap::new();
    walkdir(&start, 0, &mut data);

    let mut readme_file = File::create("./README.md").unwrap();

    write!(readme_file, "<div align=\"center\">\n\n# Rust 🦀\n\n").unwrap();
    let mut total = 0;
    for (_x, y) in data.iter() {
        total += y.len();
    }
    write!(readme_file, "Repo ini berisi semua program yang saya buat dalam rangka belajar bahasa pemrograman Rust\n\nTotal {} program telah dibuat sejauh ini 🎉\n\n</div>\n", total).unwrap();

    write!(readme_file, "\n").unwrap();
    write!(readme_file, "#### Tabel\n").unwrap();
    write!(readme_file, "\n").unwrap();
    write!(readme_file, "|Hari ke|Tanggal|Folder|Deskripsi|\n").unwrap();
    write!(readme_file, "|:--:|:--:|:--|:--|\n").unwrap();

    let mut start = 0;
    let mut is_skip_day = false;
    let mut hari_ke = 1;
    for (tanggal, projects) in data.iter() {
        let first_project = &projects[0];
        let d: Vec<&str> = tanggal.as_str().split_whitespace().collect();
        let d: i32 = d[0].parse().unwrap();

        if start == 0 {
            start = d - 1;
        }

        if hari_ke != 1 {
            let dist = (d - start) - hari_ke;
            if !is_skip_day && dist != 1 {
                write!(readme_file, "|...||||\n").unwrap();
                is_skip_day = true;
            } else if dist == 1 {
                is_skip_day = false
            }
        }
        hari_ke = d - start;

        write!(
            readme_file,
            "|{}|{}|{}|{}|\n",
            hari_ke,
            tanggal,
            first_project.get("folder").unwrap(),
            first_project.get("desc").unwrap()
        )
        .unwrap();
        for project in projects[1..].iter() {
            write!(
                readme_file,
                "|||{}|{}|\n",
                project.get("folder").unwrap(),
                project.get("desc").unwrap()
            )
            .unwrap();
        }
    }

    write!(
        readme_file,
        "\n\n_dibuat otomatis menggunakan [gen_readme.rs](/gen_readme.rs)_\n"
    )
    .unwrap();

    println!("{}", read_to_string("README.md").unwrap().as_str());
}
