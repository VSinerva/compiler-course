{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:

rustPlatform.buildRustPackage rec {
  pname = "compiler-course";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "VSinerva";
    repo = pname;
    ref = "main";
    hash = lib.fakeHash; # Use this to get the new hash
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
