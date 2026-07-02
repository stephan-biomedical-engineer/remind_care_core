#!/bin/bash
# Script adaptado para gerar o certificado SSL inicial no Servidor (VPS).
# IMPORTANTE: Só rode isso DENTRO da VPS após ter apontado o DNS do domínio.

if ! docker compose version >/dev/null 2>&1; then
  echo 'Error: docker compose is not installed.' >&2
  exit 1
fi

domains=(remindcare.com.br)
rsa_key_size=4096
data_path="./certbot"
email="telemedufu263@gmail.com" # Adicionando seu e-mail
staging=0 # Mude para 1 se quiser apenas testar limites de requisição

if [ -d "$data_path" ]; then
  read -p "Dados existentes encontrados. Quer apagar e substituir certificados antigos? (y/N) " decision
  if [ "$decision" != "Y" ] && [ "$decision" != "y" ]; then
    exit
  fi
fi

echo "### Baixando configurações de segurança TLS recomendadas ..."
mkdir -p "$data_path/conf"
curl -s https://raw.githubusercontent.com/certbot/certbot/master/certbot-nginx/certbot_nginx/_internal/tls_configs/options-ssl-nginx.conf > "$data_path/conf/options-ssl-nginx.conf"
curl -s https://raw.githubusercontent.com/certbot/certbot/master/certbot/certbot/ssl-dhparams.pem > "$data_path/conf/ssl-dhparams.pem"
echo

echo "### Criando certificado temporário (dummy) para permitir que o Nginx inicie ..."
path="/etc/letsencrypt/live/$domains"
mkdir -p "$data_path/conf/live/$domains"
docker compose -f docker-compose.prod.yml run --rm --entrypoint "\
  openssl req -x509 -nodes -newkey rsa:$rsa_key_size -days 1\
    -keyout '$path/privkey.pem' \
    -out '$path/fullchain.pem' \
    -subj '/CN=localhost'" certbot
echo

echo "### Iniciando Nginx ..."
docker compose -f docker-compose.prod.yml up --force-recreate -d nginx
echo

echo "### Apagando certificado temporário ..."
docker compose -f docker-compose.prod.yml run --rm --entrypoint "\
  rm -Rf /etc/letsencrypt/live/$domains && \
  rm -Rf /etc/letsencrypt/archive/$domains && \
  rm -Rf /etc/letsencrypt/renewal/$domains.conf" certbot
echo

echo "### Solicitando certificado real Let's Encrypt ..."
# Seleciona o parâmetro para testar sem explodir limites
if [ $staging != "0" ]; then staging_arg="--staging"; fi

docker compose -f docker-compose.prod.yml run --rm --entrypoint "\
  certbot certonly --webroot -w /var/www/certbot \
    $staging_arg \
    --email $email \
    -d $domains \
    --rsa-key-size $rsa_key_size \
    --agree-tos \
    --force-renewal" certbot
echo

echo "### Recarregando Nginx ..."
docker compose -f docker-compose.prod.yml exec nginx nginx -s reload
