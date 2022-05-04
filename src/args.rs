#[derive(Debug, PartialEq)]

pub(crate) struct Args {
    pub(crate) cc: String,
    pub(crate) headers: Vec<String>,
    pub(crate) include_guard_prefix: String,
    pub(crate) write_to: String,
    pub(crate) output_guard: String,
}

impl Args {
    const USAGE: &'static str = r#"
Usage:

./merge_headers --cc <CC> --headers "foo.h;bar.h" --include-guard-prefix="<PREFIX>" --write-to <OUTFILE>
"#;
    fn print_usage_and_exit() -> ! {
        panic!("{}", Self::USAGE.trim());
    }

    pub(crate) fn parse(mut args: Vec<String>) -> Self {
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

        Self {
            cc,
            headers,
            include_guard_prefix,
            write_to,
            output_guard,
        }
    }
}

#[allow(dead_code)]
pub(crate) fn dummy() -> Args {
    Args::parse(vec![
        String::from("--cc"),
        String::from("clang"),
        String::from("--headers"),
        String::from("fixtures/input1.h;fixtures/input2.h"),
        String::from("--include-guard-prefix"),
        String::from("FIXTURE_"),
        String::from("--write-to"),
        String::from("output.h"),
        String::from("--output-guard"),
        String::from("GUARD_H"),
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
            include_guard_prefix: String::from("FIXTURE_"),
            write_to: String::from("output.h"),
            output_guard: String::from("GUARD_H"),
        }
    )
}
