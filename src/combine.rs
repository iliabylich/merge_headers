use crate::{args::Args, println_if_debug};

pub(crate) fn combine(args: &Args, files: &[String]) -> String {
    let mut output = vec![];
    let mut includes = vec![];

    for filepath in files {
        let contents = std::fs::read_to_string(&filepath).unwrap();
        let mut out = vec![];
        let mut lines = contents.lines();
        while let Some(line) = lines.next() {
            if line.starts_with(&format!("#ifndef {}", args.include_guard_prefix)) {
                // open guard
                // skip next "#define GUARD" line
                let _ = lines.next().unwrap();
                continue;
            }
            if line.starts_with(&format!("#endif // {}", args.include_guard_prefix)) {
                // close guard
                continue;
            }
            if line.starts_with("#include <") {
                // global include
                includes.push(line.to_string());
                continue;
            }
            if line.starts_with("#include \"") {
                // local include, ignore
                continue;
            }
            out.push(line);
        }
        let out = out.join("\n");
        output.push(out);
    }

    includes.sort_unstable();
    includes.dedup();
    println_if_debug!(args.debug, "includes = {:#?}", includes);

    format!(
        "#ifndef {guard}
#define {guard}

{includes}

{contents}

#endif // {guard}
",
        guard = args.output_guard,
        includes = includes.join("\n"),
        contents = output.join("\n")
    )
}
