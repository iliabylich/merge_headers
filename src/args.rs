#[derive(Debug)]

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
        eprintln!("{}", Self::USAGE.trim());
        std::process::exit(1);
    }

    pub(crate) fn parse() -> Self {
        let mut args = std::env::args().into_iter().collect::<Vec<_>>();
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
