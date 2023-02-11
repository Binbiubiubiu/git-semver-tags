test:
	cargo nextest run --all-features

changelog:
	git cliff -o CHANGELOG.md --tag $NEW_VERSION
	git commit -a -m "Update CHANGELOG.md" || true
	
release:
	cargo release tag --execute
	git push origin master
	cargo release push --execute