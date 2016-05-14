install:
	cargo install --root /usr/local
	rm /usr/local/.crates.toml > /dev/null

man:
	groff -man -Tascii ./wsta.1 | less

wsta.md:
	groff -man -Tascii ./wsta.1 | col -bx | sed 's/^[A-Z]/## &/g' | sed '/wsta(1)/d' > wsta.md
