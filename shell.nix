{ pkgs ? import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; } }:
let 
    buildInputs = with pkgs; [
        # >1.79 to build bevy
        rust-bin.stable."1.80.0".default

        # From Bevy docs
        udev alsa-lib vulkan-loader
        xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # X11
        libxkbcommon wayland # Wayland
    ];
in pkgs.mkShell {

    inherit buildInputs;
    nativeBuildInputs = [ pkgs.pkg-config ];

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
