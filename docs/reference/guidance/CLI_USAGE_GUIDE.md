# HEC-22 CLI Usage Guide

## Overview

The `hec22` command-line tool performs hydraulic analysis of storm sewer networks following FHWA HEC-22 methodology. It reads network data from CSV files, computes flows using the rational method, routes flows through the network, and calculates the hydraulic grade line (HGL).

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at:
./target/release/hec22
```

## Basic Usage

### Minimum Required Files

The CLI requires at minimum:
1. **Nodes CSV** - Network nodes (inlets, junctions, outfalls)
2. **Conduits CSV** - Pipes connecting nodes

```bash
hec22 --nodes nodes.csv --conduits conduits.csv
```

This performs a basic hydraulic analysis with zero inflows.

### Adding Drainage Areas

To compute inflows using the rational method, add a drainage areas file:

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --intensity 4.5
```

This uses a fixed rainfall intensity of 4.5 in/hr for all drainage areas.

## Command-Line Parameters

### Required Parameters

- `--nodes <FILE>` or `-n <FILE>` - Path to nodes CSV file
- `--conduits <FILE>` or `-c <FILE>` - Path to conduits CSV file

### Optional Parameters

- `--drainage-areas <FILE>` or `-a <FILE>` - Path to drainage areas CSV file
- `--idf-curves <FILE>` - Path to IDF curves CSV file
- `--return-period <YEARS>` or `-r <YEARS>` - Return period in years (default: 10)
- `--intensity <VALUE>` or `-i <VALUE>` - Fixed rainfall intensity (default: 4.0 in/hr)
- `--units <SYSTEM>` or `-u <SYSTEM>` - Unit system: `us` or `si` (default: us)
- `--output <FILE>` or `-o <FILE>` - Output file path (default: stdout)
- `--format <FORMAT>` or `-f <FORMAT>` - Output format: `text`, `json`, or `csv` (default: text)

## Usage Examples

### Example 1: Basic Network Analysis

Analyze a network with no inflows:

```bash
hec22 --nodes nodes.csv --conduits conduits.csv
```

**Output:** HGL analysis results printed to terminal

### Example 2: Fixed Intensity Analysis

Use a fixed rainfall intensity for all drainage areas:

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --intensity 5.2 \
      --output results.txt
```

**Output:** Text report saved to `results.txt`

### Example 3: IDF Curve Analysis

Use IDF curves to automatically look up intensity based on time of concentration:

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 25 \
      --output results.txt
```

**How it works:**
1. Loads the 25-year IDF curve from `idf_curves.csv`
2. For each drainage area, reads its time of concentration (Tc)
3. Looks up rainfall intensity from the IDF curve at duration = Tc
4. Interpolates linearly if Tc falls between IDF curve points
5. Computes flow: Q = C × i × A
6. Routes flows and solves HGL

**Output:** Text report with per-area intensity values and flows

### Example 4: JSON Output

Generate JSON output for programmatic use:

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 10 \
      --format json \
      --output network_analysis.json
```

**Output:** JSON file with complete analysis results

### Example 5: CSV Output

Generate CSV tables for spreadsheet import:

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --intensity 4.8 \
      --format csv \
      --output results
```

**Output:** Two CSV files created:
- `results.nodes.csv` - Node results (HGL, depth, velocity, flooding)
- `results.conduits.csv` - Conduit results (flow, velocity, capacity)

### Example 6: SI Metric Units

Run analysis in SI metric units:

```bash
hec22 --nodes nodes_si.csv \
      --conduits conduits_si.csv \
      --drainage-areas drainage_si.csv \
      --intensity 120 \
      --units si \
      --output results_si.txt
```

**Note:** When using `--units si`:
- Intensity is in mm/hr instead of in/hr
- Elevations and lengths are in meters
- Flow is in m³/s (cms) instead of ft³/s (cfs)

## Workflow: Complete Analysis

### Step 1: Prepare Input Files

Create CSV files following the templates in `/examples/complete_network`:

