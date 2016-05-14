install:
	cargo install --root /usr/local
	rm /usr/local/.crates.toml > /dev/null

man:
	groff -man -Tascii ./wsta.1 | less

wsta.md: wsta.1
	groff -man -Tascii ./wsta.1 | col -bx | sed -re 's/[0-9]+m//g' -e  's/^[A-Z]/## &/g' -e '/wsta(1)/d' > wsta.md
