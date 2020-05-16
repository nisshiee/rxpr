help: ## print this message
	@echo "Command list:"
	@echo ""
	@printf "\033[36m%-30s\033[0m %-50s %s\n" "[Sub command]" "[Description]" "[Example]"
	@grep -E '^[/a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | perl -pe 's%^([/a-zA-Z_-]+):.*?(##)%$$1 $$2%' | awk -F " *?## *?" '{printf "\033[36m%-30s\033[0m %-50s %s\n", $$1, $$2, $$3}'

PROJECT_VERSION := $(shell grep version Cargo.toml | head -n 1 | cut -d '"' -f2)

publish-exec:
	echo git tag "v$(PROJECT_VERSION)"
	echo git push origin "v$(PROJECT_VERSION)"

publish: ## Push tag to GitHub and trigger publish GitHub Action
	@echo current branch: `git branch --show-current`
	@echo publishing version: $(PROJECT_VERSION)
	@read -p "Are you sure? " -n 1 -r; echo;\
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then\
		$(MAKE) publish-exec;\
	fi
