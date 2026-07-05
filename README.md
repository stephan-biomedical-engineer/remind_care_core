# RemindCare Backend Ecosystem 💊🦀

Plataforma de telemedicina focada na adesão a tratamentos médicos através de uma **Caixinha de Remédios Inteligente (IoT)** integrada a um aplicativo de saúde.

Este projeto evoluiu de um servidor Rust genérico para um ecossistema de microsserviços especializado em gerenciar pacientes, agendas médicas, sincronização com hardware IoT e geração de relatórios de adesão (PDF e Dashboards).

## ✨ Features do Sistema

- **Sincronização IoT Bidirecional:** Caixinhas inteligentes batem ponto (heartbeat) para baixar agendas de medicamentos e enviam eventos de uso em tempo real.
- **Microservices Architecture:** 
  - **Core API (Rust/Axum):** Alta performance para IoT, gerencia Auth, CRUD de Remédios e Devices.
  - **Report API (Node.js/Puppeteer):** Serviço dedicado para gerar PDFs e dados de dashboards estatísticos.
- **Segurança Robusta:** Autenticação JWT com Refresh Tokens (hashes Argon2) e chaves API por dispositivo.
- **Persistência Confiável:** PostgreSQL estruturado com UUIDv4, garantindo isolamento entre dados dos pacientes.
- **Infraestrutura em Containers:** Docker Compose orquestrando Bancos de Dados, APIs, e o NGINX atuando como Reverse Proxy.
- **Testes Automatizados:** Ampla suíte de testes de integração via `cargo test`.

---

## 🏗️ Arquitetura do Repositório

O projeto é dividido em três blocos principais:

```text
remindcare_backend/
├── core_api/             # 🦀 Backend Principal em Rust (Axum, Tokio, SQLx)
├── report_api/           # 🟩 Serviço Node.js para PDFs (Express, Puppeteer)
├── nginx/                # 🌐 Reverse Proxy para unificar rotas (/api/v1/...)
├── docker-compose.yml    # Orquestração local/dev
└── docker-compose.prod.yml # Orquestração de produção com Nginx TLS
```

### O Fluxo da Aplicação (Jornada)
1. **Frontend / Paciente:** Acessa o aplicativo, cadastra medicamentos e emite relatórios. Tudo bate no `Nginx`, que redireciona rotas genéricas para a `Core API` e rotas `/reports` para a `Report API`.
2. **Caixinha IoT:** Dispositivo físico provisionado com o sistema. Faz `heartbeats` constantes para a `Core API` baixando horários. Ao abrir a gaveta no horário certo, dispara um log (`onTime`, `late` ou `warning`).
3. **Dashboards e Relatórios:** A `Report API` lê os logs salvos no Postgres e consolida os dados, enviando JSON para o Dashboard ou gerando um PDF com a avaliação do tratamento para o paciente levar ao médico.

---

## 🛠️ Tecnologias Utilizadas

**Core API (Rust):**
- **Framework:** Axum & Tokio
- **Banco de Dados:** PostgreSQL & SQLx
- **Autenticação & Segurança:** JWT, Argon2, validator

**Report API (Node.js):**
- **Framework:** Express
- **Geração de PDF:** EJS & Puppeteer

**Infraestrutura:**
- NGINX (Reverse Proxy, SSL/TLS)
- Docker & Docker Compose
- Let's Encrypt (Certbot)

---

## 🚀 Como executar o ecossistema (Docker Compose)

O sistema inteiro sobe com um único comando, unindo banco, microsserviços e proxy.

### 1. Configure as Variáveis de Ambiente
Você precisará ter os arquivos `.env` configurados baseados no `.env.example`.

Para ambiente de desenvolvimento:
```bash
cp .env.example .env.dev
```

### 2. Suba os containers
```bash
docker compose --env-file .env.dev up --build -d
```
Isso irá levantar o PostgreSQL (rodando as migrations em Rust automaticamente), o `core_api`, o `report_api` e o `nginx`.

### 3. Teste o funcionamento
```bash
# Healthcheck do Core (Rust)
curl http://localhost/api/v1/health

# Healthcheck dos Relatórios (Node)
curl http://localhost/api/v1/reports/health
```

---

## 📊 Testes Automatizados (Core API)

A API principal em Rust possui forte cobertura de testes de integração isolados em um banco de teste, validando desde Auth até Sincronização de Dispositivos.

Para executar os testes:
```bash
cd core_api
cargo test
```
*(Certifique-se de preencher o arquivo `core_api/.env.test` com as configurações do banco de testes)*

---

## 🧭 Próximos Passos (Roadmap)
- [x] Migração de IDs numéricos para UUIDv4 (Escalabilidade global)
- [x] Separação de rotas do Node.js (Dashboard Stats vs PDF)
- [x] Validação de 100% dos testes
- [ ] Implementação do Worker de Notificações Assíncronas (E-mail/Push para horário de remédios)
- [ ] Desenvolvimento do Dashboard Frontend (React) consumindo as rotas `/stats`
- [ ] Otimizações no peso das imagens Docker (Alpine/Distroless)

---

> Desenvolvido com 🦀 & ☕ para salvar vidas através da adesão médica.
