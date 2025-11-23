# HEC-22 Worked Examples - Extracted for Test Cases

This document contains all worked examples extracted from HEC-22 documentation, organized by chapter and formatted for use as test cases in software development.

**Purpose:** Provide verified test cases with known inputs and expected outputs for validation of HEC-22 calculation implementations.

**Source:** Extracted from HEC-22 4th Edition documentation and equation reference files.

---

## Table of Contents

1. [Chapter 9: Storm Drain Conduits](#chapter-9-storm-drain-conduits)
2. [Chapter 10: Detention and Retention](#chapter-10-detention-and-retention)
3. [Chapter 11: Urban Stormwater Quality](#chapter-11-urban-stormwater-quality)
4. [Chapter 12: Pump Stations](#chapter-12-pump-stations)
5. [Quick Reference Examples](#quick-reference-examples)

---

## Chapter 9: Storm Drain Conduits

### Example 9.1: Complete Storm Drain System Design

**Source:** `reference/examples/example_problem_1.md`

**Problem Statement:**
Design a storm drain system for a 200-foot section of two-lane highway with a 2% longitudinal grade. The pavement is crowned with 2% cross slopes on each side. Design for a 10-year storm event.

#### Inputs

**Site Data:**
- Roadway length: 200 ft
- Longitudinal slope (SL): 0.02 (2%)
- Cross slope (Sx): 0.02 (2%)
- Pavement type: Asphalt
- Manning's n for gutter: 0.016

**Hydrologic Data:**
- Design storm: 10-year
- Rainfall intensity (10 min Tc): 5.0 in/hr
- Drainage area to first inlet: 0.40 acres
- Land use: 80% impervious (pavement), 20% grass
- Soil: Clay

**Design Criteria:**
- Maximum allowable spread: 8 ft
- Pipe material: RCP (n = 0.013)
- Minimum pipe size: 18 inches
- Minimum pipe slope: 0.004
- Minimum cover: 2.0 ft

#### Expected Outputs

**Hydrologic Analysis:**
- Composite runoff coefficient (C): 0.73
- Design flow at first inlet (Q₁): 1.46 cfs

**Gutter Flow Analysis:**
- Spread at inlet (T): 4.34 ft
- Depth at curb (d): 1.04 inches
- Check: T < 8 ft (allowable) ✓

**Inlet Design (Combination: 5-ft curb + 2'×2' grate):**
- Length for 100% interception (Lt): 116.8 ft
- Curb opening efficiency: 7.6%
- Grate efficiency: 92%
- Combined efficiency (with clogging factor 0.80): 73%
- Intercepted flow: 1.07 cfs
- Bypass flow: 0.39 cfs

**Pipe C-1 (IN-1 to MH-1):**
- Diameter: 18 inches
- Length: 100 ft
- Slope: 0.025 (2.5%)
- Upstream invert: 122.0 ft
- Downstream invert: 119.5 ft
- Design flow: 1.07 cfs
- Full capacity: 15.6 cfs
- Velocity: ~4.4 ft/s

**Pipe C-2 (MH-1 to OUT-1):**
- Diameter: 18 inches
- Length: 100 ft
- Slope: 0.020 (2.0%)
- Upstream invert: 119.5 ft
- Downstream invert: 117.5 ft
- Design flow: 3.28 cfs
- Full capacity: 15.6 cfs
- Velocity: ~4.0 ft/s

**HGL Analysis:**
- HGL at IN-1: 122.45 ft (below rim 125.0 ft) ✓
- HGL at MH-1: 120.1 ft
- No flooding occurs

#### Calculation Steps

**Step 1: Composite Runoff Coefficient**
```
C_pavement = 0.85
C_grass = 0.22
C_composite = (0.85 × 0.80 × 0.40 + 0.22 × 0.20 × 0.40) / 0.40
C_composite = (0.272 + 0.0176) / 0.40 = 0.725 ≈ 0.73
```

**Step 2: Peak Discharge (Rational Method)**
```
Q = C × i × A
Q = 0.73 × 5.0 × 0.40 = 1.46 cfs
```

**Step 3: Gutter Spread**
```
T = [Q × n / (Ku × Sx^(5/3) × SL^(1/2))]^(3/8)
T = [1.46 × 0.016 / (0.56 × 0.02^(5/3) × 0.02^(1/2))]^(3/8)
T = [0.02336 / 0.000583]^(3/8)
T = 40.07^(3/8) = 4.34 ft
```

**Step 4: Curb Opening Length for 100% Interception**
```
Lt = 0.6 × Q^0.42 × SL^0.3 / (n × Sx^0.6)
Lt = 0.6 × 1.46^0.42 × 0.02^0.3 / (0.016 × 0.02^0.6)
Lt = 0.236 / 0.00202 = 116.8 ft
```

**Step 5: Curb Opening Efficiency**
```
E_curb = 1 - (1 - L/Lt)^1.8
E_curb = 1 - (1 - 5/116.8)^1.8
E_curb = 1 - 0.957^1.8 = 0.076 (7.6%)
```

**Step 6: Pipe Full Capacity (Manning's)**
```
Q_full = 0.463 × D^(8/3) / n × S^(1/2)
Q_full = 0.463 × 1.5^(8/3) / 0.013 × 0.025^(1/2)
Q_full = 98.8 × 0.158 = 15.6 cfs
```

---

### Example 9.2: Rational Method Calculation

**Source:** `reference/equations/rational_method.md:209-232`

#### Inputs
- Drainage area: 5.0 acres
- Land use: 60% impervious (parking lot), 40% lawn (clay, average slope)
- Design storm: 10-year
- Time of concentration: 15 minutes
- Rainfall intensity (from IDF): 4.5 in/hr
- C_impervious: 0.85
- C_lawn: 0.22

#### Expected Outputs
- Composite C: 0.60
- Peak discharge (Q): 13.5 cfs

#### Calculation Steps

**Step 1: Composite C**
```
C = (0.85 × 0.60 × 5.0 + 0.22 × 0.40 × 5.0) / 5.0
C = (2.55 + 0.44) / 5.0
C = 2.99 / 5.0 = 0.598 ≈ 0.60
```

**Step 2: Peak Discharge**
```
Q = C × i × A
Q = 0.60 × 4.5 × 5.0
Q = 13.5 cfs
```

---

### Example 9.3: Gutter Flow Spread Calculation

**Source:** `reference/equations/gutter_flow.md:143-166`

#### Inputs
- Flow rate (Q): 2.5 cfs
- Longitudinal slope (SL): 0.02 (2%)
- Cross slope (Sx): 0.02 (2%)
- Manning's n: 0.016
- Ku (US units): 0.56

#### Expected Outputs
- Spread (T): 5.8 ft
- Depth at curb (d): 1.4 inches

#### Calculation Steps

**Step 1: Calculate Spread**
```
T = [Q × n / (Ku × Sx^(5/3) × SL^(1/2))]^(3/8)
T = [2.5 × 0.016 / (0.56 × 0.02^(5/3) × 0.02^(1/2))]^(3/8)
T = [0.04 / (0.56 × 0.00736 × 0.1414)]^(3/8)
T = [0.04 / 0.000583]^(3/8)
T = 68.6^(3/8) ≈ 5.8 ft
```

**Step 2: Depth at Curb**
```
d = Sx × T
d = 0.02 × 5.8 = 0.116 ft ≈ 1.4 inches
```

---

### Example 9.4: Curb-Opening Inlet on Grade

**Source:** `reference/equations/inlet_design.md:234-260`

#### Inputs
- Flow rate (Q): 3.0 cfs
- Longitudinal slope (SL): 0.02
- Cross slope (Sx): 0.02
- Manning's n: 0.016
- Inlet length (L): 5 ft

#### Expected Outputs
- Length for 100% interception (Lt): 156 ft
- Efficiency (E): 5.5%
- Interpretation: Single 5-ft inlet intercepts only 5.5% of flow

#### Calculation Steps

**Step 1: Length for 100% Interception**
```
Lt = 0.6 × Q^0.42 × SL^0.3 / (n × Sx^0.6)
Lt = 0.6 × 3.0^0.42 × 0.02^0.3 / (0.016 × 0.02^0.6)
Lt = 0.6 × 1.538 × 0.342 / (0.016 × 0.126)
Lt = 0.316 / 0.00202 = 156 ft
```

**Step 2: Efficiency for Given Length**
```
E = 1 - (1 - L/Lt)^1.8
E = 1 - (1 - 5/156)^1.8
E = 1 - 0.968^1.8
E = 1 - 0.945 = 0.055 (5.5%)
```

---

## Chapter 10: Detention and Retention

### Example 10.1: Preliminary Detention Basin Sizing

**Source:** `reference/equations/detention_retention.md:353-384`

#### Inputs
- Drainage area: 38 acres
- Pre-development peak: 50 ft³/s
- Post-development peak: 131 ft³/s
- Design storm: 0.1 AEP (10-year)
- Target release: 50 ft³/s
- Time of concentration (Tc): 20 minutes = 1,200 seconds
- Basin dimensions: L = 100 ft, W = 50 ft, side slope Z = 4:1

#### Expected Outputs
- Time of inflow (ti): 2,400 seconds
- Preliminary storage volume (Vs): 97,200 ft³
- Required depth (D): ~4.5 ft (iterative solution)

#### Calculation Steps

**Step 1: Preliminary Storage (Triangular Hydrograph)**
```
t_i = 2 × T_c = 2 × 1,200 = 2,400 seconds
V_s = 0.5 × 2,400 × (131 - 50)
V_s = 0.5 × 2,400 × 81
V_s = 97,200 ft³
```

**Step 2: Basin Sizing (Trapezoidal)**
```
V = L×W×D + (L+W)×Z×D² + (4/3)×Z²×D³
97,200 = 100×50×D + (100+50)×4×D² + (4/3)×16×D³
97,200 = 5,000D + 600D² + 21.33D³
```

**Step 3: Solve Iteratively**
```
D ≈ 4.5 ft
```

**Note:** This is preliminary. Verify with storage routing and iterate on outlet structure sizing.

---

### Example 10.2: Simple Detention Basin (Validation Problem)

**Source:** `reference/guidance/chapter_10_design_notes.md:888-902`

#### Inputs
- Drainage area: 38 acres
- Pre-development peak: 50 ft³/s
- Post-development peak: 131 ft³/s
- Design storm: 0.1 AEP

#### Expected Results
- Preliminary storage: 28,000-32,000 ft³
- Weir length: ~1.6 ft (after iteration)
- Peak outflow: ~50 ft³/s
- Maximum stage: ~4.5 ft

**Note:** See HEC-22 Example 10.14 for complete solution

---

### Example 10.3: Water Budget Analysis (Validation Problem)

**Source:** `reference/guidance/chapter_10_design_notes.md:904-921`

#### Inputs
- Drainage area: 100 acres
- Pool surface area: 3 acres
- Pool bottom area: 2 acres
- Runoff coefficient: 0.3
- Annual rainfall: 50 inches
- Annual evaporation: 35 inches
- Infiltration rate: 0.1 in/hr

#### Expected Results
- Annual runoff: 5,445,000 ft³
- Annual evaporation: 381,150 ft³
- Annual infiltration: 6,359,760 ft³
- Net budget: -1,295,910 ft³ (does not maintain pool)

#### Interpretation
The retention pond will not maintain its permanent pool with these parameters. Need to either:
- Increase drainage area
- Reduce infiltration (liner)
- Provide supplemental water source

**Note:** See HEC-22 Example 10.13 for complete solution

---

## Chapter 11: Urban Stormwater Quality

### Example 11.1: Bioretention Basin Design

**Source:** `reference/equations/stormwater_quality.md:256-312`

#### Inputs
- Drainage area: 10 acres
- Imperviousness: 75%
- Design rainfall (P): 1.0 inch (water quality storm)
- Annual rainfall: 40 inches
- TSS concentration: 100 mg/L
- BMP: Bioretention basin
- Media depth: 3 ft
- Media porosity: 0.25
- Infiltration rate: 2 ft/hr
- Drawdown time: 24 hours

#### Expected Outputs
- Volumetric runoff coefficient (Rv): 0.725
- Water quality volume (WQv): 26,318 ft³
- Annual runoff: 26.1 inches
- Annual TSS loading: 5,899 lb/yr
- Bioretention basin surface area: 9,570 ft² (0.22 acres)
- TSS removal: 5,014 lb/yr (85% efficiency)

#### Calculation Steps

**Step 1: Volumetric Runoff Coefficient**
```
Rv = 0.05 + 0.009 × I
Rv = 0.05 + 0.009 × 75
Rv = 0.05 + 0.675 = 0.725
```

**Step 2: Water Quality Volume**
```
WQv = Rv × P × A × 3,630
WQv = 0.725 × 1.0 × 10 × 3,630
WQv = 26,318 ft³
```

**Step 3: Annual Pollutant Loading (TSS)**
```
R = Annual runoff = 40 × 0.9 × 0.725 = 26.1 inches
L = c × R × C × A
L = 0.226 × 26.1 × 100 × 10
L = 5,899 lb/yr TSS
```

**Step 4: Size Bioretention Basin**
```
V_bio = WQv / [(n × d) + (K × t)]
V_bio = 26,318 / [(0.25 × 3) + (2 × 24/24)]
V_bio = 26,318 / [0.75 + 2]
V_bio = 26,318 / 2.75
V_bio = 9,570 ft²
```

**Step 5: TSS Removal**
```
L_removed = 5,899 × 0.85 = 5,014 lb/yr TSS
```

---

## Chapter 12: Pump Stations

### Example 12.1: Pump Station Design

**Source:** `reference/equations/pump_stations.md:376-436`

#### Inputs
- Design discharge (Q): 10 ft³/s
- Static head (Hs): 15 ft
- Discharge pipe: 12-inch diameter, 200 ft long
- Pipe material: PVC (n = 0.011)
- Fittings:
  - 2 elbows (K = 0.9 each)
  - 1 check valve (K = 2.5)
  - 1 gate valve (K = 0.2)
  - Exit loss (K = 1.0)
- Pump efficiency: 75%

#### Expected Outputs
- Pipe area: 0.785 ft²
- Velocity: 12.7 ft/s
- Friction loss (Hf): 4.96 ft
- Velocity head (Hv): 2.5 ft
- Minor losses (Hl): 13.8 ft
- Total Dynamic Head (TDH): 36.3 ft
- Water Horsepower (WHP): 41.3 hp
- Brake Horsepower (BHP): 55 hp
- Motor selection: 60 hp (next standard size)

#### Calculation Steps

**Step 1: Velocity**
```
D = 12 in = 1.0 ft
A = π × D² / 4 = π × 1.0² / 4 = 0.785 ft²
V = Q / A = 10 / 0.785 = 12.7 ft/s
```

**Step 2: Friction Loss (Manning's)**
```
R = D / 4 = 1.0 / 4 = 0.25 ft
Hf = (n² × L × V²) / (2.22 × R^(4/3))
Hf = (0.011² × 200 × 12.7²) / (2.22 × 0.25^(4/3))
Hf = (0.000121 × 200 × 161.3) / (2.22 × 0.354)
Hf = 3.90 / 0.786 = 4.96 ft
```

**Step 3: Velocity Head**
```
Hv = V² / (2g)
Hv = 12.7² / (2 × 32.2)
Hv = 161.3 / 64.4 = 2.5 ft
```

**Step 4: Minor Losses**
```
K_total = 2(0.9) + 2.5 + 0.2 + 1.0 = 5.5
Hl = K_total × (V² / 2g)
Hl = 5.5 × (12.7² / 64.4)
Hl = 5.5 × 2.5 = 13.8 ft
```

**Step 5: Total Dynamic Head**
```
TDH = Hs + Hf + Hv + Hl
TDH = 15 + 4.96 + 2.5 + 13.8 = 36.3 ft
```

**Step 6: Horsepower Requirements**
```
WHP = (Q × TDH) / 8.8
WHP = (10 × 36.3) / 8.8 = 41.3 hp

BHP = WHP / efficiency
BHP = 41.3 / 0.75 = 55 hp
```

**Step 7: Motor Selection**
Select 60 hp motor (next standard size above 55 hp)

---

## Quick Reference Examples

### Gutter Flow Equations

| Parameter | Example Value | Formula | Result |
|-----------|--------------|---------|--------|
| Q (cfs) | 2.5 | Given | 2.5 |
| n | 0.016 | Given | 0.016 |
| Sx | 0.02 | Given | 0.02 |
| SL | 0.02 | Given | 0.02 |
| Ku | 0.56 | Constant | 0.56 |
| T (ft) | - | [Q×n/(Ku×Sx^(5/3)×SL^(1/2))]^(3/8) | 5.8 |
| d (in) | - | Sx × T × 12 | 1.4 |

### Rational Method Quick Check

| Area (ac) | C | i (in/hr) | Q (cfs) |
|-----------|---|-----------|---------|
| 1.0 | 0.50 | 3.0 | 1.5 |
| 5.0 | 0.60 | 4.5 | 13.5 |
| 10.0 | 0.80 | 5.0 | 40.0 |
| 25.0 | 0.70 | 4.0 | 70.0 |

### Pipe Capacity (Manning's) Quick Reference

**n = 0.013 (RCP), S = 0.01 (1%)**

| Diameter (in) | D (ft) | Q_full (cfs) | V_full (ft/s) |
|---------------|--------|--------------|---------------|
| 15 | 1.25 | 9.4 | 7.7 |
| 18 | 1.50 | 15.6 | 8.8 |
| 24 | 2.00 | 32.8 | 10.4 |
| 30 | 2.50 | 58.8 | 12.0 |
| 36 | 3.00 | 95.5 | 13.5 |

---

## Usage Notes for Test Development

### Tolerance Recommendations

When implementing these examples as test cases, use appropriate tolerances:

- **Flow rates (Q):** ±1% or ±0.01 cfs (whichever is larger)
- **Dimensions (spread, depth):** ±2% or ±0.1 ft/in
- **Efficiencies:** ±2%
- **Volumes:** ±3%
- **Velocities:** ±5%
- **HGL elevations:** ±0.1 ft
- **Intermediate calculations:** May vary slightly due to rounding

### Calculation Precision

- Use at least 3 significant figures for intermediate calculations
- Final answers may be rounded to 2-3 significant figures
- Exponents: Use exact values (5/3, 8/3, 0.42, 1.8, etc.)
- Constants: Ku = 0.56, g = 32.2 ft/s², π = 3.14159...

### Common Constants

```python
# US Customary Units
Ku = 0.56              # Gutter flow constant
g = 32.2               # ft/s²
alpha = 3630           # Volume conversion
C = 0.226              # Pollutant loading coefficient

# Manning's n values
n_concrete = 0.016     # Concrete gutter
n_RCP = 0.013          # Reinforced concrete pipe
n_PVC = 0.011          # PVC pipe
n_CMP = 0.024          # Corrugated metal pipe
```

### Test Case Structure

Each test should include:
1. **Test ID:** Unique identifier
2. **Description:** What is being tested
3. **Inputs:** All required parameters with units
4. **Expected Output:** Known correct result
5. **Tolerance:** Acceptable variance
6. **Source:** Reference to HEC-22 section/example
7. **Dependencies:** Any prerequisite calculations

### Example Test Implementation

```python
def test_rational_method_example_9_2():
    """Test Case: Rational Method - Example 9.2

    Source: HEC-22 Ch.9, reference/equations/rational_method.md:209-232
    """
    # Inputs
    area = 5.0  # acres
    c_impervious = 0.85
    c_lawn = 0.22
    pct_impervious = 0.60
    intensity = 4.5  # in/hr

    # Calculate composite C
    c_composite = (c_impervious * pct_impervious +
                   c_lawn * (1 - pct_impervious))

    # Calculate Q
    q = c_composite * intensity * area

    # Expected results
    assert abs(c_composite - 0.60) < 0.01, f"C = {c_composite}, expected 0.60"
    assert abs(q - 13.5) < 0.1, f"Q = {q}, expected 13.5 cfs"
```

---

## Additional Resources

### Complete Examples in HEC-22 Manual

These worked examples reference complete solutions in HEC-22:
- Example 10.1-10.14: Detention basin design (various configurations)
- Example 10.13: Water budget analysis
- Example 10.14: Simple detention basin

### Cross-References

- **TEST_CASE_REFERENCE.md:** 54+ formula-level test cases
- **example_problem_1.md:** Complete storm drain design
- **Equation files:** Detailed formula documentation
- **Design notes:** Implementation guidance and Python examples

---

**Document Version:** 1.0
**Extraction Date:** November 2025
**Extracted By:** Automated extraction from HEC-22 documentation
**Status:** Ready for test case implementation
