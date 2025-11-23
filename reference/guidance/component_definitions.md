# Drainage System Components

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual
Similar to StormCAD and HydroCAD component definitions

## Overview

A storm drainage system consists of four primary components:

1. **Outfalls** - Discharge points from the system
2. **Junctions** - Connection points (manholes, junction boxes)
3. **Inlets** - Collection points for surface runoff
4. **Conduits** - Pipes and channels that convey flow

---

## 1. Outfalls

### Definition
An outfall is the terminal point of a drainage system where stormwater is discharged to a receiving water body, larger storm sewer system, or other approved discharge location.

### Key Characteristics

| Property | Description | Typical Values |
|----------|-------------|----------------|
| **Outfall Type** | Type of receiving system | Stream, river, lake, existing storm system, detention pond |
| **Invert Elevation** | Elevation at bottom of outfall pipe | Site-specific (ft or m) |
| **Tailwater Condition** | Water surface elevation at outfall | Critical depth, normal depth, fixed elevation |
| **Boundary Condition** | Hydraulic condition at outfall | Free outfall, submerged outfall |

### Design Considerations

- **Free Outfall**: Flow exits to atmosphere, controlled by critical depth
- **Submerged Outfall**: Downstream water level affects hydraulics
- **Tailwater Elevation**: May vary with flow in receiving system
- **Energy Dissipation**: May require outlet protection (riprap, concrete apron)
- **Environmental Permits**: Often require regulatory approval

### Required Input Data

```
Outfall ID:          Unique identifier
Description:         Name/location
Invert Elevation:    Bottom of pipe elevation (ft)
Tailwater Type:      Fixed, variable, critical depth
Tailwater Elevation: Water surface elevation if fixed (ft)
```

### Example

```
ID: OUT-001
Description: Discharge to Mill Creek
Invert Elevation: 100.50 ft
Tailwater Type: Normal depth in creek
Receiving Water: Mill Creek
```

---

## 2. Junctions (Manholes)

### Definition
A junction is a structure where two or more conduits connect, allowing for changes in pipe size, slope, or direction. Manholes are the most common type of junction.

### Types of Junctions

1. **Standard Manhole**: Circular structure with removable lid for maintenance
2. **Junction Box**: Rectangular or square structure
3. **Inlet with Pipe Connection**: Inlet serving as junction point
4. **Clean-out**: Small access point for maintenance

### Key Characteristics

| Property | Description | Typical Values |
|----------|-------------|----------------|
| **Junction Type** | Structure type | Manhole, junction box, inlet |
| **Rim Elevation** | Ground surface elevation | Site-specific (ft) |
| **Invert Elevation** | Bottom of lowest pipe | Site-specific (ft) |
| **Sump Depth** | Depth below lowest invert | 0.0 - 1.0 ft |
| **Diameter** | Size of structure | 4 ft (standard), 5-8 ft (larger flows) |
| **Maximum Flow Depth** | Water depth in structure | Calculated |
| **Energy Loss** | Head loss through junction | 0.05 - 0.25 ft |

### Energy Losses at Junctions

Junction losses account for:
- **Inlet losses**: Flow entering from lateral pipes
- **Bend losses**: Change in flow direction
- **Turbulence**: Mixing and flow separation

#### Loss Coefficient Method

```
HL = K × V²/(2g)
```

**Where:**
- HL = Head loss (ft)
- K = Loss coefficient (0.05 - 0.25 typical)
- V = Velocity in downstream pipe (ft/s)
- g = 32.2 ft/s²

### Standard Manhole Dimensions

| Pipe Diameter | Minimum Manhole Diameter |
|---------------|--------------------------|
| ≤ 24 inches | 4 feet |
| 27 - 36 inches | 5 feet |
| 42 - 60 inches | 6 feet |
| > 60 inches | 8 feet or box structure |

### Required Input Data

```
Junction ID:         Unique identifier
Description:         Name/location
Rim Elevation:       Ground surface elevation (ft)
Invert Elevation:    Bottom of structure (ft)
Sump Depth:          Additional depth below lowest pipe (ft)
Structure Type:      Manhole, junction box, etc.
Diameter:            Structure diameter (ft)
Loss Coefficient:    K value for energy loss
```

### Example

```
ID: MH-101
Description: Manhole at Sta 1+50
Rim Elevation: 125.30 ft
Invert Elevation: 118.50 ft
Sump Depth: 0.5 ft
Structure Type: Standard Manhole
Diameter: 4 ft
Loss Coefficient: 0.15
```

---

