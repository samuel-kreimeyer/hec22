# Manning's Equation for Storm Drain Pipes

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual

## Manning's Equation

Manning's equation is the most widely used formula for determining the hydraulic capacity of storm drain pipes for gravity flow.

### Full Flow (Pipe Flowing Full)

```
Q = (1.486/n) × A × R^(2/3) × S^(1/2)
```

**Where:**
- Q = Flow rate (cfs)
- n = Manning's roughness coefficient
- A = Cross-sectional area of flow (ft²)
- R = Hydraulic radius = A/P (ft)
- P = Wetted perimeter (ft)
- S = Slope of the energy grade line (ft/ft), approximately equal to pipe slope

### For Circular Pipes

```
Q = (1.486/n) × (π/4) × D² × (D/4)^(2/3) × S^(1/2)
```

Simplifies to:

```
Q = 0.463 × (D^(8/3) / n) × S^(1/2)
```

**Where:**
- D = Pipe diameter (ft)
- S = Pipe slope (ft/ft)
- n = Manning's roughness coefficient

### Velocity Equation

```
V = (1.486/n) × R^(2/3) × S^(1/2)
```

**Where:**
- V = Velocity (ft/s)

### Normal Depth (Partially Full Flow)

For partially full circular pipes, the flow can be calculated using:

```
Q = (1.486/n) × A × R^(2/3) × S^(1/2)
```

Where A and R depend on the depth of flow (d) and pipe diameter (D).

## Design Considerations

1. **Maximum Velocity**: Typically limited to 10-15 ft/s to prevent scour
2. **Minimum Velocity**: At least 2.5-3.0 ft/s for self-cleaning (at design flow)
3. **Minimum Slope**: Typically 0.004 ft/ft (0.4%) for 12-inch pipes and larger
4. **Capacity**: Pipes are typically designed to flow 80-95% full at design flow

## Related Equations

### Hydraulic Radius for Circular Pipe (Full Flow)
```
R = D/4
```

### Hydraulic Radius for Partially Full Circular Pipe
```
R = A/P
```

Where A and P are functions of the flow depth and pipe diameter.

## Units

**SI Units Version:**
```
Q = (1.0/n) × A × R^(2/3) × S^(1/2)
```

Where:
- Q = Flow rate (m³/s)
- A = Area (m²)
- R = Hydraulic radius (m)
- S = Slope (m/m)
