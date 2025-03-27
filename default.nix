{rustPlatform, pkgs}:
rustPlatform.buildRustPackage {
  pname = "cypher-rs";
  version = "0.1.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkgs.gcc
    pkgs.cmake
    pkgs.pkg-config
    pkgs.perl
  ];

}
