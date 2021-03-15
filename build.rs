use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // перемещения конфигурации памяти в путь поиска компоновщика
    fs::copy("memory.x", out_dir.join("memory.x"))?;

    // указывает путь поиска для компоновщика
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
