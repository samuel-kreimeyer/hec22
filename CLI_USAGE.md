# HEC-22 CLI Tool - User Guide

## Overview

The HEC-22 CLI tool performs hydraulic analysis of storm sewer networks using the FHWA HEC-22 methodology. It accepts CSV files as input and produces comprehensive hydraulic analysis reports.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/samuel-kreimeyer/hec22.git
cd hec22

# Build the project
cargo build --release

# The binary will be at target/release/hec22
```

### Running with Cargo

You can run the tool directly with cargo without building:

```bash
cargo run -- --nodes nodes.csv --conduits conduits.csv --drainage-areas drainage_areas.csv --intensity 4.0
```

## Quick Start

The tool requires at minimum two CSV files: nodes and conduits. A third file for drainage areas is optional but recommended for complete hydraulic analysis.

```bash
# Basic usage with template files
cargo run -- \
  --nodes templates/nodes.csv \
  --conduits templates/conduits.csv \
  --drainage-areas templates/drainage_areas.csv \
  --intensity 4.0
```

## Command-Line Options

### Required Arguments

- `--nodes, -n <FILE>` - Path to nodes CSV file
  - Defines inlets, junctions, and outfalls in your network
  - Must include: id, type, invert_elev

- `--conduits, -c <FILE>` - Path to conduits CSV file
  - Defines pipes, gutters, and channels connecting nodes
  - Must include: id, from_node, to_node, length

### Optional Arguments

- `--drainage-areas, -a <FILE>` - Path to drainage areas CSV file
  - Defines catchments contributing runoff to inlets
  - If omitted, analysis assumes zero inflow (useful for pipe capacity checks)

- `--intensity, -i <VALUE>` - Rainfall intensity (default: 4.0)
  - Units: in/hr for US customary, mm/hr for SI metric
  - Used with drainage areas for rational method flow calculation

- `--units, -u <SYSTEM>` - Unit system (default: us)
  - `us` - US Customary (ft, cfs, in/hr)
  - `si` - SI Metric (m, m³/s, mm/hr)

- `--output, -o <FILE>` - Output file path
  - If not specified, results print to stdout (terminal)

- `--format, -f <FORMAT>` - Output format (default: text)
  - `text` - Human-readable report with tables
  - `json` - Structured JSON output for further processing
  - `csv` - Two CSV files (nodes and conduits results)

### Help and Version

```bash
# Show help message
cargo run -- --help

# Show version
cargo run -- --version
```

## Input File Formats

### nodes.csv

Defines the network nodes. Each row is one node.

**Example:**
```csv
id,type,invert_elev,rim_elev,x,y,diameter,inlet_type,boundary_condition
IN-001,inlet,130.0,134.0,0.0,0.0,,combination,
MH-001,junction,120.0,125.0,450.0,0.0,4.0,,
OUT-001,outfall,115.0,,700.0,0.0,,,free
```

**Required Columns:**
- `id` - Unique identifier (e.g., IN-001, MH-001, OUT-001)
- `type` - One of: inlet, junction, manhole, outfall
- `invert_elev` - Invert elevation (ft or m)

**Optional Columns:**
- `rim_elev` - Rim elevation (required for inlets/junctions)
- `x`, `y` - Coordinates for mapping
- `diameter` - Junction diameter in ft (for junctions)
- `inlet_type` - grate, curb, combination, slotted (for inlets)
- `boundary_condition` - free, normal, fixed (for outfalls)

### conduits.csv

Defines the pipes and gutters. Each row is one conduit.

**Example:**
```csv
id,from_node,to_node,type,diameter,length,slope,manning_n,material
P-001,IN-001,MH-001,pipe,18.0,200.0,0.024,0.013,RCP
P-002,IN-002,MH-001,pipe,24.0,250.0,0.020,0.013,RCP
```

**Required Columns:**
- `id` - Unique identifier (e.g., P-001)
- `from_node` - Upstream node ID
- `to_node` - Downstream node ID
- `length` - Length in feet (or meters)

**Conditional Columns:**
- `type` - pipe (default), gutter, or channel
- `diameter` - Pipe diameter in inches (required for pipes)
- `cross_slope` - Cross slope ft/ft (required for gutters)
- `long_slope` - Longitudinal slope ft/ft (for gutters)

**Optional Columns:**
- `slope` - Slope in ft/ft (computed from node elevations if omitted)
- `manning_n` - Manning's roughness (defaults based on material)
- `material` - RCP, CMP, PVC, HDPE (sets default Manning's n)

### drainage_areas.csv

Defines drainage catchments. Each row is one drainage area.

**Example:**
```csv
id,area,runoff_coef,time_of_conc,outlet_node,land_use
DA-001,1.5,0.80,15.0,IN-001,commercial
DA-002,1.0,0.75,12.0,IN-002,residential
```

**Required Columns:**
- `id` - Unique identifier (e.g., DA-001)
- `area` - Drainage area in acres (or hectares for SI)
- `runoff_coef` - Rational method C value (0.0 to 1.0)
- `time_of_conc` - Time of concentration in minutes
- `outlet_node` - Node where this area drains to

**Optional Columns:**
- `land_use` - commercial, industrial, residential, etc.

## Output Formats

### Text Output (default)

Human-readable tables with node and conduit results, plus design violations.

```bash
cargo run -- -n templates/nodes.csv -c templates/conduits.csv -a templates/drainage_areas.csv -i 4.0
```

Example output:
```
================================================================================
HYDRAULIC ANALYSIS RESULTS
================================================================================

