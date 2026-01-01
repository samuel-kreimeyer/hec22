# Storm Drainage Design Workflow

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual

## Overview

This document outlines the step-by-step process for designing a highway storm drainage system following HEC-22 methodology.

---

## Phase 1: Preliminary Data Collection

### 1.1 Site Information
- [ ] Topographic survey data
- [ ] Roadway alignment and profile
- [ ] Existing drainage features
- [ ] Soil types and infiltration rates
- [ ] Utility locations

### 1.2 Hydrologic Data
- [ ] Design storm frequency (10-yr, 25-yr, 50-yr, 100-yr)
- [ ] Local rainfall data (IDF curves)
- [ ] Watershed boundaries
- [ ] Land use and surface types
- [ ] Existing drainage patterns

### 1.3 Regulatory Requirements
- [ ] Local drainage criteria
- [ ] Environmental permits required
- [ ] Outfall approvals
- [ ] Floodplain regulations
- [ ] Water quality requirements

---

## Phase 2: Hydrologic Analysis

### 2.1 Delineate Drainage Areas
1. Identify high points and ridges
2. Delineate subbasin boundaries
3. Determine flow paths
4. Identify inlet locations
5. Calculate drainage area for each inlet

### 2.2 Determine Runoff Coefficients
1. Identify surface types in each subbasin
2. Assign C values from rational_method.md
3. Calculate composite C for mixed surfaces
4. Apply frequency adjustment if needed
5. Document assumptions

**Example Table:**
| Subarea | Surface Type | Area (ac) | C Value |
|---------|--------------|-----------|---------|
| A1 | Pavement | 0.30 | 0.85 |
| A1 | Lawn | 0.20 | 0.22 |
| A1 | **Composite** | **0.50** | **0.61** |

### 2.3 Calculate Time of Concentration
1. Identify hydraulically most distant point
2. Calculate overland flow time (Ti)
3. Calculate channel/gutter flow time
4. Calculate pipe flow time for downstream points
5. Sum all components: Tc = Ti + Tt
6. Check minimum Tc (typically 5-10 min)

### 2.4 Determine Design Rainfall Intensity
1. Obtain local IDF curves
2. Enter with design storm frequency and Tc
3. Read rainfall intensity i (in/hr)
4. Document source of IDF data

### 2.5 Calculate Design Flow Rates
For each inlet and system node:
```
Q = C × i × A
```

Create summary table:
| Node | Area (ac) | C | Tc (min) | i (in/hr) | Q (cfs) |
|------|-----------|---|----------|-----------|---------|
| IN-1 | 0.50 | 0.61 | 10 | 5.2 | 1.59 |
| IN-2 | 0.75 | 0.70 | 12 | 4.8 | 2.52 |

---

## Phase 3: Surface Drainage Design

### 3.1 Gutter Flow Analysis
1. Determine pavement cross slope Sx
2. Determine longitudinal slope SL
3. Select Manning's n for gutter
4. Calculate spread T for design flow
5. Verify spread is within allowable limits
6. Calculate depth at curb

**Reference:** gutter_flow.md

### 3.2 Inlet Design - On Grade
1. Select inlet type (grate, curb-opening, combination)
2. Calculate required length Lt for 100% interception
3. Select actual inlet length L
4. Calculate inlet efficiency E
5. Calculate intercepted flow Qi
6. Calculate bypass flow Qb
7. Apply clogging factor
8. Verify spread and depth are acceptable

**Reference:** inlet_design.md

### 3.3 Inlet Design - Sag Locations
1. Identify sag points (low points)
2. Calculate total inflow to sag
3. Select inlet type and size
4. Calculate weir capacity
5. Calculate orifice capacity
6. Design capacity = min(weir, orifice) × clogging factor
7. Verify capacity > design flow
8. Determine ponding depth if overtopped

### 3.4 Inlet Spacing
1. Start at upstream end
2. Calculate flow intercepted by each inlet
3. Add bypass from upstream inlet
4. Continue downstream, accumulating flows
5. Verify spread limits at all locations
6. Adjust inlet sizes or spacing as needed

---

## Phase 4: Storm Sewer System Design

### 4.1 Layout Storm Sewer Network
1. Connect inlets to manholes/junctions
2. Route pipes along right-of-way
3. Minimize number of bends
4. Plan for reasonable cover depths
5. Identify outfall location(s)

### 4.2 Establish Pipe Inverts
1. Start at outfall, work upstream
2. Set outfall invert elevation
3. Determine minimum cover requirements
4. Set upstream invert: Invert_up = Invert_dn + S × L
5. Verify adequate depth at each inlet
6. Check for conflicts with utilities

### 4.3 Preliminary Pipe Sizing
For each conduit:
1. Determine design flow Q (accumulated from upstream)
2. Select trial pipe diameter D
3. Determine slope S from inverts
4. Verify minimum slope for self-cleansing
5. Calculate capacity using Manning's equation
6. Verify capacity > design flow
7. Check velocity limits (2.5 ft/s < V < 10 ft/s)

**Reference:** manning_equation.md

### 4.4 Create System Tables

**Nodes Table:**
| Node | Type | Rim El | Inv El | Notes |
|------|------|--------|--------|-------|
| ... | ... | ... | ... | ... |

**Conduits Table:**
| Pipe | From | To | D (in) | L (ft) | n | Slope | Up Inv | Dn Inv |
|------|------|----|----|--------|---|-------|--------|--------|
| ... | ... | ... | ... | ... | ... | ... | ... | ... |

