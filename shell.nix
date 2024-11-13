let 
	pkgs = import <nixpkgs> {};
in
	pkgs.mkShell {
		packages = [
			pkgs.cargo
			pkgs.rustc

			pkgs.rust-analyzer
			pkgs.rustfmt

			# If the dependencies need system libs, you usually need pkg-config + the lib
			pkgs.pkg-config
			pkgs.openssl
		];

		env = {
			RUST_BACKTRACE = "full";
		};
	}
