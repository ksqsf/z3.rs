fn main() {
    #[cfg(feature = "static-link-z3")]
    build_z3();
}

#[cfg(feature = "static-link-z3")]
fn build_z3() {
    let mut cfg = cmake::Config::new("z3");
    cfg
        // Don't build `libz3.so`, build `libz3.a` instead.
        .define("Z3_BUILD_LIBZ3_SHARED", "false")
        // Don't build the Z3 repl.
        .define("Z3_BUILD_EXECUTABLE", "false")
        // Don't build the tests.
        .define("Z3_BUILD_TEST_EXECUTABLES", "false");

    if cfg!(target_os = "windows") {
        cfg.cxxflag("-DWIN32");
        cfg.cxxflag("-D_WINDOWS");
    }

    let dst = cfg.build();

    // Z3 needs a C++ standard library. Customize which one we use with the
    // `CXXSTDLIB` environment variable, if needed.
    let cxx = match std::env::var("CXXSTDLIB") {
        Ok(s) if s.is_empty() => None,
        Ok(s) => Some(s),
        Err(_) => {
            let target = std::env::var("TARGET").unwrap();
            if target.contains("msvc") {
                None
            } else if target.contains("apple") {
                Some("c++".to_string())
            } else if target.contains("freebsd") {
                Some("c++".to_string())
            } else if target.contains("openbsd") {
                Some("c++".to_string())
            } else {
                Some("stdc++".to_string())
            }
        }
    };

    if let Some(cxx) = cxx {
        println!("cargo:rustc-link-lib={}", cxx);
    }

    let lib = dst.join("lib");

    // For some reason Z3 builds as `libz3.lib`, but on windows the "lib" prefix
    // is not a convention.
    if cfg!(target_os = "windows") {
        let from = lib.join("libz3.lib");
        let to = lib.join("z3.lib");
        std::fs::copy(&from, &to).expect(&format!(
            "failed to copy `{}` to `{}`",
            from.display(),
            to.display()
        ));
    }

    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=static=z3");
}
