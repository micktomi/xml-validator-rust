# myDATA Validation Rules Reference

Αυτό το έγγραφο περιγράφει όλους τους κανόνες επικύρωσης που εφαρμόζονται στα τιμολόγια myDATA.

## Τύποι Κανόνων

Το validation engine υποστηρίζει τους παρακάτω τύπους λογικής:

### 1. LineValueAllowed
Ελέγχει αν μια τιμή σε γραμμή τιμολογίου είναι σε λίστα επιτρεπτών τιμών.

### 2. HeaderDependencyLine
Ελέγχει τιμές γραμμών βάσει του τύπου παραστατικού (header).

### 3. CounterpartRequired
Ελέγχει αν απαιτείται Λήπτης για συγκεκριμένους τύπους παραστατικών.

### 4. ClassificationRequired
Ελέγχει αν απαιτούνται Χαρακτηρισμοί Εσόδων.

### 5. CurrencyExchangeRate
Ελέγχει αν απαιτείται Ισοτιμία για ξένα νομίσματα.

### 6. CounterpartCountry
Ελέγχει τη χώρα του Λήπτη.

### 7. NegativeAmountsOnly
Ελέγχει ότι όλα τα ποσά είναι αρνητικά (για Credit Notes).

### 8. NoNegativeAmounts
Ελέγχει ότι δεν υπάρχουν αρνητικά ποσά.

### 9. ClassificationTypeRequired
Ελέγχει την παρουσία συγκεκριμένων τύπων χαρακτηρισμών.

---

## Ενεργοί Κανόνες (Production)

### SECTION 1: COUNTERPART VALIDATION

#### CP-001: Counterpart Required for B2B
- **Severity**: Error
- **Applies to**: 1.1, 1.2, 2.1, 5.1
- **Description**: Τα B2B τιμολόγια απαιτούν υποχρεωτικά Λήπτη με ΑΦΜ
- **Message**: "Το παραστατικό τύπου {invoice_type} απαιτεί Λήπτη (Counterpart)"

#### CP-002: Intra-EU Non-Greek Counterpart
- **Severity**: Error
- **Applies to**: 1.2
- **Description**: Ενδοκοινοτικές παραδόσεις δεν επιτρέπουν Ελληνικό λήπτη
- **Message**: "Ενδοκοινοτικές παραδόσεις (1.2) δεν μπορούν να έχουν Ελληνικό λήπτη (GR)"

---

### SECTION 2: VAT VALIDATION BY INVOICE TYPE

#### VAT-001: Sales Invoice (1.1) - Standard Rates
- **Severity**: Error
- **Allowed VAT**: 1 (24%), 2 (13%), 3 (6%), 7 (0%), 8 (Exempt)
- **Description**: Τιμολόγια πώλησης επιτρέπουν μόνο standard συντελεστές

#### VAT-002: Intra-EU Sales (1.2) - Zero/Exempt Only
- **Severity**: Error
- **Allowed VAT**: 7 (0%), 8 (Exempt)
- **Description**: Ενδοκοινοτικές παραδόσεις πρέπει να είναι 0% ή Άνευ ΦΠΑ

#### VAT-003: Service Invoice (2.1)
- **Severity**: Error
- **Allowed VAT**: 1, 2, 3, 7, 8
- **Description**: Τιμολόγια παροχής υπηρεσιών - standard rates

#### VAT-004: Retail Receipt (11.1 - ΑΛΠ)
- **Severity**: Error
- **Allowed VAT**: 1, 2, 3, 7, 8
- **Description**: Αποδείξεις λιανικής πώλησης

#### VAT-005: Service Receipt (11.2 - ΑΠΥ)
- **Severity**: Error
- **Allowed VAT**: 1, 2, 3, 7, 8
- **Description**: Αποδείξεις παροχής υπηρεσιών

#### VAT-006: Credit Note (5.1)
- **Severity**: Error
- **Allowed VAT**: 1, 2, 3, 7, 8
- **Description**: Πιστωτικά τιμολόγια

---

### SECTION 3: LEGACY VAT RATES

#### VAT-LEGACY-001: Legacy Rate Warning
- **Severity**: Warning
- **Description**: Προειδοποίηση για χρήση παλαιών συντελεστών (17%, 9%, 4%)
- **Note**: Οι παλαιοί συντελεστές (4, 5, 6) είναι έγκυροι μόνο για συγκεκριμένες περιόδους

---

