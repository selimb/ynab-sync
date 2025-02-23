.PHONY: orm-migrate-refresh

orm-migrate-refresh:
	sea-orm-cli migrate --verbose refresh