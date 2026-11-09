# Flake-parts module for style checking.
{ inputs, ... }: {
  imports = [
    inputs.git-hooks.flakeModule
  ];

  perSystem = { ... }: {
    pre-commit = {
      settings.hooks = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
      };
    };

    # TODO Add a formatter here to re-format every file.
  };
}