---

## Phase 5: Hydraulic Analysis

### 5.1 Hydraulic Grade Line (HGL) Analysis
1. Start at outfall with tailwater elevation
2. Calculate losses through each conduit:
   - Friction loss (Manning's equation)
   - Entrance loss
   - Exit loss
   - Bend loss
3. Calculate HGL at upstream end
4. Check for surcharge (HGL > crown)
5. Verify HGL < rim elevation at all nodes

### 5.2 Energy Grade Line (EGL) Analysis
```
EGL = HGL + V²/(2g)
```

### 5.3 Iteration and Refinement
1. Identify problem areas:
   - Insufficient capacity
   - Excessive velocity
   - Surcharged manholes
   - HGL above ground
2. Modify design:
   - Increase pipe size
   - Adjust slopes
   - Add relief pipes
   - Lower inverts
3. Re-analyze until satisfactory

---

## Phase 6: Special Considerations

### 6.1 Junction Losses
Apply appropriate loss coefficients:
- Through flow: K = 0.05
- 45° bend: K = 0.10
- 90° bend: K = 0.15
- Multiple inlets: K = 0.20

### 6.2 Outlet Protection
Required when:
- Velocity > 5 ft/s at outfall
- Discharge to erodible channel
- Discharge near structures

Design:
- Riprap size and extent
- Concrete apron
- Energy dissipation basin

### 6.3 Scour Protection
At inlet outfalls:
- Trash racks
- Riprap around structure
- Concrete collars

### 6.4 Maintenance Access
- Manholes at changes in direction
- Manholes at changes in slope
- Maximum spacing: 400-500 ft
- All dead ends require cleanouts

---

## Phase 7: Documentation

### 7.1 Design Calculations
- [ ] Drainage area maps
- [ ] Runoff coefficient calculations
- [ ] Time of concentration calculations
- [ ] Rainfall intensity documentation
- [ ] Flow rate calculations
- [ ] Gutter flow analysis
- [ ] Inlet design calculations
- [ ] Pipe sizing calculations
- [ ] HGL/EGL profiles

### 7.2 Design Drawings
- [ ] Plan view showing:
  - Inlet locations
  - Pipe alignments
  - Manhole locations
  - Drainage area boundaries
- [ ] Profile view showing:
  - Ground line
  - Pipe inverts
  - HGL/EGL
  - Rim elevations

### 7.3 Design Tables
- [ ] Inlet schedule
- [ ] Manhole schedule
- [ ] Pipe schedule
- [ ] Flow summary table

### 7.4 Technical Specifications
- [ ] Pipe materials and standards
- [ ] Inlet standards
- [ ] Manhole standards
- [ ] Bedding and backfill requirements
- [ ] Testing requirements

---

## Quality Control Checklist

### Hydrologic Analysis
- [ ] Drainage areas verified
- [ ] C values appropriate and documented
- [ ] Tc calculations reasonable
- [ ] IDF curves from approved source
- [ ] Design flows verified

### Hydraulic Analysis
- [ ] All pipes meet minimum slope
- [ ] Velocities within limits (2.5 - 10 ft/s)
- [ ] No surcharged manholes (or approved)
- [ ] HGL below ground at all points
- [ ] Adequate cover over all pipes
- [ ] No utility conflicts

### Design Standards
- [ ] Minimum pipe sizes met
- [ ] Inlet types appropriate for location
- [ ] Manhole spacing acceptable
- [ ] Outlet protection provided
- [ ] Maintenance access provided

### Documentation
- [ ] All assumptions documented
- [ ] Calculations clear and reproducible
- [ ] Drawings complete and accurate
- [ ] Specifications appropriate

---

## Common Design Iterations

### Issue: Pipe Surcharged
**Solutions:**
1. Increase pipe diameter
2. Increase pipe slope (lower upstream invert)
3. Provide dual pipes
4. Add relief system

### Issue: Velocity Too Low (< 2.5 ft/s)
**Solutions:**
1. Decrease pipe size (if capacity permits)
2. Increase slope
3. Accept lower velocity with maintenance plan

### Issue: Velocity Too High (> 10 ft/s)
**Solutions:**
1. Increase pipe diameter
2. Decrease slope (if feasible)
3. Provide energy dissipation

### Issue: Spread Exceeds Limit
**Solutions:**
1. Add more inlets
2. Use larger/longer inlets
3. Add local depression at inlet
4. Use combination inlets

### Issue: Inadequate Cover
**Solutions:**
1. Use smaller pipe
2. Reroute alignment
3. Provide structural support
4. Adjust roadway profile (major projects)

---

## Design Software Tools

While this reference supports manual calculations, the following software can automate the hydraulic analysis:

- **StormCAD**: Commercial software by Bentley
- **HydroCAD**: Commercial hydrologic modeling
- **EPA SWMM**: Free dynamic modeling software
- **CivilStorm**: Bentley's comprehensive tool
- **AutoCAD Civil 3D**: With Storm and Sanitary Analysis

**Note:** All software should follow HEC-22 methodology. Manual verification of key calculations is recommended.

---

## Summary

The storm drainage design process follows these major steps:

1. **Collect Data** → 2. **Hydrologic Analysis** → 3. **Surface Drainage** → 4. **Storm Sewer Design** → 5. **Hydraulic Analysis** → 6. **Refinement** → 7. **Documentation**

Each step builds upon the previous, and iteration is normal. A well-designed system balances hydraulic performance, constructability, cost, and maintainability.
