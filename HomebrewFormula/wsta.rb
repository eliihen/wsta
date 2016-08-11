class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.4.0.tar.gz"
  sha256 "db72da542fa7dc8c1e01e9b7cf5e888d47795d20ba4acd21793fd9177a5a717d"

  depends_on 'gpg' => :build
  depends_on 'rust' => :build
  depends_on 'openssl'

  def install
    system "cargo", "build", "--release"

    bin.mkpath
    bin.install "target/release/wsta"

    man.mkpath
    man1.install "wsta.1"
  end
end

