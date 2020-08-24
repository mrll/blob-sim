let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  rust = import ./nix/rust.nix { inherit sources; };

in with pkgs;

mkShell {
  buildInputs = [ rust alsaLib libGL libudev pkgconfig xlibs.libX11 ];

  APPEND_LIBRARY_PATH = stdenv.lib.makeLibraryPath [
    libGL
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
