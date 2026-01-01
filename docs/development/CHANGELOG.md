# Changelog

All notable changes to the HEC-22 Urban Drainage Analysis System.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## 2026-01-01

### Documentation
- Fixed broken documentation links in docs/README.md (theory/ paths corrected to reference/)
- Updated main README.md project structure to reflect current directory layout
- Clarified location of HEC-22 chapter files in documentation
- Updated project structure diagram to accurately show docs/reference/ organization

### Cleanup
- Documentation cleanup and consolidation pass
- Verified all documentation cross-references are accurate

## 2025-12-04

### Added
- Comprehensive Chapter 9 verification tests with full output
- Comprehensive Chapter 9 implementation review documentation

### Documentation
- Organized development notes under `docs/development/`

## 2025-12-03

### Fixed
- **CRITICAL**: Correct composite gutter depressed section slope calculation (HEC-22 Chapter 5)
- **CRITICAL**: Complete all HEC-22 Chapter 5 gutter flow fixes - all tests passing
- **CRITICAL**: Correct critical errors in HEC-22 Chapter 5 gutter flow calculations
  - Fixed Equation 5.2 exponents (1.67 and 2.67 instead of 5/3 and 8/3)
  - Fixed Equation 5.4 spread calculation
  - Fixed Equation 5.7 flow ratio calculation
  - Implemented robust bisection method for composite gutter iteration

### Testing
- Adjusted composite gutter test parameters for bypass flow validation

### Merged Pull Requests
- #33: Fix gutter flow calculations

## 2025-12-02

### Added
- **FEATURE**: Implement HEC-22 Equation 7.11 for curb-opening inlet depression
  - Added effective cross slope calculation
  - Added depression depth and gutter width parameters
  - Modified interception calculations to use effective slopes

### Fixed
- **CRITICAL**: Correct curb-opening inlet formula to use HEC-22 Equation 7.10
  - Fixed formula to use proper slopes instead of velocity
  - Corrected equation reference from 7-11 to 7.10

### Documentation
- Added comprehensive inlet hydraulics review report (`INLET_HYDRAULICS_REVIEW.md`)
- Updated review report with fix status and CompositeGutter notes

### Merged Pull Requests
- #32: Review inlet hydraulics implementation

## 2025-12-01

### Fixed
- Correct sag inlet behavior - no bypass flow allowed per HEC-22 standards

### Added
- Comprehensive inlet bypass test with series configuration
- Grate sizing function for sag inlets with opening ratio support

### Refactored
- Implement HEC-22 Example 7.5 procedure for sag grate sizing

### Testing
- Comprehensive tests for tributary flow isolation

### Merged Pull Requests
- #31: Fix sag inlet flow behavior
- #30: Test hydraulic flows

## 2025-11-30

### Fixed
- Resolve compilation errors in visualization module tests
- Correct HEC-22 Table 7.5 grate opening ratios to match actual values
- Add HEC-22 Table 7.5 grate types with correct opening ratios
- Correct outfall EGL to include velocity head from discharge conduit

### Refactored
- Fully consolidate all CSV files to `templates/` directory
- Consolidate CSV files to `templates/` directory

### Documentation
- Reorganize documentation structure for better clarity

### Merged Pull Requests
- #29: Consolidate CSV files
- #28: Consolidate CSV files (alternate branch)
- #27: Improve visualizations

## 2025-11-29

### Changed
- Updated SVG visualizations

## 2025-11-28

### Added
- **FEATURE**: Upgrade solver to use FHWA Access Hole Method for junction losses
- **FEATURE**: Implement all 33 HEC-22 Chapter 9 hydraulic equations with comprehensive documentation
  - Junction losses (Equations 9.9-9.14)
  - Friction losses (Equations 9.1-9.8)
  - Energy grade line calculations
