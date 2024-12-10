{
  pkgs ? import <nixpkgs> { },
}:
let
  cache = toString ./.nix-files;
  rustToolchain = "stable";
in
with pkgs;
mkShell rec {

  buildInputs = [
    openssl
    pkg-config
    rustup
    gnuplot
  ];

  RUSTUP_TOOLCHAIN = rustToolchain;
  RUSTUP_HOME = "${cache}/.rustup";
  CARGO_HOME = "${cache}/.cargo";

  shellHook = ''
    export LD_LIBRARY_PATH=${lib.makeLibraryPath [ stdenv.cc.cc ]}
  '';
}
