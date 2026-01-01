# Detention and Retention Equations

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 10

## Storage Volume Estimation

### Loss-of-Natural-Storage Method

```
Q_s = Q_a - Q_b
```

**Where:**
- Q_s = Storage needed (inches of depth)
- Q_a = Post-development runoff depth (inches)
- Q_b = Pre-development runoff depth (inches)

```
V_s = α × A × Q_s
```

**Where:**
- V_s = Required storage volume (ft³)
- α = 3,630 (conversion factor for US customary units)
- A = Drainage area (acres)
- Q_s = Storage needed (inches)

**Reference:** Equations 10.1-10.3

**Applicability:**
- Preliminary estimate only - must verify with routing
- More conservative for retention than detention
- Use consistent time durations for Q_a and Q_b

### Triangular Hydrograph Method

```
V_s = 0.5 × t_i × (q_i - q_o)
```

**Where:**
- V_s = Required storage volume (ft³)
- t_i = Duration of basin inflow = 2 × T_c (seconds)
- q_i = Peak inflow rate (ft³/s)
- q_o = Peak outflow rate (target release rate) (ft³/s)
- T_c = Time of concentration

**Reference:** Equation 10.4

**Applicability:**
- Works best with Rational Method
- Requires time of concentration
- Good for preliminary design
- Simple and reasonably accurate

## Basin Geometry - Stage-Storage Relationships

### Rectangular Basin (Horizontal Bottom)

```
V = L × W × D
```

**Where:**
- V = Storage volume (ft³)
- L = Basin length (ft)
- W = Basin width (ft)
- D = Depth of ponding (ft)

**Reference:** Equation 10.7

### Rectangular Basin (Sloped Bottom)

```
V = (L / tan(θ)) × h² + (L × W) × h
```

**Where:**
- V = Storage volume (ft³)
- L = Basin length (ft)
- W = Basin width (ft)
- h = Depth of ponding (ft)
- θ = Angle of bottom slope

**Reference:** Equations 10.8-10.9

### Trapezoidal Basin

```
V = L×W×D + (L + W)×Z×D² + (4/3)×Z²×D³
```

**Where:**
- V = Storage volume (ft³)
- L = Basin length at base (ft)
- W = Basin width at base (ft)
- D = Depth of ponding (ft)
- Z = Side slope factor (H:V), e.g., 3.0 for 3:1 slope

**Reference:** Equations 10.10-10.11

**Note:** This is the most commonly used equation for detention basin design.

### Irregular Basin - Average-End Area Method

```
V = [(A₁ + A₂) / 2] × Δh
```

**Where:**
- V = Incremental storage volume (ft³)
- A₁ = Surface area at elevation 1 (ft²)
- A₂ = Surface area at elevation 2 (ft²)
- Δh = Change in elevation (ft)

**Reference:** Equation 10.18

**Application:** Calculate cumulative storage by summing increments from bottom to top.

### Irregular Basin - Conic Section Method (More Accurate)

```
V = (Δh / 3) × (A₁ + √(A₁×A₂) + A₂)
```

**Where:**
- V = Incremental storage volume (ft³)
- A₁ = Surface area at lower elevation (ft²)
- A₂ = Surface area at upper elevation (ft²)
- Δh = Change in elevation (ft)

**Reference:** Equation 10.19

**Note:** Provides better accuracy than average-end area method, especially when areas change significantly.

## Outlet Hydraulics

### Orifice Discharge

```
Q = C_o × A_o × √(2 × g × h_o)
```

**Where:**
- Q = Discharge through orifice (ft³/s)
- C_o = Discharge coefficient (typically 0.6)
- A_o = Orifice area (ft²)
- g = Gravitational acceleration (32.2 ft/s²)
- h_o = Effective head on orifice (ft)

**Reference:** Equation 10.23

**Discharge Coefficients:**
- C_o = 0.6 for square-edged openings
- C_o = 0.4 for ragged edges (torch-cut)

**Critical Design Details:**
- Head measured from centroid of opening to water surface
- For circular orifice: h_o = (stage - invert elevation) - D/2
- For submerged orifice: h_o = difference in water surface elevations
- Pipes < 1 ft diameter: treat as orifice when h_o/D > 1.5
- Pipes > 1 ft diameter: use HDS-5 culvert hydraulics

### Sharp-Crested Weir (No End Contractions)

```
Q = C_w × √(2g) × L × h^1.5
```

**Where:**
- Q = Discharge over weir (ft³/s)
- C_w = Weir coefficient (typically 0.37)
- g = Gravitational acceleration (32.2 ft/s²)
- L = Weir length (ft)
- h = Head above weir crest (ft)

**Reference:** Equation 10.32

**Simplified form with g = 32.2 ft/s²:**
```
Q = 2.96 × C_w × L × h^1.5
```

### Sharp-Crested Weir (With End Contractions)

```
Q = C_w × √(2g) × (L - 0.2×h) × h^1.5
```

**Where:**
- L = Weir length (ft)
- h = Head above weir crest (ft)
- C_w = 0.415 for h/h_c < 0.3

**Reference:** Equation 10.33

**Note:** End contractions occur when weir does not span full width of channel.

### Broad-Crested Weir

```
Q = C_w × √(2g) × L × h^1.5
```

**Where:**
- C_w = Broad-crested weir coefficient (from Table 10.7)
- C_w ranges from 0.29 to 0.41
- C_w = 0.41 for well-rounded upstream edge
- C_w = 0.29 for sharp corners

**Note:** Coefficient depends on head and breadth of weir crest. Refer to Table 10.7 in HEC-22 for specific values.

### V-Notch Weir

```
Q = C_w × √(2g) × tan(θ/2) × h^2.5
```

