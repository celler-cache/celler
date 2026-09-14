# For distribution from this repository as well as CI, we use Crane to build
# Celler.

{ stdenv
, lib
, craneLib
, installShellFiles
, jq
}:

let
  ignoredPaths = [
    ".ci"
    ".github"
    "book"
    "flake"
    "integration-tests"
    "nixos"
    "target"
  ];

  commonArgs = {
    pname = "celler";
    version = "0.1.0";

    src = lib.cleanSourceWith {
      filter = name: type: !(type == "directory" && builtins.elem (baseNameOf name) ignoredPaths);
      src = lib.cleanSource ./.;
    };

    nativeBuildInputs = [
      # pkg-config
      installShellFiles
    ];

    buildInputs = [
      # Nothing yet.
    ];

    doCheck = false;

    CELLER_DISTRIBUTOR = "celler";
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  celler = craneLib.buildPackage (commonArgs // {

    inherit cargoArtifacts;

    postInstall = lib.optionalString (stdenv.hostPlatform == stdenv.buildPlatform) ''
      if [[ -f $out/bin/celler ]]; then
        installShellCompletion --cmd celler \
          --bash <($out/bin/celler gen-completions bash) \
          --zsh <($out/bin/celler gen-completions zsh) \
          --fish <($out/bin/celler gen-completions fish)
      fi
    '';

    meta = with lib; {
      description = "Multi-tenant Nix binary cache system";
      homepage = "https://github.com/blitz/celler";
      license = licenses.asl20;
      maintainers = with maintainers; [ blitz ];
      platforms = platforms.linux ++ platforms.darwin;
      mainProgram = "celler";
    };
  });

  # Celler interacts with Nix directly and its tests require trusted-user access
  # to nix-daemon to import NARs, which is not possible in the build sandbox.
  # In the CI pipeline, we build the test executable inside the sandbox, then
  # run it outside.
  celler-tests = craneLib.mkCargoDerivation (commonArgs // {
    pname = "celler-tests";

    inherit cargoArtifacts;

    nativeBuildInputs = commonArgs.nativeBuildInputs ++ [ jq ];

    doCheck = true;

    buildPhaseCargoCommand = "";
    checkPhaseCargoCommand = "cargoWithProfile test --no-run --message-format=json >cargo-test.json";
    doInstallCargoArtifacts = false;

    installPhase = ''
      runHook preInstall

      mkdir -p $out/bin
      jq -r 'select(.reason == "compiler-artifact" and .target.test and .executable) | .executable' <cargo-test.json | \
        xargs -I _ cp _ $out/bin

      runHook postInstall
    '';
  });
in
{
  inherit celler celler-tests;
}
