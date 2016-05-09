class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.2.0.tar.gz"
  sha256 "205a90215f5413a520f7d6ba7507c66e4273796751a6d8053ce9f2216127f1d9"

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

