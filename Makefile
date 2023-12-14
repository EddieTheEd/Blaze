.DEFAULT_GOAL := help

## Linux - NB: windows will be the default and other os will be something like linux-help
lhelp: ## Show all Makefile targets
	@echo -e "NOTE: This has only been tested on Unix/macOS!\nPrerequisites:\n- Make\n- npm\n- cargo(from rustup)\n\nRun 'make setup', then 'make serve'.\n\nThese are all the available functions:\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' # does not work on windows
lsetup: ## Install all dependencies for serving Blaze locally
	@npm install live-server@1.2.2 ## Live server from NPM https://www.npmjs.com/package/live-server
	@curl https://sh.rustup.rs -sSf | sh
lserve: ## Configure base_url to be root directory, then run live-server
	@sed -i 's\base_url\temp_base_url\g' blazeconfig.toml
	@echo "base_url = '/'" > localbaseurl.txt
	@cat localbaseurl.txt >> blazeconfig.toml
	@rm localbaseurl.txt
	@cargo run
	@sed -i 's\temp_base_url\base_url\g' blazeconfig.toml
	@sed -i '$$d' blazeconfig.toml
	@npx pagefind --site output
	@live-server output
lserveff: ## Configure base_url to be root directory, then run live-server for firefox developer edition
	@sed -i 's\base_url\temp_base_url\g' blazeconfig.toml
	@echo "base_url = '/'" > localbaseurl.txt
	@cat localbaseurl.txt >> blazeconfig.toml
	@rm localbaseurl.txt
	@cargo run
	@sed -i 's\temp_base_url\base_url\g' blazeconfig.toml
	@sed -i '$$d' blazeconfig.toml
	@npx pagefind --site output
	@live-server output --browser=/usr/lib/firefox-developer-edition/firefox --no-browser
lreset: ## Clear any additional files, excluding content or user made files, in particular the node_modules, output and target folders
	@rm -rf node_modules
	@rm -rf output
	@rm -rf target
	@rm -rf package-lock.json
	@rm -rf package.json
lrun: lreset lsetup lserve ## Serve Blaze locally
lupdate: ## Update Blaze to latest version. Will ignore blaze.config, content. All thanks to Jzhao's Quartz 3 Makefile! 
	@git remote show upstream || (echo "remote 'upstream' not present, setting 'upstream'" && git remote add upstream https://github.com/EddieTheEd/Blaze.git)
	@git fetch upstream
	@echo -e "\033[1mNOTE: Press 'q' to escape the log, once you've looked over(or can't be bothered to read) the commits.\033[0m"
	@git log --oneline --decorate --graph ..upstream/main
	@git checkout -p upstream/main -- Makefile blaze/ .github .gitignore src/ Cargo.toml blazeconfig.toml ## Remove "Makefile" if you have customised your Makefile!
	git pull
	git add .
	git commit -m "Update Blaze"
	git push
	
## Windows
## Convert the lhelp function to windows commands
help:
	@echo -e "NOTE: This has only been tested on Unix!\nPrerequisites:\n- Make\n- npm\n- cargo(from rustup)\n\nRun 'make setup', then 'make serve'.\n\nThese are all the available functions:\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {gsub(/lhelp/, "help"); printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' # does not work on windows





