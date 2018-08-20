class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.5.0.tar.gz"
  sha256 "97d277faf0a423910c74e1036df724f16362839196c56d0986de7db15d6ba629"

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

