let
  pkgs = import <nixpkgs> { };
  compiler-project = import ./default.nix { rustPlatform = pkgs.rustPlatform; };
  source-code = builtins.fetchGit {
    url = "https://gitea.vsinerva.fi/VSinerva/compiler-course.git";
    name = "compiler-course-src";
    ref = "main";
  };
in
pkgs.dockerTools.buildImage {
  name = "compiler-course-docker";
  copyToRoot = pkgs.buildEnv {
    name = "project-source";
    paths = [ source-code ];
    extraPrefix = "/Project_Source";
  };
  config = {
    ExposedPorts = {
      "3000/tcp" = { };
    };
    Cmd = [ "${compiler-project}/bin/compiler-course" ];
  };
}
