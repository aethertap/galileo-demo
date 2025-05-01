
let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-unstable";
  pkgs = import nixpkgs {config = {}; overlays = [];};
in

pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ ];
    buildInputs = with pkgs; [ 
      neovim
      luarocks
      aider-chat
    ];
}

