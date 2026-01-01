# Bentonville, Arkansas - Complex Network Example

This example represents a large-scale highway drainage system design for Bentonville, Arkansas. It demonstrates a complex network configuration with multiple bypass inlets, various inlet types, and interconnected drainage paths.

## Network Characteristics

- **Location**: Bentonville, Arkansas (36.3719°N, 94.2027°W)
- **Elevation**: Base elevation ~1279 ft NAVD88
- **Network Size**:
  - 64 nodes (inlets, junctions, and outfalls)
  - 58 conduits (pipes and connections)
  - 64 drainage areas
- **Rainfall Data**: NOAA Atlas 14 Volume 9 Version 2 IDF curves

## Network Components

### Nodes (nodes.csv)
The network includes:
- **Curb inlets**: Both on-grade and sag configurations
- **Grate inlets**: Perpendicular bar configuration
- **Combination inlets**: Both grate and curb opening components
- **Junctions**: Pipe connection points
- **Outfalls**: Free outfall boundary conditions
- **Bypass configuration**: Many inlets bypass to downstream locations when capacity is exceeded

Station naming follows highway convention (e.g., "111+64 LT", "112+35 RT" for left/right sides).

### Conduits (conduits.csv)
Pipe network characteristics:
- **Circular pipes**: 18", 24", 36", 48", and 60" diameter
- **Arch pipes**: Various sizes (24"x36", 36"x59", 40"x65", 45"x73")
- **Material**: Reinforced Concrete Pipe (RCP)
- **Manning's n**: 0.013 throughout
- **Slopes**: Range from 0.005 to 0.173 ft/ft

### Drainage Areas (drainage_areas.csv)
- **Land use**: Commercial development
- **Runoff coefficient**: 0.8 (typical for commercial areas)
- **Time of concentration**: Ranges from 5.0 to 11.5 minutes
- **Area sizes**: 0.05 to 0.69 acres

### IDF Curves (idf_curves.csv)
NOAA Atlas 14 precipitation frequency estimates:
- **Return periods**: 1, 2, 5, 10, 25, 50, 100, 200, 500, and 1000 years
- **Durations**: 5 minutes to 24 hours (5, 10, 15, 30, 60, 120, 180, 360, 720, 1440 minutes)
- **Intensities**: In inches/hour
- **Source**: PF_Intensity_English_PDS.csv contains the original NOAA data with metadata

## Usage

To analyze this network with the HEC-22 tool:

```bash
# Example command (adjust based on actual CLI interface)
hec22 analyze \
  --nodes examples/bentonville_complex_network/nodes.csv \
  --conduits examples/bentonville_complex_network/conduits.csv \
  --drainage-areas examples/bentonville_complex_network/drainage_areas.csv \
  --idf-curves examples/bentonville_complex_network/idf_curves.csv \
  --return-period 10
```

## Key Features Demonstrated

1. **Bypass inlet chains**: Multiple inlets bypass to downstream locations (e.g., 112+35 LT → 111+64 LT → 111+65 RT)
2. **Sag inlet locations**: Critical low points that collect all incoming flow
3. **Mixed inlet types**: Combination of curb-only, grate-only, and combination inlets
4. **Variable pipe sizes**: Pipes increase in size as they move downstream
5. **Cross-drainage**: Pipes crossing between left and right sides of the roadway
6. **Multiple outfalls**: Three separate outfall points (200+19 RT, 203+48 RT, 216+90 RT)

## Design Criteria

This network uses typical highway drainage design criteria:
- **Clogging factor**: 15% (0.15) for inlet capacity reduction
- **Local depression**: 2 inches at inlet locations
- **Inlet spacing**: Variable based on hydraulic capacity requirements

## Data Source

These CSV files were recovered from the project's git history (commit f3b4ee4) and represent a real-world design scenario used for testing and validation of the HEC-22 drainage analysis tool.

## Notes

- Some nodes have missing coordinate data (x, y fields are empty)
- The PF_Intensity_English_PDS.csv file contains the raw NOAA Atlas 14 data with additional metadata
- Duplicate pipe entries exist in the conduits.csv (e.g., two pipes from 208+17 RT to 206+53 RT), which may represent different design alternatives or require consolidation