**Where:**
- Q = Discharge (ft³/s)
- C_w = Weir coefficient (typically 0.31)
- θ = Angle of v-notch (degrees)
- h = Head above bottom of notch (ft)
- g = 32.2 ft/s²

**Reference:** Equation 10.35

**Common V-Notch Angles:**
- 90° notch: tan(45°) = 1.0
- 60° notch: tan(30°) = 0.577

### Proportional Weir

**Geometry:**
```
x/b = 1 - 0.315 × arctan((y/a)^0.5)
```

**Discharge:**
```
Q = C_w × √(2g) × a^0.5 × b × (h - a/3)
```

**Where:**
- x, y = Coordinates of weir shape
- a, b = Weir dimensions
- h = Head above weir invert

**Reference:** Equations 10.36-10.37

**Note:** Proportional weirs provide linear head-discharge relationship, reducing required storage but more complex to construct.

## Storage Routing

### Modified Puls Method

**Continuity Equation:**
```
(I₁ + I₂)/2 - (O₁ + O₂)/2 = ΔS/Δt
```

**Rearranged for solution:**
```
(2S₂/Δt + O₂) = (I₁ + I₂) + (2S₁/Δt - O₁)
```

**Where:**
- I₁, I₂ = Inflow at beginning and end of time step (ft³/s)
- O₁, O₂ = Outflow at beginning and end of time step (ft³/s)
- S₁, S₂ = Storage at beginning and end of time step (ft³)
- Δt = Time step (seconds)

**Reference:** Equation 10.45

**Solution Process:**
1. Known: I₁, I₂, S₁, O₁
2. Calculate right-hand side: (I₁ + I₂) + (2S₁/Δt - O₁)
3. Find stage where: 2S(stage)/Δt + O(stage) = RHS
4. Interpolate from stage-storage and stage-discharge curves
5. Repeat for each time step

**Time Step Guidelines:**
- Δt should be ≤ time to peak / 5
- Typical values: 0.05 to 0.2 hours (3 to 12 minutes)
- Smaller Δt increases accuracy but computation time
- Check sensitivity to Δt selection

## Single-Stage Riser Design

### Orifice Sizing for Target Discharge

```
A_o = Q_target / (C_o × √(2×g×(E₁-E_o-H_o/2)))
```

**Where:**
- A_o = Required orifice area (ft²)
- Q_target = Target release rate (ft³/s)
- E₁ = Maximum water surface elevation (ft)
- E_o = Orifice invert elevation (ft)
- H_o = Orifice height (ft)
- C_o = Discharge coefficient (0.6 typical)
- g = 32.2 ft/s²

**Reference:** Equation 10.39

### Weir Sizing for Target Discharge

```
L_w = Q_target / (C_w × √(2g) × (E₁-E_o)^1.5)
```

**Where:**
- L_w = Required weir length (ft)
- Q_target = Target release rate (ft³/s)
- E₁ = Maximum water surface elevation (ft)
- E_o = Weir crest elevation (ft)
- C_w = Weir coefficient (0.37 typical)

**Reference:** Equation 10.41

## Design Considerations

### Typical Design Parameters

| Parameter | Typical Value | Notes |
|-----------|---------------|-------|
| Design AEP | 0.5, 0.1, 0.01 | Multi-stage often used |
| Freeboard | 1-2 ft minimum | Above maximum storage elevation |
| Side Slopes | 3:1 to 5:1 (H:V) | For safety and stability |
| Minimum Velocity | 2.5-3.0 ft/s | Self-cleansing in outlet pipe |
| Detention Time | 12-48 hours | Extended detention for water quality |

### Outlet Structure Selection

| Head Range | Preferred Outlet Type |
|------------|----------------------|
| Low head (< 5 ft) | Orifice |
| Medium head (5-15 ft) | Orifice or weir |
| High head (> 15 ft) | Weir or pipe outlet |
| Variable head | Multi-stage riser |

### Storage Volume Ranges

**Preliminary Estimates (as fraction of peak inflow volume):**
- Simple detention: 50-75% of peak inflow volume
- Extended detention: 75-100% of peak inflow volume
- Retention: 100-150% of peak inflow volume

**Note:** Always verify with storage routing analysis.

## Example Calculation

**Given:**
- Drainage area: 38 acres
- Pre-development peak: 50 ft³/s
- Post-development peak: 131 ft³/s
- Design storm: 0.1 AEP (10-year)
- Target release: 50 ft³/s

**Step 1: Preliminary Storage (Triangular Hydrograph)**

Assuming T_c = 20 minutes = 1,200 seconds:
```
t_i = 2 × T_c = 2 × 1,200 = 2,400 seconds
V_s = 0.5 × 2,400 × (131 - 50)
V_s = 0.5 × 2,400 × 81
V_s = 97,200 ft³
```

**Step 2: Basin Sizing (Trapezoidal)**

For L = 100 ft, W = 50 ft, Z = 4:1 slope:
```
97,200 = 100×50×D + (100+50)×4×D² + (4/3)×16×D³
97,200 = 5,000D + 600D² + 21.33D³
```

Solving iteratively: D ≈ 4.5 ft

**Step 3: Verify with routing and iterate on outlet structure sizing**

See HEC-22 Chapter 10 examples for complete routing procedures.

## SI Units Conversion

For SI units (metric):
- Use α = 101.2 (instead of 3,630)
- All dimensions in meters
- Flow in m³/s
- g = 9.81 m/s²

## References

- HEC-22 Chapter 10 (FHWA 2024): Detention and Retention
- HDS-5: Hydraulic Design of Highway Culverts (for outlet pipe sizing)
- HEC-HMS: Hydrologic Modeling System (for hydrograph generation)
