# CSV Templates for HEC-22 Data Input

This directory contains example CSV files that demonstrate the format required for importing drainage network data.

## File Descriptions

### nodes.csv
Defines network nodes (inlets, junctions/manholes, outfalls).

**Required Columns:**
- `id` - Unique node identifier
- `type` - Node type: "inlet", "junction" (or "manhole"), or "outfall"
- `invert_elev` - Invert elevation (ft)

**Optional Columns:**
- `rim_elev` - Rim/ground elevation (ft) - required for inlets and junctions
- `x` - X coordinate (ft)
- `y` - Y coordinate (ft)
- `diameter` - Junction diameter (ft) - for junctions only
- `inlet_type` - Inlet type: "grate", "curb", "combination", or "slotted" - for inlets only
- `boundary_condition` - Boundary condition: "free", "normal", or "fixed" - for outfalls only

### conduits.csv
Defines pipes and gutters connecting nodes.

**Required Columns:**
- `id` - Unique conduit identifier
- `from_node` - Upstream node ID
- `to_node` - Downstream node ID
- `length` - Conduit length (ft)

**Required for Pipes:**
- `diameter` - Pipe diameter (inches)

**Required for Gutters:**
- `cross_slope` - Cross slope (ft/ft)
- `long_slope` - Longitudinal slope (ft/ft)

**Optional Columns:**
- `type` - Conduit type: "pipe" (default) or "gutter"
- `slope` - Slope (ft/ft) - used if `long_slope` not provided
- `manning_n` - Manning's roughness coefficient
- `material` - Pipe material: "RCP", "CMP", "PVC", "HDPE"

### drainage_areas.csv
Defines drainage subcatchments and their properties.

**Required Columns:**
- `id` - Unique drainage area identifier
- `area` - Drainage area (acres)
- `runoff_coef` - Runoff coefficient (0.0-1.0)
- `time_of_conc` - Time of concentration (minutes)
- `outlet_node` - Node ID where this area drains to

**Optional Columns:**
- `land_use` - Land use type: "Commercial", "Industrial", "Residential", "Open Space", "Transportation", "Agricultural", or "Mixed"

### gutter_parameters.csv
Defines gutter and curb properties at inlet locations.

**Required Columns:**
- `node_id` - Node ID (must be an inlet)
- `cross_slope` - Gutter cross slope (ft/ft)
- `long_slope` - Gutter longitudinal slope (ft/ft)

**Optional Columns:**
- `curb_height` - Curb height (inches)
- `gutter_width` - Gutter width (ft)
- `manning_n` - Manning's n (default: 0.016 for concrete)
- `depression` - Local depression depth (inches)
- `depression_width` - Depression width (ft)

## Usage

### Loading CSV Files (Rust)

```rust
use hec22::csv;

// Parse nodes
let nodes = csv::parse_nodes_csv("templates/nodes.csv")?;

// Parse conduits
let conduits = csv::parse_conduits_csv("templates/conduits.csv")?;

// Parse drainage areas
let areas = csv::parse_drainage_areas_csv("templates/drainage_areas.csv")?;

// Parse gutter parameters
let gutter_params = csv::parse_gutter_parameters_csv("templates/gutter_parameters.csv")?;
```

### Future CLI Usage

```bash
# Solve network from CSV files
hec22 solve \
  --nodes templates/nodes.csv \
  --conduits templates/conduits.csv \
  --areas templates/drainage_areas.csv \
  --output results.txt

# With gutter analysis
hec22 solve \
  --nodes templates/nodes.csv \
  --conduits templates/conduits.csv \
  --areas templates/drainage_areas.csv \
  --gutters templates/gutter_parameters.csv \
  --output results.txt
```

## Tips

- **Excel/Google Sheets**: Create your data in a spreadsheet, then export as CSV
- **Required vs Optional**: Only required columns must be present; optional columns can be omitted or left blank
- **Units**: All elevations and lengths in feet (US customary)
- **Connectivity**: Every `from_node` and `to_node` in conduits must reference a valid node `id`
- **Drainage areas**: Each `outlet_node` must reference a valid node `id`

## Example Workflow

1. **Create your network in Excel**:
   - One sheet for nodes
   - One sheet for conduits
   - One sheet for drainage areas
   - One sheet for gutter parameters (if needed)

2. **Export each sheet as CSV**:
   - Save as `project_nodes.csv`, `project_conduits.csv`, etc.

3. **Run analysis** (future CLI):
   ```bash
   hec22 solve --nodes project_nodes.csv --conduits project_conduits.csv --areas project_areas.csv
   ```

4. **Review results**:
   - Check HGL violations
   - Check gutter spread
   - Adjust design as needed

## Design Standards Reference

### Typical Manning's n Values
- RCP (Reinforced Concrete Pipe): 0.013
- CMP (Corrugated Metal Pipe): 0.024
- PVC/HDPE (smooth): 0.011
- Concrete gutter: 0.016

### Typical Runoff Coefficients
- Commercial/Industrial: 0.70-0.95
- Residential (dense): 0.50-0.70
- Residential (low density): 0.30-0.50
- Lawns/Open space: 0.10-0.30

### Typical Gutter Cross Slopes
- Standard pavement: 0.02 (2%)
- Steep cross slope: 0.03-0.04 (3-4%)
- ADA-compliant: 0.02 maximum (2%)

### Minimum Pipe Sizes
- Storm drain laterals: 15-18 inches
- Storm mains: 18-24 inches
- Driveway culverts: 12 inches minimum
