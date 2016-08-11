class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.4.0.tar.gz"
  sha256 "dbb5c5900d595254c73b485493a96618ee5d12402e17d1aa3a57dcdb5cac3b5d"

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

