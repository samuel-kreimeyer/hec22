# Rational Method for Runoff Calculation

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 2

## Rational Formula

The Rational Method is used to estimate peak discharge from small drainage areas (typically < 200 acres).

### Basic Equation

```
Q = C × i × A
```

**Where:**
- Q = Peak discharge (cfs)
- C = Runoff coefficient (dimensionless)
- i = Rainfall intensity (in/hr)
- A = Drainage area (acres)

## Runoff Coefficient (C)

The runoff coefficient represents the fraction of rainfall that becomes runoff.

### Runoff Coefficients by Land Use

| Surface/Land Use | C Value | Range |
|------------------|---------|-------|
| **Pavement and Roofs** |
| Asphalt pavement | 0.85 | 0.80 - 0.95 |
| Concrete pavement | 0.90 | 0.80 - 0.95 |
| Brick pavement | 0.85 | 0.75 - 0.85 |
| Roofs | 0.90 | 0.75 - 0.95 |
| **Lawns and Landscaping** |
| Lawns, sandy soil, flat (< 2%) | 0.10 | 0.05 - 0.10 |
| Lawns, sandy soil, average (2-7%) | 0.15 | 0.10 - 0.15 |
| Lawns, sandy soil, steep (> 7%) | 0.20 | 0.15 - 0.20 |
| Lawns, clay soil, flat (< 2%) | 0.17 | 0.13 - 0.17 |
| Lawns, clay soil, average (2-7%) | 0.22 | 0.18 - 0.22 |
| Lawns, clay soil, steep (> 7%) | 0.30 | 0.25 - 0.35 |
| **Developed Areas** |
| Business/Downtown | 0.85 | 0.70 - 0.95 |
| Industrial, light | 0.70 | 0.50 - 0.80 |
| Industrial, heavy | 0.80 | 0.60 - 0.90 |
| Residential, suburban | 0.40 | 0.25 - 0.40 |
| Residential, single-family | 0.50 | 0.30 - 0.50 |
| Residential, multi-unit detached | 0.60 | 0.40 - 0.60 |
| Residential, multi-unit attached | 0.70 | 0.60 - 0.75 |
| Apartment complexes | 0.65 | 0.50 - 0.70 |
| **Streets and Drives** |
| Gravel | 0.50 | 0.40 - 0.70 |
| Asphalt/Concrete | 0.85 | 0.70 - 0.95 |
| **Undeveloped Areas** |
| Pasture/Range | 0.25 | 0.10 - 0.40 |
| Forest/Woods | 0.15 | 0.10 - 0.25 |
| Cultivated land | 0.30 | 0.20 - 0.40 |

### Composite Runoff Coefficient

For areas with mixed land use:

```
C_composite = (C₁×A₁ + C₂×A₂ + ... + Cₙ×Aₙ) / A_total
```

**Where:**
- C₁, C₂, ..., Cₙ = Runoff coefficients for different subareas
- A₁, A₂, ..., Aₙ = Areas of different subareas (acres)
- A_total = Total drainage area (acres)

## Frequency Adjustment Factor (Cf)

For storm frequencies other than 10-year, adjust the C value:

```
C_adjusted = Cf × C
```

### Frequency Adjustment Factors

| Return Period | Cf Factor |
|---------------|-----------|
| 2 to 10 years | 1.00 |
| 25 years | 1.10 |
| 50 years | 1.20 |
| 100 years | 1.25 |

**Note:** Cf should only be applied when C is based on 10-year or less storm frequency.

## Rainfall Intensity (i)

Rainfall intensity is determined from Intensity-Duration-Frequency (IDF) curves.

### IDF Equation (General Form)

```
i = a / (Tc + b)^c
```

**Where:**
- i = Rainfall intensity (in/hr)
- Tc = Time of concentration (minutes)
- a, b, c = Regional coefficients from IDF curves

### Alternative IDF Equation

```
i = K × Tr^m / Tc^n
```

**Where:**
- Tr = Return period (years)
- K, m, n = Regional coefficients

## Time of Concentration (Tc)

Time of concentration is the time for runoff to travel from the most hydraulically distant point to the point of interest.

```
Tc = Ti + Tt
```

**Where:**
- Tc = Time of concentration (min)
- Ti = Inlet time (overland flow time) (min)
- Tt = Travel time in pipe/channel (min)

### Overland Flow Time (Inlet Time)

#### Kinematic Wave Equation

```
Ti = 0.93 × (n × L)^0.6 / (i^0.4 × S^0.3)
```

**Where:**
- Ti = Overland flow time (min)
- n = Manning's roughness coefficient
- L = Overland flow length (ft)
- i = Rainfall intensity (in/hr) - requires iteration
- S = Slope (ft/ft)

#### Kirpich Equation (for concentrated flow in channels)

```
Tt = 0.0078 × L^0.77 / S^0.385
```

**Where:**
- Tt = Travel time (min)
- L = Flow length (ft)
- S = Slope (ft/ft)

#### Velocity Method

```
Tt = L / (60 × V)
```

**Where:**
- Tt = Travel time (min)
- L = Flow length (ft)
- V = Velocity (ft/s)

### Minimum Time of Concentration

Recommended minimum Tc = 5 to 10 minutes (to avoid unrealistically high intensities)

## Design Procedure

### Step-by-Step Process

1. **Delineate drainage area**: Determine watershed boundaries and area A (acres)

2. **Determine land use**: Identify different surface types and their areas

3. **Select runoff coefficient**:
   - Assign C values to each surface type
   - Calculate composite C if needed
   - Apply frequency adjustment factor Cf if applicable

4. **Calculate time of concentration**:
   - Estimate overland flow time Ti
   - Calculate pipe/channel travel time Tt
   - Sum to get Tc
   - Check minimum Tc requirement

5. **Determine rainfall intensity**:
   - Use local IDF curves
   - Enter with Tc and design storm frequency
   - Read intensity i (in/hr)

6. **Calculate peak discharge**:
   ```
   Q = C × i × A
   ```

## Limitations

The Rational Method is subject to these limitations:

1. **Drainage area**: Best for areas < 200 acres (80 hectares)
2. **Uniform rainfall**: Assumes rainfall is uniform over the drainage area
3. **Peak flow only**: Provides peak discharge, not hydrograph
4. **Single outlet**: Designed for single outlet point
5. **Constant intensity**: Assumes rainfall intensity is constant during Tc

## Example Calculation

**Given:**
- Drainage area: 5.0 acres
- Land use: 60% impervious (parking lot), 40% lawn (clay, average slope)
- Design storm: 10-year
- Time of concentration: 15 minutes
- Rainfall intensity (from IDF): 4.5 in/hr

**Solution:**

1. Composite C:
   ```
   C = (0.85 × 0.60 × 5.0) + (0.22 × 0.40 × 5.0) / 5.0
   C = (2.55 + 0.44) / 5.0
   C = 0.598 ≈ 0.60
   ```

2. Peak discharge:
   ```
   Q = C × i × A
   Q = 0.60 × 4.5 × 5.0
   Q = 13.5 cfs
   ```

## Modified Rational Method

For detention design, use the Modified Rational Method which considers:
- Storage volumes
- Hydrograph shape
- Routing through detention

See HEC-22 Chapter 5 for detention design procedures.