- Correct manhole loss calculation per HEC-22 Sections 9.6.6-9.6.7
- Visualize junction losses as discrete vertical drops in profile view
- Implement junction loss calculation in HGL solver
- HGL/EGL visualization to profile views
- **FEATURE**: Implement Phase 6 visualization capabilities
  - Network plan view with SVG export
  - Profile view with elevation profiles
  - Interactive HTML viewer

### Fixed
- Show vertical steps in crown line when pipe diameter changes

### Documentation
- Document all 33 equations from HEC-22 Chapter 9 (`reference/chapter_9_equations.md`)

### Improved
- Enhance profile view with improved styling and clarity
- Improve profile view with realistic junction structures

### Merged Pull Requests
- #26: Implement hydraulic equations
- #25: Document Chapter 9 equations
- #23: Add network visualization
- #22: Add network visualization (continued)

## 2025-11-27

### Added
- **FEATURE**: HEC-22 Equation 9.9 for junction energy loss

### Merged Pull Requests
- #21: Verify junction energy loss

## 2025-11-26

### Added
- **FEATURE**: Add IDF curves support to CLI and update documentation
- **FEATURE**: Add ATLAS14 rainfall data utility (`atlas14_fetch`)
  - Fetch real NOAA ATLAS14 precipitation frequency data
  - Generate IDF curves in CSV format
  - Support for custom return periods and durations
- Automatic gutter profile selection for inlet calculations

### Fixed
- Correct inlet hydraulics equations and expand CSV templates

### Documentation
- Update README to reflect current project state
- Add ATLAS14 utility documentation

### Merged Pull Requests
- #20: Review inlet hydraulics
- #19: Update README status
- #18: ATLAS14 rainfall utility

## 2025-11-25

### Added
- **FEATURE**: Add CLI tool for hydraulic analysis
  - CSV input parsing (nodes, conduits, drainage areas)
  - Multiple output formats (text, JSON, CSV)
  - Automatic peak flow computation
  - HGL/EGL analysis
- **FEATURE**: Add comprehensive CSV template support
  - Multiple pipe shapes (circular, rectangular, elliptical, arch)
  - IDF curves integration
  - Automatic peak flow calculation
- Unbalanced branching network integration test
- Comprehensive network integration test with flow routing
- **FEATURE**: Implement topological sort for flow routing

### Documentation
- Add comprehensive development roadmap with CLI MVP as top priority

### Merged Pull Requests
- #17: Add peak flow and pipe shapes
- #16: CLI hydraulic analysis
- #15: Add unbalanced network test
- #14: Fix topological sort routing
- #13: Add network integration test

## 2025-11-24

### Added
- **FEATURE**: CSV parser for tabular data input (Phase 2.1)
  - Node table parser (inlets, junctions, outfalls)
  - Conduit table parser (pipes, gutters)
  - Drainage area parser
- Chapter 5 verification tests with comprehensive gutter flow validation
- **FEATURE**: Inlet capacity calculations with bypass flow tracking (Chapter 7)
  - Grate inlets on grade and in sag
  - Curb-opening inlets on grade and in sag
  - Combination inlets
- **FEATURE**: Comprehensive gutter spread calculations (Chapter 5)
  - Uniform (triangular) gutters
  - Composite gutters with depressed sections
- **FEATURE**: Implement hydraulic calculations and HGL/EGL solver
  - Manning's equation for pipe flow
  - Energy equation for hydraulic grade line
  - Friction losses in pipes
- **FEATURE**: Add comprehensive Rust type definitions for drainage network model
  - Nodes (Inlet, Junction, Outfall)
  - Conduits (pipes, channels)
  - Drainage areas
  - Analysis results structures

### Documentation
- Consolidate and clean up documentation
- Add comprehensive JSON schema for drainage network modeling

### Merged Pull Requests
- #12: Review project roadmap
- #11: Documentation consolidation
- #10: Drainage schema design
- #9: Drainage schema design (continued)

## 2025-11-23

