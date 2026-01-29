#!/usr/bin/env python3
"""
fix_afm_numbers.py - Replace invalid test AFMs with valid ones
"""

import os
from pathlib import Path
import xml.etree.ElementTree as ET

# Valid Greek AFMs for testing
VALID_ISSUER_AFM = "090000045"  # ΥΠΟΥΡΓΕΙΟ ΟΙΚΟΝΟΜΙΚΩΝ
VALID_COUNTERPART_AFM = "094014201"  # ΕΦΚΑ

def fix_afm_in_file(filepath):
    """Replace AFM numbers with valid test values"""
    try:
        tree = ET.parse(filepath)
        root = tree.getroot()
        
        fixed = False
        
        # Fix issuer AFM
        for issuer in root.findall('.//issuer'):
            vat_elem = issuer.find('vatNumber')
            country_elem = issuer.find('country')
            
            # Only fix GR AFMs
            if vat_elem is not None and country_elem is not None:
                if country_elem.text == 'GR':
                    old_afm = vat_elem.text
                    if old_afm != VALID_ISSUER_AFM:
                        vat_elem.text = VALID_ISSUER_AFM
                        print(f"  ✓ Fixed issuer AFM: {old_afm} → {VALID_ISSUER_AFM}")
                        fixed = True
        
        # Fix counterpart AFM
        for counterpart in root.findall('.//counterpart'):
            vat_elem = counterpart.find('vatNumber')
            country_elem = counterpart.find('country')
            
            # Only fix GR AFMs
            if vat_elem is not None and country_elem is not None:
                if country_elem.text == 'GR':
                    old_afm = vat_elem.text
                    if old_afm != VALID_COUNTERPART_AFM:
                        vat_elem.text = VALID_COUNTERPART_AFM
                        print(f"  ✓ Fixed counterpart AFM: {old_afm} → {VALID_COUNTERPART_AFM}")
                        fixed = True
        
        # Special handling for E3 in retail invoices
        for header in root.findall('.//invoiceHeader'):
            invoice_type = header.find('invoiceType')
            if invoice_type is not None and invoice_type.text == '3.1':
                # Retail - add E3 classification if missing
                summary = root.find('.//invoiceSummary')
                if summary is not None:
                    if not summary.findall('incomeClassification'):
                        # Add E3 for retail
                        net_value_elem = summary.find('totalNetValue')
                        if net_value_elem is not None:
                            classification = ET.SubElement(summary, 'incomeClassification')
                            
                            cls_type = ET.SubElement(classification, 'classificationType')
                            cls_type.text = 'E3_561_001'
                            
                            cls_cat = ET.SubElement(classification, 'classificationCategory')
                            cls_cat.text = 'category1_3'  # Retail sales
                            
                            amount = ET.SubElement(classification, 'amount')
                            amount.text = net_value_elem.text
                            
                            print(f"  ✓ Added E3 classification for retail invoice")
                            fixed = True
        
        if fixed:
            tree.write(filepath, encoding='UTF-8', xml_declaration=True)
            return True
        else:
            print(f"  → No changes needed")
            return False
        
    except Exception as e:
        print(f"  ✗ Error: {e}")
        return False

def main():
    test_dir = Path('.')
    
    print("🔧 Fixing AFM numbers in all XMLs...")
    print(f"   Using valid test AFMs:")
    print(f"   - Issuer: {VALID_ISSUER_AFM}")
    print(f"   - Counterpart: {VALID_COUNTERPART_AFM}")
    print()
    
    xml_files = [f for f in test_dir.glob('*.xml')]
    
    if not xml_files:
        print("No XML files found!")
        return
    
    fixed_count = 0
    
    for xml_file in sorted(xml_files):
        print(f"Processing: {xml_file.name}")
        if fix_afm_in_file(xml_file):
            fixed_count += 1
    
    print()
    print(f"✅ Fixed {fixed_count} files")
    print()
    print("🔄 Re-upload to validator now!")

if __name__ == '__main__':
    main()
