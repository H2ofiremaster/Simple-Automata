use std::{ffi::OsString, fs, path::Path};

use regex::Regex;

fn main() -> std::io::Result<()> {
    let resources = Path::new("resources/");

    for file in (fs::read_dir(resources)?).flatten() {
        let path = file.path();
        if path.extension().is_some_and(|e| e == "css")
            && path
                .file_stem()
                .is_some_and(|s| !s.to_string_lossy().ends_with("_f"))
        {
            format_file(&path)?;
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    println!();
    Ok(())
}

fn format_file(path: &Path) -> std::io::Result<()> {
    println!("Formatting {path:?}");
    let regex = Regex::new(r"var\((--.*)\)").map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Regex failed to build: {err}"),
        )
    })?;
    let contents = fs::read_to_string(path)?;
    let filtered = regex.replace_all(&contents, |captures: &regex::Captures| {
        println!("Found variable: {}", &captures[0]);
        let default = String::from("#000");
        let variable = captures.get(1).map(|m| m.as_str());
        let Some(variable) = variable else {
            return default;
        };
        println!("{variable}");
        let Some(variable_index) = contents.find(variable) else {
            return default;
        };
        println!(
            "Index: {variable_index} \"{}\"",
            &contents[variable_index..variable_index + variable.len() + 16]
        );
        let Some(end_index) = contents[variable_index..].find(';') else {
            return default;
        };
        let end_index = variable_index + end_index;
        println!(
            "End Index: {end_index} \"{}\"",
            &contents[end_index..=end_index + 2]
        );
        // panic!();
        let start_index = variable_index + variable.len() + ": ".len();
        let value = &contents[start_index..end_index];
        value.to_string()
    });
    let mut file_name = path
        .file_stem()
        .map_or_else(OsString::new, std::ffi::OsStr::to_os_string);
    file_name.push("_f.");
    file_name.push(path.extension().unwrap_or(&OsString::new()));
    fs::write(path.with_file_name(file_name), filtered.as_bytes())
}
