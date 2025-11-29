# HEC-22 Urban Drainage Analysis System

## Overview

This project provides a comprehensive framework for analyzing urban drainage systems in highway design contexts, following the **FHWA HEC-22 (4th Edition, 2024)** methodology.

Similar to commercial tools like StormCAD and HydroCAD, this system allows users to input information about drainage system components and perform hydraulic calculations for storm sewer design.

## System Components

The drainage system model includes four primary component types:

1.  **Outfalls** - Discharge points from the drainage system
2.  **Junctions** - Connection points (manholes, junction boxes)
3.  **Inlets** - Collection points for surface runoff (grates, curb openings)
4.  **Conduits** - Pipes and channels that convey flow between components

Data can be input in tabular form (spreadsheets, CSV files, or databases) for efficient analysis.

## Reference Materials

The `reference/` directory contains comprehensive technical documentation based on HEC-22:

### Chapters (`reference/chapters/`)

Individual HEC-22 chapters extracted for easy reference:

- **HEC22 Chapter 2.pdf** - Hydrologic Analysis
- **HEC22 Chapter 3.pdf** - Roadside and Median Channels
- **HEC22 Chapter 4.pdf** - Gutter Flow and Inlet Design
- **HEC22 Chapter 5.pdf** - Storm Drain Systems
- **HEC22 Chapter 6.pdf** - Roadside and Median Channels (Advanced)
  - Open channel flow concepts (energy, Froude number, flow regimes)
  - Manning's equation for channel design
  - Channel design parameters (geometry, slope, freeboard)
  - Stable channel design using shear stress approach
- **HEC22 Chapter 7.pdf** - Inlet Design
  - Inlet types and selection (grate, curb-opening, combination, slotted)
  - Hydraulic efficiency and bicycle/pedestrian/ADA safety
  - Interception capacity on continuous grades
  - Inlet interception in sag locations
- **HEC22 Chapter 8.pdf** - Storm Drain Structures
  - Inlet structures (catch basins)
  - Access holes (manholes)
  - Junction chambers
  - Transitions, flow splitters, and inverted siphons
- **HEC22 Chapter 9.pdf** - Storm Drain Conduits
  - Energy Grade Line (EGL) and Hydraulic Grade Line (HGL) concepts
  - Energy loss calculation methods (friction, exit, bend, junction)
  - Hydraulic grade line evaluation methodology
- **HEC22 Chapter 10.pdf** - Detention and Retention
  - Design objectives (peak flow attenuation, volume control)
  - Stage-storage and stage-discharge relationships
  - Storage routing using modified Puls method
- **HEC22 Chapter 11.pdf** - Urban Stormwater Quality
  - Best Management Practice (BMP) alternatives
  - Pollutant load estimation
  - Green infrastructure and Low Impact Development (LID)
- **HEC22 Chapter 12.pdf** - Pump Stations
  - Pump types and selection (axial, radial, mixed flow)
  - System curve and pump performance curve development
  - Storage and mass curve routing
- **HEC22 Appendix A.pdf** - Design Charts and Nomographs
- **HEC22 Appendix B.pdf** - Design Examples
- **HEC22 Appendix C.pdf** - Equations and Formulas

These chapters are automatically extracted from the complete HEC-22 PDF using `extract_chapters.py`.

### Equations (`reference/equations/`)

Core hydraulic and hydrologic equations for drainage design:

- **`manning_equation.md`** - Manning's equation for pipe flow capacity
- **`gutter_flow.md`** - Surface drainage and gutter flow analysis
- **`inlet_design.md`** - Inlet hydraulic design
- **`rational_method.md`** - Runoff calculations
- **`open_channel_flow.md`** - Open channel flow for roadside/median channels

### Constants (`reference/constants/`)

Design constants and coefficients:

- **`manning_n_values.md`** - Roughness coefficients

### Guidance (`reference/guidance/`)

Design procedures, component definitions, and implementation guides:

