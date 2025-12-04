# HEC-22 Development Documentation

This directory contains technical documentation, implementation notes, and development history for the HEC-22 drainage analysis library.

## Overview

This project implements the FHWA HEC-22 (4th Edition, 2024) Urban Drainage Design Manual methodology in Rust, providing accurate hydraulic calculations for stormwater drainage systems.

## Documentation Index

### Core Architecture

- **[RUST_TYPES.md](./RUST_TYPES.md)** - Rust type system architecture
  - Data model for drainage networks
  - JSON schema mappings
  - Type safety features and validation
  - Usage examples and integration guides

### Implementation Fixes

Located in **[fixes/](./fixes/)** directory:

- **[GUTTER_FIX_SUMMARY.md](./fixes/GUTTER_FIX_SUMMARY.md)** (Dec 2025)
  - Summary of critical gutter flow equation corrections
  - All HEC-22 Chapter 5 equation fixes
  - Test results showing 8/8 tests passing
  - Impact assessment and verification

- **[GUTTER_IMPLEMENTATION_ERRORS.md](./fixes/GUTTER_IMPLEMENTATION_ERRORS.md)**
  - Detailed documentation of errors found in gutter flow calculations
  - Analysis of incorrect exponents in Equations 5.2, 5.4, 5.7
  - Composite gutter algorithm issues
  - Priority-ranked fix requirements

- **[INLET_EQUATION_FIXES.md](./fixes/INLET_EQUATION_FIXES.md)**
  - Curb-opening efficiency exponent correction (0.6 → 1.8)
  - Frontal flow ratio formula fix (HEC-22 Equation 4-14)
  - Gutter profile selection (uniform vs composite)
  - CSV template expansions for inlet parameters

- **[INLET_HYDRAULICS_REVIEW.md](./fixes/INLET_HYDRAULICS_REVIEW.md)** (Dec 2025)
  - Comprehensive review of inlet hydraulic calculations
  - Equation traceability analysis (HEC-22 Chapter 7)
  - Verification against worked examples
  - Critical findings and recommendations

### Reference Materials

Located in **[references/](./references/)** directory:

- **[CHAPTER_5_EQUATIONS.md](./references/CHAPTER_5_EQUATIONS.md)**
  - Complete HEC-22 Chapter 5 equation reference
  - Roadway pavement drainage equations (5.1 - 5.16)
  - Worked examples with step-by-step solutions
  - Implementation notes for iterative methods

## Key Milestones

### December 2025 - Critical Equation Fixes

#### Gutter Flow (Chapter 5)
- ✅ Fixed Equation 5.2 exponents (5/3 → 1.67, 8/3 → 2.67)
- ✅ Corrected Equation 5.7 flow ratio calculation
- ✅ Implemented robust bisection method for composite gutters
- ✅ All test cases passing (Examples 5.1, 5.2)

#### Inlet Hydraulics (Chapter 7)
- ✅ Fixed Equation 7.10 formula (was using velocity instead of slopes)
- ✅ Implemented Equation 7.11 for effective cross slope
- ✅ Corrected efficiency exponent (0.6 → 1.8)
- ✅ Fixed frontal flow ratio calculation
- ✅ Added depression support for curb-opening inlets

### Status Summary

| Component | Status | Test Coverage |
|-----------|--------|---------------|
| Gutter Flow (Ch 5) | ✅ Complete | 8/8 passing |
| Inlet Design (Ch 7) | ✅ Complete | Verified against examples |
| Composite Gutters | ✅ Complete | Bisection method validated |
| Depression Effects | ✅ Complete | Equation 7.11 implemented |

## Development Guidelines

### Adding New Features

1. **Reference HEC-22 equations explicitly**
   - Include equation numbers in docstrings
   - Reference specific sections and pages
   - Document any deviations or assumptions

2. **Create worked examples as tests**
   - Use actual HEC-22 example problems
   - Verify results match published values
   - Document expected vs actual results

3. **Update documentation**
   - Add to appropriate section in `docs/development/`
   - Update this README with links
   - Cross-reference related documents

### Code Review Checklist

- [ ] Equation numbers referenced in comments
- [ ] Correct exponents and coefficients verified
- [ ] Unit conversion constants correct (K_u values)
- [ ] Test cases from HEC-22 examples
- [ ] Documentation updated
- [ ] Roundtrip tests passing

## Testing Philosophy

All implementations are verified against:

1. **HEC-22 Worked Examples** - Direct comparison with published solutions
2. **Equation Verification Tests** - Individual equation accuracy
3. **Roundtrip Tests** - Consistency checks (e.g., spread → flow → spread)
4. **Regression Tests** - Prevent future breakage of fixed issues

## Known Issues and Future Work

### Completed
- ✅ Gutter flow equations corrected
- ✅ Inlet depression effects implemented
- ✅ Composite gutter algorithm fixed

### Future Enhancements
- [ ] V-shaped gutter examples (5.3, 5.4)
- [ ] Circular channel calculations (Example 5.5)
- [ ] Gutter flow time calculations (Example 5.6)
- [ ] Combination inlet sag conditions

## Related Documentation

### Project Documentation
- [Main README](../../README.md) - Project overview and quick start
- [CLI Usage Guide](../../CLI_USAGE.md) - Command-line tool documentation

### Reference Documentation
- [Equation References](../../reference/equations/) - HEC-22 equation library
- [Design Guidance](../../reference/guidance/) - Implementation guides
- [Test Case Reference](../../reference/TEST_CASE_REFERENCE.md) - Test data sources

### Examples
- [Complete Network Example](../../examples/complete_network/) - CSV templates and usage
- [Visualizations](../../examples/visualizations/) - Plotting and analysis

## Contributing

When adding development notes:

1. Place fix summaries in `fixes/`
2. Place reference materials in `references/`
3. Update this README with links and descriptions
4. Use clear, descriptive filenames with dates if relevant
5. Include equation numbers and HEC-22 references

## Version History

### Current Development
- Active development on branch: `claude/organize-dev-notes-*`
- Latest stable features merged to main

### Recent Changes
- **2025-12-04**: Organized development documentation structure
- **2025-12-03**: Completed all HEC-22 Chapter 5 gutter flow fixes
- **2025-12-02**: Implemented Equation 7.11 for inlet depressions
- **2025-12-01**: Conducted comprehensive inlet hydraulics review

## Resources

- [HEC-22 Manual (FHWA)](https://www.fhwa.dot.gov/engineering/hydraulics/pubs/10009/10009.pdf)
- [Urban Drainage Design Manual](https://www.fhwa.dot.gov/engineering/hydraulics/pubs/)
- [GitHub Repository](https://github.com/samuel-kreimeyer/hec22)
- [Issue Tracker](https://github.com/samuel-kreimeyer/hec22/issues)

---

**Last Updated**: 2025-12-04
**Maintainer**: Development Team
