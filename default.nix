{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  cargoLock.outputHashes = {
    # "did-jwk-0.1.1" = pkgs.lib.fakeSha256;
    "did-jwk-0.1.1" = "sha256-byxaWQDR35ioADSjWqGX/h8ht4FjXNh+mdtfD0LW8Sk=";
  };
  src = pkgs.lib.cleanSource ./.;
  nativeBuildInputs = with pkgs; [
    rustc
    rust-analyzer
    cargo
    clippy
    rustfmt
  ];
  meta = with pkgs.lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = with licenses; [ unfree ];
    maintainers = with maintainers; [ jceb ];
  };
}
