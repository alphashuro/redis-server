{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    redis
  ];

  shellHook = ''
    echo nix shell!
  '';

  # MY_ENVIRONMENT_VARIABLE = "";
}
