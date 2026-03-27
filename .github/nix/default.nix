{ lib
, stdenv
, fetchurl
, dpkg
, autoPatchelfHook
, wrapGAppsHook3
, webkitgtk_4_1
, gtk3
, libayatana-appindicator
, openssl
, glib-networking
}:

stdenv.mkDerivation rec {
  pname = "clippy-clipboard";
  version = "{{VERSION}}";

  src = fetchurl {
    url = "https://github.com/0-don/clippy/releases/download/v${version}/clippy_${version}_amd64.deb";
    sha256 = "{{SHA256}}";
  };

  nativeBuildInputs = [
    dpkg
    autoPatchelfHook
    wrapGAppsHook3
  ];

  buildInputs = [
    webkitgtk_4_1
    gtk3
    libayatana-appindicator
    openssl
    glib-networking
  ];

  unpackPhase = ''
    dpkg-deb -x $src .
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp -r usr/* $out/
  '';

  meta = with lib; {
    description = "Clipboard Manager built with Rust and Typescript";
    homepage = "https://github.com/0-don/clippy";
    license = licenses.mit;
    maintainers = [];
    platforms = [ "x86_64-linux" ];
    mainProgram = "clippy";
  };
}
