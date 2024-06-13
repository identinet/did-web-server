# Documentation: https://nixos.wiki/wiki/Flakes
# Documentation: https://yuanwang.ca/posts/getting-started-with-flakes.html
{
  description = "NixOS docker image";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, nixpkgs-unstable, flake-utils, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs { system = system; config.allowUnfree = true; };
        unstable = import nixpkgs-unstable { system = system; config.allowUnfree = true; };
        # unstable = nixpkgs-unstable.legacyPackages.${system};
        did_web_server_pkg = unstable.callPackage ./default.nix { };
        manifest = pkgs.lib.importJSON ./manifest.json;
        pkgVersionsEqual = x: y:
          let
            attempt = builtins.tryEval
              (assert builtins.substring 0 (builtins.stringLength x) y == x; y);
          in
          if attempt.success then
            attempt.value
          else
          # Version can be bumped in the prerelease or build version to create a
          # custom local revision, see https://semver.org/
            abort "Version mismatch: ${y} doesn't start with ${x}";
        version = pkgVersionsEqual "${did_web_server_pkg.version}" manifest.version;
      in
      with pkgs; rec {
        # Development environment: nix develop
        devShells.default = mkShell {
          name = manifest.name;
          nativeBuildInputs = [
            just
            skopeo
            deno
            unstable.nushell
            # nodePackages.semver
            unstable.cargo-watch
            did_web_server_pkg.nativeBuildInputs
          ];
        };

        packages.docker = pkgs.dockerTools.streamLayeredImage
          {
            # Documentation: https://ryantm.github.io/nixpkgs/builders/images/dockertools/
            name = "${manifest.registry.name}/${manifest.name}";
            tag = version;
            # created = "now";
            # author = "not yet supported";
            maxLayers = 125;
            contents = with pkgs.dockerTools; [
              usrBinEnv
              binSh
              caCertificates
              # fakeNss
              # busybox
              # nix
              # coreutils
              # gnutar
              # gzip
              # gnugrep
              # which
              # curl
              # less
              # findutils
              did_web_server_pkg
              # entrypoint
            ];
            enableFakechroot = true;
            fakeRootCommands = ''
              set -exuo pipefail
              mkdir -p /run/dws
              # chown 65534:65534 /run/dws
              # mkdir /tmp
              # chmod 1777 /tmp
            '';
            config = {
              # Valid values, see: https://github.com/moby/docker-image-spec
              # and https://oci-playground.github.io/specs-latest/
              ExposedPorts = {
                "8000/tcp" = { };
              };
              Entrypoint = [ "${tini}/bin/tini" "--" ];
              Cmd = [ "${did_web_server_pkg}/bin/did-web-server" ];
              # Env = ["VARNAME=xxx"];
              WorkingDir = "/run/dws";
              # User 'nobody' and group 'nogroup'
              User = "65534";
              Group = "65534";
              Labels = {
                # Well-known annotations: https://github.com/opencontainers/image-spec/blob/main/annotations.md
                "org.opencontainers.image.ref.name" = "${manifest.name}:${manifest.version}";
                "org.opencontainers.image.licenses" = manifest.license;
                "org.opencontainers.image.description" = manifest.description;
                "org.opencontainers.image.documentation" = manifest.registry.url;
                "org.opencontainers.image.version" = manifest.version;
                "org.opencontainers.image.vendor" = manifest.author;
                "org.opencontainers.image.authors" =
                  builtins.elemAt manifest.contributors 0;
                "org.opencontainers.image.url" = manifest.homepage;
                "org.opencontainers.image.source" = manifest.repository.url;
                "org.opencontainers.image.revision" = manifest.version;
                # "org.opencontainers.image.base.name" =
                #   "${manifest.registry.url}/${manifest.name}/${manifest.version}";
              };
            };
          };

        # The default package when a specific package name isn't specified: nix build
        packages.default = packages.docker;
      });
}
