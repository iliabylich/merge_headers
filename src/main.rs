mod args;
use args::Args;

mod dependency_graph;
use dependency_graph::DependencyGraph;

#[macro_export]
macro_rules! println_if_debug {
    ($debug:expr, $fmt_string:expr, $( $arg:expr ),*) => {
        if $debug {
            eprintln!($fmt_string, $( $arg ),*);
        }
    };
}

fn main() {
    let args = Args::parse();
    eprintln!("Running with args = {:#?}", args);

    let graph = DependencyGraph::new(args.debug, &args.cc, &args.headers);
    let sorted = graph.sorted();
    println_if_debug!(args.debug, "Sorted list:\n{:#?}", sorted);

    // Merging
    let mut output = vec![];
    let mut includes = vec![];

    for filepath in sorted {
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

    let output = format!(
        "#ifndef {guard}
#define {guard}

{includes}

{contents}

#endif // {guard}
",
        guard = args.output_guard,
        includes = includes.join("\n"),
        contents = output.join("\n")
    );

    std::fs::write(&args.write_to, output).unwrap();
    eprintln!("Writing to {}", args.write_to);
}
