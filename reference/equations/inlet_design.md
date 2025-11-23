# Inlet Design Equations

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 4
Original theory from Izzard (1950)

## Inlet Types

1. **Grate Inlets**: Openings in the gutter covered by grates
2. **Curb-Opening Inlets**: Vertical openings in the curb
3. **Combination Inlets**: Both grate and curb opening
4. **Slotted Drain Inlets**: Continuous slot along gutter

## Inlets on Grade (Continuous Grade)

### Curb-Opening Inlet Length for 100% Interception

```
Lt = Ku × Q^0.42 × SL^0.3 / (n × Sx^0.6)
```

**Where:**
- Lt = Required curb-opening length for 100% interception (ft)
- Ku = 0.6 (US customary units), 0.817 (SI units)
- Q = Gutter flow rate (cfs)
- SL = Longitudinal slope (ft/ft)
- Sx = Cross slope (ft/ft)
- n = Manning's roughness coefficient

### Curb-Opening Inlet Efficiency (L < Lt)

When the inlet length L is less than Lt:

```
Qi = Q × (1 - (1 - L/Lt)^1.8)
```

Or efficiency:
```
E = Qi/Q = 1 - (1 - L/Lt)^1.8
```

**Where:**
- Qi = Flow intercepted by inlet (cfs)
- Q = Total gutter flow approaching inlet (cfs)
- L = Actual inlet length (ft)
- Lt = Length required for 100% interception (ft)

### Grate Inlet Interception (Frontal Flow)

For frontal flow interception:

```
Qi = Eo × Q
```

Where Eo is the ratio of frontal flow to total flow (see gutter_flow.md)

For side flow (splash-over velocity):
```
Vo = Kv × √(gL)
```

**Where:**
- Vo = Splash-over velocity (ft/s)
- Kv = 0.295 for SI units with conversion
- g = Gravitational acceleration (32.2 ft/s²)
- L = Grate length (ft)

Grate efficiency for side flow:
```
Rs = 1 / (1 + (V/Vo)^2.67)
```

**Where:**
- Rs = Ratio of side flow intercepted
- V = Velocity of flow in gutter (ft/s)

Total interception by grate:
```
Qi = Eo × Q + Rs × (1 - Eo) × Q
```

### Combination Inlet

For a combination inlet (curb opening + grate):

```
Qi = Qg + Qc
```

Where:
- Qg = Flow intercepted by grate
- Qc = Flow intercepted by curb opening (from bypass flow)

## Inlets in Sag Locations

In sag vertical curves, all flow is intercepted (no bypass). The inlet acts as a weir at low depths and as an orifice at higher depths.

### Weir Flow (Low Depth)

For curb-opening inlets:
```
Q = Cw × L × d^1.5
```

**Where:**
- Q = Flow capacity (cfs)
- Cw = Weir coefficient = 2.3 (for vertical curb opening)
- L = Curb-opening length (ft)
- d = Depth of water at curb (ft)

For grate inlets:
```
Q = Cw × P × d^1.5
```

**Where:**
- P = Perimeter of grate (excluding side against curb) (ft)
- Cw = Weir coefficient = 3.0 for grates

### Orifice Flow (High Depth)

For curb-opening inlets:
```
Q = Co × A × √(2gd)
```

**Where:**
- Q = Flow capacity (cfs)
- Co = Orifice coefficient = 0.67
- A = Clear opening area (ft²) = h × L
- h = Vertical height of opening (ft)
- g = 32.2 ft/s²
- d = Depth of water above centroid of opening (ft)

For grate inlets:
```
Q = Co × Ag × √(2gd)
```

**Where:**
- Ag = Clear opening area of grate (ft²)
- Co = Orifice coefficient = 0.67

### Transition Depth

The transition from weir to orifice flow occurs when:
```
d/h ≈ 1.0 to 1.4
```

Where:
- d = Depth of water
- h = Height of opening

## Inlet Spacing on Grade

### Required Inlet Spacing

```
L_spacing = Qi / Q × L_total
```

Where Q is the total flow along the section.

Alternatively:
```
L_spacing = (Ql - Qb) / qi
```

**Where:**
- L_spacing = Distance between inlets (ft)
- Ql = Flow rate at downstream end of section (cfs)
- Qb = Allowable bypass (carryover) flow (cfs)
- qi = Flow per unit length (cfs/ft)

## Clogging Factors

Design capacity should account for clogging:

### Clogging Reduction Factors

| Inlet Type | Clogging Factor |
|------------|-----------------|
| Grate inlets | 0.50 - 0.65 (use 50-65% of calculated capacity) |
| Curb-opening inlets | 0.90 (use 90% of calculated capacity) |
| Combination inlets | 0.80 (use 80% of calculated capacity) |

**Effective capacity:**
```
Q_design = Cf × Q_calculated
```

Where Cf = clogging factor

## Standard Inlet Dimensions

### Typical Curb-Opening Inlets
- Height: 4 - 6 inches
- Length: 5 - 10 feet
- Throat width: 1.5 - 2.0 feet

### Typical Grate Inlets
- Width: 1.5 - 2.0 feet
- Length: 2 - 4 feet
- Bar spacing: 1.5 - 2.0 inches

## Design Procedure for Inlets on Grade

1. Determine gutter flow rate Q from hydrologic analysis
2. Calculate spread T and depth d
3. For curb-opening inlet:
   - Calculate Lt for 100% interception
   - If L < Lt, calculate efficiency E
4. For grate inlet:
   - Calculate frontal flow Eo × Q
   - Calculate side flow interception Rs
   - Total interception = Eo × Q + Rs × (1 - Eo) × Q
5. Apply clogging factor
6. Calculate bypass flow Qb = Q - Qi
7. Bypass becomes flow to next inlet downstream

## Design Procedure for Inlets in Sag

1. Determine total flow Q entering sag
2. Select inlet type and dimensions
3. Calculate weir capacity
4. Calculate orifice capacity
5. Design capacity = minimum of (weir, orifice) × clogging factor
6. Check that capacity exceeds design flow
7. Determine ponding depth if flow exceeds capacity

## Example: Curb-Opening Inlet on Grade

**Given:**
- Q = 3.0 cfs
- SL = 0.02
- Sx = 0.02
- n = 0.016
- L = 5 ft (inlet length)

**Find interception:**

```
Lt = 0.6 × 3.0^0.42 × 0.02^0.3 / (0.016 × 0.02^0.6)
Lt = 0.6 × 1.538 × 0.342 / (0.016 × 0.126)
Lt = 0.316 / 0.00202
Lt = 156 ft
```

Since L (5 ft) << Lt (156 ft):
```
E = 1 - (1 - 5/156)^1.8
E = 1 - (0.968)^1.8
E = 1 - 0.945
E = 0.055 or 5.5%
```

This single 5-ft inlet intercepts only 5.5% of flow. Multiple inlets would be needed.
