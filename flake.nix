{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    extra-pkgs.url = "github:ShaddyDC/extra-nixpkgs";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        lib,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
          config = {
            allowUnfree = true;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
            gdb
            cmake
            clang-tools
            clang
            (
              rust-bin
              .stable
              .latest
              .default
              .override
              {
                extensions = ["rust-src"];
              }
            )
            rust-analyzer
            boost
            jujutsu
          ];
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          RUST_LOG = "info";
        };
      };
    };
}
