//! Tests for -Zcheck-cfg.

use cargo_test_support::{basic_manifest, is_nightly, project};

macro_rules! x {
    ($tool:tt => $what:tt $(of $who:tt)?) => {{
        #[cfg(windows)]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg ",
                    $what, '(', $($who,)* ')', "[..]")
        }
        #[cfg(not(windows))]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg '",
                    $what, '(', $($who,)* ')', "'", "[..]")
        }
    }};
    ($tool:tt => $what:tt of $who:tt with $($values:tt)*) => {{
        #[cfg(windows)]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg \"",
                    $what, '(', $who, $(", ", "/\"", $values, "/\"",)* ")", '"', "[..]")
        }
        #[cfg(not(windows))]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg '",
                    $what, '(', $who, $(", ", "\"", $values, "\"",)* ")", "'", "[..]")
        }
    }};
}

#[cargo_test]
fn features() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn features_with_deps() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/" }

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn features_with_opt_deps() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/", optional = true }

                [features]
                default = ["bar"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "bar" "default" "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn features_with_namespaced_features() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/", optional = true }

                [features]
                f_a = ["dep:bar"]
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn well_known_names() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=names")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[cargo_test]
fn well_known_values() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[cargo_test]
fn cli_all_options() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("build -v -Zcheck-cfg=features,names,values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn features_with_cargo_check() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn well_known_names_with_check() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("check -v -Zcheck-cfg=names")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[cargo_test]
fn well_known_values_with_check() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("check -v -Zcheck-cfg=values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[cargo_test]
fn features_test() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("test -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn features_doctest() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc and rustdoc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                default = ["f_a"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.cargo("test -v --doc -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "default" "f_a" "f_b"))
        .with_stderr_contains(x!("rustdoc" => "values" of "feature" with "default" "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn well_known_names_test() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("test -v -Zcheck-cfg=names")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[cargo_test]
fn well_known_values_test() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("test -v -Zcheck-cfg=values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[cargo_test]
fn well_known_names_doctest() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc and rustdoc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.cargo("test -v --doc -Zcheck-cfg=names")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustdoc" => "names"))
        .run();
}

#[cargo_test]
fn well_known_values_doctest() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc and rustdoc command line
        return;
    }

    let p = project()
        .file("Cargo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.cargo("test -v --doc -Zcheck-cfg=values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustdoc" => "values"))
        .run();
}

#[cargo_test]
fn features_doc() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustdoc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                default = ["f_a"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.cargo("doc -v -Zcheck-cfg=features")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustdoc" => "values" of "feature" with "default" "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn build_script_feedback() {
    if !is_nightly() {
        // rustc-check-cfg: is only availaible on nightly
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"fn main() { println!("cargo:rustc-check-cfg=names(foo)"); }"#,
        )
        .build();

    p.cargo("build -v -Zcheck-cfg=output")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .run();
}

#[cargo_test]
fn build_script_doc() {
    if !is_nightly() {
        // rustc-check-cfg: is only availaible on nightly
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"fn main() { println!("cargo:rustc-check-cfg=names(foo)"); }"#,
        )
        .build();
    p.cargo("doc -v -Zcheck-cfg=output")
        .with_stderr_does_not_contain("rustc [..] --check-cfg [..]")
        .with_stderr_contains(x!("rustdoc" => "names" of "foo"))
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc [..] build.rs [..]`
[RUNNING] `[..]/build-script-build`
[DOCUMENTING] foo [..]
[RUNNING] `rustdoc [..] src/main.rs [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .run();
}

#[cargo_test]
fn build_script_override() {
    if !is_nightly() {
        // rustc-check-cfg: is only availaible on nightly
        return;
    }
    let target = cargo_test_support::rustc_host();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.5.0"
                authors = []
                links = "a"
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("build.rs", "")
        .file(
            ".cargo/config",
            &format!(
                r#"
                    [target.{}.a]
                    rustc-check-cfg = ["names(foo)"]
                "#,
                target
            ),
        )
        .build();

    p.cargo("build -v -Zcheck-cfg=output")
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .run();
}

#[cargo_test]
fn build_script_test() {
    if !is_nightly() {
        // rustc-check-cfg: is only availaible on nightly
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file(
            "build.rs",
            r#"fn main() { 
                println!("cargo:rustc-check-cfg=names(foo)");
                println!("cargo:rustc-cfg=foo");
            }"#,
        )
        .file(
            "src/lib.rs",
            r#"
                ///
                /// ```
                /// extern crate foo;
                ///
                /// fn main() {
                ///     foo::foo()
                /// }
                /// ```
                ///
                #[cfg(foo)]
                pub fn foo() {}

                #[cfg(foo)]
                #[test]
                fn test_foo() {
                    foo()
                }
            "#,
        )
        .file("tests/test.rs", "#[cfg(foo)] #[test] fn test_bar() {}")
        .build();

    p.cargo("test -v -Zcheck-cfg=output")
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .with_stderr_contains(x!("rustdoc" => "names" of "foo"))
        .with_stdout_contains("test test_foo ... ok")
        .with_stdout_contains("test test_bar ... ok")
        .with_stdout_contains_n("test [..] ... ok", 3)
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .run();
}

#[cargo_test]
fn config_valid() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            ".cargo/config.toml",
            r#"
                [unstable]
                check-cfg = ["features", "names", "values"]
            "#,
        )
        .build();

    p.cargo("build -v -Zcheck-cfg=features,names,values")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[cargo_test]
fn config_invalid() {
    if !is_nightly() {
        // --check-cfg is a nightly only rustc command line
        return;
    }

    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            ".cargo/config.toml",
            r#"
                [unstable]
                check-cfg = ["va"]
            "#,
        )
        .build();

    p.cargo("build")
        .masquerade_as_nightly_cargo(&["check-cfg"])
        .with_stderr_contains("error: unstable check-cfg only takes `features`, `names`, `values` or `output` as valid inputs")
        .with_status(101)
        .run();
}
