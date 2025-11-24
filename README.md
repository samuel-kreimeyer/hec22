# HEC-22 Urban Drainage Analysis System

## Overview

This project provides a comprehensive framework for analyzing urban drainage systems in highway design contexts, following the **FHWA HEC-22 (4th Edition, 2024)** methodology.

Similar to commercial tools like StormCAD and HydroCAD, this system allows users to input information about drainage system components and perform hydraulic calculations for storm sewer design.

## System Components

The drainage system model includes four primary component types:

1. **Outfalls** - Discharge points from the drainage system
2. **Junctions** - Connection points (manholes, junction boxes)
3. **Inlets** - Collection points for surface runoff (grates, curb openings)
4. **Conduits** - Pipes and channels that convey flow between components

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
  - Channel lining materials (flexible vs. rigid)
  - Superelevation in bends
  - Complete design procedure with worked examples
- **HEC22 Chapter 7.pdf** - Inlet Design
  - Inlet types and selection (grate, curb-opening, combination, slotted)
  - Hydraulic efficiency and bicycle/pedestrian/ADA safety
  - Grate inlet types and performance (P-1-7/8, curved vane, tilt-bar, reticuline)
  - Interception capacity on continuous grades
  - Grate inlet equations (frontal flow, side flow, splash-over velocity)
  - Curb-opening inlet design (weir and orifice flow)
  - Depressed gutter sections and local depression
  - Combination inlets and sweeper configurations
  - Inlet interception in sag locations
  - Inlet spacing and location criteria
  - Flanking inlets for sag vertical curves
  - Median and roadside ditch inlet design
  - Embankment inlets with downdrains
  - Clogging considerations and debris handling