## 3. Inlets

### Definition
An inlet is a structure that collects surface runoff and introduces it into the storm drainage system. Inlets are the primary interface between surface drainage and the subsurface pipe network.

### Inlet Types

1. **Grate Inlet**: Opening covered by grate
2. **Curb-Opening Inlet**: Vertical opening in curb
3. **Combination Inlet**: Both grate and curb opening
4. **Slotted Drain Inlet**: Continuous slot
5. **Area Inlet**: Large grated area for parking lots

### Location Types

1. **On-Grade**: Located on continuous slope, flow bypasses if not intercepted
2. **In-Sag**: Located at low point, intercepts all flow

### Key Characteristics

| Property | Description | Typical Values |
|----------|-------------|----------------|
| **Inlet Type** | Configuration | Grate, curb-opening, combination |
| **Location** | On-grade or sag | On-grade, sag |
| **Rim Elevation** | Surface elevation | Site-specific (ft) |
| **Invert Elevation** | Bottom of outlet pipe | Site-specific (ft) |
| **Grate Length** | Length of grate | 2 - 4 ft |
| **Grate Width** | Width of grate | 1.5 - 2.0 ft |
| **Curb Height** | Height of opening | 4 - 6 inches |
| **Curb Length** | Length of opening | 5 - 10 ft |
| **Clogging Factor** | Reduction for debris | 0.50 - 0.90 |
| **Local Depression** | Additional depth at inlet | 0 - 2 inches |
| **Depression Width** | Width of depression | 1.5 - 2.0 ft |

### Capacity Equations

See `inlet_design.md` for detailed equations.

### Standard Inlet Configurations

#### Grate Inlet
- Width: 1.5 - 2.0 ft
- Length: 2 - 4 ft
- Bar spacing: 1.5 - 2.0 inches
- AASHTO M-306 (highway grates) or local standards

#### Curb-Opening Inlet
- Height: 4 - 6 inches
- Length: 5 - 10 ft
- Throat width: 1.5 - 2.0 ft

#### Combination Inlet
- Grate: 2 ft × 2 ft typical
- Curb opening: 5 ft length typical
- Most effective for high flow rates

### Required Input Data

```
Inlet ID:            Unique identifier
Description:         Name/location
Inlet Type:          Grate, curb-opening, combination
Location Type:       On-grade, sag
Rim Elevation:       Surface elevation (ft)
Invert Elevation:    Outlet pipe elevation (ft)
Grate Length:        Length if grate inlet (ft)
Grate Width:         Width if grate inlet (ft)
Curb Height:         Opening height if curb inlet (in)
Curb Length:         Opening length if curb inlet (ft)
Clogging Factor:     Reduction factor (0.5 - 0.9)
Local Depression:    Additional depth (in)
Depression Width:    Width of depression (ft)
Contributing Area:   Drainage area to inlet (acres)
Runoff Coefficient:  C value for contributing area
```

### Example

```
ID: IN-001
Description: Inlet at Sta 0+25 RT
Inlet Type: Combination
Location Type: On-grade
Rim Elevation: 128.75 ft
Invert Elevation: 124.50 ft
Grate Length: 2.0 ft
Grate Width: 1.67 ft
Curb Height: 6 in
Curb Length: 5.0 ft
Clogging Factor: 0.80
Local Depression: 2 in
Depression Width: 2.0 ft
Contributing Area: 0.5 acres
Runoff Coefficient: 0.85
```

---

## 4. Conduits

### Definition
A conduit is a pipe or channel that conveys stormwater between system components (inlets, junctions, outfalls).

### Conduit Types

1. **Circular Pipe**: Most common, various materials
2. **Box Culvert**: Rectangular, for larger flows
3. **Arch Pipe**: Elliptical or arch shape
4. **Open Channel**: Trapezoidal or rectangular ditch

### Key Characteristics

| Property | Description | Typical Values |
|----------|-------------|----------------|
| **Shape** | Cross-section geometry | Circular, rectangular, arch |
| **Material** | Pipe material | RCP, CMP, PVC, HDPE |
| **Diameter/Size** | Pipe size | 12 - 96 inches (circular) |
| **Length** | Conduit length | Site-specific (ft) |
| **Manning's n** | Roughness coefficient | 0.011 - 0.024 (see manning_n_values.md) |
| **Upstream Invert** | Elevation at upstream end | Site-specific (ft) |
| **Downstream Invert** | Elevation at downstream end | Site-specific (ft) |
| **Slope** | (Downstream - Upstream) / Length | Typically 0.004 - 0.10 |
| **Entrance Loss Coeff** | Ke for inlet | 0.2 - 0.9 |
| **Exit Loss Coeff** | Ke for outlet | 1.0 |

