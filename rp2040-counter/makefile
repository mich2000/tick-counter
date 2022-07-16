build:
	cargo b --release && elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040-env ./rp2040-env.uf2

upload:
	test -f rp2040-env.uf2 && mount /dev/sdb1 /mnt/pico/ && cp rp2040-env.uf2 /mnt/pico/. && umount /mnt/pico && echo "succeeded"