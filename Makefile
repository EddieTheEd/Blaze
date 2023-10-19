.DEFAULT_GOAL := help

help: ## Show all Makefile targets
	echo "NOTE: This has only been tested on Unix/macOS!\nPrerequisites:\n- Make\n- npm\n- cargo(from rustup)\nRun 'make setup', then 'make serve'.\n\nThese are all the available functions:\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
setup: ## Install all dependencies for serving Blaze locally
	npm install live-server@1.2.2 ## Live server from NPM https://www.npmjs.com/package/live-server
serve: ## Serve Blaze locally
	sed -i 's\base_url\temp_base_url\g' blazeconfig.toml
	echo "base_url = '/'" > localbaseurl.txt
	cat localbaseurl.txt >> blazeconfig.toml
	rm localbaseurl.txt
	cargo run
	sed -i 's\temp_base_url\base_url\g' blazeconfig.toml
	sed -i '$$d' blazeconfig.toml
	npx pagefind --site output
	live-server output

