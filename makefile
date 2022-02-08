.PHONY: git release

git:
	git add .
	git commit
	git push -u origin master

release:
	cargo build --release