{ pkgs }: {
  deps = [
    pkgs.rustc
    pkgs.cargo
    pkgs.gcc
    pkgs.pkg-config
    pkgs.openssl
  ];
}
