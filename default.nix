{
  pkgs,
  lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "vale-ls";
  version = "v0.3.5";
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [pkgs.pkg-config];

  buildInputs =
    [pkgs.openssl.dev]
    ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
      frameworks.Security
      frameworks.CoreFoundation
      frameworks.SystemConfiguration
    ]);

  # Needed to get openssl-sys to use pkg-config.
  # Doesn't seem to like OpenSSL 3
  OPENSSL_NO_VENDOR = 1;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
