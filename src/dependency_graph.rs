use crate::println_if_debug;
use std::collections::{HashMap, HashSet};
use std::process::Command;

pub(crate) struct DependencyGraph {
    deps: HashMap<String, HashSet<String>>,
    debug: bool,
}

fn get_deps(debug: bool, cc: &str, filepath: &str) -> HashSet<String> {
    let args = ["-MT", filepath, "-MM", filepath];
    println_if_debug!(debug, "Running {} {:?}", cc, args.join(" "));

    let output = Command::new(cc)
        .args(args)
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to execute '{} {:?}':\n{}", cc, args, error);
        });
    if !output.stderr.is_empty() {
        println_if_debug!(
            debug,
            "Stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let output = String::from_utf8(output.stdout).unwrap_or_else(|_err| {
        panic!("{} {:?} returned utf-8-incompatible", cc, args,);
    });

    let deps = output.split(':').collect::<Vec<_>>();
    if deps.len() != 2 {
        panic!("{} {:?} returns incompatible output:\n{}", cc, args, output);
    }
    let deps = deps[1]
        .split_whitespace()
        // Makefile separator
        .filter(|s| *s != "\\")
        // Exclude itself
        .filter(|s| *s != filepath)
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    println_if_debug!(debug, "Deps of {}: {:#?}\n", filepath, deps);
    deps
}

impl DependencyGraph {
    pub(crate) fn new(debug: bool, cc: &str, headers: &[String]) -> Self {
        let mut deps_map = HashMap::new();
        for header in headers {
            let deps = get_deps(debug, cc, header);
            deps_map.insert(header.clone(), deps);
        }
        Self {
            deps: deps_map,
            debug,
        }
    }

    pub(crate) fn sorted(self) -> Vec<String> {
        let Self { mut deps, debug } = self;
        let mut output = vec![];

        loop {
            if deps.is_empty() {
                return output;
            }

            let zero_dep_header = deps
                .iter()
                .find(|(_filepath, deps)| deps.is_empty())
                .unwrap_or_else(|| {
                    panic!("Unable to find a header with no dependencies: {:?}", deps);
                })
                .0
                .clone();
            println_if_debug!(debug, "Taking zero_dep_header = {}", zero_dep_header);

            deps.remove(&zero_dep_header);
            for single_file_deps in deps.values_mut() {
                single_file_deps.remove(&zero_dep_header);
            }

            output.push(zero_dep_header);
        }
    }
}
