{
  callPackage,
  rust-analyzer,
  rustfmt,
  clippy,
  lldb,
  perl,
  openssl,
  ...
}: let
  mainPkg = callPackage ./default.nix {};
in
  mainPkg.overrideAttrs (oa: {
    nativeBuildInputs =
      [
        # Additional rust tooling
        rust-analyzer
        rustfmt
        clippy
        lldb
        perl
      ]
      ++ (oa.nativeBuildInputs or []);

      propagatedBuildInputs = [ openssl ];

  })
