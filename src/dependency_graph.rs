use std::collections::{HashMap, HashSet};
use std::process::Command;

pub(crate) struct DependencyGraph {
    deps: HashMap<String, HashSet<String>>,
}

fn get_deps(cc: &str, filepath: &str) -> HashSet<String> {
    let args = ["-MT", filepath, "-MM", filepath];
    eprintln!("Running {} {:?}", cc, args.join(" "));

    let output = Command::new(cc)
        .args(args)
        .output()
        .unwrap_or_else(|error| {
            eprintln!("Failed to execute '{} {:?}':\n{}", cc, args, error);
            std::process::exit(1);
        });
    if !output.stderr.is_empty() {
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    let output = String::from_utf8(output.stdout).unwrap_or_else(|_err| {
        eprintln!("{} {:?} returned utf-8-incompatible", cc, args,);
        std::process::exit(1);
    });

    let deps = output.split(':').collect::<Vec<_>>();
    if deps.len() != 2 {
        eprintln!("{} {:?} returns incompatible output:\n{}", cc, args, output);
        std::process::exit(1);
    }
    let deps = deps[1]
        .split_whitespace()
        // Makefile separator
        .filter(|s| *s != "\\")
        // Exclude itself
        .filter(|s| *s != filepath)
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    eprintln!("Deps of {}: {:#?}\n", filepath, deps);
    deps
}

impl DependencyGraph {
    pub(crate) fn new(cc: &str, headers: &[String]) -> Self {
        let mut deps_map = HashMap::new();
        for header in headers {
            let deps = get_deps(cc, header);
            deps_map.insert(header.clone(), deps);
        }
        Self { deps: deps_map }
    }

    pub(crate) fn sorted(self) -> Vec<String> {
        let Self { mut deps } = self;
        let mut output = vec![];

        loop {
            if deps.is_empty() {
                return output;
            }

            let zero_dep_header = deps
                .iter()
                .find(|(_filepath, deps)| deps.is_empty())
                .unwrap_or_else(|| {
                    eprintln!("Unable to find a header with no dependencies: {:?}", deps);
                    std::process::exit(1);
                })
                .0
                .clone();
            eprintln!("Taking zero_dep_header = {}", zero_dep_header);

            deps.remove(&zero_dep_header);
            for single_file_deps in deps.values_mut() {
                single_file_deps.remove(&zero_dep_header);
            }

            output.push(zero_dep_header);
        }
    }
}
