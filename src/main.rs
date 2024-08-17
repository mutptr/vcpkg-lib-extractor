use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let vcpkg_installed_dir = "vcpkg_installed";

    // Traverse through all directories in vcpkg_installed_dir
    traverse_directories(Path::new(vcpkg_installed_dir)).unwrap();
}

fn traverse_directories(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let sub_path = path.join(dir_name);

            let mut file = File::create("vcpkg.h")?;
            writeln!(file, "#if defined _DEBUG").ok();
            let debug_lib = sub_path.join("debug").join("lib");
            if let Ok(lib_files) = get_lib_files(&debug_lib) {
                for lib_file in lib_files {
                    writeln!(file, "    #pragma comment(lib, \"{}\")", lib_file).ok();
                }
            }
            writeln!(file, "#else").ok();
            let lib = sub_path.join("lib");
            if let Ok(lib_files) = get_lib_files(&lib) {
                for lib_file in lib_files {
                    writeln!(file, "    #pragma comment(lib, \"{}\")", lib_file).ok();
                }
            }
            writeln!(file, "#endif").ok();
        }
    }
    Ok(())
}

fn get_lib_files(dir: &Path) -> io::Result<Vec<String>> {
    let mut lib_files = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                lib_files.push(file_name.to_owned());
            }
        }
    }
    Ok(lib_files)
}
