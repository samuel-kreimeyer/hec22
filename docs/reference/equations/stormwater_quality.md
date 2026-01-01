# Stormwater Quality Equations

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 11

## Pollutant Load Estimation

### Simple Method for Pollutant Loading

```
L = c × R × C × A
```

**Where:**
- L = Average annual loading (lb for chemicals, billion colonies for bacteria)
- c = Unit conversion factor
  - c = 0.226 for chemical pollutants
  - c = 1000 for bacteria
- R = Annual runoff depth (inches)
- C = Pollutant concentration
  - mg/L for chemical pollutants
  - 1000/mL for bacteria
- A = Drainage area (acres)

**Reference:** Equation 11.1

**Applicability:**
- Sites less than 1 mi² (640 acres)
- Provides average annual loading estimate
- Requires local pollutant concentration data
- Simple screening-level analysis

### Annual Runoff Depth Estimation

For use in Simple Method:
```
R = P × Pj × Rv
```

**Where:**
- R = Annual runoff depth (inches)
- P = Annual precipitation (inches)
- Pj = Fraction of annual rainfall events that produce runoff (typically 0.9)
- Rv = Volumetric runoff coefficient

**Volumetric Runoff Coefficient:**
```
Rv = 0.05 + 0.009 × I
```

**Where:**
- I = Percent imperviousness of drainage area

## Water Quality Volume (WQV)

### Water Quality Volume Calculation

```
WQv = Q × A = (Rv × P) × A
```

**Where:**
- WQv = Water quality volume (ft³)
- Q = Depth of runoff to be treated (inches)
- Rv = Volumetric runoff coefficient
- P = Rainfall depth for water quality design storm (inches)
- A = Drainage area (acres)

**Reference:** Equation 11.2

**Conversion to cubic feet:**
```
WQv (ft³) = (Rv × P × A) × 3,630
```

Where 3,630 is the conversion factor from acre-inches to cubic feet.

**Common Design Rainfall Depths:**
- P = 0.5 inch (first flush - common for impervious areas)
- P = 0.5 inch over entire catchment
- P = 1.0 inch over entire catchment

**Note:** Treating volumes greater than 1.0 inch provides only minor improvement in pollutant removal efficiency.

### Volumetric Runoff Coefficient (Rv)

**Simple Estimation:**
```
Rv = 0.05 + 0.009 × I
```

**Where:**
- Rv = Volumetric runoff coefficient (dimensionless)
- I = Percent imperviousness (0-100)

**Detailed Estimation (by land cover):**
```
Rv = Σ(Rv_i × A_i) / A_total
```

**Where:**
- Rv_i = Runoff coefficient for land cover type i
- A_i = Area of land cover type i (acres)
- A_total = Total drainage area (acres)

**Typical Rv Values by Land Cover:**

| Land Cover Type | Rv |
|-----------------|-----|
| Forest/Undisturbed | 0.05 - 0.15 |
| Meadow/Pasture | 0.10 - 0.25 |
| Cultivated land | 0.20 - 0.35 |
| Residential (0.5 acre lots) | 0.30 - 0.40 |
| Residential (0.25 acre lots) | 0.40 - 0.50 |
| Residential (townhouse) | 0.50 - 0.60 |
| Commercial/Industrial | 0.70 - 0.85 |
| Impervious surfaces | 0.85 - 0.95 |

## BMP Sizing

### Extended Detention Dry Pond Volume

```
V_wq = WQv / (1 - f)
```

**Where:**
- V_wq = Required basin volume for water quality (ft³)
- WQv = Water quality volume (ft³)
- f = Porosity of basin (typically 0 for dry ponds)

**For extended detention:**
- Detention time: 24-48 hours typical
- Release rate controlled by low-flow orifice

### Wet Pond (Retention) Permanent Pool Volume

**Minimum permanent pool volume:**
```
V_pp = 2 × WQv
```

**Where:**
- V_pp = Permanent pool volume (ft³)
- WQv = Water quality volume (ft³)

**Recommended pool depth:**
- Average depth: 3-6 feet
- Maximum depth: 8-10 feet
- Minimum depth: 2.5 feet (avoid stagnation)

### Infiltration/Exfiltration Trench Volume

```
V_trench = WQv / n
```

**Where:**
- V_trench = Required trench volume (ft³)
- WQv = Water quality volume (ft³)
- n = Porosity of trench media (typically 0.3-0.4 for stone)

**Trench drawdown time:**
```
t_drain = (V_trench × n) / (K × A_bottom)
```

**Where:**
- t_drain = Drawdown time (hours)
- K = Infiltration rate of native soil (ft/hr)
- A_bottom = Bottom area of trench (ft²)
- n = Porosity

**Design requirement:** Typically t_drain ≤ 72 hours

### Bioretention/Rain Garden Volume

```
V_bio = WQv / [(n × d) + (K × t)]
```

**Where:**
- V_bio = Required surface area of bioretention (ft²)
- WQv = Water quality volume (ft³)
- n = Porosity of bioretention media (typically 0.25)
- d = Depth of bioretention media (ft, typically 2-4 ft)
- K = Infiltration rate (ft/hr)
- t = Drawdown time (hours, typically 24-48)

## Pollutant Removal Efficiency

### First-Order Removal Model

```
C_out = C_in × exp(-k × t / d)
```

