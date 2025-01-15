let
  nixpkgs = <nixpkgs>;
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };
in
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
  ];
}
