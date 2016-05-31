class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.2.1.tar.gz"
  sha256 "48c2c1a73cab9955df0c2cb494d536e904dafa19a7c8ac7c6dac64ae2cb6240a"

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

