#[macro_export]
macro_rules! badges {
    (repo = $repo_name:literal) => {
        $crate::badges!(repo = $repo_name, msrv = env!("CARGO_PKG_RUST_VERSION"))
    };
    (repo = $repo_name:literal, msrv = $rust_version:expr) => {
        concat!(
            "[![crates.io](https://img.shields.io/crates/v/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=rust)](https://crates.io/crates/", env!("CARGO_PKG_NAME"), ")\n",
            "[![docs.rs](https://img.shields.io/docsrs/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=docs.rs)](https://docs.rs/", env!("CARGO_PKG_NAME"), ")\n",
            "![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)\n",
            "![msrv](https://img.shields.io/badge/msrv-", $rust_version, "-blue?style=flat-square&logo=rust)\n",
            "[![github](https://img.shields.io/github/stars/nik-rev/", $repo_name, ")](https://github.com/nik-rev/", $repo_name, ")"
        )
    };
}

#[macro_export]
#[rustfmt::skip]
macro_rules! add_dependency {
    () => {
        concat!(
            "```toml\n",
            env!("CARGO_CRATE_NAME"), " = \"",  env!("CARGO_PKG_VERSION_MAJOR"), ".", env!("CARGO_PKG_VERSION_MINOR"), "\"\n",
            "```\n",
        )
    };
}
