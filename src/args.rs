#[derive(Debug, PartialEq)]

pub(crate) struct Args {
    pub(crate) cc: String,
    pub(crate) headers: Vec<String>,
    pub(crate) include_guard_prefix: String,
    pub(crate) write_to: String,
    pub(crate) output_guard: String,
    pub(crate) debug: bool,
}

impl Args {
    const USAGE: &'static str = r#"
Usage:

./merge_headers --cc <CC> --headers "foo.h;bar.h" --include-guard-prefix="<PREFIX>" --write-to <OUTFILE>
"#;
    fn print_usage_and_exit() -> ! {
        panic!("{}", Self::USAGE.trim());
    }

    pub(crate) fn parse<T: Into<String>, S: IntoIterator<Item = T>>(args: S) -> Self {
        let mut args = args
            .into_iter()
            .map(|e| {
                let s: String = e.into();
                s
            })
            .collect::<Vec<_>>();
        let mut get_arg = |key: &str| {
            let key_idx = args
                .iter()
                .enumerate()
                .find(|&(_idx, e)| e == key)
                .unwrap_or_else(|| {
                    eprintln!("Unable to get {} CLI argument", key);
                    Args::print_usage_and_exit()
                })
                .0;
            let _key = args.remove(key_idx);
            if key_idx >= args.len() {
                eprintln!("No {} CLI option given", key);
                Args::print_usage_and_exit();
            }
            let value = args.remove(key_idx);
            value
        };

        let cc = get_arg("--cc");
        let headers = get_arg("--headers")
            .split(";")
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        let include_guard_prefix = get_arg("--include-guard-prefix");
        let write_to = get_arg("--write-to");
        let output_guard = get_arg("--output-guard");
        let debug = std::env::var("MERGE_HEADERS_DEBUG").is_ok();

        Self {
            cc,
            headers,
            include_guard_prefix,
            write_to,
            output_guard,
            debug,
        }
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub(crate) fn dummy() -> Args {
    Args::parse(vec![
        "--cc",
        "clang",
        "--headers",
        "fixtures/input1.h;fixtures/input2.h",
        "--include-guard-prefix",
        "GUARD_",
        "--write-to",
        "output.h",
        "--output-guard",
        "GUARD_H",
    ])
}

#[test]
fn test_args() {
    assert_eq!(
        dummy(),
        Args {
            cc: String::from("clang"),
            headers: vec![
                String::from("fixtures/input1.h"),
                String::from("fixtures/input2.h"),
            ],
            include_guard_prefix: String::from("GUARD_"),
            write_to: String::from("output.h"),
            output_guard: String::from("GUARD_H"),
            debug: false
        }
    )
}
