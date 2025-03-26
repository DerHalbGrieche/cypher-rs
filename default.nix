{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "cypher-rs";
  version = "0.1.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
