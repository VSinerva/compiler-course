{
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "compiler-course";
  version = "0.1.0";

  # You would not use this in an actual nix package!
  src = builtins.fetchGit {
    url = "https://gitea.vsinerva.fi/VSinerva/compiler-course.git";
    name = "compiler-course-src";
    ref = "main";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
