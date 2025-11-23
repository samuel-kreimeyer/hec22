# Gutter Flow Equations

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 4

## Gutter Flow Capacity

### Modified Manning's Equation for Gutter Flow

```
Q = (Ku/n) × Sx^(5/3) × SL^(1/2) × T^(8/3)
```

**Where:**
- Q = Gutter flow rate (cfs)
- Ku = 0.56 (US customary units)
- n = Manning's roughness coefficient
- Sx = Cross slope (ft/ft)
- SL = Longitudinal slope (ft/ft)
- T = Spread (width of flow in gutter) (ft)

### Depth of Flow at Curb

```
d = Sx × T
```

**Where:**
- d = Depth of flow at curb (ft)
- Sx = Cross slope (ft/ft)
- T = Spread (ft)

### Gutter Flow Velocity

```
V = Q / A
```

Where:
```
A = (1/2) × d × T = (1/2) × Sx × T²
```

**Therefore:**
```
V = (2Q) / (Sx × T²)
```

Or alternatively:
```
V = (Ku/n) × Sx^(2/3) × SL^(1/2) × T^(2/3)
```

## Flow with Composite Cross Section

When gutter has a depressed section (e.g., local depression at curb):

### Equivalent Cross Slope

```
Sx' = Sx + (Sw × W / T)
```

**Where:**
- Sx' = Equivalent cross slope
- Sx = Pavement cross slope
- Sw = Additional depression slope
- W = Width of depressed section
- T = Total spread

### Flow Distribution

Flow in the depressed section:
```
Qw / Q = (1 + Sw/Sx × W/T)^(8/3) / (1 + W/T)^(8/3)
```

**Where:**
- Qw = Flow in depressed section (cfs)
- Q = Total gutter flow (cfs)

## Frontal Flow (Flow Intercepted by Inlet)

### Eo (Ratio of Frontal Flow to Total Flow)

```
Eo = Qw / Q
```

For uniform cross slope (no depression):
```
Eo = 1 - (1 - W/T)^(8/3)
```

Where W = inlet width

## Gutter Geometry Parameters

### Typical Cross Slopes

| Surface Type | Cross Slope (Sx) |
|--------------|------------------|
| Concrete/Asphalt pavement | 0.015 - 0.020 (1.5% - 2.0%) |
| Crowned roadway (each side) | 0.015 - 0.025 (1.5% - 2.5%) |
| Minimum recommended | 0.015 (1.5%) |

### Typical Longitudinal Slopes

| Condition | Slope (SL) |
|-----------|------------|
| Minimum recommended | 0.004 (0.4%) |
| Flat terrain | 0.004 - 0.010 |
| Rolling terrain | 0.010 - 0.050 |
| Steep terrain | > 0.050 |

### Maximum Allowable Spread (T)

| Road Type | Allowable Spread |
|-----------|------------------|
| Major arterial (design frequency) | 6 ft from curb |
| Collector streets | 8 ft from curb |
| Local streets | 10 ft from curb |
| All streets (50-yr or 100-yr) | Should not encroach on adjacent traffic lane |

## Design Considerations

### Velocity Limits
- **Minimum velocity**: 2.0 - 3.0 ft/s (for self-cleaning)
- **Maximum velocity**: 5.0 - 10.0 ft/s (to prevent scour and erosion)

### Depth Limits
- **Maximum depth at curb**: Typically 0.15 - 0.30 ft depending on spread limits

### Design Steps

1. Determine design flow rate Q (from hydrologic analysis)
2. Select allowable spread T based on roadway classification
3. Determine cross slope Sx and longitudinal slope SL
4. Select Manning's n (typically 0.016 for concrete gutter)
5. Calculate required spread or verify capacity
6. Check depth at curb and velocity

## Example Calculation

**Given:**
- Q = 2.5 cfs
- SL = 0.02 (2%)
- Sx = 0.02 (2%)
- n = 0.016
- Ku = 0.56

**Find spread T:**

```
T = [Q × n / (Ku × Sx^(5/3) × SL^(1/2))]^(3/8)
T = [2.5 × 0.016 / (0.56 × 0.02^(5/3) × 0.02^(1/2))]^(3/8)
T = [0.04 / (0.56 × 0.00736 × 0.1414)]^(3/8)
T = [0.04 / 0.000583]^(3/8)
T = (68.6)^(3/8)
T ≈ 5.8 ft
```

**Depth at curb:**
```
d = Sx × T = 0.02 × 5.8 = 0.116 ft ≈ 1.4 inches
```

## SI Units Version

For SI units (metric), use Ku = 0.376:

```
Q = (0.376/n) × Sx^(5/3) × SL^(1/2) × T^(8/3)
```

Where:
- Q = Flow rate (m³/s)
- Sx, SL = Slopes (m/m)
- T = Spread (m)
