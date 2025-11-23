#!/usr/bin/env python3
"""
HEC22 Chapter Extractor
Extracts individual chapters from the HEC22 PDF into separate files.
"""

import re
import sys
from pathlib import Path
try:
    from PyPDF2 import PdfReader, PdfWriter
except ImportError:
    print("Installing PyPDF2...")
    import subprocess
    subprocess.check_call([sys.executable, "-m", "pip", "install", "PyPDF2", "-q"])
    from PyPDF2 import PdfReader, PdfWriter


def find_chapter_pages(pdf_path):
    """
    Analyze PDF to find chapter and appendix page ranges.
    Returns a list of tuples: (title, start_page, end_page)
    """
    reader = PdfReader(pdf_path)
    total_pages = len(reader.pages)

    # Track first occurrence of each chapter/appendix
    seen_chapters = {}
    seen_appendices = {}

    # Pattern to match chapter headings - looking for chapter title pages
    # More specific: must be near start of page or have substantial text indicating chapter start
    chapter_pattern = re.compile(r'CHAPTER\s+(\d+)', re.IGNORECASE)
    appendix_pattern = re.compile(r'APPENDIX\s+([A-Z])', re.IGNORECASE)

    print(f"Scanning {total_pages} pages for chapter markers...")
    print("Looking for first occurrence of each chapter/appendix...\n")

    # Skip front matter (TOC, etc.) - actual content starts around page 30
    start_scan_page = 30

    for page_num in range(start_scan_page, total_pages):
        try:
            page = reader.pages[page_num]
            text = page.extract_text()

            # Check for chapter - only record first occurrence
            chapter_matches = chapter_pattern.findall(text)
            if chapter_matches:
                chapter_num = chapter_matches[0]  # Take first match on page
                if chapter_num not in seen_chapters:
                    # Additional validation: chapter title page usually has less dense text
                    # or has the chapter number near the beginning
                    first_100_chars = text[:100]
                    if 'CHAPTER' in first_100_chars.upper() or len(text) < 500:
                        seen_chapters[chapter_num] = page_num
                        print(f"  Chapter {chapter_num:2s} starts at page {page_num + 1}")

            # Check for appendix - only record first occurrence
            appendix_matches = appendix_pattern.findall(text)
            if appendix_matches:
                appendix_letter = appendix_matches[0]
                if appendix_letter not in seen_appendices:
                    first_100_chars = text[:100]
                    if 'APPENDIX' in first_100_chars.upper() or len(text) < 500:
                        seen_appendices[appendix_letter] = page_num
                        print(f"  Appendix {appendix_letter} starts at page {page_num + 1}")

        except Exception as e:
            print(f"  Warning: Could not process page {page_num + 1}: {e}")
            continue

    # Build chapter list from seen items
    chapters = []

    for chapter_num, start_page in sorted(seen_chapters.items(), key=lambda x: int(x[0])):
        chapters.append({
            'type': 'chapter',
            'number': chapter_num,
            'title': f"Chapter {chapter_num}",
            'start_page': start_page
        })

    for appendix_letter, start_page in sorted(seen_appendices.items()):
        chapters.append({
            'type': 'appendix',
            'number': appendix_letter,
            'title': f"Appendix {appendix_letter}",
            'start_page': start_page
        })

    # Sort all by start page
    chapters.sort(key=lambda x: x['start_page'])

    # Calculate end pages (each chapter ends where the next one starts)
    for i in range(len(chapters) - 1):
        chapters[i]['end_page'] = chapters[i + 1]['start_page'] - 1

    # Last chapter/appendix goes to the end
    if chapters:
        chapters[-1]['end_page'] = total_pages - 1

    return chapters


def extract_chapter(pdf_path, output_path, start_page, end_page):
    """
    Extract pages from start_page to end_page (inclusive) to a new PDF.
    """
    reader = PdfReader(pdf_path)
    writer = PdfWriter()

    for page_num in range(start_page, end_page + 1):
        writer.add_page(reader.pages[page_num])

    with open(output_path, 'wb') as output_file:
        writer.write(output_file)


def main():
    pdf_path = Path(__file__).parent / "reference" / "guidance" / "hif24006.pdf"

    if not pdf_path.exists():
        print(f"Error: PDF not found at {pdf_path}")
        return 1

    print(f"Analyzing: {pdf_path}\n")

    # Find chapters
    chapters = find_chapter_pages(pdf_path)

    if not chapters:
        print("No chapters found!")
        return 1

    print(f"\n{'='*70}")
    print("CHAPTER PAGE RANGES")
    print(f"{'='*70}")

    for ch in chapters:
        page_count = ch['end_page'] - ch['start_page'] + 1
        print(f"{ch['title']:20} Pages {ch['start_page']+1:4d}-{ch['end_page']+1:4d}  ({page_count:3d} pages)")

    print(f"{'='*70}\n")

    # Check if we should extract (not a dry run)
    if len(sys.argv) > 1 and sys.argv[1] == '--extract':
        output_dir = Path(__file__).parent / "chapters"
        output_dir.mkdir(exist_ok=True)

        print("Extracting chapters...")
        for ch in chapters:
            if ch['type'] == 'chapter':
                output_name = f"HEC22 Chapter {ch['number']}.pdf"
            else:
                output_name = f"HEC22 Appendix {ch['number']}.pdf"

            output_path = output_dir / output_name
            extract_chapter(pdf_path, output_path, ch['start_page'], ch['end_page'])
            print(f"  ✓ {output_name}")

        print(f"\nAll chapters extracted to: {output_dir}")
    else:
        print("DRY RUN - No files created.")
        print("Run with --extract flag to create chapter PDFs.\n")

    return 0


if __name__ == '__main__':
    sys.exit(main())
