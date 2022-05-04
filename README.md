## `merge-headers`

This repo contains a very simple naive script to merge multiple header files
to a single file. It's somewhat similar to what JS compilers do these days.

It does very slow and ineficient topological sort to merge files correctly (i.e. first write files that have no depedencies and therefore write the most API-heavy file at the end)

### Usage

```sh
$ ./merge-headers \
    --cc <C COMPILER> \
    --headers "filelist separated by ;" \
    --write-to <OUTFILE> \
    --include-guard-prefix <HEADERS INCLUDE GUARD PREFIX> \
    --output-guard <OUTFILE INCLUDE GUARD>
```

### Example

`Makefile` has a very simple `test` task:

```sh
test:
    cargo run -- \
        --cc clang \
		--headers "fixtures/input1.h;fixtures/input2.h" \
		--write-to merged.h \
		--include-guard-prefix FIXTURE_ \
		--output-guard GUARD_H
```

`fixtures/input1.h` has an include guard and no dependencies:

```c
#ifndef FIXTURE_INPUT1_H
#define FIXTURE_INPUT1_H

#include <stdio.h>

void input1(void);

#endif // FIXTURE_INPUT1_H
```

`fixtures/input2.h` also has an include guard and depends on `fixture1.h`:

```c
#ifndef FIXTURE_INPUT2_H
#define FIXTURE_INPUT2_H

#include <stdbool.h>
#include "input1.h"

void input2(void);

#endif // FIXTURE_INPUT2_H
```

Merged output is:

```c
#ifndef GUARD_H
#define GUARD_H

#include <stdbool.h>
#include <stdio.h>

void input1(void);

void input2(void);

#endif // GUARD_H
```

As you can see:

1. Original guards starting with `FIXTURE_` have been removed.
2. Files have been ordered (fixture2 depends on fixture1, so its content is placed last)
3. New guard has been added
4. Global `#include` directives have been copied (and de-duplicated).

### Requirements

This script has only one requirement:

1. Include guard in header files must end with `#endif <GUARD PREFIX>`, otherwise it's not removed.

### Testing

1. Clone the repo
2. Run `cargo test`

### Releases

You can download pre-compiled executable from the [Releases pages](https://github.com/iliabylich/merge_headers/releases).

Currently the following triplets are supported:

1. `x86_64-apple-darwin` (i.e. Apple Intel)
2. `x86_64-unknown-linux-gnu` (i.e. Linux with shared libc)

If you want to build it manually:

1. Clone the repo
2. Run `cargo build --release`
3. `target/release/merge-headers` is your executable.

