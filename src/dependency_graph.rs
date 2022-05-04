use crate::println_if_debug;
use std::collections::{HashMap, HashSet};
use std::process::Command;

pub(crate) struct DependencyGraph {
    deps: HashMap<String, HashSet<String>>,
}

fn get_deps(cc: &str, filepath: &str) -> HashSet<String> {
    let args = ["-MT", filepath, "-MM", filepath];
    println_if_debug!("Running {} {:?}", cc, args.join(" "));

    let output = Command::new(cc)
        .args(args)
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to execute '{} {:?}':\n{}", cc, args, error);
        });
    if !output.stderr.is_empty() {
        println_if_debug!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
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

    println_if_debug!("Deps of {}: {:#?}\n", filepath, deps);
    deps
}

impl DependencyGraph {
    pub(crate) fn new<T: Into<String>, S: IntoIterator<Item = T>>(cc: &str, headers: S) -> Self {
        let mut deps_map = HashMap::new();
        for header in headers {
            let header: String = header.into();
            let deps = get_deps(cc, &header);
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
                    panic!("Unable to find a header with no dependencies: {:?}", deps);
                })
                .0
                .clone();
            println_if_debug!("Taking zero_dep_header = {}", zero_dep_header);

            deps.remove(&zero_dep_header);
            for single_file_deps in deps.values_mut() {
                single_file_deps.remove(&zero_dep_header);
            }

            output.push(zero_dep_header);
        }
    }
}

#[test]
fn test_get_deps() {
    assert_eq!(get_deps("clang", "fixtures/input1.h"), HashSet::from([]));

    assert_eq!(
        get_deps("clang", "fixtures/input2.h"),
        HashSet::from([String::from("fixtures/input1.h")])
    );
}

#[test]
fn test_dependency_graph() {
    assert_eq!(
        DependencyGraph::new("clang", ["fixtures/input1.h", "fixtures/input2.h"]).sorted(),
        // input2.h depends on input1.h
        ["fixtures/input1.h", "fixtures/input2.h"]
    )
}
