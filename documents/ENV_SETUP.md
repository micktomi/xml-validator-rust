# Environment Setup Guide

Αυτό το project χρησιμοποιεί διαφορετικά `.env` αρχεία για κάθε environment:

## 📁 Αρχεία Environment

### 1. `.env` (Git ignored)
**Χρήση:** Τρέχον active environment - default για local development
```bash
cp .env.example .env
# Ή
cp .env.local .env
```

### 2. `.env.example` (Commited)
**Χρήση:** Template με όλες τις διαθέσιμες μεταβλητές
- Χρησιμοποίησε αυτό ως αφετηρία
- Περιέχει documentation για κάθε μεταβλητή

### 3. `.env.local` (Git ignored)
**Χρήση:** Local development χωρίς Docker
```bash
# Ρύθμιση
cp .env.local .env
cargo run

# Χαρακτηριστικά
DATABASE_URL=postgres://user:password@localhost:5432/aade_db
SERVER_ADDR=127.0.0.1:3000
ENVIRONMENT=development
```

### 4. `.env.docker` (Commited)
**Χρήση:** Docker development με docker-compose
```bash
# Το docker-compose.yml ορίζει τις μεταβλητές αυτόματα
docker-compose up

# Αν θέλεις explicit env file:
docker-compose --env-file .env.docker up

# Χαρακτηριστικά
DATABASE_URL=postgres://user:password@db:5432/aade_db  # 'db' είναι το service name
SERVER_ADDR=0.0.0.0:3000  # Bind σε όλα τα interfaces
```

### 5. `.env.production` (Git ignored)
**Χρήση:** Production deployment (Render, Heroku, κλπ.)
```bash
# ⚠️ ΜΗΝ κάνεις commit production credentials!
# Όρισε τις μεταβλητές στο Dashboard του service provider

# Στο Render:
# 1. Πήγαινε στο Dashboard > Environment
# 2. Πρόσθεσε κάθε μεταβλητή ξεχωριστά

# Χαρακτηριστικά
ENVIRONMENT=production
DATABASE_URL=<actual-production-url>
CORS_ALLOWED_ORIGINS=https://yourdomain.com
```

## 🚀 Quick Setup

### Local Development
```bash
# Χωρίς Docker
cp .env.local .env
cargo run

# Με Docker
docker-compose up
```

### Production (Render)
```bash
# Στο Render Dashboard, όρισε:
DATABASE_URL=<from-render-postgres>
ENVIRONMENT=production
CORS_ALLOWED_ORIGINS=https://your-app.onrender.com
PORT=3000
RUST_LOG=info
```

## 🔒 Security

**Git Ignored (ΠΟΤΕ μην κάνεις commit):**
- `.env`
- `.env.local`
- `.env.production`

**Commited (Safe for templates):**
- `.env.example`
- `.env.docker`

## 📊 Διαφορές

| Μεταβλητή | Local | Docker | Production |
|-----------|-------|--------|------------|
| `DATABASE_URL` | `@localhost:5432` | `@db:5432` | `@render-host` |
| `SERVER_ADDR` | `127.0.0.1:3000` | `0.0.0.0:3000` | `0.0.0.0:3000` |
| `ENVIRONMENT` | `development` | `development` | `production` |
| `CORS` | Permissive | Permissive | Strict |

## ⚙️ Environment Variables Reference

```bash
# Database (REQUIRED)
DATABASE_URL=postgres://user:password@host:5432/dbname

# Server
SERVER_ADDR=0.0.0.0:3000  # Docker/Production
SERVER_ADDR=127.0.0.1:3000  # Local
PORT=3000  # Optional, overrides port in SERVER_ADDR

# Environment
ENVIRONMENT=development  # or 'production'

# CORS
CORS_ALLOWED_ORIGINS=https://domain1.com,https://domain2.com

# Logging
RUST_LOG=info  # or debug, trace, warn, error
```
