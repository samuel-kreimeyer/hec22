# HEC-22 Documentation

This directory contains comprehensive documentation for the HEC-22 drainage analysis system, organized into three main categories:

## Documentation Structure

### 📖 User Documentation (`user/`)

Documentation for users of the HEC-22 CLI tool and CSV templates:

- **[CLI_USAGE.md](user/CLI_USAGE.md)** - CLI tool usage guide
  - Command-line options and flags
  - Input file formats
  - Output formats (text, JSON, CSV)
  - Troubleshooting

- **[ATLAS14_UTILITY.md](user/ATLAS14_UTILITY.md)** - ATLAS14 rainfall data utility
  - Fetch NOAA precipitation frequency data
  - Generate IDF curves for any location
  - Integration with HEC-22 CLI

- **[COMPLETE_NETWORK_EXAMPLE.md](user/COMPLETE_NETWORK_EXAMPLE.md)** - Complete network example
  - Demonstrates all features (multiple pipe shapes, inlet types, manhole geometries)
  - 7-node network with 6 pipes
  - Peak flow calculations
  - CLI usage examples

### 🔬 Theory and Engineering (`theory/`)

Engineering theory, equations, and hydraulic design notes:

- **[CHAPTER_5_EQUATIONS.md](theory/CHAPTER_5_EQUATIONS.md)** - HEC-22 Chapter 5 equations
  - Gutter flow calculations
  - Uniform and composite gutters
  - Flow ratios and spread calculations

- **[flow_routing.md](theory/flow_routing.md)** - Flow routing methodology
  - Network flow routing algorithms
  - Topological sorting
  - Energy balance concepts

### 💻 Software Development (`development/`)

Technical documentation for contributors and library developers:

- **[CHANGELOG.md](development/CHANGELOG.md)** - Project changelog
  - Version history
  - Feature additions
  - Bug fixes
  - Breaking changes

- **[RUST_TYPES.md](development/RUST_TYPES.md)** - Rust type system documentation
  - Type definitions and data structures
  - Module organization
  - API usage examples
  - JSON serialization

- **[INLET_EQUATION_FIXES.md](development/INLET_EQUATION_FIXES.md)** - Inlet equation corrections
  - Changelog of equation fixes
  - HEC-22 standard alignment
  - Before/after comparisons with line number references

- **[INLET_HYDRAULICS_REVIEW.md](development/INLET_HYDRAULICS_REVIEW.md)** - Comprehensive inlet hydraulics review
  - Equation traceability analysis
  - HEC-22 Example verification
  - Code structure analysis
  - Implementation status

- **[GUTTER_FIX_SUMMARY.md](development/GUTTER_FIX_SUMMARY.md)** - Gutter flow implementation fixes
  - Critical errors fixed
  - Equation corrections
  - Test results summary

- **[GUTTER_IMPLEMENTATION_ERRORS.md](development/GUTTER_IMPLEMENTATION_ERRORS.md)** - Detailed gutter implementation error documentation
  - Specific error descriptions
  - Root cause analysis
  - Fix verification

- **[issues.md](development/issues.md)** - Development issues and notes
  - Known issues
  - Future enhancements
  - Technical debt

## Reference Materials

Additional technical references are available in the `reference/` directory (project root):

- **HEC-22 Chapters** (`../reference/chapters/`) - Individual PDF chapters
- **Equations** (`../reference/equations/`) - Hydraulic and hydrologic equations
  - `manning_equation.md` - Pipe flow capacity
  - `gutter_flow.md` - Surface drainage and gutter flow
  - `inlet_design.md` - Inlet hydraulic design
  - `rational_method.md` - Runoff calculations
  - `open_channel_flow.md` - Open channel flow
- **Constants** (`../reference/constants/`) - Manning's n values and design constants
- **Guidance** (`../reference/guidance/`) - Design workflows and implementation guides
  - `component_definitions.md` - Data model specifications
  - `design_workflow.md` - Step-by-step design process
  - `IMPLEMENTATION_GUIDE.md` - Advanced topics (Chapters 10-12)
  - `CLI_USAGE_GUIDE.md` - CLI usage guide
- **Test Cases** (`../reference/TEST_CASE_REFERENCE.md`) - Validation test cases

## Templates and Examples

CSV templates and example files:

- **Templates** (`../templates/`) - CSV input templates
  - Node templates (inlets, junctions, outfalls)
  - Conduit templates (various pipe shapes)
  - Drainage area templates
  - IDF curve templates
  - Gutter parameter templates

- **Examples** (`../examples/`) - Rust code examples
  - Network construction
  - Hydraulic calculations
  - Visualization generation

## Quick Links

**For Users:**
- [Main README](../README.md) - Project overview and getting started
- [CLI Usage](user/CLI_USAGE.md) - Command-line tool documentation
- [CSV Templates](../templates/README.md) - Input file format documentation
- [ATLAS14 Utility](user/ATLAS14_UTILITY.md) - Rainfall data utility

**For Engineers:**
- [Chapter 5 Equations](theory/CHAPTER_5_EQUATIONS.md) - Gutter flow equations
- [Flow Routing](theory/flow_routing.md) - Network routing methodology
- [Reference Equations](../reference/equations/) - All hydraulic equations

**For Developers:**
- [Changelog](development/CHANGELOG.md) - Version history and changes
- [Rust Types](development/RUST_TYPES.md) - Type system and API documentation
- [Inlet Review](development/INLET_HYDRAULICS_REVIEW.md) - Inlet implementation review
- [Gutter Fixes](development/GUTTER_FIX_SUMMARY.md) - Gutter implementation fixes
- [Test Cases](../reference/TEST_CASE_REFERENCE.md) - Validation and verification

## Contributing

Documentation improvements are welcome! When contributing:

1. **User docs** - Keep language accessible, focus on practical usage
2. **Theory docs** - Include equations, references to HEC-22 sections, worked examples
3. **Developer docs** - Include code examples and references to source files with line numbers
4. **Formatting** - Use GitHub-flavored Markdown
5. **Links** - Use relative paths for cross-references

For questions or suggestions, please open an issue on GitHub.

## Document Organization Guidelines

### User Documentation
- Focus on "how to use" rather than "how it works"
- Include practical examples
- Keep technical jargon minimal
- Provide troubleshooting tips

### Theory Documentation
- Reference HEC-22 sections and equations
- Include worked examples
- Explain the engineering theory
- Show calculations step-by-step

### Development Documentation
- Include code snippets with file paths and line numbers
- Document known issues and fixes
- Maintain changelog for all significant changes
- Track technical debt and future enhancements

---

**Last Updated:** December 2025
**Project Version:** Phase 6 (Visualization) in progress
**Based on:** FHWA HEC-22, 4th Edition (February 2024)
