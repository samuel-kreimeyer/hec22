# Open Channel Flow Equations

**Reference:** HEC-22 Chapter 6 - Roadside and Median Channels

## Energy Equation

### Total Energy (Head)

The total energy at any cross-section in an open channel:

```
E_t = Z + y + V²/(2g)
```

Where:
- `E_t` = Total energy, ft (m)
- `Z` = Elevation of channel bottom above datum, ft (m)
- `y` = Flow depth, ft (m)
- `V` = Mean velocity, ft/s (m/s)
- `g` = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

### Energy Balance Between Sections

```
Z₁ + y₁ + V₁²/(2g) = Z₂ + y₂ + V₂²/(2g) + h_L
```

Where:
- `h_L` = Energy (head) lost between sections 1 and 2 to friction and turbulence, ft (m)

## Specific Energy

Specific energy is the energy head relative to the channel bottom:

```
E = y + V²/(2g)
```

Where:
- `E` = Specific energy, ft (m)
- `y` = Flow depth, ft (m)
- `V` = Mean velocity, ft/s (m/s)
- `g` = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Critical Flow Relationship:**
- At critical flow, critical depth = 2/3 of specific energy
- Velocity head = 1/3 of specific energy

## Froude Number

The Froude number classifies flow as subcritical, critical, or supercritical:

```
Fr = V / √(gy)
```

Where:
- `Fr` = Froude number (dimensionless)
- `V` = Mean velocity of flow, ft/s (m/s)
- `g` = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
- `y` = Flow depth, ft (m)

**Flow Classification:**
- **Fr < 1:** Subcritical flow (depth > critical depth, velocity < critical velocity)
- **Fr = 1:** Critical flow (minimum specific energy)
- **Fr > 1:** Supercritical flow (depth < critical depth, velocity > critical velocity)

## Channel Geometry

### Trapezoidal and Triangular Channels

Cross-sectional area:
```
A = Bd + zd²
```

Wetted perimeter:
```
P = B + 2d(z² + 1)^0.5
```

Surface width:
```
T = B + 2dz
```

Where:
- `A` = Cross-sectional area, ft² (m²)
- `P` = Wetted perimeter, ft (m)
- `T` = Surface width, ft (m)
- `B` = Bottom width, ft (m) (B = 0 for triangular channels)
- `d` = Maximum flow depth, ft (m)
- `z` = Horizontal side slope dimension 1:z (V:H)

## Shear Stress

### Average Shear Stress

```
τ = γRS
```

Where:
- `τ` = Average shear stress, lb/ft² (N/m²)
- `γ` = Unit weight of water, 62.4 lb/ft³ at 60°F (9.81 kN/m³ at 15°C)
- `R` = Hydraulic radius, ft (m)
- `S` = Average bed slope or energy slope, ft/ft (m/m)

### Maximum Shear Stress (Straight Channel)

```
τ_d = γdS
```

Where:
- `τ_d` = Maximum shear stress, lb/ft² (N/m²)
- `d` = Maximum depth of flow, ft (m)

### Side Shear Stress (Trapezoidal Channel)

```
τ_s = K₁ τ_d
```

Where:
- `τ_s` = Side shear stress, lb/ft² (N/m²)
- `K₁` = Ratio of channel side to bottom shear stress
- `τ_d` = Shear stress at maximum depth, lb/ft² (N/m²)

**K₁ Values:**
```
K₁ = 0.77                    for z ≤ 1.5
K₁ = 0.066z + 0.67           for 1.5 < z < 5
K₁ = 1.0                     for 5 ≤ z
```

### Riprap Side Slope Sizing

For side slopes steeper than 1:3 (V:H):

```
D₅₀,sides = (K₁/K₂) D₅₀,bottom
```

Where:
- `D₅₀` = Riprap median size, ft (m)
- `K₁` = Ratio of shear stresses on sides and bottom
- `K₂` = Ratio of tractive force on sides and bottom

```
K₂ = √[1 - (sin²Θ)/(sin²Φ)]
```

Where:
- `Θ` = Angle of side slope
- `Φ` = Angle of repose for channel lining material

### Bend Shear Stress

```
τ_b = K_b τ_d
```

Where:
- `τ_b` = Bend shear stress, lb/ft² (N/m²)
- `K_b` = Ratio of channel bend to bottom shear stress
- `τ_d` = Maximum channel shear stress, lb/ft² (N/m²)

**K_b Values:**
```
K_b = 2.00                                           for R_c/T ≤ 2
K_b = 2.38 - 0.206(R_c/T) + 0.0073(R_c/T)²          for 2 < R_c/T < 10
K_b = 1.05                                           for 10 ≤ R_c/T
```

