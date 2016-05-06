install:
	cargo install --root /usr/local
	rm /usr/local/.crates.toml > /dev/null

man:
	groff -man -Tascii ./wsta.1 | less