**Where:**
- C_out = Outlet concentration (mg/L)
- C_in = Inlet concentration (mg/L)
- k = First-order removal rate constant (ft/yr)
- t = Detention time (years)
- d = Average depth (ft)

### Mass Balance for Pollutant Removal

```
L_removed = L_in × E
```

**Where:**
- L_removed = Annual pollutant load removed (lb/yr)
- L_in = Annual pollutant load entering BMP (lb/yr)
- E = Pollutant removal efficiency (fraction)

**Load entering BMP:**
```
L_in = c × R × C_in × A
```

Using Simple Method (Equation 11.1)

## Typical Pollutant Concentrations

**Highway/Urban Runoff Event Mean Concentrations (EMC):**

| Pollutant | Typical Range (mg/L) | Median (mg/L) |
|-----------|----------------------|---------------|
| Total Suspended Solids (TSS) | 20 - 500 | 78 |
| Total Phosphorus (TP) | 0.1 - 2.0 | 0.26 |
| Total Nitrogen (TN) | 1.0 - 10.0 | 2.0 |
| Total Lead (Pb) | 0.01 - 0.5 | 0.11 |
| Total Zinc (Zn) | 0.05 - 0.6 | 0.16 |
| Total Copper (Cu) | 0.01 - 0.2 | 0.034 |
| Chemical Oxygen Demand (COD) | 20 - 200 | 65 |
| Oil and Grease | 1 - 15 | 5 |

**Note:** Concentrations vary significantly by location, land use, and traffic volume. Use local data when available.

## BMP Pollutant Removal Efficiencies

**Typical Removal Efficiencies by BMP Type:**

| BMP Type | TSS | TP | TN | Metals (Zn, Cu, Pb) |
|----------|-----|----|----|---------------------|
| Extended Detention (24-hr) | 60-90% | 20-40% | 10-30% | 40-70% |
| Wet Pond (Retention) | 70-90% | 40-70% | 30-50% | 50-80% |
| Infiltration Trench | 80-95% | 50-70% | 40-65% | 70-90% |
| Bioretention/Rain Garden | 80-95% | 60-80% | 40-60% | 70-90% |
| Sand Filter | 70-90% | 30-50% | 20-40% | 50-70% |
| Grassed Swale | 40-70% | 20-40% | 10-30% | 30-60% |
| Filter Strip | 30-60% | 10-30% | 10-20% | 20-50% |

**Note:** Actual performance depends on design, maintenance, and site conditions.

## Example Calculation

**Given:**
- Drainage area: 10 acres
- Imperviousness: 75%
- Design rainfall: P = 1.0 inch (water quality storm)
- Annual rainfall: 40 inches
- TSS concentration: 100 mg/L
- BMP: Bioretention basin

**Step 1: Calculate Volumetric Runoff Coefficient**
```
Rv = 0.05 + 0.009 × I
Rv = 0.05 + 0.009 × 75
Rv = 0.05 + 0.675
Rv = 0.725
```

**Step 2: Calculate Water Quality Volume**
```
WQv = Rv × P × A × 3,630
WQv = 0.725 × 1.0 × 10 × 3,630
WQv = 26,318 ft³
```

**Step 3: Calculate Annual Pollutant Loading (TSS)**
```
R = Annual runoff = 40 × 0.9 × 0.725 = 26.1 inches
L = c × R × C × A
L = 0.226 × 26.1 × 100 × 10
L = 5,899 lb/yr TSS
```

**Step 4: Size Bioretention Basin**

Assuming:
- Media depth: 3 ft
- Media porosity: 0.25
- Infiltration rate: 2 ft/hr
- Drawdown time: 24 hours

```
V_bio = WQv / [(n × d) + (K × t)]
V_bio = 26,318 / [(0.25 × 3) + (2 × 24/24)]
V_bio = 26,318 / [0.75 + 2]
V_bio = 26,318 / 2.75
V_bio = 9,570 ft²
```

Surface area needed ≈ 9,570 ft² (approximately 0.22 acres)

**Step 5: Estimate TSS Removal**

Using 85% removal efficiency for bioretention:
```
L_removed = 5,899 × 0.85 = 5,014 lb/yr TSS removed
```

## Design Considerations

### First Flush Concept

- Most pollutants are washed off in the initial period of runoff
- First 0.5 to 1.0 inch of rainfall captures majority of pollutant load
- Designing for first flush is cost-effective approach

### Treatment Train Approach

Multiple BMPs in series provide enhanced treatment:
```
E_total = 1 - (1 - E₁) × (1 - E₂) × ... × (1 - Eₙ)
```

**Where:**
- E_total = Combined removal efficiency
- E₁, E₂, ..., Eₙ = Individual BMP efficiencies

### Maintenance Impact

Pollutant removal efficiency decreases without proper maintenance:
- Regular sediment removal
- Vegetation management
- Inspection and repair
- Typical maintenance reduction: 20-50% loss in efficiency

## SI Units Conversion

For SI units (metric):
- Use c = 10.0 for chemical pollutants (kg output)
- Use c = 10,000 for bacteria (billion colonies output)
- R in mm, C in mg/L, A in hectares
- For WQv: multiply by 10 to convert ha-mm to m³

## References

- HEC-22 Chapter 11 (FHWA 2024): Urban Stormwater Quality
- FHWA Highway Runoff Database (HRDB)
- SELDM: Stochastic Empirical Loading and Dilution Model
- EPA SWMM: Stormwater Management Model
- State DOT BMP design manuals
