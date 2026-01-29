# AADE Invoice Validator

Σύστημα επικύρωσης τιμολογίων myDATA της ΑΑΔΕ (Ανεξάρτητη Αρχή Δημοσίων Εσόδων).

## Χαρακτηριστικά

- ✅ **Επικύρωση XML** τιμολογίων σύμφωνα με τις προδιαγραφές myDATA
- ✅ **19+ Production Rules** - Comprehensive validation για όλους τους τύπους παραστατικών
- ✅ **Invoice Type Support** - 17 διαφορετικοί τύποι (1.1, 1.2, 2.1, 5.1, 11.1, κ.ά.)
- ✅ **ΑΦΜ Validation** - Έλεγχος εγκυρότητας με αλγόριθμο MOD 11
- ✅ **VAT Rules** - Έλεγχος συντελεστών ανά τύπο παραστατικού
- ✅ **Classification Rules** - Έλεγχος χαρακτηρισμών εσόδων (E3_561, E3_881)
- ✅ **Counterpart Validation** - B2B vs B2C, Intra-EU checks
- ✅ **Currency & Exchange Rate** - Αυτόματος έλεγχος ισοτιμίας
- ✅ **Negative Amounts** - Credit note validation
- ✅ **Batch Validation** - Πολλαπλά αρχεία ταυτόχρονα
- ✅ **Database Logging** - Καταγραφή όλων των επικυρώσεων σε PostgreSQL
- ✅ **Production-Ready** - CORS, Error Handling, Performance Optimized

## Απαιτήσεις

- Rust 1.70+
- PostgreSQL 14+

## Εγκατάσταση Development

1. **Clone το repository**
```bash
git clone <repository-url>
cd aade
```

2. **Αντιγραφή .env.example**
```bash
cp .env.example .env
```

3. **Ρύθμιση Database**
```bash
# Δημιουργία database
createdb aade_db

# Ενημέρωση DATABASE_URL στο .env
DATABASE_URL=postgres://username:password@localhost:5432/aade_db
```

4. **Build και εκτέλεση**
```bash
cargo build --release
cargo run --release
```

## Production Deployment

### 1. Environment Variables

Δημιουργήστε ένα `.env` αρχείο με τις παρακάτω ρυθμίσεις:

```bash
# ΥΠΟΧΡΕΩΤΙΚΟ
DATABASE_URL=postgres://username:password@host:5432/aade_db

# Production Mode
ENVIRONMENT=production

# CORS - ΠΟΛΥ ΣΗΜΑΝΤΙΚΟ για production
# Ορίστε τα domains που επιτρέπονται (χωρισμένα με κόμμα)
CORS_ALLOWED_ORIGINS=https://your-frontend-domain.com,https://www.your-domain.com

# Server Address
SERVER_ADDR=0.0.0.0:3000

# Logging
RUST_LOG=info
```

### 2. Database Setup

```bash
# Εκτέλεση migrations
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### 3. Build για Production

```bash
cargo build --release
```

Το εκτελέσιμο θα βρίσκεται στο: `target/release/aade-validator`

### 4. Systemd Service (Linux)

Δημιουργήστε το αρχείο `/etc/systemd/system/aade-validator.service`:

```ini
[Unit]
Description=AADE Invoice Validator
After=network.target postgresql.service

[Service]
Type=simple
User=aade
WorkingDirectory=/opt/aade-validator
Environment="RUST_LOG=info"
EnvironmentFile=/opt/aade-validator/.env
ExecStart=/opt/aade-validator/aade-validator
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Ενεργοποίηση:
```bash
sudo systemctl daemon-reload
sudo systemctl enable aade-validator
sudo systemctl start aade-validator
sudo systemctl status aade-validator
```

### 5. Nginx Reverse Proxy (Προτεινόμενο)

```nginx
server {
    listen 80;
    server_name validator.your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Για μεγάλα XML αρχεία
        client_max_body_size 10M;
    }
}
```

## Validation Rules

Το σύστημα εφαρμόζει **19+ production-ready validation rules** που καλύπτουν:

