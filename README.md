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

## Utilities

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
├── extract_chapters.py                # Script to extract PDF chapters
├── reference/                         # Reference materials
│   ├── chapters/                      # Individual HEC-22 chapters (PDFs)
│   ├── equations/                     # Hydraulic equations
│   ├── constants/                     # Design constants
│   ├── guidance/                      # Design procedures
│   │   ├── component_definitions.md
│   │   ├── design_workflow.md
│   │   ├── IMPLEMENTATION_GUIDE.md    # Advanced implementation guidance
│   │   └── hif24006.pdf               # Complete HEC-22 manual
│   └── TEST_CASE_REFERENCE.md         # Comprehensive test cases and examples
└── LICENSE                            # Project license
```

## Next Steps

Future development of this project may include:

1. **Computational Tools**
   - Python/JavaScript implementation of equations
   - Web-based calculator interface
   - Automated HGL analysis

2. **Data Management**
   - Database schema for component storage
   - Import/export to common formats (CSV, JSON, XML)
   - Integration with GIS data

3. **Visualization**
   - Network diagram generation
   - Profile plotting (HGL/EGL)
   - Drainage area mapping

4. **Advanced Features**
   - Detention pond design
   - Water quality BMPs
   - Pump station analysis
   - Dynamic modeling (EPA SWMM integration)

## Contributing

This is an open-source educational and professional resource. Contributions, corrections, and enhancements are welcome.

## License

See LICENSE file for details.

## Acknowledgments

- Federal Highway Administration (FHWA) for HEC-22 methodology
- U.S. Department of Transportation
- Hydraulic engineering community

---

**Document Version:** 1.1
**Last Updated:** November 2025
**Based on:** FHWA HEC-22, 4th Edition (February 2024)