### Added
- Comprehensive HGL calculation and junction loss documentation to Chapter 9
- Extract worked examples for test case development
- **FEATURE**: Add equation documentation for Chapters 10-12
  - Chapter 10: Detention and Retention (Modified Puls routing)
  - Chapter 11: Urban Stormwater Quality (BMP design)
  - Chapter 12: Pump Stations (pump selection, system curves)
- Comprehensive test case reference documentation
- **FEATURE**: HEC22 chapter extraction script (`extract_chapters.py`)
  - Automatically extract individual chapters from complete PDF
  - Extract appendices separately

### Documentation
- Document Chapter 12 (Pump Stations) with comprehensive coverage
- Document Chapter 11 (Urban Stormwater Quality) with comprehensive coverage
- Document Chapter 10 (Detention and Retention) with comprehensive coverage
- Document Chapter 9 (Storm Drain Conduits) with comprehensive coverage
- Document Chapter 8 (Storm Drain Structures) with comprehensive coverage
- Document Chapter 7 (Inlet Design) with comprehensive coverage
- Document Chapter 6 (Roadside and Median Channels) and add open channel flow equations
- Update README to document test case reference and chapter extraction

### Merged Pull Requests
- #7: Review README design process
- #6: Update docs Chapter 10
- #5: Update docs Chapter 10 (continued)
- #4: Update docs Chapter 10 (initial)
- #3: HEC22 chapter extractor
- #2: HEC22 chapter extractor (continued)

## 2025-11-22

### Added
- Initial project setup
- **FEATURE**: Add HEC-22 reference materials for urban drainage analysis
  - Complete HEC-22 4th Edition PDF
  - Reference equations for Manning's equation, gutter flow, inlet design
  - Design constants (Manning's n values)
  - Design workflow guidance

### Merged Pull Requests
- #1: Drainage analysis setup

---

## Change Categories

### Features
Major new functionality additions:
- CLI tool with CSV input/output
- Hydraulic calculations (Manning, gutter flow, inlet design)
- HGL/EGL solver with junction losses
- ATLAS14 rainfall data utility
- Visualization system (network plan, profile views)
- Topological flow routing
- IDF curve support

### Fixes
Critical bug fixes and corrections:
- HEC-22 equation corrections (Chapters 5, 7, 9)
- Composite gutter calculations
- Inlet hydraulics formulas
- Sag inlet bypass behavior
- Grate opening ratios

### Documentation
Documentation improvements:
- Comprehensive equation documentation (all HEC-22 chapters)
- Development roadmap
- Test case reference
- Chapter extraction automation
- Organized docs structure

### Testing
Test coverage additions:
- Chapter 5 verification (gutter flow)
- Chapter 7 verification (inlet capacity)
- Chapter 9 verification (junction losses)
- Network integration tests
- Tributary flow tests
- Inlet bypass tests

### Refactoring
Code organization improvements:
- CSV file consolidation to `templates/`
- Documentation reorganization
- Solver upgrade to FHWA Access Hole Method

---

## Version History Summary

### Phase 1: Initial Setup (Nov 22-23, 2025)
- Project initialization
- HEC-22 reference material collection
- Documentation framework
- Chapter extraction automation

### Phase 2: Core Implementation (Nov 24-25, 2025)
- Rust type system and JSON schema
- Hydraulic calculations (Manning, gutter, inlet)
- HGL/EGL solver
- CSV parser
- CLI tool MVP

### Phase 3: Refinement & Fixes (Nov 26 - Dec 3, 2025)
- ATLAS14 rainfall utility
- Visualization system (Phase 6)
- Junction loss implementation (Chapter 9)
- Critical equation fixes (Chapters 5, 7)
- Comprehensive testing suite
- Documentation consolidation

---

**Current Status**: Phase 6 (Visualization) in progress
**Next Steps**: Continue Phase 6 development, consider Phase 3 (Design Automation)

**Based on**: FHWA HEC-22, 4th Edition (February 2024)
