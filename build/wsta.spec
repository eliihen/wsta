Name:           wsta
Version:        0.5.0
Release:        1%{?dist}
Summary:        The WebSocket Transfer Agent

License:        GPLv3
URL:            https://github.com/esphen/wsta
Source:         https://github.com/esphen/%{name}/archive/%{version}.tar.gz

BuildRequires:  openssl-devel
BuildRequires:  gcc

%description
wsta is a cli tool written in rust for interfacing with WebSocket services.
wsta has the simple philosophy of getting out of your way and letting you work
your UNIX magic on the WebSocket traffic directly. The way it does this is to be
as pipe-friendly as possible, letting you chain it into complex pipelines or
bash scripts as you see fit.

%prep
%setup
# Extract rust
mkdir $RPM_SOURCE_DIR/rust
tar -xzf rust-1.*-$RPM_ARCH-unknown-linux-gnu.tar.gz -C $RPM_SOURCE_DIR/rust --strip-components 1
# Install rust
sh $RPM_SOURCE_DIR/rust/install.sh --without="rust-docs" --prefix="$RPM_SOURCE_DIR" --disable-ldconfig


%build
# Add new binaries to PATH
export PATH="$PATH:$RPM_SOURCE_DIR/bin"
CARGO_HOME=$RPM_BUILD_DIR/$RPM_PACKAGE_NAME-$RPM_PACKAGE_VERSION/.cargo cargo build --release


%install
# Add new binaries to PATH
export PATH="$PATH:$RPM_SOURCE_DIR/bin"
CARGO_HOME=$RPM_BUILD_DIR/$RPM_PACKAGE_NAME-$RPM_PACKAGE_VERSION/.cargo cargo install --root $RPM_BUILD_ROOT/usr --bin wsta
rm -vf $RPM_BUILD_ROOT/usr/.crates.toml
# Add man page
mkdir -pv $RPM_BUILD_ROOT/usr/local/share/man/man1
cp -v wsta.1 $RPM_BUILD_ROOT/usr/local/share/man/man1/wsta.1


%files
%doc README.md
%doc %attr(0444,root,root) /usr/local/share/man/man1/wsta.1
/usr/bin/wsta


%changelog
* Sun Dec 18 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Add support for setting custom ping message
* Fri Aug 12 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Fix argument conflict - profile argument -p is now -P
* Thu Aug 11 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Add support for config files and profiles
- Use all headers from SetCookie when using --login
- Add windows support
- Update dependencies
* Fri Jun 03 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Add support for binary data
- Only frames are printed to stdout, rest is now stderr
* Tue May 17 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Set Origin header based on WS URL
- Make exit codes more consistent
- Update dependencies
* Sun May 08 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Change syntax to be wsta [OPTIONS] URL [MESSAGES ...]
- Is now quiet by default
- Add --ping
- Add man page
* Thu May 05 2016 Espen Henriksen <dev+wsta@henriksen.is>
- Initial RPM release of wsta