- **HEC22 Chapter 8.pdf** - Storm Drain Structures
  - Inlet structures (catch basins) - types and configuration
  - Drop inlets, inlets with sumps, curb inlets, combination inlets
  - Inlet materials (cast-in-place and pre-cast concrete)
  - Inlet location and spacing based on hydraulic effectiveness
  - Access holes (manholes) for inspection and maintenance
  - Access hole configuration (circular, 24" minimum shaft, 4-5 ft bottom chamber)
  - Concentric vs. eccentric cone designs
  - Benching and flow channels to reduce energy losses
  - Access hole depth (typically 5-13 ft) and safety features
  - Access hole spacing criteria by pipe size (300-1000 ft)
  - Placement at junctions, pipe changes, and alignment changes
  - Junction chambers for large storm drain connections
  - Rectangular, circular, or irregular chamber shapes
  - Transitions between pipe sizes and shapes
  - Transition ratios (5:1 to 20:1) to minimize energy loss
  - Flow splitters for diverting high flows
  - Inverted siphons (depressed pipes) under obstructions
  - Multi-barrel siphon design and sediment flushing
  - Flap gates to prevent back-flooding from receiving waters
  - Materials selection and structural considerations
  - Maintenance access and safety requirements
- **HEC22 Chapter 9.pdf** - Storm Drain Conduits
  - Storm drainage system components (inlets, conduits, outfalls)
  - Flow type assumptions (steady and uniform flow)
  - Open channel vs. pressure flow design philosophies
  - Hydraulic capacity using Manning's equation for conduits
  - Manning's roughness coefficients for various pipe materials
  - Conduit shape alternatives (circular, oval, arch, box)
  - Energy Grade Line (EGL) and Hydraulic Grade Line (HGL) concepts:
    - EGL: longitudinal line connecting points of total energy (elevation + velocity + pressure head)
    - HGL: water surface level in open channels, conceptual pressure line in closed conduits
    - HGL determination: subtract velocity head (V²/2g) from EGL at any point
    - Surcharging occurs when HGL exceeds top elevation of structures
    - Starting point: tailwater depth or (yc + D)/2 at outfall, work upstream
  - Storm drain outfalls and tailwater elevation considerations
  - Energy losses - detailed calculation methods:
    - Pipe friction losses: hf = Sf × L where Sf = [(Qn)/(KQD^2.67)]²
    - Exit losses: H₀ = 1.0[(V₀²/2g) - (Vd²/2g)] for sudden expansion
    - Bend losses: Hb = 0.0033(Δ)(V²/2g) where Δ = angle in degrees
    - Transition losses: He = Ke[(V₂²/2g) - (V₁²/2g)] for expansions/contractions
  - Junction losses using momentum equations:
    - Hj = [(Q₀V₀) - (QᵢVᵢ) - (QₗVₗcos θj)] / [0.5g(A₀ + Aᵢ)] + hᵢ - h₀
    - Accounts for flow changes at pipe junctions with lateral inflows
    - Interior angle θj measured between trunk and lateral pipes
  - Approximate method for inlet and access hole energy loss:
    - Hah = Kah(V₀²/2g) where Kah varies by configuration
    - Straight run: Kah = 0.15 to 0.50
    - 90° angle: Kah = 1.00 to 1.50
    - Used for preliminary design only, not final EGL calculations
  - FHWA comprehensive method for access hole energy losses:
    - Three-step procedure: initial energy level, adjustments, exit losses
    - Initial energy: Eai = max(Eaio, Eais, Eaiu)
      - Outlet control: Eaio = Ei + Ki(Vi²/2g) where Ki = 0.2
      - Submerged inlet: Eais = D₀(DI)² where DI = Q/[A(D₀g)^0.5]
      - Unsubmerged inlet: Eaiu = 1.6D₀(DI)^0.67
    - Adjustments: Ea = Eai + HB + Hθ + HP
      - Benching: HB = CB(Eai - Ei) where CB ranges from -0.98 to 0.0
      - Angled inflow: Hθ = Cθ(Eai - Ei) where Cθ = 4.5(ΣQj/Q₀)cos(θw/2)
      - Plunging flow: HP = CP(Eai - Ei) where CP = Σ(Qkhk)/Q₀
    - Exit losses: EGL₀ = Ea + K₀(V₀²/2g) where K₀ = 0.4
    - Non-iterative, handles surcharged and supercritical flows
  - Inlet control (weir and orifice) and outlet control conditions
  - Benching, angled inflow, and plunging flow adjustments
  - Design storm frequency selection (0.1 to 0.02 AEP)
  - Time of concentration and discharge determination
  - Minimum velocity (3 ft/s) and self-cleansing requirements
  - Minimum pipe grades and cover requirements
  - Location, alignment, and curved storm drain considerations
  - Maintenance considerations and inspection requirements
  - Preliminary design procedure (9-step process)
  - Hydraulic grade line evaluation methodology (9-step procedure):
    - Step 1: Determine tailwater elevation at outfall
    - Step 2: Estimate HGL/EGL at downstream end of each pipe (5 cases in Table 9.6)
    - Step 3: Estimate HGL/EGL at upstream end of pipe (4 flow conditions in Table 9.7)
      - Condition A: Full flow (surcharge) - HGLi ≥ TOCi
      - Condition B: Downstream-controlled partial flow - TOCi ≥ HGLi > BOCi + yn and yc
      - Condition C: Subcritical partial flow - BOCi + yn ≥ HGLi > BOCi + yc
      - Condition D: Supercritical partial flow - BOCi + yc ≥ HGLi (losses not carried upstream)
    - Step 4: Calculate EGL/HGL at each structure using FHWA comprehensive method
    - Step 5-8: Repeat for all pipes and structures working upstream
    - Step 9: Compare EGL elevations to ground surface, verify minimum cover
    - Key concept: Start at outfall, work upstream; subcritical carries losses, supercritical doesn't
  - Complete worked example with EGL/HGL calculations (4-structure system)
  - Supercritical and subcritical flow identification
  - Debris and clogging prevention strategies
- **HEC22 Chapter 10.pdf** - Detention and Retention
  - Design objectives (peak flow attenuation, volume control, multiple storm events)
  - Design challenges (release timing, downstream impacts, hydrograph synchronization)
  - Safety considerations (public access control, emergency escape, velocity hazards)
  - Maintenance requirements (inspections, mowing, sediment/debris control, structural repairs)
  - Detention facilities (dry ponds) - temporary storage with controlled release
  - Retention facilities (wet ponds) - permanent pool with water quality benefits
  - Preliminary storage volume estimation methods:
    - Loss-of-natural-storage method
    - Actual inflow/estimated release method
    - Rational Method triangular hydrograph method
    - NRCS TR-55 procedure
  - Stage-storage relationships for various basin geometries:
    - Rectangular basins
    - Trapezoidal basins
    - Circular pipes and conduits (prismoidal formula)
    - Irregular basins (average-end area and conic section methods)
  - Dead storage vs. active storage concepts
  - Stage-discharge relationships (performance curves):
    - Discharge pipes (single-stage and multi-stage)
    - Orifice flow equations and coefficients
    - Weir flow (sharp-crested, broad-crested, V-notch, proportional)
    - Composite stage-discharge curves
    - Emergency spillways
  - Single-stage and multi-stage riser design procedures
  - Water budgets for wet ponds (rainfall, runoff, evaporation, infiltration)
  - Storage routing using modified Puls (storage-indication) method
  - Complete detention design procedure with iterative approach
  - Landlocked storage analysis (mass routing for karst areas)
  - Worked examples with detailed calculations
- **HEC22 Chapter 11.pdf** - Urban Stormwater Quality
  - BMP (Best Management Practice) alternatives and selection
  - Pollutant load estimation (Simple Method, HRDB, SELDM)
  - Water quality volume (WQV) and first flush concepts
  - Structural BMPs - Storage-based:
    - Extended detention dry ponds (24-48 hour detention)
    - Wet ponds (retention ponds with permanent pool)
    - Water budget analysis for wet ponds
  - Structural BMPs - Infiltration-based:
    - Infiltration/exfiltration trenches (complete, partial, water quality systems)
    - Infiltration basins (up to 50 acre drainage areas)
    - Sand filters (peat-sand filters, compartment systems)
  - Green infrastructure and Low Impact Development (LID):
    - Bioretention areas (rain gardens)
    - Bioswales and vegetated swales
    - Stormwater curb extensions and planters
    - Stormwater tree systems
    - Permeable pavements (porous asphalt, pervious concrete, pavers)
    - Grassed swales with check dams and level spreaders
    - Filter strips for sheet flow treatment
    - Constructed wetlands
  - Pollutant removal efficiencies by BMP type
  - Ultra-urban BMPs (small footprint, underground):
    - Water quality inlets (oil/grit separators)
    - Hydrodynamic devices and filter inserts
    - Sumps and pre-treatment systems
  - Non-structural BMPs:
    - Storm drain cleaning and street sweeping
    - Landscaping practices and trash management
    - Slope stabilization and winter maintenance
  - BMP selection criteria and site suitability
- **HEC22 Chapter 12.pdf** - Pump Stations
  - When to use pump stations (gravity alternatives exhausted)
  - Pump station types:
    - Wet-pit stations (pumps submerged in wet well)
    - Dry-pit stations (separate wet well and dry well)
    - Submersible vs. shaft-driven configurations
  - Pump types and selection:
    - Axial flow pumps (low head, high discharge, propeller-type)
    - Radial flow pumps (high head, centrifugal force, debris handling)
    - Mixed flow pumps (intermediate applications, multi-stage)
  - Pump sizing and system curves:
    - Total Dynamic Head (TDH = static + friction + velocity + minor losses)
    - System curve development
    - Pump performance curves (head, efficiency, power)
    - Operating point determination
  - Number of pumps (2-3 minimum, equal sizing, automatic alternation)
  - Net Positive Suction Head (NPSH) requirements and cavitation prevention
  - Pump station components:
    - Water-level sensors (float switches, electronic probes, ultrasonic)
    - Power systems (electric motors, fuel-driven engines, backup power)
    - Discharge systems (force mains, piping, frost protection)
    - Valving (flap gates, check valves, gate valves, air/vacuum valves)
    - Trash racks and grit chambers
    - Monitoring systems and ITS integration
  - Site planning and hydrology:
    - Location selection and access
    - Design storm (typically 0.02 AEP for major highways)
    - Drainage area considerations
    - Collection system design
    - Hazardous materials spill protection
  - Storage and mass curve routing:
    - Balancing storage volume vs. pump capacity
    - Inflow mass curve development
    - Stage-storage relationships
    - Stage-discharge curves
    - Pump cycling time requirements
    - Iterative optimization of start/stop elevations
  - Safety considerations (confined space, ventilation, security)
  - Maintenance and operation requirements
- **HEC22 Appendix A.pdf** - Design Charts and Nomographs
- **HEC22 Appendix B.pdf** - Design Examples
- **HEC22 Appendix C.pdf** - Equations and Formulas

These chapters are automatically extracted from the complete HEC-22 PDF using `extract_chapters.py`.

### Equations (`reference/equations/`)

Core hydraulic and hydrologic equations for drainage design:

- **`manning_equation.md`** - Manning's equation for pipe flow capacity
  - Full flow and partial flow equations
  - Circular pipe formulas
  - Velocity calculations
  - Design considerations (min/max velocities, slopes)

- **`gutter_flow.md`** - Surface drainage and gutter flow analysis
  - Modified Manning's equation for gutters
  - Spread and depth calculations
  - Composite cross sections
  - Frontal flow ratios

- **`inlet_design.md`** - Inlet hydraulic design
  - Curb-opening inlet equations
  - Grate inlet efficiency
  - Combination inlets
  - On-grade vs. sag location analysis
  - Weir and orifice flow in sag locations

- **`rational_method.md`** - Runoff calculations
  - Rational formula (Q = C × i × A)
  - Runoff coefficients by land use
  - Time of concentration methods
  - Rainfall intensity (IDF curves)
  - Frequency adjustment factors

- **`open_channel_flow.md`** - Open channel flow for roadside/median channels
  - Energy equation and specific energy
  - Froude number and flow classification
  - Shear stress analysis (straight and bends)
  - Superelevation in channel bends
  - Channel geometry equations
  - Stable channel design criteria
  - Manning's n values for channel linings

### Constants (`reference/constants/`)

Design constants and coefficients:

- **`manning_n_values.md`** - Roughness coefficients
  - Concrete pipes
  - Corrugated metal pipes
  - PVC and HDPE pipes
  - Open channels and gutters
  - Selection guidelines

### Guidance (`reference/guidance/`)

Design procedures and component definitions:

- **`component_definitions.md`** - Detailed component specifications
  - Outfall types and properties
  - Junction/manhole design
  - Inlet types and configurations
  - Conduit materials and sizing
  - Tabular data structure for computational use

- **`design_workflow.md`** - Step-by-step design process
  - Data collection requirements
  - Hydrologic analysis procedures
  - Surface drainage design
  - Storm sewer system layout
  - Hydraulic analysis (HGL/EGL)
  - Quality control checklists

- **`chapter_10_design_notes.md`** - Detention and Retention Design Implementation
  - Computational implementation guidance for Chapter 10
  - Design objectives and facility types (detention vs. retention)
  - Preliminary storage volume estimation methods
  - Stage-storage relationships (rectangular, trapezoidal, irregular basins)
  - Stage-discharge relationships (orifices, weirs, composite curves)
  - Storage routing (Modified Puls method)
  - Single-stage and multi-stage riser design procedures
  - Water budget analysis for wet ponds
  - Design iteration workflows and optimization
  - Python code examples and data structures
  - Common pitfalls and best practices
  - Testing strategy and validation data

- **`chapter_11_design_notes.md`** - Urban Stormwater Quality Implementation
  - Computational implementation guidance for Chapter 11
  - BMP alternatives and selection criteria
  - Pollutant load estimation (Simple Method, HRDB, SELDM)
  - Water quality volume (WQV) calculations
  - Structural BMPs (storage-based and infiltration-based)
  - Green infrastructure and LID practices
  - Pollutant removal efficiency databases
  - Ultra-urban and non-structural BMPs
  - BMP sizing and design algorithms
  - Treatment train design
  - Python code examples and data structures
  - Performance monitoring and validation
  - Testing strategy and best practices

- **`chapter_12_design_notes.md`** - Pump Station Design Implementation
  - Computational implementation guidance for Chapter 12
  - Pump station types (wet-pit vs. dry-pit)
  - Pump types and selection (axial, radial, mixed flow)
  - Total Dynamic Head (TDH) calculations
  - System curve and pump performance curve development
  - Number of pumps and sizing optimization
  - NPSH requirements and cavitation prevention
  - Pump station component selection
  - Site planning and hydrology considerations
  - Storage and mass curve routing procedures
  - Stage-storage and stage-discharge relationships
  - Iterative design optimization
  - Python code examples and data structures
  - Safety considerations and maintenance planning
  - Testing strategy and validation data

### Examples (`reference/examples/`)

Worked examples demonstrating the methodology:

- **`example_problem_1.md`** - Complete storm drain design
  - Hydrologic analysis
  - Gutter flow calculations
  - Inlet design (combination inlet)
  - Pipe sizing and layout
  - HGL analysis
  - Design summary tables

### Test Cases (`reference/TEST_CASE_REFERENCE.md`)

Comprehensive test case documentation for automated testing:

- **`TEST_CASE_REFERENCE.md`** - Formula validation and test cases
  - 20+ detailed test cases with known inputs and expected outputs
  - Complete worked example integrating all formulas
  - Step-by-step calculations for validation
  - All key formulas: rational method, gutter flow, inlet design, Manning's equation
  - Design constants and validation criteria
  - Ready-to-use format for unit, integration, and regression testing
  - Basis for future automated test suite development

This document consolidates all formulas and the worked example into a format specifically designed for creating automated test cases. Each test case includes inputs, expected outputs, and detailed calculation steps for verification.

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
|---------|------|---------------|------------------|-------------|
| IN-001 | Inlet | 128.75 | 124.50 | Inlet at Sta 0+25 |
| MH-101 | Junction | 125.30 | 118.50 | Manhole at Sta 1+50 |
| OUT-001 | Outfall | -- | 100.50 | Discharge to creek |

### Conduits Table

| Conduit ID | From Node | To Node | Diameter (in) | Length (ft) | n | Slope | Up Invert | Dn Invert |
|------------|-----------|---------|---------------|-------------|---|-------|-----------|-----------|
| C-101 | IN-001 | MH-101 | 18 | 120 | 0.013 | 0.0067 | 124.30 | 118.60 |
| C-102 | MH-101 | OUT-001 | 24 | 250 | 0.013 | 0.0716 | 118.50 | 100.60 |

### Drainage Areas Table

| Subarea ID | Area (acres) | Land Use | C Value | Tc (min) | Rainfall (in/hr) | Q (cfs) |
|------------|--------------|----------|---------|----------|------------------|---------|
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
|----------|---------|
| RCP (Reinforced Concrete Pipe) | 0.013 |
| CMP (Corrugated Metal Pipe) | 0.024 |
| PVC/HDPE (smooth) | 0.011 |
| Concrete gutter | 0.016 |

## Runoff Coefficients (Common)

| Surface Type | C Value |
|--------------|---------|
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
│   │   ├── HEC22 Chapter 2.pdf
│   │   ├── HEC22 Chapter 3.pdf
│   │   ├── HEC22 Chapter 4.pdf
│   │   ├── HEC22 Chapter 5.pdf
│   │   ├── HEC22 Chapter 6.pdf
│   │   ├── HEC22 Chapter 7.pdf
│   │   ├── HEC22 Chapter 8.pdf
│   │   ├── HEC22 Chapter 9.pdf
│   │   ├── HEC22 Chapter 10.pdf
│   │   ├── HEC22 Chapter 11.pdf
│   │   ├── HEC22 Appendix A.pdf
│   │   ├── HEC22 Appendix B.pdf
│   │   └── HEC22 Appendix C.pdf
│   ├── equations/                     # Hydraulic equations
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
│   │   └── hif24006.pdf               # Complete HEC-22 manual
│   ├── examples/                      # Worked examples
│   │   └── example_problem_1.md
│   └── TEST_CASE_REFERENCE.md         # Comprehensive test cases for formulas
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

**Document Version:** 1.0
**Last Updated:** November 2025
**Based on:** FHWA HEC-22, 4th Edition (February 2024)
