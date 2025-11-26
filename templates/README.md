# HEC-22 CSV Templates

This directory contains CSV templates for defining drainage networks using the HEC-22 methodology.

## Overview of Templates

### Core Templates (✅ Fully Implemented)
- **nodes.csv** - Network nodes (inlets, junctions, outfalls)
- **conduits.csv** - Pipes and channels connecting nodes
- **drainage_areas.csv** - Subcatchment/drainage area definitions
- **idf_curves.csv** - Intensity-Duration-Frequency curves for rainfall analysis

### Planned Templates (🚧 Future Features)
- **design_storms.csv** - Design storm event definitions (planned for future release)
- **gutter_parameters.csv** - Gutter and curb inlet parameters (planned for future release)

### Extended Examples
- **nodes_extended_example.csv** - Shows rectangular manholes
- **conduits_extended_example.csv** - Shows various pipe shapes (circular, rectangular, elliptical, arch)

## Quick Start

1. Copy the template files you need to your project directory
2. Edit the CSV files with your project-specific data
3. Use the HEC-22 CLI to analyze your network:

```bash
# Basic analysis with fixed intensity
hec22 --nodes nodes.csv --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --intensity 4.5 --output results.txt

# Using IDF curves (automatic intensity lookup by Tc)
hec22 --nodes nodes.csv --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 10 --output results.txt
```

## Template Details

### nodes.csv

Defines network nodes including inlets, junctions (manholes), and outfalls.

**Common Columns (All Nodes):**
- `id` - Unique node identifier (e.g., "IN-001", "MH-001", "OUT-001")
- `type` - Node type: "inlet", "junction", "outfall"
- `invert_elev` - Invert elevation (ft or m)
- `rim_elev` - Ground/rim elevation (ft or m)
- `x`, `y` - Coordinates for mapping (optional)

**Junction-Specific Columns:**
- `shape` - Manhole shape: "circular" or "rectangular"
- `diameter` - Diameter in inches or mm (for circular manholes)
- `width` - Width in feet or meters (for rectangular manholes)
- `height` - Height in feet or meters (for rectangular manholes)

**Inlet-Specific Columns (HEC-22 Chapter 7 Parameters):**
- `inlet_type` - Type: "grate", "curb_opening", "combination", "slotted"
- `inlet_location` - Location: "on_grade" (continuous slope) or "sag" (low point)
- `grate_length` - Grate length parallel to flow (ft)
- `grate_width` - Grate width perpendicular to flow (ft)
- `bar_configuration` - Bar orientation: "parallel" or "perpendicular"
- `curb_opening_length` - Curb opening length (ft)
- `curb_opening_height` - Curb opening height (ft)
- `throat_type` - Throat configuration: "horizontal", "inclined", "vertical"
- `local_depression` - Local depression depth at inlet (inches)
- `clogging_factor` - Clogging reduction factor (0.0-1.0, typically 0.15-0.50)
- `grate_count` - Number of grates (for sag inlets)

**Outfall-Specific Columns:**
- `boundary_condition` - Boundary condition: "free", "normal", "fixed"

**Examples:**

```csv
# Grate inlet on grade with perpendicular bars
IN-001,inlet,100.5,105.5,0,0,,,,,grate,on_grade,3.0,2.0,perpendicular,,,,,0.15,1,

# Combination inlet (grate + curb opening) on grade
IN-002,inlet,98.2,103.2,120,0,,,,,combination,on_grade,2.5,1.5,perpendicular,5.0,0.5,horizontal,2.0,0.20,1,

# Curb opening inlet with vertical throat
IN-003,inlet,96.5,101.5,200,0,,,,,curb_opening,on_grade,,,,,8.0,0.5,vertical,,0.10,1,

# Grate inlet at sag location (2 grates)
IN-004,inlet,94.0,99.0,300,0,,,,,grate,sag,4.0,2.5,perpendicular,,,,,0.50,2,

# Circular manhole (4 ft diameter)
MH-001,junction,95.0,100.0,240,0,circular,4.0,,,,,,,,,,,,,

# Rectangular manhole (6 ft x 6 ft)
MH-002,junction,90.5,95.5,360,0,rectangular,,6.0,6.0,,,,,,,,,,,

# Free outfall
OUT-001,outfall,85.0,,480,0,,,,,,,,,,,,,,,free
```

**Inlet Design Guidelines (HEC-22):**

