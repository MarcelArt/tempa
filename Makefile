install:
	@cargo install --path .

install-git:
	@cargo install --git https://github.com/MarcelArt/create-godot-rust-game --branch master

uninstall:
	@cargo uninstall