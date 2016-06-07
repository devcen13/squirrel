extern crate cmake;

fn add_squirrel_defines(cmake_cfg: &mut cmake::Config) {
	cmake_cfg.define("INSTALL_LIB_DIR", ".");
    if cfg!(feature = "use_double") {
        cmake_cfg.define("SQUSEDOUBLE", "");
    }
    if cfg!(feature = "use_unicode") {
        cmake_cfg.define("SQUNICODE", "");
    }
}

fn export_squirrel(path: std::path::PathBuf) {
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=squirrel_static");
    println!("cargo:rustc-flags=-l dylib=stdc++");
}


fn main() {
    let mut cmake_cfg = cmake::Config::new("squirrel");
    add_squirrel_defines(&mut cmake_cfg);
    let path = cmake_cfg.build();
    export_squirrel(path);
}
