# Visualization Examples

This directory contains example visualizations generated from the HEC-22 drainage analysis system using the template network in `templates/`.

## Files

### Network Plan View (`network_plan.svg`)
Top-down view of the drainage network showing:
- **Green circles**: Inlets (IN-001, IN-002)
- **Blue circles**: Junctions/Manholes (MH-001, MH-002)
- **Red circle**: Outfall (OUT-001)
- **Gray lines with arrows**: Pipe conduits showing flow direction

### Profile View with HGL/EGL (`profile_with_hgl_egl.svg`)
Elevation profile along the main drainage path (IN-001 → MH-001 → MH-002 → OUT-001) showing:
- **Black line (thick)**: Pipe invert elevations
- **Brown line**: Ground/rim elevations
- **Blue line**: Hydraulic Grade Line (HGL) - water surface elevation
- **Orange line**: Energy Grade Line (EGL) - total energy elevation
- **Black dots**: Node locations with labels
- **Station labels**: Cumulative distance along the profile

This visualization matches **HEC-22 Figure 9.6** and demonstrates:
- Energy losses through the system
- HGL violations at IN-001 and IN-002 (HGL above rim elevation = flooding!)
- Energy dissipation from upstream to downstream

### Interactive HTML Viewer (`network_viewer.html`)
Combined visualization with both network plan and profile views. Features:
- Pan and zoom controls
- Mouse drag to pan
- Download SVG button
- Network statistics
- **Open in a web browser** to interact with the visualization

## How to Generate

These visualizations were generated using:

```bash
./target/release/hec22 \
  --nodes templates/nodes.csv \
  --conduits templates/conduits.csv \
  --drainage-areas templates/drainage_areas.csv \
  --intensity 4.0 \
  --export-network-plan examples/visualizations/network_plan.svg \
  --export-profile examples/visualizations/profile_with_hgl_egl.svg \
  --export-html examples/visualizations/network_viewer.html
```

## Understanding the Profile View

The profile view is critical for hydraulic analysis:

1. **HGL (Blue line)**: Shows where the water surface is in the system
   - If HGL > Rim elevation → Flooding occurs
   - Slope of HGL indicates head loss through pipes

2. **EGL (Orange line)**: Shows total energy (pressure + velocity head)
   - Always above HGL
   - Vertical drop = energy loss at junctions
   - Slope indicates friction losses

3. **Violations in this example**:
   - IN-001: HGL = 106.37 ft, Rim = 105.50 ft → 0.87 ft of flooding!
   - IN-002: HGL = 103.69 ft, Rim = 103.20 ft → 0.49 ft of flooding!

## Technical Details

- **Network**: 5 nodes, 4 conduits
- **Analysis Method**: HEC-22 9-step HGL/EGL procedure (Chapter 9)
- **Flow Method**: Rational method (Q = C × i × A)
- **Rainfall Intensity**: 4.0 in/hr (fixed)
- **Pipe Material**: Reinforced Concrete Pipe (RCP), n = 0.013
- **Coordinate System**: US Customary units (feet, cfs)

## Viewing the Files

- **SVG files**: Open in any modern web browser or SVG-compatible application
- **HTML file**: Open `network_viewer.html` in a web browser for the interactive experience

## References

Based on:
- FHWA HEC-22 (4th Edition, 2024), Chapter 9: Storm Drain Conduits
- Figures 9.6 and 9.11: HGL/EGL Profile Examples