NODE RESULTS
--------------------------------------------------------------------------------
Node ID      HGL (ft)   EGL (ft)   Depth (ft) Velocity   Flooding
--------------------------------------------------------------------------------
IN-001           106.37     107.48       0.00       0.00        YES
MH-001            99.22     100.56       0.00       0.00         No
OUT-001           85.00      85.00       0.00       0.00         No

CONDUIT RESULTS
----------------------------------------------------------------------------------------------------
Conduit ID   Flow (cfs) Velocity   Depth (ft) Capacity %   Froude #   Regime
----------------------------------------------------------------------------------------------------
P-001              2.55       8.44       0.34        11.3%       0.00 N/A

================================================================================
DESIGN VIOLATIONS
================================================================================

[ERROR] HGL violation at IN-001: HGL at 106.37 ft is 0.87 ft above rim elevation
```

### JSON Output

Structured JSON for programmatic processing or integration with other tools.

```bash
cargo run -- \
  -n templates/nodes.csv \
  -c templates/conduits.csv \
  -a templates/drainage_areas.csv \
  -i 4.0 \
  --format json \
  --output results.json
```

### CSV Output

Generates two CSV files: `<basename>.nodes.csv` and `<basename>.conduits.csv`

```bash
cargo run -- \
  -n templates/nodes.csv \
  -c templates/conduits.csv \
  -a templates/drainage_areas.csv \
  -i 4.0 \
  --format csv \
  --output results
```

Produces:
- `results.nodes.csv` - Node results (HGL, EGL, depth, velocity, flooding)
- `results.conduits.csv` - Conduit results (flow, velocity, capacity, Froude number)

## Understanding the Results

### Node Results

- **HGL (Hydraulic Grade Line)** - Water surface elevation (pressure head + elevation)
- **EGL (Energy Grade Line)** - HGL + velocity head
- **Depth** - Flow depth in the node
- **Velocity** - Flow velocity at the node
- **Flooding** - YES if HGL exceeds rim elevation (water backs up)

### Conduit Results

- **Flow** - Flow rate in cfs (or m³/s)
- **Velocity** - Average flow velocity in fps (or m/s)
- **Depth** - Flow depth in the pipe/gutter
- **Capacity %** - Percentage of full pipe capacity being used
- **Froude Number** - Flow regime indicator (< 1 = subcritical, > 1 = supercritical)
- **Regime** - Flow classification (subcritical, critical, supercritical)

### Design Violations

The tool checks for common design issues:

- **HGL violation** - Water level exceeds rim elevation (flooding risk)
- **Velocity violation** - Flow too slow (sediment) or too fast (erosion)
- **Capacity violation** - Pipe over 100% capacity
- **Cover violation** - Insufficient cover depth
- **Spread violation** - Gutter spread exceeds limit

## Example Workflows

### 1. Check Existing Network Capacity

Analyze an existing network without runoff to check pipe capacities:

```bash
cargo run -- \
  --nodes existing_nodes.csv \
  --conduits existing_conduits.csv \
  --output capacity_check.txt
```

### 2. Design Storm Analysis

Run a 10-year, 24-hour storm analysis:

```bash
cargo run -- \
  --nodes design_nodes.csv \
  --conduits design_conduits.csv \
  --drainage-areas catchments.csv \
  --intensity 5.2 \
  --output 10yr_storm.txt
