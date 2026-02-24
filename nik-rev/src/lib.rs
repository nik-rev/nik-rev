use std::env;

pub fn export_env_vars_for_docs() {
    let repo_name = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let repo_name = repo_name.strip_prefix("https://github.com/").unwrap();
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let msrv = env::var("CARGO_PKG_RUST_VERSION").unwrap_or_else(|_| "nightly".to_string());

    let badges = format!(
            "[![crates.io](https://img.shields.io/crates/v/{pkg_name}?style=flat-square&logo=rust)](https://crates.io/crates/{pkg_name})
[![docs.rs](https://img.shields.io/docsrs/{pkg_name}?style=flat-square&logo=docs.rs)](https://docs.rs/{pkg_name})
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-{msrv}-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/{repo_name})](https://github.com/nik-rev/{repo_name})"
        );

    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();

    let add_dep = format!("```toml\n{pkg_name} = \"{major}.{minor}\"\n```\n");

    println!("cargo:rustc-env=GENERATED_BADGES={}", badges);
    println!("cargo:rustc-env=GENERATED_ADD_DEP={}", add_dep);
}
