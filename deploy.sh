#!/bin/bash
# Script simplificado de CI/CD para rodar no Servidor VPS
# Ele atualiza o código, constrói as imagens e sobe sem causar downtime

echo "🚀 Iniciando deploy de produção..."

# 1. Puxar o código mais recente da branch principal
echo "📥 Baixando atualizações do GitHub..."
git pull origin main

# 2. Build da imagem Rust otimizada
echo "🔨 Construindo container da API..."
docker compose -f docker-compose.prod.yml build api

# 3. Levantar a infraestrutura sem desligar os antigos antes do build
echo "🔄 Trocando containers (Zero Downtime)..."
docker compose -f docker-compose.prod.yml up -d

echo "✅ Deploy concluído com sucesso!"
