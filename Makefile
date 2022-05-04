INPUT1 = fixtures/input1.h
INPUT2 = fixtures/input2.h

test: merged.h
	cat merged.h

merged.h: $(INPUT1) $(INPUT2)
	DEBUG=1 \
		cargo run -- \
		--cc clang \
		--headers "$(INPUT1);$(INPUT2)" \
		--write-to merged.h \
		--include-guard-prefix FIXTURE_ \
		--output-guard GUARD_H

clean:
	rm -f merged.h
