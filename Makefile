install:
	@cargo install --path .

install-git:
	@cargo install --git https://github.com/MarcelArt/tempa --branch master

uninstall:
	@cargo uninstall