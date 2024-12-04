# use PowerShell instead of sh:
set shell := ["nu", "-c"]

# Build, serve and watch
serve:
	trunk serve --watch src --open

# Format source code
format:
    leptosfmt src/*.rs
    cargo fmt

# Publish to GitHub pages
#
# Note: Replace relative paths by absolute paths
publish:
	trunk build --release
	try { rm --recursive docs }
	cp --recursive dist docs
	open docs/index.html | str replace --all `href="/` `href="./` | str replace --all `from '/` `from './` | str replace --all `init('/` `init('./` | save docs/index.html --force

clean:
	cargo clean
	trunk clean
