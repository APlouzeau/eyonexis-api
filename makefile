.PHONY: dev down reset-db

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Mettre des couleurs pour le terminal (optionnel mais sympa)
BLUE=\033[0;34m
GREEN=\033[0;32m
RED=\033[0;31m
YELLOW=\033[0;33m
NC=\033[0m # No Color

PROJECT := eyonexis
ENV ?= dev
ENV_TARGET := docker-compose.$(ENV).yml
DB_CONTAINER := $(PROJECT)-db-$(ENV)

run:
	@echo "${BLUE}🟢 Démarrage de l'environnement de développement...${NC}"
	@docker compose -f $(ENV_TARGET) up --wait --build -d
	@echo ""
	@echo "${GREEN}========================================================${NC}"
	@echo "${GREEN}🚀 Environnement prêt !${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo "⚙️  API DB Port  : ${YELLOW}${DB_PORT}${NC}"
	@echo "🦀 Rust Server  : http://localhost:3001/api"
	@echo "🩺 Health Check : http://localhost:3001/api/health"
	@echo "${GREEN}========================================================${NC}"
	@echo "Astuce: N'oublie pas de lancer 'cargo run' dans un autre terminal pour l'API"

dev:
	@echo "${BLUE}🟢 Démarrage de l'environnement de développement...${NC}"
	@docker compose -f $(ENV_TARGET) up --wait --build -d
	@echo ""
	@echo "${GREEN}========================================================${NC}"
	@echo "${GREEN}🚀 Environnement prêt !${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo "⚙️  API DB Port  : ${YELLOW}${DB_PORT}${NC}"
	@echo "🦀 Rust Server  : http://localhost:3001/api"
	@echo "🩺 Health Check : http://localhost:3001/api/health"
	@echo "${GREEN}========================================================${NC}"
	@echo "${GREEN}🚀 Démarrage du serveur en mode watch... ${NC}"
	systemfd --no-pid -s http::3001 -- cargo watch -x run
	@echo "${GREEN}========================================================${NC}"
	@echo "${GREEN}🚀 Serveur prêt !${NC}"
	@echo "${GREEN}========================================================${NC}"

down:
	@echo "${RED}🔴 Arrêt de l'environnement Docker...${NC}"
	@docker compose -f $(ENV_TARGET) down

db-reset: down
	@echo "${YELLOW}⚠️  Réinitialisation totale de la base de données pour dev...${NC}"
	@echo "🗑️  Suppression du volume local db_data (un mot de passe sudo peut être demandé)..."
	@docker volume rm -f ${DB_CONTAINER}_db_data
	@echo "🔄 Relance de l'environnement..."
	@$(MAKE) run
	@$(MAKE) db-drop
	@$(MAKE) db-migrate
	@$(MAKE) db-seed

db-drop:
	@echo "${GREEN}========================================================${NC}"
	@echo "${YELLOW}⚠️  Réinitialisation du schéma de la base de données...${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo ""
	docker exec ${DB_CONTAINER} dropdb -U ${DB_USER} ${DB_NAME}
	docker exec ${DB_CONTAINER} createdb -U ${DB_USER} ${DB_NAME}
	@echo ""
	@echo "${GREEN}========================================================${NC}"
	@echo "Base de données supprimée et recréé"
	@echo "${GREEN}========================================================${NC}"
	@echo ""

db-migrate:
	@echo "${GREEN}========================================================${NC}"
	@echo "${YELLOW}Création du schéma de la base de données...${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo ""
	sqlx migrate run
	@echo "${GREEN}========================================================${NC}"
	@echo "${YELLOW}Migration effectuée${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo ""

db-seed: 
	@echo "${GREEN}========================================================${NC}"
	@echo "${YELLOW}Insertion de données dans la base...${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo ""
	docker exec -i ${DB_CONTAINER} psql -U ${DB_USER} ${DB_NAME} < seed.sql
	@echo "${GREEN}========================================================${NC}"
	@echo "${YELLOW}Données insérées avec succès${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo ""