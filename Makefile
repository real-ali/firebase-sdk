CRATES_DIR = crates
CRATES = firebase-core firebase-authentication firebase-firestore firebase-messaging firebase-realtime-db firebase-store

# Default version fallback (optional)
version ?= 0.1.0

.PHONY: Publish update_version publish_all

Publish: update_version publish_all

update_version:
	@echo "🔧 Updating version to $(version) in all Cargo.toml files..."
	@for crate in $(CRATES); do \
		echo "Updating $$crate/Cargo.toml..."; \
		sed -i.bak -E 's/^version = ".*"/version = "$(version)"/' $(CRATES_DIR)/$$crate/Cargo.toml; \
		rm -f $(CRATES_DIR)/$$crate/Cargo.toml.bak; \
	done
	@echo "Updating root Cargo.toml version..."
	sed -i.bak -E 's/^version = ".*"/version = "$(version)"/' Cargo.toml
	rm -f Cargo.toml.bak
	@echo "✅ Version updated to $(version)"

publish_all:
	@echo "🚀 Publishing all crates with version $(version)..."
	@for crate in $(CRATES); do \
		echo "Publishing $$crate..."; \
		cargo publish --manifest-path=$(CRATES_DIR)/$$crate/Cargo.toml || exit 1; \
	done
	@echo "🎉 All crates published successfully!"
