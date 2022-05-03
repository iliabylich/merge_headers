mod args;
use args::Args;

mod dependency_graph;
use dependency_graph::DependencyGraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Running with args = {:#?}", args);

    let graph = DependencyGraph::new(&args.cc, &args.headers);
    let sorted = graph.sorted();
    eprintln!("Sorted list:\n{:#?}", sorted);

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
                // let include = line
                //     .strip_prefix("#include <")
                //     .unwrap()
                //     .strip_suffix(">")
                //     .unwrap();
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
    eprintln!("includes = {:#?}", includes);

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
    Ok(())
}
