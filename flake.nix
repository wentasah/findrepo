{
  description = "findrepo";

  outputs = inputs@{ self, ... }: {
    # Default overlay, for use in dependent flakes
    overlays.default = final: prev: {
      findrepo = (import ./Cargo.nix { pkgs = final; }).rootCrate.build;
    };
  };
}