Where:
- `R_c` = Radius to centerline of channel, ft (m)
- `T` = Top (water surface) width, ft (m)

### Length of Protection Downstream of Bend

```
L_p = (K_u R^(7/6)) / n_b
```

Where:
- `L_p` = Length of protection downstream of curve point of tangency, ft (m)
- `n_b` = Manning's n in the channel bend
- `R` = Hydraulic radius, ft (m)
- `K_u` = Unit conversion constant, 0.604 in CU (0.736 in SI)

## Superelevation in Bends

Difference in water surface elevation between inner and outer banks:

```
Δd = (V²T) / (gR_c)
```

Where:
- `Δd` = Difference in water surface elevation, ft (m)
- `V` = Average velocity, ft/s (m/s)
- `T` = Surface width of channel, ft (m)
- `g` = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
- `R_c` = Radius to centerline of channel, ft (m)

**Note:** Valid for subcritical flow conditions only.

The water surface at:
- Outer bank = centerline elevation + Δd/2
- Inner bank = centerline elevation - Δd/2

## Stable Channel Design Criterion

```
τ_p ≥ SF × τ_d
```

Where:
- `τ_p` = Permissible shear stress for channel lining, lb/ft² (N/m²)
- `SF` = Safety factor (typically 1.0 to 1.5)
- `τ_d` = Calculated maximum shear stress, lb/ft² (N/m²)

## Manning's n Values for Channel Linings

| Lining Type | n (min) | n (typical) | n (max) |
|-------------|---------|-------------|---------|
| **Rigid Linings** |
| Concrete | 0.011 | 0.013 | 0.015 |
| Grouted Riprap | 0.028 | 0.030 | 0.040 |
| Stone Masonry | 0.030 | 0.032 | 0.042 |
| Soil Element | 0.020 | 0.022 | 0.025 |
| Asphalt | 0.016 | 0.016 | 0.018 |
| **Unlined** |
| Bare Soil | 0.016 | 0.020 | 0.025 |
| Rock Cut | 0.025 | 0.035 | 0.045 |
| **RECP (Rolled Erosion Control Products)** |
| Open-weave textile | 0.022 | 0.025 | 0.028 |
| Erosion control blanket | 0.028 | 0.035 | 0.045 |
| Turf reinforcement mat | 0.024 | 0.030 | 0.036 |

## Design Considerations

### Flow Regimes

- **Subcritical Flow (Fr < 1):**
  - Occurs on mild slopes
  - Downstream conditions control flow
  - Depth > critical depth
  - Disturbances travel both upstream and downstream

- **Supercritical Flow (Fr > 1):**
  - Occurs on steep slopes
  - Upstream conditions control flow
  - Depth < critical depth
  - Disturbances swept downstream only

- **Critical Flow (Fr = 1):**
  - Unstable condition
  - Small changes can cause flow regime shifts
  - Should generally be avoided in design

### Hydraulic Jump

When flow transitions from supercritical to subcritical:
- Occurs when Fr > 1 and channel condition changes (obstacle, slope reduction)
- Results in rapid increase in depth and decrease in velocity
- Creates high turbulence
- Can cause scour and erosion
- Location varies with flow rate
- Should be avoided in roadside/median channels when possible
- If unavoidable, use protective linings

### Channel Stability

Factors affecting stability:
- Shear stress magnitude
- Channel lining permissible shear stress
- Side slope steepness (> 1:3 V:H increases risk)
- Bend geometry and secondary currents
- Lining type (flexible vs. rigid)
- Soil erodibility
- Vegetation establishment

## Applications

### Typical Design Discharge

- Permanent channels: 0.2 to 0.1 AEP (5 to 10-year return period)
- Temporary channels: 0.5 AEP (2-year return period)

### Freeboard

- Minimum: Prevent overflow from debris, waves, superelevation
- Steep channels: Consider freeboard = total energy depth
- Temporary channels: Freeboard optional

### Side Slopes

- Safety consideration: 1V:3H or flatter (traversable by errant vehicles)
- Stability consideration: Not steeper than angle of repose
- Influence from roadway geometric design criteria

## References

- HEC-22 Chapter 6 (FHWA 2024): Roadside and Median Channels
- HEC-15 (FHWA 2005): Design of Roadside Channels with Flexible Linings
- Chow (1959): Open Channel Hydraulics
- AASHTO Roadside Design Guide (2011)

---

**Note:** This reference focuses on equations for steady uniform flow in prismatic channels. For non-uniform or unsteady flow conditions, more advanced hydraulic analysis may be required.
