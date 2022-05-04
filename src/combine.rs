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
            out.push(line.to_string());
        }
        output.append(&mut out);
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
    remove_2_cons_empty_lines(output)
}

fn remove_2_cons_empty_lines(input: String) -> String {
    let mut output = input.lines().collect::<Vec<_>>();

    let mut empty_lines_to_trim = vec![];
    for (idx, line1) in output.iter().enumerate() {
        if line1.is_empty() {
            if let Some(line2) = output.get(idx + 1) {
                if line2.is_empty() {
                    empty_lines_to_trim.push(idx);
                }
            }
        }
    }
    for idx in empty_lines_to_trim.iter().rev() {
        output.remove(*idx);
    }
    let mut output = output.join("\n");
    output.push_str("\n");
    output
}

#[test]
fn test_combine() {
    assert_eq!(
        combine(
            &crate::args::dummy(),
            &[
                String::from("fixtures/input1.h"),
                String::from("fixtures/input2.h")
            ]
        ),
        "#ifndef GUARD_H
#define GUARD_H

#include <stdbool.h>
#include <stdio.h>

void input1(void);

void input2(void);

#endif // GUARD_H
"
    )
}