### SECTION 4: CLASSIFICATION RULES

#### CLS-001: B2B Classification Required
- **Severity**: Error
- **Applies to**: 1.1, 1.2, 2.1
- **Description**: Τα B2B τιμολόγια απαιτούν τουλάχιστον 1 χαρακτηρισμό εσόδου
- **Min Required**: 1

#### CLS-002: Intra-EU E3_881 Classification
- **Severity**: Warning
- **Applies to**: 1.2
- **Description**: Ενδοκοινοτικές παραδόσεις συνήθως απαιτούν E3_881_003
- **Note**: Προειδοποίηση, όχι υποχρεωτικό error

---

### SECTION 5: NEGATIVE AMOUNTS

#### NEG-001: Credit Notes Must Be Negative
- **Severity**: Error
- **Applies to**: 5.1
- **Description**: Πιστωτικά τιμολόγια πρέπει να έχουν αρνητικά ποσά

#### NEG-002: Normal Invoices - No Negatives
- **Severity**: Error
- **Applies to**: 1.1, 1.2, 2.1, 11.1, 11.2
- **Description**: Κανονικά παραστατικά δεν επιτρέπουν αρνητικά ποσά

---

### SECTION 6: CURRENCY VALIDATION

#### CUR-001: Exchange Rate Required
- **Severity**: Error
- **Description**: Παραστατικά σε ξένο νόμισμα (≠EUR) απαιτούν Ισοτιμία
- **Default Currency**: EUR

---

### SECTION 7: SPECIAL BUSINESS RULES

#### BIZ-001: Retail Receipts (ΑΛΠ) - Info
- **Severity**: Info
- **Description**: Πληροφοριακό - ΑΛΠ συνήθως δεν απαιτούν Λήπτη

#### BIZ-002: Service Receipts (ΑΠΥ) - Info
- **Severity**: Info
- **Description**: Πληροφοριακό - ΑΠΥ συνήθως δεν απαιτούν Λήπτη

---

### SECTION 8: DATA QUALITY WARNINGS

#### QUALITY-001: Uncommon VAT Rate
- **Severity**: Warning
- **Description**: Προειδοποίηση για ασυνήθιστους συντελεστές ΦΠΑ

#### QUALITY-002: Zero VAT in Regular Sales
- **Severity**: Warning
- **Description**: 0% ή Exempt VAT σε κανονικό τιμολόγιο πώλησης
- **Note**: Ελέγξτε αν ισχύει εξαίρεση (π.χ. Άρθρο 43)

---

### SECTION 9: COMPREHENSIVE CHECKS

#### VAT-MASTER-001: Valid VAT Categories
- **Severity**: Error
- **Description**: Όλες οι γραμμές πρέπει να έχουν έγκυρη κατηγορία ΦΠΑ (1-8)

---

## Hardcoded Business Rules

Εκτός από τα YAML rules, υπάρχουν και hardcoded έλεγχοι:

### BR-001: Net Amount Totals
- Έλεγχος συνολικού καθαρού ποσού

### BR-002: VAT Amount Totals
- Έλεγχος συνολικού ΦΠΑ

### BR-003: Issuer VAT (AFM)
- Έλεγχος εγκυρότητας ΑΦΜ εκδότη (μόνο GR)

### BR-004: Counterpart VAT (AFM)
- Έλεγχος εγκυρότητας ΑΦΜ λήπτη (μόνο GR)

### BR-005: Future Dates
- Η ημερομηνία έκδοσης δεν μπορεί να είναι μελλοντική

### BR-VAT-CALC: Line VAT Calculation
- Έλεγχος υπολογισμού ΦΠΑ ανά γραμμή (tolerance: ±0.05 EUR)

### BR-CLS-TOTAL: Classification Totals
- Έλεγχος ότι το άθροισμα χαρακτηρισμών ταιριάζει με το καθαρό ποσό

---

## Υποστηριζόμενοι Τύποι Παραστατικών