-   **`component_definitions.md`** - Detailed component specifications for the data model.
-   **`design_workflow.md`** - A step-by-step guide to the overall design process.
-   **`IMPLEMENTATION_GUIDE.md`** - Detailed computational guidance and code examples for advanced topics, including detention/retention basins (Chapter 10), stormwater quality BMPs (Chapter 11), and pump stations (Chapter 12).

### Test Cases and Examples (`reference/TEST_CASE_REFERENCE.md`)

Comprehensive test case and worked example documentation for automated testing and validation:

-   **`TEST_CASE_REFERENCE.md`** - A consolidated reference of all formula validations and worked examples. It includes detailed test cases with known inputs, expected outputs, and step-by-step calculations for all key formulas and design scenarios.

## Key Design Equations

### Pipe Flow (Manning's Equation)
```
Q = (1.486/n) × A × R^(2/3) × S^(1/2)
```

### Gutter Flow
```
Q = (0.56/n) × Sx^(5/3) × SL^(1/2) × T^(8/3)
```

### Rational Method
```
Q = C × i × A
```

### Inlet Length (100% Interception)
```
Lt = 0.6 × Q^0.42 × SL^0.3 / (n × Sx^0.6)
```

## Typical Design Workflow

1. **Hydrologic Analysis**
   - Delineate drainage areas
   - Determine runoff coefficients (C values)
   - Calculate time of concentration (Tc)
   - Determine rainfall intensity from IDF curves
   - Calculate design flow rates (Q = C × i × A)

2. **Surface Drainage Design**
   - Analyze gutter flow and spread
   - Design inlet locations and sizes
   - Calculate inlet interception efficiency
   - Determine bypass flows

3. **Storm Sewer Design**
   - Layout pipe network
   - Size pipes using Manning's equation
   - Establish pipe inverts and slopes
   - Verify minimum/maximum velocity criteria

4. **Hydraulic Analysis**
   - Calculate hydraulic grade line (HGL)
   - Check for surcharge conditions
   - Verify no flooding at manholes
   - Iterate design as needed

5. **Documentation**
   - Prepare design calculations
   - Create inlet, manhole, and pipe schedules
   - Develop plan and profile drawings
   - Write technical specifications

## Input Data Structure

### Nodes Table (Inlets, Junctions, Outfalls)

| Node ID | Type | Rim Elev (ft) | Invert Elev (ft) | Description |
|:---|:---|:---|:---|:---|
| IN-001 | Inlet | 128.75 | 124.50 | Inlet at Sta 0+25 |
| MH-101 | Junction | 125.30 | 118.50 | Manhole at Sta 1+50 |
| OUT-001 | Outfall | -- | 100.50 | Discharge to creek |

### Conduits Table

| Conduit ID | From Node | To Node | Diameter (in) | Length (ft) | n | Slope | Up Invert | Dn Invert |
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
| C-101 | IN-001 | MH-101 | 18 | 120 | 0.013 | 0.0067 | 124.30 | 118.60 |
| C-102 | MH-101 | OUT-001 | 24 | 250 | 0.013 | 0.0716 | 118.50 | 100.60 |

### Drainage Areas Table

| Subarea ID | Area (acres) | Land Use | C Value | Tc (min) | Rainfall (in/hr) | Q (cfs) |
|:---|:---|:---|:---|:---|:---|:---|
| A-001 | 0.50 | Commercial | 0.85 | 10 | 5.2 | 2.21 |
| A-002 | 0.75 | Residential | 0.50 | 12 | 4.8 | 1.80 |

## Design Standards

### Minimum Pipe Sizes
- Driveway culverts: 12 inches
- Storm laterals: 15 inches (some jurisdictions)
- Storm mains: 18 inches

### Velocity Criteria
- Minimum (self-cleansing): 2.5 - 3.0 ft/s
- Maximum (scour prevention): 10 - 15 ft/s

