# Flake-parts module for style checking.
{ lib, flake-parts-lib, self, inputs, ... }: {
  imports = [
    inputs.git-hooks.flakeModule
  ];

  perSystem = { config, self', inputs', pkgs, ... }: {
    pre-commit = {
      settings.hooks = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
      };
    };

    # TODO Add a formatter here to re-format every file.
  };
}