| Κωδικός | Όνομα | B2B | Classifications | Counterpart |
|---------|-------|-----|-----------------|-------------|
| 1.1 | Τιμολόγιο Πώλησης | Ναι | Ναι | Υποχρεωτικό |
| 1.2 | Ενδοκοινοτικές Παραδόσεις | Ναι | Ναι | Υποχρεωτικό (≠GR) |
| 1.3 | Πώληση Τρίτες Χώρες | Ναι | Ναι | Υποχρεωτικό |
| 1.4 | Πώληση Συνδεδεμένη | Ναι | Ναι | Υποχρεωτικό |
| 1.5 | Πώληση Δ.Λ. | Ναι | Ναι | Υποχρεωτικό |
| 1.6 | Πώληση Ξένο ΦΠΑ | Ναι | Ναι | Υποχρεωτικό |
| 2.1 | Παροχή Υπηρεσιών | Ναι | Ναι | Υποχρεωτικό |
| 2.2 | Παροχή Ενδοκοινοτική | Ναι | Ναι | Υποχρεωτικό |
| 2.3 | Παροχή Τρίτες Χώρες | Ναι | Ναι | Υποχρεωτικό |
| 2.4 | Παροχή Συνδεδεμένη | Ναι | Ναι | Υποχρεωτικό |
| 5.1 | Πιστωτικό Τιμολόγιο | Ναι | Ναι | Υποχρεωτικό |
| 5.2 | Χρεωστικό Τιμολόγιο | Ναι | Ναι | Υποχρεωτικό |
| 11.1 | ΑΛΠ | Όχι | Όχι | Προαιρετικό |
| 11.2 | ΑΠΥ | Όχι | Όχι | Προαιρετικό |
| 11.3 | Απλοποιημένο | Όχι | Όχι | Προαιρετικό |
| 11.4 | Πιστωτικό Λιανικής | Όχι | Όχι | Προαιρετικό |
| 11.5 | Πιστωτικό Παροχής | Όχι | Όχι | Προαιρετικό |

---

## Συντελεστές ΦΠΑ

| Κωδικός | Ποσοστό | Περιγραφή | Status |
|---------|---------|-----------|--------|
| 1 | 24% | Κανονικός συντελεστής | Ενεργός |
| 2 | 13% | Μειωμένος | Ενεργός |
| 3 | 6% | Υπερμειωμένος | Ενεργός |
| 4 | 17% | Παλαιός | Legacy |
| 5 | 9% | Παλαιός | Legacy |
| 6 | 4% | Παλαιός | Legacy |
| 7 | 0% | Μηδενικός | Ενεργός |
| 8 | - | Άνευ ΦΠΑ | Ενεργός |

---

## Κύριες Κατηγορίες Χαρακτηρισμών

### Πωλήσεις (E3_561_xxx)
- **E3_561_001**: Χονδρική πώληση εμπορευμάτων
- **E3_561_002**: Λιανική πώληση προϊόντων ιδίας παραγωγής
- **E3_561_003**: Λιανική πώληση εμπορευμάτων αντιπώλησης
- **E3_561_005**: Έσοδα από πωλήσεις για λογ/σμό τρίτων

### Υπηρεσίες (E3_562_xxx)
- **E3_562_001**: Παροχή υπηρεσιών

### Ενδοκοινοτικές (E3_881_xxx)
- **E3_881_003**: Ενδοκοινοτικές παραδόσεις

---

## Χρήση Rules

### Προσθήκη Νέου Rule

Επεξεργαστείτε το `rules/mydata_v1.yaml`:

```yaml
- id: "YOUR-RULE-ID"
  description: "Περιγραφή κανόνα"
  severity: "Error" | "Warning" | "Info"
  logic:
    type: "RuleLogicType"
    # ... παράμετροι
  error_message: "Μήνυμα λάθους {placeholder}"
```

### Testing Rules

```bash
cargo test
```

### Reload Rules

Τα rules φορτώνονται από το `rules/mydata_v1.yaml` σε κάθε validation.
Δεν χρειάζεται restart του service για αλλαγές στα rules.

---

## Βέλτιστες Πρακτικές

1. **Error vs Warning**: Χρησιμοποιήστε Error για blocking issues, Warning για προειδοποιήσεις
2. **Message Clarity**: Γράψτε ξεκάθαρα μηνύματα στα Ελληνικά
3. **Testing**: Προσθέστε test cases για νέα rules
4. **Performance**: Αποφύγετε πολύπλοκα rules που τρέχουν για κάθε γραμμή

---

## Support & Updates

Για αλλαγές στους κανόνες βάσει νέων προδιαγραφών myDATA:
1. Ενημερώστε το `rules/mydata_v1.yaml`
2. Προσθέστε documentation εδώ
3. Προσθέστε unit tests
4. Deploy νέα έκδοση

---

**Τελευταία Ενημέρωση**: 2026-01-28
**Rules Version**: 1.0
**Total Active Rules**: 19