1. **nodes.csv** - Define all inlets, junctions (manholes), and outfalls
2. **conduits.csv** - Define all pipes with diameters, lengths, slopes
3. **drainage_areas.csv** - Define subcatchments with areas, C values, and Tc
4. **idf_curves.csv** - Define rainfall IDF curves (or generate with `atlas14_fetch`)

### Step 2: Generate IDF Curves (Optional)

If you need IDF curves for your location, use the ATLAS14 utility:

```bash
atlas14_fetch --lat 40.7128 --lon -74.0060 --output idf_curves.csv
```

This fetches official NOAA ATLAS14 precipitation frequency data.

### Step 3: Run Analysis

```bash
hec22 --nodes nodes.csv \
      --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 10 \
      --output analysis_10yr.txt
```

### Step 4: Review Results

The output report includes:

**Node Results:**
- HGL (Hydraulic Grade Line) elevation
- EGL (Energy Grade Line) elevation
- Flow depth in structure
- Velocity
- Flooding status (Yes/No)

**Conduit Results:**
- Flow rate (cfs or cms)
- Flow velocity
- Flow depth
- Capacity utilization (%)
- Froude number
- Flow regime (subcritical/supercritical/critical)

**Violations:**
- HGL above rim elevations (flooding)
- Excessive velocities
- Capacity exceedances
- Other design criteria violations

### Step 5: Iterate Design

If violations are found:
1. Increase pipe sizes in conduits.csv
2. Adjust pipe slopes
3. Modify inlet locations
4. Re-run analysis

```bash
# After making changes
hec22 --nodes nodes.csv \
      --conduits conduits_revised.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --return-period 10 \
      --output analysis_revised.txt
```

## IDF Curves vs. Fixed Intensity

### When to Use Fixed Intensity (`--intensity`)

- Quick preliminary analyses
- When all drainage areas have similar Tc
- When IDF data is not available
- For simplified studies

**Limitation:** All drainage areas get the same intensity regardless of their time of concentration.

### When to Use IDF Curves (`--idf-curves`)

- Final design analyses
- When drainage areas have varying Tc values
- When accurate intensity is critical
- For regulatory submittals

**Advantage:** Each drainage area gets appropriate intensity based on its Tc, accounting for the duration-intensity relationship.

## CSV File Format Requirements

### nodes.csv

Required columns:
- `id` - Unique node identifier
- `type` - Node type: "inlet", "junction", or "outfall"
- `invert_elev` - Invert elevation (ft or m)
- `rim_elev` - Rim elevation (ft or m), required for inlets and junctions

Optional columns:
- `x`, `y` - Coordinates
- `diameter` - Junction diameter (ft or m)
- `inlet_type` - Inlet type: "grate", "curb", "combination", "slotted"
- `boundary_condition` - Outfall boundary: "free", "normal", "fixed"

### conduits.csv

Required columns:
- `id` - Unique conduit identifier
- `from_node` - Upstream node ID
- `to_node` - Downstream node ID
- `type` - Conduit type: "pipe" or "gutter"
- `diameter` - Pipe diameter (inches or mm)
- `length` - Conduit length (ft or m)

Optional columns:
- `slope` - Slope (ft/ft or m/m)
- `manning_n` - Roughness coefficient
- `material` - Pipe material: "RCP", "CMP", "PVC", "HDPE"

### drainage_areas.csv

Required columns:
- `id` - Unique area identifier
- `area` - Drainage area (acres or hectares)
- `runoff_coef` - Rational method C coefficient (0-1)
- `time_of_conc` - Time of concentration (minutes)
- `outlet_node` - Node where runoff enters network

Optional columns:
- `land_use` - Land use type

### idf_curves.csv

Required columns:
- `return_period` - Return period (years)
- `duration` - Storm duration (minutes)
- `intensity` - Rainfall intensity (in/hr or mm/hr)

**Note:** Multiple rows with the same return period form one IDF curve. Include all relevant durations for proper interpolation.

## Understanding Output

### Text Output Format

