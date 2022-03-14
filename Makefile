.PHONY: execute
execute:
	cd exec && cargo run


.PHONY: dump_shellcode
dump_shellcode:
	objdump -D -b binary -mi386 -Mx86-64 -Mintel -z shellcode.bin


.PHONY: fmt
fmt:
	cd exec && cargo fmt



.PHONY: shell
shell:
	cd shell && cargo +nightly build --release
	strip -s shell/target/release/shell
	objcopy -O binary shell/target/release/shell shellcode.bin

.PHONY: dump_shell
dump_shell: shell dump_shellcode

.PHONY: run_shell
run_shell: shell execute

.PHONY: build_shell_debug
build_shell_debug:
	cd shell && cargo +nightly build


.PHONY: tcp
tcp:
	cd rev_tcp && cargo +nightly build --release
	strip -s rev_tcp/target/release/rev_tcp
	objcopy -O binary rev_tcp/target/release/rev_tcp shellcode.bin

.PHONY: dump_tcp
dump_tcp: tcp dump_shellcode

.PHONY: run_tcp
run_tcp: tcp execute