### Pipe Materials

| Material | Abbreviation | Typical n Value | Typical Sizes |
|----------|--------------|-----------------|---------------|
| Reinforced Concrete Pipe | RCP | 0.013 | 12" - 144" |
| Corrugated Metal Pipe | CMP | 0.024 | 12" - 96" |
| PVC (Polyvinyl Chloride) | PVC | 0.011 | 4" - 48" |
| HDPE (High Density Polyethylene) | HDPE | 0.011 | 4" - 60" |
| Vitrified Clay Pipe | VCP | 0.014 | 4" - 36" |

### Minimum Pipe Sizes

| Application | Minimum Diameter |
|-------------|------------------|
| Driveway culverts | 12 inches |
| Storm laterals (public) | 15 inches |
| Storm main lines | 18 inches |
| Highway cross drains | 18 inches |

### Design Criteria

#### Minimum Slope
```
S_min = [n × V_min / (0.463 × D^(2/3))]²
```

For self-cleaning velocity (V = 2.5 - 3.0 ft/s):
- 12" pipe: 0.4% minimum
- 18" pipe: 0.3% minimum
- 24" pipe: 0.28% minimum

#### Maximum Velocity
- Typical limit: 10 - 15 ft/s (to prevent scour)
- With erosion protection: up to 20 ft/s

#### Cover Requirements
- Minimum cover: 1.0 - 2.0 ft over top of pipe
- Under roadways: Typically 2.0 ft minimum
- Under railroads: Typically 3.0 ft minimum

### Required Input Data

```
Conduit ID:          Unique identifier
Description:         Name/location
From Node:           Upstream junction/inlet ID
To Node:             Downstream junction/outfall ID
Shape:               Circular, box, arch, etc.
Material:            RCP, CMP, PVC, HDPE, etc.
Diameter/Size:       Pipe diameter or dimensions (in)
Length:              Conduit length (ft)
Manning's n:         Roughness coefficient
Upstream Invert:     Elevation at upstream end (ft)
Downstream Invert:   Elevation at downstream end (ft)
Entrance Loss Ke:    Entrance loss coefficient
Exit Loss Ke:        Exit loss coefficient (typically 1.0)
```

### Example

```
ID: C-101
Description: Pipe from IN-001 to MH-101
From Node: IN-001
To Node: MH-101
Shape: Circular
Material: RCP (Reinforced Concrete)
Diameter: 18 inches
Length: 120 ft
Manning's n: 0.013
Upstream Invert: 124.30 ft
Downstream Invert: 123.50 ft
Slope: 0.0067 (0.67%)
Entrance Loss Ke: 0.5
Exit Loss Ke: 1.0
```

---

## System Connectivity

### Network Structure

A typical storm drainage system forms a dendritic (tree-like) network:

```
[Inlet-1]---[Conduit-1]---\
                           [Junction-1]---[Conduit-3]---[Outfall]
[Inlet-2]---[Conduit-2]---/
```

### Connectivity Rules

1. **Inlets** connect to junctions or outfalls via conduits
2. **Junctions** can have multiple inlet conduits but only one outlet conduit
3. **Conduits** connect two nodes (inlet→junction, junction→junction, junction→outfall)
4. **Outfalls** are terminal nodes (no downstream connections)
5. **Flow direction** is always from higher to lower elevation

### Tabular Data Structure

For computational purposes, components can be represented in tables:

#### Nodes Table (Inlets, Junctions, Outfalls)
| Node ID | Type | Rim Elev | Invert Elev | Description |
|---------|------|----------|-------------|-------------|
| IN-001 | Inlet | 128.75 | 124.50 | ... |
| MH-101 | Junction | 125.30 | 118.50 | ... |
| OUT-001 | Outfall | -- | 100.50 | ... |

#### Conduits Table
| Conduit ID | From Node | To Node | Diameter | Length | n | Up Invert | Dn Invert |
|------------|-----------|---------|----------|--------|---|-----------|-----------|
| C-101 | IN-001 | MH-101 | 18" | 120 | 0.013 | 124.30 | 118.60 |
| C-102 | MH-101 | OUT-001 | 24" | 250 | 0.013 | 118.50 | 100.60 |

This tabular format is ideal for:
- Spreadsheet input
- Database storage
- Software import (StormCAD, HydroCAD, EPA SWMM)
- Hydraulic calculations
