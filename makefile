build:
	cd rp2040-counter && cargo b --release && elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-env ./rp2040-env.uf2 && echo "building raspberry pico code succeeded"

upload:
	cd rp2040-counter && test -f rp2040-env.uf2 && mount /dev/sdb1 /mnt/pico/ && cp rp2040-env.uf2 /mnt/pico/. && umount /mnt/pico && echo "uploading code to raspberry pico succeeded"

test:
	cd fmt_buf && cargo t && echo "testing the fmt_buf library succeeded"

fmt:
	cd fmt_buf && cargo fmt && cd ../rp2040-counter && cargo fmt && echo "beautifying projects succeeded"