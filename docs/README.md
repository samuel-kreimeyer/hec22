# HEC-22 Documentation

This directory contains comprehensive documentation for the HEC-22 drainage analysis system.

## User Documentation

Documentation for users of the HEC-22 CLI tool and CSV templates:

- **[ATLAS14_UTILITY.md](ATLAS14_UTILITY.md)** - ATLAS14 rainfall data utility
  - Fetch NOAA precipitation frequency data
  - Generate IDF curves for any location
  - Integration with HEC-22 CLI

- **[COMPLETE_NETWORK_EXAMPLE.md](COMPLETE_NETWORK_EXAMPLE.md)** - Complete network example
  - Demonstrates all features (multiple pipe shapes, inlet types, manhole geometries)
  - 7-node network with 6 pipes
  - Peak flow calculations
  - CLI usage examples

- **[../CLI_USAGE.md](../CLI_USAGE.md)** - CLI tool usage guide (in project root)
  - Command-line options and flags
  - Input file formats
  - Output formats (text, JSON, CSV)
  - Troubleshooting

- **[../templates/README.md](../templates/README.md)** - CSV template documentation
  - Template descriptions and column definitions
  - Quick start guide
  - Design guidelines and best practices

## Developer Documentation

Technical documentation for contributors and library developers:

- **[development/INLET_EQUATION_FIXES.md](development/INLET_EQUATION_FIXES.md)** - Inlet equation corrections
  - Changelog of equation fixes
  - HEC-22 standard alignment
  - Before/after comparisons with line number references

- **[development/RUST_TYPES.md](development/RUST_TYPES.md)** - Rust type system documentation
  - Type definitions and data structures
  - Module organization
  - API usage examples
  - JSON serialization

## Reference Materials

Additional technical references are available in the `reference/` directory (project root):

- **HEC-22 Chapters** (`reference/chapters/`) - Individual PDF chapters
- **Equations** (`reference/equations/`) - Hydraulic and hydrologic equations
- **Constants** (`reference/constants/`) - Manning's n values and design constants
- **Guidance** (`reference/guidance/`) - Design workflows and implementation guides
- **Test Cases** (`reference/TEST_CASE_REFERENCE.md`) - Validation test cases

## Quick Links

**For Users:**
- [Main README](../README.md) - Project overview and getting started
- [CLI Usage](../CLI_USAGE.md) - Command-line tool documentation
- [CSV Templates](../templates/README.md) - Input file format documentation

**For Developers:**
- [Rust Types](development/RUST_TYPES.md) - Type system and API documentation
- [Equation Fixes](development/INLET_EQUATION_FIXES.md) - Implementation changelog
- [Test Cases](../reference/TEST_CASE_REFERENCE.md) - Validation and verification

## Contributing

Documentation improvements are welcome! When contributing:

1. **User docs** - Keep language accessible, focus on practical usage
2. **Developer docs** - Include code examples and references to source files
3. **Formatting** - Use GitHub-flavored Markdown
4. **Links** - Use relative paths for cross-references

For questions or suggestions, please open an issue on GitHub.
