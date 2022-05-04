mod args;
use args::Args;

mod dependency_graph;
use dependency_graph::DependencyGraph;

mod combine;
use combine::combine;

#[macro_export]
macro_rules! println_if_debug {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        if std::env::var("MERGE_HEADERS_DEBUG").is_ok() {
            eprintln!($fmt_string, $( $arg ),*);
        }
    };
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let args = Args::parse(args);
    eprintln!("Running with args = {:#?}", args);

    let graph = DependencyGraph::new(&args.cc, &args.headers);
    let sorted = graph.sorted();
    println_if_debug!("Sorted list:\n{:#?}", sorted);

    let combined = combine(&args, &sorted);

    std::fs::write(&args.write_to, combined).unwrap();
    eprintln!("Writing to {}", args.write_to);
}
