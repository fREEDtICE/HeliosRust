{ pkgs }: {
	deps = [
		pkgs.vimHugeX
  pkgs.sudo
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}