### Slope Criteria
- Minimum slope: Typically 0.004 (0.4%) for 12"+ pipes
- Based on minimum velocity requirement

### Spread Limits (Gutter Flow)
- Major arterial: 6 ft maximum
- Collector streets: 8 ft maximum
- Local streets: 10 ft maximum

### Cover Requirements
- Minimum: 1.0 - 2.0 ft over top of pipe
- Under roadways: 2.0 ft minimum
- Under railroads: 3.0 ft minimum

## Manning's n Values (Common)

| Material | n Value |
|:---|:---|
| RCP (Reinforced Concrete Pipe) | 0.013 |
| CMP (Corrugated Metal Pipe) | 0.024 |
| PVC/HDPE (smooth) | 0.011 |
| Concrete gutter | 0.016 |

## Runoff Coefficients (Common)

| Surface Type | C Value |
|:---|:---|
| Asphalt/Concrete pavement | 0.85 - 0.95 |
| Roofs | 0.85 - 0.95 |
| Gravel | 0.40 - 0.70 |
| Lawns (clay soil) | 0.15 - 0.30 |
| Business/Commercial | 0.70 - 0.95 |
| Residential | 0.30 - 0.70 |

## References

This project is based on:

**Primary Reference:**
- FHWA HEC-22 (4th Edition, 2024): "Urban Drainage Design Manual"
  - Published by Federal Highway Administration
  - Hydraulic Engineering Circular No. 22

