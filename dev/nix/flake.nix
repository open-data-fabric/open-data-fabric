{
  description = "Dev environment with supporting tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/26.05";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.mermaid-cli
          ];
        };

        packages.mermaid-cli = pkgs.mermaid-cli;
      });
}
