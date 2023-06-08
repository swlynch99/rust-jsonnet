use std::env;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    if !Path::new("jsonnet/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let dir = Path::new("jsonnet");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR undefined"));

    let embedded = ["std"];
    for f in &embedded {
        let output = out_dir.join("include").join(format!("{}.jsonnet.h", f));
        let input = dir.join("stdlib").join(format!("{}.jsonnet", f));
        println!("embedding: {:?} -> {:?}", input, output);
        create_dir_all(output.parent().unwrap())?;
        let in_f = File::open(input)?;
        let mut out_f = File::create(&output)?;
        for b in in_f.bytes() {
            write!(&mut out_f, "{},", b?)?;
        }
        writeln!(&mut out_f, "0")?;
    }

    let jsonnet_core = [
        "desugarer.cpp",
        "formatter.cpp",
        "lexer.cpp",
        "libjsonnet.cpp",
        "parser.cpp",
        "pass.cpp",
        "static_analysis.cpp",
        "string_utils.cpp",
        "vm.cpp",
    ];

    let mut c = cc::Build::new();
    c.cpp(true);
    c.flag_if_supported("-std=c++17");
    c.include(out_dir.join("include"));
    c.include(dir.join("include"));
    c.include(dir.join("third_party/md5"));
    c.include(dir.join("third_party/json"));
    c.include(dir.join("third_party/rapidyaml/rapidyaml/src/"));
    c.include(dir.join("third_party/rapidyaml/rapidyaml/ext/c4core/src/"));

    for f in &jsonnet_core {
        c.file(dir.join("core").join(f));
    }

    for f in glob::glob("jsonnet/third_party/rapidyaml/rapidyaml/src/c4/yml/*.cpp")? {
        c.file(f?);
    }

    for f in glob::glob("jsonnet/third_party/rapidyaml/rapidyaml/ext/c4core/src/c4/*.cpp")? {
        c.file(f?);
    }

    c.file(dir.join("third_party/md5/md5.cpp"));

    c.compile("libjsonnet.a");

    Ok(())
}
