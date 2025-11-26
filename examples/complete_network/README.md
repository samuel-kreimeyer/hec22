# Complete Network Example

This example demonstrates all the key features of the HEC-22 drainage analysis system:

## Features Demonstrated

### 1. Automatic Peak Flow Computation
- Drainage areas reference design storms via the `design_storm` column
- IDF curves provide rainfall intensity data for multiple return periods
- Peak flow is computed using: **Q = C × i × A**
  - **C** = runoff coefficient from drainage_areas.csv
  - **i** = intensity from IDF curve (at Tc duration and design storm return period)
  - **A** = drainage area

### 2. Various Pipe Shapes
The network includes all supported pipe shapes:
- **P-001**: 18" circular RCP (traditional storm sewer)
- **P-002**: 24"×38" elliptical RCP (limited vertical clearance)
- **P-003**: 24" circular RCP (traditional storm sewer)
- **P-004**: 36"×36" rectangular box culvert (high capacity)
- **P-005**: 58"×36" arch CMP (low profile, wide span)
- **P-006**: 72"×72" rectangular box culvert (main trunk line)

### 3. Node Geometry Options
The network includes both circular and rectangular manholes:
- **MH-001**: 4 ft diameter circular manhole (standard)
- **MH-002**: 6 ft × 6 ft rectangular manhole (junction of multiple large pipes)
- **MH-003**: 8 ft × 8 ft rectangular manhole (large junction structure)

### 4. Multiple Inlet Types
- **IN-001**: Grate inlet (street inlet)
- **IN-002**: Combination inlet (grate + curb opening for better efficiency)
- **IN-003**: Grate inlet

## Network Description

This example represents a typical urban drainage system:

```
IN-001 (Commercial, 1.2 ac, C=0.85)
  │ P-001 (18" circular)
  ├─> MH-001 (4 ft circular)
IN-002 (Mixed use, 2.5 ac, C=0.70)  │
  │ P-002 (24×38 elliptical)        │
  └────────────────────────────────>┘
      │ P-003 (24" circular)
      ├─> MH-002 (6×6 ft rectangular)
IN-003 (Residential, 1.8 ac, C=0.50) │
  │ P-004 (36×36 rectangular)        │
  └────────────────────────────────>┘
      │ P-005 (58×36 arch)
      ├─> MH-003 (8×8 ft rectangular)
          │ P-006 (72×72 rectangular)
          └─> OUT-001 (Free outfall)
```

## Peak Flow Calculations

For the 10-year design storm:

### DA-001 (Commercial)
- **A** = 1.2 acres
- **C** = 0.85
- **Tc** = 8.0 minutes
- **i** (from IDF curve, 10-yr, 8 min) ≈ 8.0 in/hr (interpolated between 5 and 10 min)
- **Q** = 0.85 × 8.0 × 1.2 = **8.16 cfs**

### DA-002 (Mixed Use)
- **A** = 2.5 acres
- **C** = 0.70
- **Tc** = 12.0 minutes
- **i** (from IDF curve, 10-yr, 12 min) ≈ 7.0 in/hr (interpolated between 10 and 15 min)
- **Q** = 0.70 × 7.0 × 2.5 = **12.25 cfs**

### DA-003 (Residential)
- **A** = 1.8 acres
- **C** = 0.50
- **Tc** = 15.0 minutes
- **i** (from IDF curve, 10-yr, 15 min) = 6.52 in/hr (exact match)
- **Q** = 0.50 × 6.52 × 1.8 = **5.87 cfs**

### Total Flow at Outfall
Combined flow = 8.16 + 12.25 + 5.87 = **26.28 cfs**

## Running the Analysis

To analyze this network, you would:

1. Load all CSV files
2. Build the drainage network
3. Apply the design storm (DS-10YR)
4. Compute peak flows using IDF curves and drainage area properties
5. Run hydraulic analysis to compute HGL/EGL profiles
6. Check for flooding and capacity issues

## Files

- **nodes.csv** - Network nodes (inlets, manholes, outfall)
- **conduits.csv** - Pipes with various shapes
- **drainage_areas.csv** - Subcatchments with design storm references
- **idf_curves.csv** - IDF data for return periods 2, 5, 10, 25, 50, 100 years
- **design_storms.csv** - Design storm definitions

## Key Insights

1. **Pipe Shape Selection**:
   - Use circular for standard conditions
   - Use elliptical when vertical space is limited
   - Use rectangular for high capacity or custom dimensions
   - Use arch for very low profile applications

2. **Manhole Geometry**:
   - Circular manholes are standard for single or simple junctions
   - Rectangular manholes provide more space for complex junctions with multiple large pipes

3. **IDF Curves**:
   - Provide intensity data across a range of durations and return periods
   - System interpolates between tabulated points
   - Critical for automatic peak flow computation using the rational method

4. **Design Storm Integration**:
   - Linking drainage areas to design storms enables automatic peak flow calculation
   - No need to manually compute intensity for each drainage area
   - Consistent application of return period across entire analysis