```

### 3. Batch Analysis with Multiple Intensities

Create a script to analyze multiple storm events:

```bash
#!/bin/bash
for intensity in 2.5 4.0 5.5 7.0; do
  cargo run -- \
    -n nodes.csv \
    -c conduits.csv \
    -a areas.csv \
    -i $intensity \
    -f json \
    -o "results_${intensity}iph.json"
done
```

### 4. Export Results to Spreadsheet

Generate CSV output for further analysis in Excel:

```bash
cargo run -- \
  -n nodes.csv \
  -c conduits.csv \
  -a areas.csv \
  -i 4.0 \
  --format csv \
  --output analysis_results
```

## Design Guidelines

### Runoff Coefficients (C)

| Land Use | Typical C |
|----------|-----------|
| Commercial downtown | 0.70-0.95 |
| Industrial | 0.50-0.90 |
| Residential (dense) | 0.40-0.60 |
| Residential (suburban) | 0.30-0.50 |
| Parks/open space | 0.10-0.25 |
| Pavement (asphalt/concrete) | 0.85-0.95 |

### Manning's n Values

| Material | Manning's n |
|----------|-------------|
| RCP (Reinforced Concrete) | 0.013 |
| CMP (Corrugated Metal) | 0.024 |
| PVC | 0.010 |
| HDPE | 0.012 |
| Concrete gutter | 0.016 |

### Design Criteria

- **Minimum pipe slope:** 0.5% (0.005 ft/ft) to prevent sediment buildup
- **Maximum pipe slope:** 10% (0.10 ft/ft) to prevent erosion
- **Minimum velocity:** 2.0 fps (prevents sediment deposition)
- **Maximum velocity:** 15.0 fps (prevents erosion and pipe damage)
- **Capacity usage:** Target < 80% for design storms
- **Cover depth:** Minimum 1.5-2.0 ft under traffic, 1.0 ft elsewhere

## Troubleshooting

### Error: "Network has no outfall nodes"

**Problem:** Your network doesn't have any nodes with `type=outfall`.

**Solution:** Add at least one outfall node where water can discharge.

### Error: "Pipe slope cannot be determined"

**Problem:** The conduit doesn't have enough information to compute slope.

**Solution:** The tool now auto-computes slopes from node invert elevations. Ensure your node invert elevations are correct and decrease in the flow direction.

### Error: "Failed to parse CSV file"

**Problem:** CSV file has formatting issues or missing required columns.

**Solution:** Check that:
- File has header row with correct column names
- Required columns are present
- No extra spaces in column names
- Values are numeric where expected

### Warning: HGL violations

**Problem:** Hydraulic grade line exceeds rim elevation (flooding).

**Solutions:**
- Increase pipe diameter
- Increase pipe slope
- Add additional inlets upstream
- Add detention/retention storage

### Warning: Capacity exceeded

**Problem:** Pipe is flowing over 100% of full capacity.

**Solutions:**
- Use larger diameter pipe
- Add parallel pipes
- Reduce contributing drainage area
- Add flow control structures

## Advanced Usage

### Using the Built Binary

After building, you can use the binary directly without cargo:

```bash
# Build release version (optimized)
cargo build --release

# Run the binary
./target/release/hec22 \
  --nodes nodes.csv \
  --conduits conduits.csv \
  --drainage-areas areas.csv \
  --intensity 4.0
```

### Integration with Other Tools

The JSON output can be processed with tools like `jq`:

```bash
# Extract all nodes with flooding
cargo run -- -n nodes.csv -c conduits.csv -a areas.csv -f json | \
  jq '.node_results[] | select(.flooding == true)'

# Get maximum HGL elevation
cargo run -- -n nodes.csv -c conduits.csv -a areas.csv -f json | \
  jq '[.node_results[].hgl] | max'
```

## Support and Documentation

- **Full documentation:** See `templates/README.md` for detailed CSV format specifications
- **HEC-22 Manual:** https://www.fhwa.dot.gov/engineering/hydraulics/pubs/10009/10009.pdf
- **GitHub Repository:** https://github.com/samuel-kreimeyer/hec22
- **Issues:** Report bugs at https://github.com/samuel-kreimeyer/hec22/issues

## Version History

### Version 0.1.0 (Current)

- Initial CLI implementation
- CSV input parsing for nodes, conduits, and drainage areas
- Rational method flow computation
- HGL/EGL solver with energy losses
- Text, JSON, and CSV output formats
- Design violation detection
- US Customary and SI Metric unit support
