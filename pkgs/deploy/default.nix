{ substituteAll, runtimeShell, coreutils, nix, lib, rsync }:
substituteAll {
  name = "deploy";
  src = ./deploy.sh;
  dir = "bin";
  isExecutable = true;
  inherit runtimeShell;
  path = lib.makeBinPath [ coreutils nix rsync ];

  meta = {
    description =
      "Rebuild your NixOS configuration and switch to it, on local hosts and remote.";
    mainProgram = "deploy";
  };
}
