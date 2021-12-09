{ pkgs ? (import <nixpkgs> { }).pkgs }:
with pkgs;
mkShell {
  buildInputs = [
    python3
    poetry
    protobuf
    python39Packages.numpy
  ];
  shellHook = ''
    # fixes libstdc++ issues and libgl.so issues
    export LD_LIBRARY_PATH=${stdenv.cc.cc.lib}/lib/
    poetry shell
  '';
}
