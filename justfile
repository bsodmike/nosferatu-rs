set dotenv-load

# default recipe to display help information
default:
  @just --list

start_docker:
	docker compose up -d

tailwindcss:
	./tailwindcss -i assets/css/input.css -o public/css/output.css

dev:
	./tailwindcss -i assets/css/input.css -o public/css/output.css
	cargo r

migrate_dev_db:
	sqlx migrate run --database-url postgres://postgres:password@localhost:15434/nosferatu_dev_db