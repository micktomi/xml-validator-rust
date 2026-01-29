#!/usr/bin/env python3
"""
fix_all_xmls.py - Add missing myDATA required fields to all test XMLs
"""

import os
import sys
from pathlib import Path
import xml.etree.ElementTree as ET

def fix_xml_file(filepath):
    """Add missing required fields to invoiceSummary"""
    try:
        tree = ET.parse(filepath)
        root = tree.getroot()
        
        # Find invoiceSummary
        for summary in root.findall('.//invoiceSummary'):
            # Required fields with default values
            required_fields = {
                'totalWithheldAmount': '0.00',
                'totalFeesAmount': '0.00',
                'totalStampDutyAmount': '0.00',
                'totalDeductionsAmount': '0.00'
            }
            
            # Check and add missing fields
            for field_name, default_value in required_fields.items():
                if summary.find(field_name) is None:
                    # Find insertion point (after totalVatAmount, before totalGrossValue)
                    vat_elem = summary.find('totalVatAmount')
                    gross_elem = summary.find('totalGrossValue')
                    
                    if vat_elem is not None:
                        # Insert after totalVatAmount
                        new_elem = ET.Element(field_name)
                        new_elem.text = default_value
                        
                        # Find index and insert
                        idx = list(summary).index(vat_elem) + 1
                        summary.insert(idx, new_elem)
                        print(f"  ✓ Added {field_name} to {filepath.name}")
        
        # Write back with proper XML declaration
        tree.write(filepath, encoding='UTF-8', xml_declaration=True)
        return True
        
    except ET.ParseError as e:
        print(f"  ✗ Parse error in {filepath.name}: {e}")
        return False
    except Exception as e:
        print(f"  ✗ Error processing {filepath.name}: {e}")
        return False

def main():
    # Get directory from argument or use current
    if len(sys.argv) > 1:
        test_dir = Path(sys.argv[1])
    else:
        test_dir = Path('.')
    
    if not test_dir.exists():
        print(f"Error: Directory {test_dir} does not exist")
        sys.exit(1)
    
    print("🔧 Fixing all XML files...")
    print(f"📁 Directory: {test_dir.absolute()}")
    print()
    
    # Find all XML files (exclude .sh files)
    xml_files = [f for f in test_dir.glob('*.xml')]
    
    if not xml_files:
        print("No XML files found!")
        sys.exit(1)
    
    print(f"Found {len(xml_files)} XML files")
    print()
    
    fixed_count = 0
    error_count = 0
    
    for xml_file in sorted(xml_files):
        print(f"Processing: {xml_file.name}")
        if fix_xml_file(xml_file):
            fixed_count += 1
        else:
            error_count += 1
    
    print()
    print("━" * 50)
    print(f"✅ Fixed: {fixed_count}")
    print(f"❌ Errors: {error_count}")
    print()
    
    if error_count == 0:
        print("🎉 All XMLs fixed successfully!")
        print("   Re-run validation now.")
    else:
        print("⚠️  Some files had errors. Check output above.")

if __name__ == '__main__':
    main()
