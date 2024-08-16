{
  description = "A Rust Bevy game for GMTK Game Jam 2024";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        name = "name";
        version = "0.0.1";
        deps = with pkgs; [
          # Bevy Nix dependencies
          # https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md#nix
          pkg-config
          udev
          alsa-lib
          vulkan-loader
          # Wayland
          libxkbcommon
          wayland
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml)
            rust-analyzer

            trunk
          ] ++ deps;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps;
        };
      }
    );
}