**Related Resources:**
- [HEC-22 4th Edition (FHWA)](https://www.fhwa.dot.gov/engineering/hydraulics/pubs/hif24006.pdf)
- [Urban Drainage Design Manual Archive](https://www.fhwa.dot.gov/engineering/hydraulics/library_arc.cfm?pub_number=22&id=189)
- [HEC-22 Documentation (ROSAP)](https://rosap.ntl.bts.gov/view/dot/74311/dot_74311_DS1.pdf)

**Additional FHWA Hydraulics References:**
- HEC-12: Drainage of Highway Pavements
- HEC-14: Hydraulic Design of Energy Dissipators
- HEC-17: Highways in the River Environment
- HEC-RAS: River Analysis System

## CLI Tool

The `hec22` command-line tool is a production-ready application for hydraulic analysis of storm sewer networks. It accepts CSV files as input and produces comprehensive analysis reports.

### Features

**Input Formats:**
- CSV files for nodes, conduits, drainage areas, and IDF curves
- Ready-to-use examples in `examples/complete_network/` directory
- Support for US Customary and SI Metric units

**Analysis Capabilities:**
- Automatic peak flow computation using rational method (Q = C × i × A)
- IDF curve lookup with linear interpolation by time of concentration
- HGL/EGL solver with energy loss calculations
- Flow routing through complex branching networks
- Design violation detection (flooding, capacity, velocity)

**Pipe Shapes Supported:**
- Circular (standard storm sewers)
- Rectangular (box culverts)
- Elliptical (limited vertical clearance)
- Arch (low profile applications)

**Node Types:**
- Inlets (grate, curb opening, combination, slotted)
- Junctions/Manholes (circular or rectangular geometry)
- Outfalls (free, normal, fixed boundary conditions)

**Output Formats:**
- Text reports with formatted tables
- JSON for programmatic processing
- CSV for spreadsheet import

### Quick Start

```bash
# Build the CLI tool
cargo build --release

# Run analysis with fixed intensity
./target/release/hec22 \
  --nodes examples/complete_network/nodes.csv \
  --conduits examples/complete_network/conduits.csv \
  --drainage-areas examples/complete_network/drainage_areas.csv \
  --intensity 4.0 \
  --output results.txt

# Run analysis with IDF curves (automatic intensity lookup)
./target/release/hec22 \
  --nodes examples/complete_network/nodes.csv \
  --conduits examples/complete_network/conduits.csv \
  --drainage-areas examples/complete_network/drainage_areas.csv \
  --idf-curves examples/complete_network/idf_curves.csv \
  --return-period 10 \
  --output results.txt
```

See [CLI_USAGE.md](CLI_USAGE.md) for comprehensive documentation, examples, and troubleshooting.

---

## Utilities

### ATLAS14 Rainfall Data Utility ✅ COMPLETE

The `atlas14_fetch` utility fetches precipitation frequency data from NOAA ATLAS14 and generates IDF (Intensity-Duration-Frequency) curves in CSV format compatible with the HEC-22 CLI tool.

**Status**: Fully implemented and operational

**Usage:**

```bash
# Build the utility
cargo build --release --bin atlas14_fetch

# Fetch IDF data for a location (e.g., New York City)
./target/release/atlas14_fetch --lat 40.7128 --lon -74.0060 --output nyc_idf.csv

# Use custom return periods and durations
./target/release/atlas14_fetch --lat 34.0522 --lon -118.2437 \
  --return-periods "2,5,10,25,50,100" \
  --durations "5,10,15,30,60,120" \
  --output la_idf.csv
```

**Features:**
- ✅ Fetches **real NOAA ATLAS14 precipitation frequency data** directly from NOAA servers
- ✅ Provides official, authoritative rainfall intensity values used in professional engineering practice
- ✅ Supports both English (in/hr) and metric (mm/hr) units
- ✅ Customizable return periods and storm durations
- ✅ Outputs CSV in HEC-22 compatible format (return_period, duration, intensity)
- ✅ **IDF interpolation**: The HEC-22 library automatically interpolates between IDF curve points using linear interpolation

**Output Format:**
```csv
return_period,duration,intensity
2,5,6.82
2,10,5.49
2,15,4.75
10,5,9.35
10,10,7.54
10,15,6.52
...
```

See [docs/ATLAS14_UTILITY.md](docs/ATLAS14_UTILITY.md) for detailed documentation and examples.

### Chapter Extraction Script

The `extract_chapters.py` script automatically extracts individual chapters and appendices from the complete HEC-22 PDF manual into separate files for easier reference.

**Usage:**

```bash
# Analyze the PDF and show chapter page ranges (dry run)
python extract_chapters.py

# Extract chapters to reference/chapters/ directory
python extract_chapters.py --extract
```

**Features:**
- Automatically detects chapter and appendix boundaries
- Extracts each section to a separate PDF file
- Requires PyPDF2 (auto-installs if missing)
- Skips front matter and table of contents
- Creates descriptive filenames (e.g., "HEC22 Chapter 4.pdf")

**Output:**
- Individual chapter PDFs saved to `reference/chapters/`
- Console output shows page ranges for verification

## Project Structure

```
hec22/
├── README.md                          # This file
├── CLI_USAGE.md                       # Comprehensive CLI usage guide
├── Cargo.toml                         # Rust project configuration
├── extract_chapters.py                # Script to extract PDF chapters
│
├── src/                               # Rust source code
│   ├── lib.rs                         # Library entry point
│   ├── main.rs                        # CLI application entry point
│   ├── analysis.rs                    # Network analysis and flow routing
│   ├── conduit.rs                     # Pipe/conduit hydraulics
│   ├── csv.rs                         # CSV parsing for tabular input
│   ├── drainage.rs                    # Drainage area and runoff calculations
│   ├── gutter.rs                      # Gutter flow (HEC-22 Chapter 4/5)
│   ├── hydraulics.rs                  # Core hydraulic calculations
│   ├── inlet.rs                       # Inlet capacity and interception
│   ├── network.rs                     # Network data structures
│   ├── node.rs                        # Node types (inlet, junction, outfall)
│   ├── project.rs                     # Project-level data structures
│   ├── rainfall.rs                    # IDF curves and rainfall analysis
│   ├── solver.rs                      # HGL/EGL solver
│   └── bin/
│       └── atlas14_fetch.rs           # ATLAS14 utility for fetching NOAA data
│
├── examples/complete_network/      # CSV examples for network input
│   ├── README.md                      # Template documentation
│   ├── nodes.csv                      # Node definitions
│   ├── conduits.csv                   # Pipe/conduit definitions
│   ├── drainage_areas.csv             # Drainage area definitions
│   ├── idf_curves.csv                 # IDF curve data
│   ├── design_storms.csv              # Design storm definitions (planned)
│   ├── nodes_extended_example.csv     # Extended examples with shapes
│   └── conduits_extended_example.csv  # Extended examples with pipe types
│
├── examples/                          # Example networks and workflows
│   ├── build_network.rs               # Network construction example
│   ├── gutter_spread.rs               # Gutter spread calculation
│   ├── hydraulic_solver.rs            # HGL/EGL solver example
│   ├── inlet_bypass_workflow.rs       # Inlet interception analysis
│   ├── inlet_capacity.rs              # Inlet capacity calculations
│   ├── load_json.rs                   # JSON network loading
│   └── complete_network/              # Complete example network
│       ├── README.md                  # Network description
│       ├── nodes.csv                  # Example nodes (various shapes)
│       ├── conduits.csv               # Example pipes (circular, rectangular, etc.)
│       ├── drainage_areas.csv         # Example drainage areas
│       ├── idf_curves.csv             # Example IDF data
│       └── design_storms.csv          # Example design storms
│
├── docs/                              # Documentation
│   └── ATLAS14_UTILITY.md             # ATLAS14 utility documentation
│
├── tests/                             # Integration and verification tests
│   ├── README.md                      # Test documentation
│   ├── chapter5_verification.rs       # HEC-22 Chapter 5 verification
│   ├── json_schema_tests.rs           # JSON schema validation
│   └── network_integration_test.rs    # Full network integration tests
│
├── reference/                         # Reference materials
│   ├── chapters/                      # Individual HEC-22 chapters (PDFs)
│   ├── equations/                     # Hydraulic equations (markdown)
│   │   ├── manning_equation.md
│   │   ├── gutter_flow.md
│   │   ├── inlet_design.md
│   │   ├── rational_method.md
│   │   └── open_channel_flow.md
│   ├── constants/                     # Design constants
│   │   └── manning_n_values.md
│   ├── guidance/                      # Design procedures
│   │   ├── component_definitions.md
│   │   ├── design_workflow.md
│   │   ├── IMPLEMENTATION_GUIDE.md    # Advanced implementation guidance
│   │   └── hif24006.pdf               # Complete HEC-22 manual
│   └── TEST_CASE_REFERENCE.md         # Comprehensive test cases and examples
│
├── schema/                            # JSON schema definitions
│   └── examples/                      # Schema example files
│
└── LICENSE                            # Project license
```

## Development Roadmap

This roadmap defines the phased development of the HEC-22 drainage analysis system, prioritizing practical usability for practicing engineers.

### Phase 1: Core Hydraulics ✓ COMPLETE

**Status**: All features implemented and verified

- [x] Manning's equation for pipes and channels
- [x] Gutter flow calculations (Chapter 5)
- [x] Inlet capacity and bypass flow (Chapter 7)
- [x] HGL/EGL solver with junction losses (Chapter 9)
- [x] JSON schema and Rust type system
- [x] Verification tests against HEC-22 worked examples (59 tests passing)

**Deliverables**:
- Complete Rust library (`hec22` crate) with hydraulic calculation functions
- JSON schema for drainage network representation
- Working examples demonstrating all core features

---

### Phase 2: Tabular Input & CLI MVP ✅ MOSTLY COMPLETE

**Status**: Core features implemented and operational

**Goal**: Enable non-programmers to analyze drainage systems using spreadsheets and command-line tools

**Target User**: "I have a spreadsheet with nodes, pipes, and drainage areas. I need to check HGL and gutter spread."

#### 2.1 CSV/Excel Input Parser ✅
- [x] **Node table parser** - Read inlet/junction/outfall data from CSV
  - Columns: `id`, `type`, `invert_elev`, `rim_elev`, `x`, `y`, `shape`, `diameter`, `width`, `height`
  - Supports both circular and rectangular manholes
- [x] **Conduit table parser** - Read pipe/gutter data from CSV
  - Columns: `id`, `from_node`, `to_node`, `type`, `shape`, `diameter`, `width`, `height`, `length`, `slope`, `manning_n`, `material`
  - Supports multiple pipe shapes: circular, rectangular, elliptical, arch
- [x] **Drainage area parser** - Read subcatchment data
  - Columns: `id`, `area`, `runoff_coef`, `time_of_conc`, `outlet_node`, `land_use`, `design_storm`
- [x] **IDF curves parser** - Read rainfall intensity-duration-frequency data
  - Columns: `return_period`, `duration`, `intensity`
  - Supports linear interpolation between duration points
- [ ] **Gutter/curb parameters** - Read surface drainage properties (partial support)
  - Columns: `node_id`, `cross_slope`, `long_slope`, `curb_height`, `gutter_width`
- [x] **Unit system support** - US Customary and SI Metric units

#### 2.2 CLI Tool (`hec22`) ✅
- [x] **Command structure** - Full CLI with arguments and flags
  ```bash
  hec22 --nodes nodes.csv --conduits pipes.csv \
        --drainage-areas catchments.csv \
        --idf-curves idf.csv --return-period 10 \
        --output report.txt
  ```
- [x] **Input validation** - Check for missing nodes, disconnected networks, invalid slopes
- [x] **Multiple output formats** - Text, JSON, CSV
- [x] **Error messages** - Clear, actionable error messages for non-programmers
- [x] **Comprehensive CSV templates** - Ready-to-use examples in `examples/complete_network/` directory
- [ ] **Progress reporting** - Show analysis progress for large networks (planned)

#### 2.3 HGL Analysis & Reporting ✅
- [x] **Automatic flow assignment** - Assign drainage area flows to inlets using rational method
- [x] **Peak flow computation** - Q = C × i × A with automatic IDF curve lookup by Tc
- [x] **HGL solver execution** - Run hydraulic grade line analysis with energy losses
- [x] **EGL computation** - Energy Grade Line with velocity head
- [x] **Violation detection** - Identify nodes where HGL exceeds rim elevation
- [x] **Text report generation** - Formatted tables with node and conduit results
  ```
  === HYDRAULIC ANALYSIS RESULTS ===
  Node ID    HGL (ft)   EGL (ft)   Depth (ft)  Velocity   Flooding
  IN-001      106.37     107.48       0.00        0.00       YES
  MH-001       99.22     100.56       0.00        0.00        No
  ```
- [x] **JSON output** - Structured data for further processing
- [x] **CSV output** - Results exported to spreadsheet format

#### 2.4 Gutter Spread Reporting 🚧
- [x] **Gutter flow equations** - Implemented in library (Chapter 4/5)
- [x] **Spread calculation** - Available via library functions
- [ ] **CLI integration** - Automatic spread reporting in CLI output (planned)
- [ ] **Criteria checking** - Compare to design limits (e.g., 10 ft max) (planned)

#### 2.5 Basic Output Formats ✅
- [x] **Text report** - Human-readable summary with tables (`.txt`)
- [x] **JSON export** - Structured JSON with all results (`.json`)
- [x] **CSV export** - Results tables for Excel import (`.csv`)
- [x] **Summary statistics** - Flow rates, HGL values, violation counts
- [x] **Design violation warnings** - HGL violations, capacity issues, velocity problems

**Success Criteria**: ✅ ACHIEVED
A civil engineer with a spreadsheet can run the tool and get HGL results in under 5 minutes, without writing code.

**Deliverables**:
- Working CLI tool (`hec22`) accepting CSV inputs
- Comprehensive CSV templates with examples
- Multiple output formats (text, JSON, CSV)
- Automatic peak flow computation using IDF curves
- Complete hydraulic analysis with violation detection
- See [CLI_USAGE.md](CLI_USAGE.md) for detailed usage guide

---

### Phase 3: Design Automation

**Goal**: Automated pipe sizing and inlet spacing optimization

- [ ] **Pipe sizing optimizer** - Auto-size pipes to meet HGL criteria
- [ ] **Inlet spacing calculator** - Optimize inlet locations for spread limits
- [ ] **Iterative solver** - Adjust network until all criteria satisfied
- [ ] **Cost optimization** - Minimize total pipe material cost
- [ ] **Design alternatives** - Generate multiple solutions ranked by cost/performance

**Use Case**: "Size my pipes to prevent flooding with minimum cost"

---

### Phase 4: Advanced HEC-22 Features

**Goal**: Complete coverage of HEC-22 methodology

#### 4.1 Detention/Retention (Chapter 10)
- [ ] Stage-storage-discharge relationships
- [ ] Modified Puls routing method
- [ ] Outlet structure design (weirs, orifices, culverts)
- [ ] Pond sizing for peak flow attenuation

#### 4.2 Water Quality BMPs (Chapter 11)
- [ ] Pollutant load estimation (runoff quality)
- [ ] BMP removal efficiency calculations
- [ ] LID/Green infrastructure sizing (bioretention, permeable pavement)
- [ ] Treatment train analysis

#### 4.3 Pump Stations (Chapter 12)
- [ ] Pump selection and system curves
- [ ] Wet well volume calculations
- [ ] Storage routing with pumping
- [ ] Multiple pump operation strategies

**Deliverable**: Full HEC-22 methodology implementation

---

### Phase 5: File Format Converters & Integration

**Goal**: Interoperability with existing tools

#### 5.1 Import Formats
- [ ] **SWMM .inp** - EPA Storm Water Management Model files
- [ ] **Civil3D XML** - Autodesk Civil3D pipe network export
- [ ] **HydroCAD** - HydroCAD project files
- [ ] **Excel templates** - Pre-formatted spreadsheet layouts
- [ ] **GIS shapefiles** - Import network geometry from ArcGIS/QGIS

#### 5.2 Export Formats
- [ ] **GeoJSON** - For web mapping and GIS integration
- [ ] **PDF reports** - Professional calculation packages
- [ ] **SWMM .inp** - Export to EPA SWMM for dynamic modeling
- [ ] **DXF/DWG** - CAD drawing exchange

**Use Case**: "Convert my 20 SWMM models to this format" or "Export results to AutoCAD"

---

### Phase 6: Web Interface & Visualization 🚧 IN PROGRESS

**Goal**: Interactive design environment accessible via browser

**Status**: Basic visualization capabilities implemented! ✅

**Completed Features**:
- [x] **Network plan view** - SVG visualization of network layout showing nodes and conduits
- [x] **Profile plots** - Elevation profiles along pipe runs (ground line and pipe invert)
- [x] **SVG export** - Export visualizations as standalone SVG files
- [x] **Interactive HTML viewer** - Web-based viewer with pan, zoom, and download capabilities
- [x] **CLI integration** - Command-line flags for visualization export

**In Progress**:
- [ ] **HGL/EGL overlay** - Add hydraulic grade line and energy grade line to profile views (data structure ready, visualization pending)
- [ ] **Drainage area mapping** - Catchment boundaries and flow paths
- [ ] **Real-time editing** - Modify network and see immediate results
- [ ] **Report generation** - Export professional PDF calculation packages
- [ ] **Collaboration** - Share projects via URL, multi-user access

**Technology**: SVG + HTML/CSS/JavaScript (no framework dependencies)

**Use Case**: "Visualize my drainage network and share results with non-technical stakeholders"

**Usage Examples**:

```bash
# Export network plan view
./target/release/hec22 \
  --nodes nodes.csv --conduits pipes.csv \
  --export-network-plan network_plan.svg

# Export profile view
./target/release/hec22 \
  --nodes nodes.csv --conduits pipes.csv \
  --export-profile profile.svg \
  --profile-path "IN-001,MH-001,OUT-001"

# Export interactive HTML viewer (includes both views)
./target/release/hec22 \
  --nodes nodes.csv --conduits pipes.csv \
  --drainage-areas areas.csv \
  --export-html network_viewer.html
```

**Features**:
- **Network Plan View**: Top-down view of the drainage network
  - Color-coded nodes (green=inlet, blue=junction, red=outfall)
  - Conduit connections with flow direction arrows
  - Node labels and coordinates
  - Auto-layout for networks without coordinates

- **Profile View**: Elevation profile along a pipe run
  - Pipe invert elevations
  - Ground/rim elevations
  - Station labels and node markers
  - Automatic profile path detection (follows upstream from outfall)
  - Custom profile paths via `--profile-path` flag

- **Interactive HTML Viewer**: Web-based visualization
  - Pan and zoom controls
  - Mouse drag to pan
  - Download SVG button
  - Network statistics (node count, conduit count)
  - Responsive layout

---

### Phase 7: Production Readiness & Distribution

**Goal**: Make the tool accessible to the broader engineering community

- [ ] **Performance optimization** - Handle 1000+ node networks efficiently
- [ ] **Comprehensive error handling** - Graceful failures with helpful messages
- [ ] **API documentation** - Detailed docs for all public functions
- [ ] **User guide** - Step-by-step tutorials for common workflows
- [ ] **Example projects** - Library of worked examples from real projects
- [ ] **Packaging & distribution**
  - Rust crate on crates.io
  - Python bindings on PyPI
  - npm package for JavaScript/TypeScript
  - Pre-built CLI binaries for Windows/Mac/Linux
- [ ] **Testing & CI/CD** - Automated testing on all platforms
- [ ] **Versioning & releases** - Semantic versioning with changelogs

**Deliverable**: Production-quality tool ready for professional use

---

## Current Focus

**Current Status**: Phase 6 (Visualization) in progress! 🚧

**Recently Completed**:
- ✅ Phase 2 (CLI MVP) - Fully operational hydraulic analysis tool
- ✅ Basic visualization capabilities (network plan and profile views)
- ✅ SVG export and interactive HTML viewer

**What's Working Now**:
Engineers can now:
- Import drainage networks from spreadsheets (nodes, pipes, drainage areas)
- Run HGL/EGL analysis with automatic flow routing
- Use IDF curves with automatic intensity lookup by time of concentration
- Generate reports in text, JSON, or CSV formats
- **Visualize networks** with plan and profile views (NEW!)
- **Export visualizations** as SVG or interactive HTML (NEW!)
- Analyze networks with multiple pipe shapes (circular, rectangular, elliptical, arch)
- Fetch real NOAA ATLAS14 rainfall data using the `atlas14_fetch` utility

**Next Steps**:
- Add HGL/EGL overlay to profile visualizations
- Implement drainage area boundary visualization
- Complete gutter spread reporting integration in CLI
- Consider Phase 3 (Design Automation) for pipe sizing optimization

See [CLI_USAGE.md](CLI_USAGE.md) for complete usage instructions.

## Contributing

This is an open-source educational and professional resource. Contributions, corrections, and enhancements are welcome.

## License

See LICENSE file for details.

## Acknowledgments

- Federal Highway Administration (FHWA) for HEC-22 methodology
- U.S. Department of Transportation
- Hydraulic engineering community

---

**Document Version:** 2.1
**Last Updated:** November 2025
**Status:** Phase 1 Complete ✅ | Phase 2 Complete ✅ | Phase 6 In Progress 🚧 | Visualization Operational ✅
**Based on:** FHWA HEC-22, 4th Edition (February 2024)