- **Grate Inlets**: Most common for on-grade locations. Use perpendicular bars for better hydraulic efficiency.
- **Curb Opening Inlets**: Used where grates are impractical or for aesthetic reasons. Less efficient than grates but less prone to clogging.
- **Combination Inlets**: Most effective design, combining advantages of both types. Grate captures flow first, curb opening captures bypass.
- **Clogging Factors**:
  - Grate inlets: 0.15-0.50 (use 50% capacity reduction for conservative design)
  - Curb openings: 0.10 (use 90% capacity)
  - Combination: 0.20 (use 80% capacity)
- **Local Depression**: 1-2 inches typical, improves grate efficiency by concentrating flow
- **Bar Configuration**: Perpendicular bars more efficient but may create bicycle hazard; parallel bars safer but less efficient

### conduits.csv

Defines pipes and channels connecting nodes.

**Columns:**
- `id` - Unique conduit identifier (e.g., "P-001", "CH-001")
- `type` - Conduit type: "pipe" or "channel"
- `from_node` - Upstream node ID
- `to_node` - Downstream node ID
- `shape` - Cross-section shape: "circular", "rectangular", "elliptical", "arch"
- `diameter` - Pipe diameter in inches or mm (for circular pipes)
- `width` - Width in inches or mm (for rectangular/elliptical/arch pipes)
- `height` - Height in inches or mm (for rectangular/elliptical/arch pipes)
- `length` - Length in feet or meters
- `slope` - Slope in ft/ft or m/m
- `manning_n` - Manning's roughness coefficient
- `material` - Pipe material: "RCP", "CMP", "PVC", "HDPE", "Concrete", "Steel", "Ductile Iron"
- `cross_slope` - Cross slope for gutter/street flow (optional)
- `long_slope` - Longitudinal slope for gutter flow (optional)

**Examples:**
```csv
# 18-inch circular RCP
P-001,pipe,IN-001,MH-001,circular,18,,,120,0.005,0.013,RCP,,

# 36x24 elliptical pipe
P-002,pipe,IN-002,MH-001,elliptical,,36,24,100,0.008,0.013,RCP,,

# 48x48 rectangular box culvert
P-003,pipe,MH-001,MH-002,rectangular,,48,48,120,0.0375,0.013,Concrete,,

# 72x48 arch pipe
P-004,pipe,MH-002,MH-003,arch,,72,48,120,0.025,0.024,CMP,,
```

### drainage_areas.csv

Defines drainage areas (subcatchments) that contribute runoff to the network.

**Columns:**
- `id` - Unique drainage area identifier
- `area` - Drainage area (acres or hectares)
- `runoff_coef` - Rational method C coefficient (0.0-1.0)
- `time_of_conc` - Time of concentration in minutes
- `outlet_node` - Node ID where runoff enters the network
- `land_use` - Land use type: "Commercial", "Industrial", "Residential", "Open Space", etc.
- `design_storm` - Design storm ID for automatic peak flow calculation (references design_storms.csv)

**Examples:**
```csv
# Commercial area, 0.75 acres, C=0.85, Tc=10 min
DA-001,0.75,0.85,10.0,IN-001,Commercial,DS-10YR

# Residential area, 1.25 acres, C=0.50, Tc=15 min
DA-002,1.25,0.50,15.0,IN-002,Residential,DS-10YR
```

### idf_curves.csv

Defines Intensity-Duration-Frequency (IDF) curves for rainfall analysis. Each row represents one point on an IDF curve.

**Columns:**
- `return_period` - Return period in years (e.g., 2, 5, 10, 25, 50, 100)
- `duration` - Storm duration in minutes
- `intensity` - Rainfall intensity in in/hr or mm/hr

**Example:**
```csv
return_period,duration,intensity
10,5,9.35
10,10,7.54
10,15,6.52
10,30,4.86
10,60,3.39
10,120,2.17
```

**Common durations:** 5, 10, 15, 30, 60, 120, 360, 720, 1440 minutes

**Common return periods:** 2, 5, 10, 25, 50, 100 years

### design_storms.csv (🚧 Planned Feature)

**Status:** This feature is planned for a future release. Currently, use IDF curves with the `--return-period` CLI parameter instead.

Defines design storm events for runoff analysis.

