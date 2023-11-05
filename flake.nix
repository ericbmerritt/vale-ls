{
  description = "A flake for building vale-ls.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
          vale-ls = pkgs.callPackage ./. {inherit nixpkgs system rust-overlay;};
        in rec {
          packages = {
            inherit vale-ls;
            default = packages.vale-ls;
          };
        }
      );
}