```
NODE RESULTS
-----------------------------------------------------------
Node ID      HGL (ft)   EGL (ft)   Depth (ft)  Velocity    Flooding
-----------------------------------------------------------
IN-001       105.20     105.45     4.70        3.25        No
MH-001       100.80     101.15     5.80        3.65        No
OUT-001      85.50      85.75      0.50        2.10        No

CONDUIT RESULTS
------------------------------------------------------------------------
Conduit ID   Flow (cfs)  Velocity   Depth (ft)  Capacity %  Froude #    Regime
------------------------------------------------------------------------
P-001        2.80        3.25       0.65        45.2%       0.85        Subcritical
P-002        5.60        3.65       0.85        58.7%       0.92        Subcritical
```

### JSON Output Format

```json
{
  "timestamp": "2024-11-26T10:30:00Z",
  "scenarioName": "Design Storm",
  "nodeResults": [
    {
      "nodeId": "IN-001",
      "hgl": 105.20,
      "egl": 105.45,
      "depth": 4.70,
      "velocity": 3.25,
      "flooding": false
    }
  ],
  "conduitResults": [
    {
      "conduitId": "P-001",
      "flow": 2.80,
      "velocity": 3.25,
      "depth": 0.65,
      "capacityUsed": 0.452,
      "froudeNumber": 0.85
    }
  ],
  "violations": []
}
```

## Troubleshooting

### "No IDF curve found for return period X years"

**Problem:** The IDF curves file doesn't contain the requested return period.

**Solution:** Check that `idf_curves.csv` includes rows with `return_period` matching the `--return-period` value (e.g., 10, 25, 50, 100).

### "Failed to parse nodes file: missing field"

**Problem:** CSV file is missing a required column.

**Solution:** Ensure the CSV has all required columns. See "CSV File Format Requirements" above.

### "Network validation failed: disconnected components"

**Problem:** Some nodes are not connected to the outfall.

**Solution:** Check that all conduits reference valid node IDs and that there's a continuous path from all inlets to an outfall.

### "Warning - could not interpolate intensity for Tc=X min"

**Problem:** Time of concentration is outside the range of IDF curve durations.

**Solution:**
- Ensure IDF curves include short durations (5, 10 min) for small areas
- Include long durations (60, 120, 360 min) for large areas
- The CLI will fall back to `--intensity` value when interpolation fails

## Integration with Other Tools

### Using with ATLAS14 Utility

Generate IDF curves for any US location:

```bash
# Step 1: Fetch NOAA data
atlas14_fetch --lat 34.0522 --lon -118.2437 \
              --return-periods "2,5,10,25,50,100" \
              --durations "5,10,15,30,60,120" \
              --output la_idf.csv

# Step 2: Run analysis
hec22 --nodes nodes.csv --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves la_idf.csv \
      --return-period 25 \
      --output analysis_25yr.txt
```

### Exporting for GIS

Use JSON output for GIS integration:

```bash
hec22 --nodes nodes.csv --conduits conduits.csv \
      --drainage-areas drainage_areas.csv \
      --idf-curves idf_curves.csv \
      --format json --output network.json

# Then convert JSON to GeoJSON (custom script)
python convert_to_geojson.py network.json > network.geojson
```

## Design Criteria Checking

The CLI automatically checks for design violations:

- **Flooding:** HGL above rim elevation
- **Capacity:** Pipe flow exceeds capacity
- **Velocity:** Flow velocity outside acceptable range (2.5-15 ft/s)

Violations are reported at the end of the output:

```
DESIGN VIOLATIONS
========================================
[ERROR] HGL violation at MH-001: HGL at 110.85 ft is 0.65 ft above rim elevation
[WARNING] Capacity violation at P-003: Flow at 125% of capacity
```

## See Also

- **[Templates README](../../examples/complete_network/README.md)** - CSV file format reference
- **[ATLAS14 Utility](../../docs/ATLAS14_UTILITY.md)** - Fetching NOAA rainfall data
- **[Component Definitions](component_definitions.md)** - Detailed data model documentation
- **[Design Workflow](design_workflow.md)** - Complete design process guidance