**Columns:**
- `id` - Unique storm identifier (e.g., "DS-10YR")
- `name` - Descriptive name (e.g., "10-Year 24-Hour")
- `return_period` - Return period in years
- `duration` - Storm duration in minutes (e.g., 1440 for 24 hours)
- `total_depth` - Total rainfall depth in inches or mm (optional)
- `distribution` - Temporal distribution: "SCS Type I", "SCS Type IA", "SCS Type II", "SCS Type III", "Uniform", "Custom"
- `peak_intensity` - Peak intensity in in/hr or mm/hr (optional, can be computed from IDF curves)

**Examples:**
```csv
# 10-year, 24-hour storm with SCS Type II distribution
DS-10YR,10-Year 24-Hour,10,1440,5.1,SCS Type II,

# 100-year, 24-hour storm
DS-100YR,100-Year 24-Hour,100,1440,8.0,SCS Type II,
```

## Using IDF Curves with Drainage Areas (✅ Implemented)

The CLI automatically computes peak flow for each drainage area using:
1. Time of concentration (Tc) from the drainage area
2. IDF curves to lookup rainfall intensity for the design storm's return period and duration equal to Tc
3. Linear interpolation between IDF curve points if Tc falls between durations
4. Rational method formula: **Q = C × i × A**

**Workflow:**
1. Create your IDF curves in `idf_curves.csv` (or use `atlas14_fetch` to get NOAA data)
2. Define drainage areas in `drainage_areas.csv` with time of concentration values
3. Run the CLI with `--idf-curves` and `--return-period` parameters:

```bash
hec22 --nodes nodes.csv --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 10 \
      --output results.txt
```

4. The system will:
   - Load the IDF curve for the specified return period (e.g., 10-year)
   - For each drainage area, find the rainfall intensity for duration = Tc
   - Interpolate linearly if Tc falls between IDF curve duration points
   - Compute peak flow: Q = C × i × A
   - Display the computed intensity and flow for each drainage area

## Pipe Shape Guidelines

### Circular Pipes
- Most common for storm sewers
- Specify `diameter` only
- Available in standard sizes: 12", 15", 18", 21", 24", 30", 36", 42", 48", 54", 60", 72", 84", 96"

### Rectangular Box Culverts
- Used for high capacity or shallow cover situations
- Specify `width` and `height`
- Common sizes: 4'×4', 6'×6', 8'×8', 10'×10'
- Can be custom sized

### Elliptical Pipes
- Used where vertical clearance is limited
- Specify `width` (horizontal span) and `height` (vertical rise)
- Common sizes: 14"×23", 19"×30", 24"×38", 29"×45", 34"×53", 38"×60", 48"×76", 58"×91"
- More hydraulically efficient than equivalent circular pipe of same height

### Arch Pipes
- Used where headroom is limited but width is available
- Specify `width` (horizontal span) and `height` (vertical rise)
- Common sizes: 18"×11", 22"×13", 36"×22", 58"×36", 73"×45"
- Lower profile than circular or elliptical

## Manning's n Values by Material

Typical Manning's roughness coefficients:
- **RCP** (Reinforced Concrete Pipe): 0.013
- **CMP** (Corrugated Metal Pipe): 0.024
- **PVC**: 0.011
- **HDPE**: 0.011
- **Concrete**: 0.013
- **Steel**: 0.012
- **Ductile Iron**: 0.013

## SCS Rainfall Distributions

- **SCS Type I**: Pacific maritime climate (Alaska, coastal Oregon/Washington)
- **SCS Type IA**: Pacific coast and intermountain regions (California, parts of Pacific Northwest)
- **SCS Type II**: Most of the US, moderate climates (Midwest, Northeast, most of US)
- **SCS Type III**: Gulf of Mexico and Atlantic coastal areas (Texas, Louisiana, Florida)
- **Uniform**: Constant intensity throughout the storm (rare, used for conservative estimates)
- **Custom**: User-defined hyetograph

## Units

The system supports both US customary and SI metric units. Ensure consistency within each file:

**US Customary:**
- Length: feet, inches
- Elevation: feet
- Flow: cfs (cubic feet per second)
- Area: acres
- Intensity: in/hr

**SI Metric:**
- Length: meters, millimeters
- Elevation: meters
- Flow: cms (cubic meters per second)
- Area: hectares
- Intensity: mm/hr

## Additional Resources

- See `/examples/` directory for complete example networks
- Refer to FHWA HEC-22 Urban Drainage Design Manual for methodology details
- Visit the project documentation for API usage
