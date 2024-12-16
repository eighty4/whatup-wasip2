use std::io;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    gen_wit_bindings();
}

fn gen_wit_bindings() {
    // wit-bindgen rust wit/fn_build.wit --out-dir src/bindings
    Command::new("wit-bindgen")
        .arg("rust")
        .arg("wit/fn_build.wit")
        .arg("--out-dir")
        .arg("src/bindings")
        .stdout(io::stdout())
        .stderr(io::stderr())
        .output()
        .unwrap();
}
