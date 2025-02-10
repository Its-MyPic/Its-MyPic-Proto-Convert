use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {:?}", out_dir);
    fs::create_dir_all(&out_dir.join("protos")).unwrap();
    prost_build::Config::new()
        .out_dir(out_dir.join("protos"))
        .compile_protos(&["src/protos/data.proto"], &["src/protos/"])
        .unwrap();
}
