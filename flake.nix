{
  description = "Jaspers nix config";

  inputs = {
    # Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

    # Home manager
    home-manager.url = "github:nix-community/home-manager/release-24.11";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";

    # Add flake-utils
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, home-manager, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages = {
          deploy = pkgs.callPackage ./pkgs/deploy { };
          web-app = pkgs.callPackage ./app { };
        };
      }) // {
        # NixOS configuration entrypoint
        # Available through 'nixos-rebuild --flake .#scan-server'
        nixosConfigurations = {
          scan-server = nixpkgs.lib.nixosSystem {
            specialArgs = { inherit inputs self; };
            modules = [ ./nixos/configuration.nix ];
          };
        };

        # Standalone home-manager configuration entrypoint
        # Available through 'home-manager --flake .#your-username@your-hostname'
        # homeConfigurations = {
        #   # FIXME replace with your username@hostname
        #   "your-username@your-hostname" = home-manager.lib.homeManagerConfiguration {
        #     pkgs = nixpkgs.legacyPackages.x86_64-linux; # Home-manager requires 'pkgs' instance
        #     extraSpecialArgs = {inherit inputs self;};
        #     # > Our main home-manager configuration file <
        #     modules = [./home-manager/home.nix];
        #   };
        # };
      };
}
