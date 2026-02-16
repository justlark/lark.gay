#!/bin/sh
set -eu

ACME_HOME="/home/ejabberd/.acme.sh"
CERT_DIR="/home/ejabberd/certs/floof.chat"
DOMAIN="floof.chat"
ALT_DOMAIN="xmpp.floof.chat"

# Ensure volume directories exist
mkdir -p /home/ejabberd/database
mkdir -p /home/ejabberd/logs
mkdir -p /home/ejabberd/upload
mkdir -p /home/ejabberd/certs
mkdir -p "$ACME_HOME"

# Issue certificate if not already present
if [ ! -f "$CERT_DIR/fullchain.cer" ]; then
    echo "Issuing certificate for $DOMAIN and $ALT_DOMAIN..."
    "$ACME_HOME"/acme.sh --issue \
        --dns dns_cf \
        --domain "$DOMAIN" \
        --domain "$ALT_DOMAIN" \
        --keylength ec-256 \
        --cert-home /home/ejabberd/certs \
        --config-home "$ACME_HOME"
fi

# Start background renewal loop
(
    while true; do
        sleep 86400
        echo "Checking for certificate renewal..."
        if "$ACME_HOME"/acme.sh --renew \
            --domain "$DOMAIN" \
            --keylength ec-256 \
            --cert-home /home/ejabberd/certs \
            --config-home "$ACME_HOME" 2>&1; then
            echo "Certificate renewed, reloading ejabberd..."
            /home/ejabberd/bin/ejabberdctl reload_config || true
        fi
    done
) &

exec /home/ejabberd/bin/ejabberdctl foreground
