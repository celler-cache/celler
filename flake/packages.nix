{ lib
, inputs
, makeCranePkgs
, ...
}:

{
  config = {
    _module.args.makeCranePkgs = lib.mkDefault (pkgs:
      let
        craneLib = inputs.crane.mkLib pkgs;
      in
      pkgs.callPackage ../crane.nix {
        inherit craneLib;
      });

    perSystem =
      { self'
      , pkgs
      , cranePkgs
      , ...
      }: (lib.mkMerge [
        {
          _module.args = {
            cranePkgs = makeCranePkgs pkgs;
          };

          packages = {
            default = self'.packages.celler;

            inherit (cranePkgs)
              celler
              celler-tests
              ;

            book = pkgs.callPackage ../book {
              celler = self'.packages.celler;
            };
          };
        }
      ]);
  };
}
