{
  description = "Dependencies";

  # inputs.nixpkgs.url = "github:identinet/nixpkgs/identinet";
  inputs.nixpkgs_unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, nixpkgs_unstable, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        unstable = nixpkgs_unstable.legacyPackages.${system};
        allOsPackages = with pkgs; [
          # Nix packages: https://search.nixos.org/packages
          # Shared dependencies
          bashInteractive
          just # Simple make replacement https://just.systems/
          unstable.nushell # Nu Shell https://www.nushell.sh/
          nodejs_20 # JS interpreter https://nodejs.org/en/
        ];
        linuxOnlyPackages = with pkgs;
          [
            # datree # kubernetes configuration validation and verification https://datree.io/
          ];
      in
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs =
            if pkgs.system == "x86_64-linux" then
              allOsPackages ++ linuxOnlyPackages
            else
              allOsPackages;
          buildInputs = [ ];
        };
      });
}
