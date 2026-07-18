# RemindCare - Backend Ecosystem 🦀📦

Bem-vindo ao repositório unificado (Monorepo) do ecossistema de servidores da **RemindCare**. Este repositório orquestra todos os serviços em nuvem responsáveis pela lógica de negócios, geração de relatórios, persistência de dados e distribuição da nossa Smart Pillbox IoT e do aplicativo móvel.

## 🏗️ Arquitetura do Sistema

Nosso backend foi projetado sob a ótica de microsserviços, containerizados e provisionados automaticamente via Docker Compose, garantindo alta disponibilidade (99.8% Uptime) e baixíssima latência (< 50ms).

O monorepo é dividido nos seguintes módulos principais:

### 1. Core API (`/core_api`)
O cérebro da operação. Desenvolvida em **Rust** usando o framework **Axum**, é a API principal de alta performance.
- **Responsabilidades:** Autenticação JWT, Gestão de Usuários, Controle da Grade de Medicamentos, Pareamento de Caixas IoT.
- **Hardware & IoT:** Processa os pulsos de vida (*heartbeats*) da caixa, determina status Online/Offline e controla a ingestão retroativa (Resiliência Offline).
- **Push Notifications:** Integração com Firebase Cloud Messaging (FCM) para disparar alertas de atraso de medicamentos aos familiares.
- **Banco de Dados:** PostgreSQL com SQLx (migrações e cache de queries `.sqlx`).

### 2. Report API (`/report_api`)
Microsserviço de geração documental. Desenvolvido em **Node.js** com **Express** e **Puppeteer**.
- **Responsabilidades:** Ouve as chamadas do *Core API*, renderiza o histórico de adesão do paciente e converte em formato PDF utilizando *headless browser*. Usado para compartilhar resultados com os médicos responsáveis.

### 3. Landing Page Vue (`/landing_page_vue`)
O código-fonte do nosso site de vendas e distribuição do aplicativo móvel.
- Desenvolvido em **Vue.js (Vite)** com design voltado para conversão, estéticas *Glassmorphism* e Dark Mode premium.

### 4. Servidor Estático NGINX (`/landing_page`)
Pasta de distribuição que hospeda o *build* final do Vue.js e o binário gerado do aplicativo móvel (`remindcare-release.apk`), servidos nativamente pelo container do Nginx para altíssima velocidade de download.

## 🚀 Como fazer o Deploy

A infraestrutura foi pensada para automação (Zero-Downtime Deployment). Tudo pode ser provisionado na VPS com apenas um comando, que atualizará imagens do docker, compilará o Rust em modo otimizado e reiniciará os *containers*:

```bash
# Baixe a versão mais recente da branch main
git pull origin main

# Execute o script de deploy automatizado
./deploy.sh
```

**O que o `deploy.sh` faz por debaixo dos panos?**
1. Atualiza as imagens base do Docker (Rust, Node, Nginx, Postgres).
2. Usa Multi-stage Builds: Compila o projeto Rust e baixa as bibliotecas NPM.
3. Levanta todos os *containers* via `docker-compose up -d`.
4. Roda o Nginx e gerencia certificados (Certbot) para HTTPS automático.

## 🛠️ Tecnologias Utilizadas

- **Rust (Axum, Tokio, SQLx)**
- **Node.js (Express, Puppeteer)**
- **Vue.js + Vite**
- **Docker & Docker Compose**
- **PostgreSQL 16**
- **NGINX**

## 💡 Comandos Úteis (Desenvolvimento Local)

### Atualizar Cache do SQLx (Rust)
Sempre que criar uma nova rota com queries ao banco, atualize o cache para o Docker conseguir compilar offline no servidor:
```bash
cd core_api
cargo sqlx prepare
```

### Rodar Testes de Integração
Garantimos a qualidade do código antes do deploy. 
```bash
cd core_api
cargo test
```

### Enviar Novo APK do Flutter para a Landing Page
Após dar *build* no Flutter localmente, envie direto para a VPS:
```bash
scp app-release.apk root@<SEU_IP>:/root/remind_care_core/landing_page/remindcare-release.apk
```

---
*Ecossistema projetado por e para a RemindCare.*
