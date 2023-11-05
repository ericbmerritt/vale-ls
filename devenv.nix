{
  pkgs,
  lib,
  ...
}: {
  # https://devenv.sh/packages/
  packages = with pkgs;
    [git alejandra statix ripgrep]
    ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
      frameworks.Security
      frameworks.CoreFoundation
      frameworks.SystemConfiguration
    ]);

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;
}
