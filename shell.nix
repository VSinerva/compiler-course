let
  nixpkgs = <nixpkgs>;
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };
  build-command = pkgs.writeScriptBin "build-nix-package" ''
    nix-build -E 'with import <nixpkgs> {}; callPackage ./default.nix {}'
  '';
in
pkgs.mkShell {
  packages =
    (with pkgs; [
      cargo
      rustc
      rustfmt
      clippy
      docker
    ])
    ++ [ build-command ];
}
