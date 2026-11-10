{
  description = "signal-herdr-nexus — generated ordinary herdr-nexus Signal contract";

  inputs.base.url = "github:LiGoldragon/build-base";

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix shell github:LiGoldragon/build-base#toolchain github:LiGoldragon/build-base#crate2nix -c crate2nix generate
  outputs =
    { self, base }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # Every test runs with every feature on.
      memberChecks = { check, ... }: { signal-herdr-nexus = check "signal-herdr-nexus" [ "default" "datom" ]; };

      extraChecks =
        { pkgs, toolchain, check, doc, ... }:
        let
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
        in
        {
          # The tests under the default feature set alone.
          test-default = check "signal-herdr-nexus" [ "default" ];
          fmt = pkgs.runCommand "signal-herdr-nexus-fmt" { nativeBuildInputs = [ toolchain ]; } ''
            rustfmt --check --edition ${manifest.package.edition} ${rust}/src/lib.rs ${rust}/tests/*.rs
            touch $out
          '';
          doc = doc "signal-herdr-nexus" [ "default" "datom" ] [ "-D" "warnings" ];
        };
    };
}
