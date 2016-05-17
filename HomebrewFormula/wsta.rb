class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.2.1.tar.gz"
  sha256 "7923e9bd8310b5a72d8390324bd5150615d0e587ec793f808e12cddbb03e238f"

  depends_on 'gpg' => :build
  depends_on 'multirust' => :build
  depends_on 'openssl'

  def install
    system "multirust", "update", "beta"
    system "multirust", "override", "beta"
    system "cargo", "build", "--release"

    bin.mkpath
    bin.install "target/release/wsta"

    man.mkpath
    man1.install "wsta.1"
  end
end

