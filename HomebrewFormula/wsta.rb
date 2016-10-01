class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.4.1.tar.gz"
  sha256 "0c031dbf490c98dbc5dab07f16945ba353cd3cd18780094aee17f4782ec0ea57"

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