- **Counterpart Validation** (2 rules): Έλεγχος υποχρεωτικότητας & χώρας λήπτη
- **VAT Validation** (7 rules): Συντελεστές ανά τύπο παραστατικού
- **Classification Rules** (2 rules): Χαρακτηρισμοί εσόδων B2B
- **Negative Amounts** (2 rules): Credit Notes & Debit Notes
- **Currency Rules** (1 rule): Έλεγχος ισοτιμίας
- **Data Quality** (3 rules): Warnings για ασυνήθιστες τιμές
- **Hardcoded Checks** (6 rules): Totals, AFM, Dates, VAT calculations

📖 **Πλήρης τεκμηρίωση**: [RULES_REFERENCE.md](./RULES_REFERENCE.md)

### Υποστηριζόμενοι Τύποι Παραστατικών

- 1.1 - Τιμολόγιο Πώλησης
- 1.2 - Ενδοκοινοτικές Παραδόσεις
- 1.3-1.6 - Παραλλαγές Πώλησης
- 2.1-2.4 - Τιμολόγια Παροχής Υπηρεσιών
- 5.1-5.2 - Πιστωτικά/Χρεωστικά
- 11.1-11.5 - Αποδείξεις Λιανικής (ΑΛΠ, ΑΠΥ)

## API Endpoints

### Health Checks

- `GET /health/ready` - Ετοιμότητα (έλεγχος database)
- `GET /health/live` - Λειτουργία service

### Validation

- `POST /validate` - Επικύρωση ενός XML τιμολογίου
  - Content-Type: `text/xml` ή `application/xml`
  - Body: XML περιεχόμενο

- `POST /validate/batch` - Επικύρωση πολλαπλών XML
  - Content-Type: `multipart/form-data`
  - Πολλαπλά αρχεία .xml

### Response Format

```json
{
  "status": "Green" | "Yellow" | "Red",
  "risk_score": 0-100,
  "summary": "Μήνυμα",
  "errors": [
    {
      "code": "BR-001",
      "field": "totalNetValue",
      "value_found": "100.00",
      "reason": "Calculated Net Amount (110.00) mismatch",
      "severity": "Error"
    }
  ],
  "suggestions": []
}
```

## Ασφάλεια

### Production Checklist

- [ ] Ρύθμιση `ENVIRONMENT=production`
- [ ] Ορισμός συγκεκριμένων CORS origins
- [ ] Χρήση HTTPS (μέσω reverse proxy)
- [ ] Ασφαλής αποθήκευση credentials (PostgreSQL)
- [ ] Firewall rules για database
- [ ] Regular backups της βάσης δεδομένων
- [ ] Monitoring και logging

## Testing

```bash
# Unit tests
cargo test

# Clippy linting
cargo clippy

# Format check
cargo fmt --check
```

## Logs

Το σύστημα χρησιμοποιεί `tracing` για structured logging.

Επίπεδα logging:
- `error`: Κρίσιμα σφάλματα
- `warn`: Προειδοποιήσεις
- `info`: Γενικές πληροφορίες (default)
- `debug`: Λεπτομερή μηνύματα
- `trace`: Πολύ λεπτομερή

Ρύθμιση μέσω `RUST_LOG`:
```bash
RUST_LOG=aade_validator=debug,sqlx=warn cargo run
```

## Monitoring

Προτεινόμενα μετρικά για παρακολούθηση:
- Health endpoint response time
- Validation success/failure rate
- Database connection pool status
- Request latency
- Memory usage

## Troubleshooting

### Database Connection Issues

```bash
# Έλεγχος σύνδεσης
psql $DATABASE_URL

# Επαναφορά migrations
sqlx migrate revert
sqlx migrate run
```

### CORS Errors

Βεβαιωθείτε ότι το `CORS_ALLOWED_ORIGINS` περιλαμβάνει το σωστό domain του frontend σας.

### High Memory Usage

Το σύστημα μπορεί να επεξεργαστεί μεγάλα XML αρχεία. Για batch operations, περιορίστε τον αριθμό των ταυτόχρονων αιτημάτων.

## Υποστήριξη

Για ζητήματα και ερωτήσεις, δημιουργήστε issue στο repository.

## License

[Προσθέστε license information]
# aade-validation-engine
