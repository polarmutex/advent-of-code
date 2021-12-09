{
  description = "advent of code";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, ... }@inputs: inputs.flake-utils.lib.eachSystem [
    "x86_64-linux"
  ]
    (system:
      let pkgs = import nixpkgs {
        inherit system;
      };
      in
      {
        devShell = pkgs.mkShell rec {
          name = "aoc-prj";
          packages = with pkgs; [
            python3
            poetry
            python39Packages.numpy
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib/
            source $(poetry env info --path)/bin/activate
          '';
        };
      });
}
