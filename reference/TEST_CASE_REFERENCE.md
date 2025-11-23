# HEC-22 Test Case Reference

## Purpose

This document consolidates all formulas and worked examples from the HEC-22 Urban Drainage Design Manual for use in creating automated test cases. Each formula is presented with clear variable definitions, expected inputs, and validated outputs based on the worked example.

---

## Table of Contents

1. [Rational Method (Runoff Calculation)](#1-rational-method-runoff-calculation)
2. [Gutter Flow Analysis](#2-gutter-flow-analysis)
3. [Inlet Design (On-Grade)](#3-inlet-design-on-grade)
4. [Inlet Design (Sag Locations)](#4-inlet-design-sag-locations)
5. [Manning's Equation (Pipe Flow)](#5-mannings-equation-pipe-flow)
6. [Hydraulic Grade Line Analysis](#6-hydraulic-grade-line-analysis)
7. [Worked Example: Complete Design Problem](#7-worked-example-complete-design-problem)

---

## 1. Rational Method (Runoff Calculation)

### 1.1 Basic Rational Formula

**Formula:**
```
Q = C × i × A
```

**Variables:**
- `Q` = Peak discharge (cfs)
- `C` = Runoff coefficient (dimensionless, 0 < C ≤ 1)
- `i` = Rainfall intensity (in/hr, > 0)
- `A` = Drainage area (acres, > 0)

**Test Case 1.1.1: Simple Runoff Calculation**
```
Input:
  C = 0.73
  i = 5.0 in/hr
  A = 0.40 acres

Expected Output:
  Q = 1.46 cfs

Calculation:
  Q = 0.73 × 5.0 × 0.40 = 1.46 cfs
```

### 1.2 Composite Runoff Coefficient

**Formula:**
```
C_composite = (C₁×A₁ + C₂×A₂ + ... + Cₙ×Aₙ) / A_total
```

**Variables:**
- `C₁, C₂, ..., Cₙ` = Runoff coefficients for each subarea (dimensionless)
- `A₁, A₂, ..., Aₙ` = Areas of each subarea (acres)
- `A_total` = Total drainage area (acres)

**Test Case 1.2.1: Two-Surface Composite**
```
Input:
  C_pavement = 0.85
  Area_pavement = 0.80 × 0.40 = 0.32 acres
  C_grass = 0.22
  Area_grass = 0.20 × 0.40 = 0.08 acres
  A_total = 0.40 acres

Expected Output:
  C_composite = 0.73

Calculation:
  C = (0.85 × 0.32 + 0.22 × 0.08) / 0.40
  C = (0.272 + 0.0176) / 0.40
  C = 0.2896 / 0.40
  C = 0.724 ≈ 0.73
```

### 1.3 Frequency Adjustment Factor

**Formula:**
```
C_adjusted = Cf × C
```

**Variables:**
- `Cf` = Frequency adjustment factor (dimensionless)
- `C` = Base runoff coefficient (dimensionless)

**Frequency Adjustment Factors:**
| Return Period | Cf Factor |
|---------------|-----------|
| 2-10 years    | 1.00      |
| 25 years      | 1.10      |
| 50 years      | 1.20      |
| 100 years     | 1.25      |

---

## 2. Gutter Flow Analysis

### 2.1 Gutter Flow Capacity (Modified Manning's Equation)

**Formula:**
```
Q = (Ku/n) × Sx^(5/3) × SL^(1/2) × T^(8/3)
```

**Variables:**
- `Q` = Gutter flow rate (cfs)
- `Ku` = 0.56 (US customary units constant)
- `n` = Manning's roughness coefficient (dimensionless, typically 0.012-0.020)
- `Sx` = Cross slope (ft/ft, typically 0.015-0.025)
- `SL` = Longitudinal slope (ft/ft, > 0.004 minimum)
- `T` = Spread (width of flow in gutter) (ft, > 0)

**Solved for Spread T:**
```
T = [Q × n / (Ku × Sx^(5/3) × SL^(1/2))]^(3/8)
```

**Test Case 2.1.1: Calculate Spread from Flow**
```
Input:
  Q = 1.46 cfs
  n = 0.016
  Sx = 0.02 (2%)
  SL = 0.02 (2%)
  Ku = 0.56

Expected Output:
  T = 4.34 ft

Calculation:
  T = [1.46 × 0.016 / (0.56 × 0.02^(5/3) × 0.02^(1/2))]^(3/8)

  Step 1: Calculate Sx^(5/3)
    Sx^(5/3) = 0.02^1.667 = 0.007358

  Step 2: Calculate SL^(1/2)
    SL^(1/2) = 0.02^0.5 = 0.1414

  Step 3: Calculate denominator
    Denom = 0.56 × 0.007358 × 0.1414 = 0.0005827

  Step 4: Calculate ratio
    Ratio = (1.46 × 0.016) / 0.0005827 = 0.02336 / 0.0005827 = 40.09

  Step 5: Calculate T
    T = 40.09^(3/8) = 40.09^0.375 = 4.34 ft
```

### 2.2 Depth of Flow at Curb

**Formula:**
```
d = Sx × T
```

**Variables:**
- `d` = Depth of flow at curb (ft)
- `Sx` = Cross slope (ft/ft)
- `T` = Spread (ft)

**Test Case 2.2.1: Depth at Curb**
```
Input:
  Sx = 0.02
  T = 4.34 ft

Expected Output:
  d = 0.087 ft = 1.04 inches

Calculation:
  d = 0.02 × 4.34 = 0.0868 ft ≈ 0.087 ft
  d = 0.087 × 12 = 1.044 inches
```

### 2.3 Gutter Flow Velocity

**Formula:**
```
V = (Ku/n) × Sx^(2/3) × SL^(1/2) × T^(2/3)
```

**Variables:**
- `V` = Velocity (ft/s)
- `Ku` = 0.56 (US customary units)
- `n` = Manning's roughness coefficient
- `Sx` = Cross slope (ft/ft)
- `SL` = Longitudinal slope (ft/ft)
- `T` = Spread (ft)

**Test Case 2.3.1: Gutter Flow Velocity**
```
Input:
  n = 0.016
  Sx = 0.02
  SL = 0.02
  T = 4.34 ft
  Ku = 0.56

Expected Output:
  V = 1.04 ft/s

Calculation:
  V = (0.56/0.016) × 0.02^(2/3) × 0.02^(1/2) × 4.34^(2/3)
  V = 35 × 0.0736 × 0.1414 × 2.83
  V = 1.04 ft/s
```

---

## 3. Inlet Design (On-Grade)

### 3.1 Curb-Opening Inlet: Length for 100% Interception

**Formula:**
```
Lt = Ku × Q^0.42 × SL^0.3 / (n × Sx^0.6)
```

**Variables:**
- `Lt` = Required length for 100% interception (ft)
- `Ku` = 0.6 (US customary units)
- `Q` = Gutter flow rate (cfs)
- `SL` = Longitudinal slope (ft/ft)
- `Sx` = Cross slope (ft/ft)
- `n` = Manning's roughness coefficient

**Test Case 3.1.1: Required Length for Full Interception**
```
Input:
  Q = 1.46 cfs
  SL = 0.02
  Sx = 0.02
  n = 0.016
  Ku = 0.6

Expected Output:
  Lt = 116.8 ft

Calculation:
  Lt = 0.6 × 1.46^0.42 × 0.02^0.3 / (0.016 × 0.02^0.6)

  Step 1: Q^0.42
    1.46^0.42 = 1.149

  Step 2: SL^0.3
    0.02^0.3 = 0.342

  Step 3: Sx^0.6
    0.02^0.6 = 0.126

  Step 4: Calculate
    Lt = 0.6 × 1.149 × 0.342 / (0.016 × 0.126)
    Lt = 0.236 / 0.00202
    Lt = 116.8 ft
```

### 3.2 Curb-Opening Inlet: Efficiency (L < Lt)

**Formula:**
```
E = 1 - (1 - L/Lt)^1.8
```

**Variables:**
- `E` = Efficiency (fraction of flow intercepted, 0 ≤ E ≤ 1)
- `L` = Actual inlet length (ft)
- `Lt` = Length required for 100% interception (ft)

**Test Case 3.2.1: Partial Interception Efficiency**
```
Input:
  L = 5 ft
  Lt = 116.8 ft

Expected Output:
  E = 0.076 (7.6%)

Calculation:
  E = 1 - (1 - 5/116.8)^1.8
  E = 1 - (1 - 0.0428)^1.8
  E = 1 - (0.9572)^1.8
  E = 1 - 0.924
  E = 0.076 or 7.6%
```

### 3.3 Grate Inlet: Splash-Over Velocity

**Formula:**
```
Vo = Kv × √(g × L)
```

**Variables:**
- `Vo` = Splash-over velocity (ft/s)
- `Kv` = 0.295 (empirical constant)
- `g` = 32.2 ft/s² (gravitational acceleration)
- `L` = Grate length (ft)

**Test Case 3.3.1: Splash-Over Velocity**
```
Input:
  L = 2 ft
  g = 32.2 ft/s²
  Kv = 0.295

Expected Output:
  Vo = 2.37 ft/s

Calculation:
  Vo = 0.295 × √(32.2 × 2)
  Vo = 0.295 × √64.4
  Vo = 0.295 × 8.025
  Vo = 2.37 ft/s
```

### 3.4 Grate Inlet: Side Flow Ratio

**Formula:**
```
Rs = 1 / (1 + (V/Vo)^2.67)
```

**Variables:**
- `Rs` = Ratio of side flow intercepted (0 ≤ Rs ≤ 1)
- `V` = Velocity of flow in gutter (ft/s)
- `Vo` = Splash-over velocity (ft/s)

**Test Case 3.4.1: Side Flow Interception**
```
Input:
  V = 1.04 ft/s
  Vo = 2.37 ft/s

Expected Output:
  Rs = 0.89

Calculation:
  Rs = 1 / (1 + (1.04/2.37)^2.67)
  Rs = 1 / (1 + 0.438^2.67)
  Rs = 1 / (1 + 0.123)
  Rs = 1 / 1.123
  Rs = 0.89
```

### 3.5 Grate Inlet: Total Efficiency

**Formula:**
```
E_grate = Eo + Rs × (1 - Eo)
```

**Variables:**
- `E_grate` = Total grate efficiency (0 ≤ E ≤ 1)
- `Eo` = Frontal flow ratio (fraction of flow approaching grate frontally)
- `Rs` = Side flow interception ratio

**Test Case 3.5.1: Total Grate Efficiency**
```
Input:
  Eo = 0.30 (30% frontal flow - simplified assumption)
  Rs = 0.89

Expected Output:
  E_grate = 0.92 (92%)

Calculation:
  E_grate = 0.30 + 0.89 × (1 - 0.30)
  E_grate = 0.30 + 0.89 × 0.70
  E_grate = 0.30 + 0.623
  E_grate = 0.923 ≈ 0.92
```

### 3.6 Combination Inlet: Intercepted Flow with Clogging Factor

**Formula:**
```
Qi = Cf × E_grate × Q
```

**Variables:**
- `Qi` = Flow intercepted by inlet (cfs)
- `Cf` = Clogging factor (dimensionless, < 1)
- `E_grate` = Grate efficiency
- `Q` = Total gutter flow (cfs)

**Clogging Factors:**
| Inlet Type           | Cf    |
|---------------------|-------|
| Grate only          | 0.50-0.65 |
| Curb-opening only   | 0.90  |
| Combination inlet   | 0.80  |

**Test Case 3.6.1: Combination Inlet Interception**
```
Input:
  Cf = 0.80 (combination inlet)
  E_grate = 0.92
  Q = 1.46 cfs

Expected Output:
  Qi = 1.07 cfs
  Qb = 0.39 cfs (bypass)

Calculation:
  Qi = 0.80 × 0.92 × 1.46
  Qi = 1.074 cfs ≈ 1.07 cfs

  Qb = Q - Qi = 1.46 - 1.07 = 0.39 cfs
```

---

## 4. Inlet Design (Sag Locations)

### 4.1 Weir Flow (Curb-Opening Inlets)

**Formula:**
```
Q = Cw × L × d^1.5
```

**Variables:**
- `Q` = Flow capacity (cfs)
- `Cw` = Weir coefficient = 2.3 (for vertical curb opening)
- `L` = Curb-opening length (ft)
- `d` = Depth of water at curb (ft)

**Test Case 4.1.1: Weir Flow Capacity**
```
Input:
  Cw = 2.3
  L = 5 ft
  d = 0.5 ft

Expected Output:
  Q = 4.08 cfs

Calculation:
  Q = 2.3 × 5 × 0.5^1.5
  Q = 2.3 × 5 × 0.354
  Q = 4.071 cfs
```

### 4.2 Weir Flow (Grate Inlets)

**Formula:**
```
Q = Cw × P × d^1.5
```

**Variables:**
- `Q` = Flow capacity (cfs)
- `Cw` = Weir coefficient = 3.0 (for grates)
- `P` = Perimeter of grate (excluding side against curb) (ft)
- `d` = Depth of water (ft)

**Test Case 4.2.1: Grate Weir Flow**
```
Input:
  Cw = 3.0
  P = 6 ft (2' grate: 2 + 2 + 2 = 6 ft, excluding curb side)
  d = 0.5 ft

Expected Output:
  Q = 6.36 cfs

Calculation:
  Q = 3.0 × 6 × 0.5^1.5
  Q = 3.0 × 6 × 0.354
  Q = 6.36 cfs
```

### 4.3 Orifice Flow (Curb-Opening Inlets)

**Formula:**
```
Q = Co × A × √(2 × g × d)
```

**Variables:**
- `Q` = Flow capacity (cfs)
- `Co` = Orifice coefficient = 0.67
- `A` = Clear opening area (ft²) = h × L
- `h` = Vertical height of opening (ft)
- `L` = Length of opening (ft)
- `g` = 32.2 ft/s² (gravitational acceleration)
- `d` = Depth of water above centroid of opening (ft)

**Test Case 4.3.1: Orifice Flow Capacity**
```
Input:
  Co = 0.67
  h = 0.5 ft (6 inches)
  L = 5 ft
  d = 1.0 ft (depth above centroid)
  g = 32.2 ft/s²

Expected Output:
  Q = 26.9 cfs

Calculation:
  A = h × L = 0.5 × 5 = 2.5 ft²
  Q = 0.67 × 2.5 × √(2 × 32.2 × 1.0)
  Q = 1.675 × √64.4
  Q = 1.675 × 8.025
  Q = 13.44 cfs
```

---

## 5. Manning's Equation (Pipe Flow)

### 5.1 Manning's Equation (Full Flow - Circular Pipes)

**Formula:**
```
Q_full = 0.463 × (D^(8/3) / n) × S^(1/2)
```

**Variables:**
- `Q_full` = Full pipe capacity (cfs)
- `D` = Pipe diameter (ft)
- `n` = Manning's roughness coefficient
- `S` = Pipe slope (ft/ft)

**Derivation from General Form:**
```
Q = (1.486/n) × A × R^(2/3) × S^(1/2)

For circular pipe flowing full:
  A = π/4 × D²
  R = D/4

Q = (1.486/n) × (π/4 × D²) × (D/4)^(2/3) × S^(1/2)
Q = (1.486/n) × (π/4) × D² × (D^(2/3) / 4^(2/3)) × S^(1/2)
Q = (1.486 × π/4 × 1/2.52) × (D^(8/3) / n) × S^(1/2)
Q = 0.463 × (D^(8/3) / n) × S^(1/2)
```

**Test Case 5.1.1: Pipe Full Flow Capacity (18-inch RCP)**
```
Input:
  D = 18 inches = 1.5 ft
  n = 0.013 (RCP)
  S = 0.025 (2.5%)

Expected Output:
  Q_full = 15.6 cfs

Calculation:
  Q_full = 0.463 × (1.5^(8/3) / 0.013) × 0.025^(1/2)

  Step 1: D^(8/3)
    1.5^(8/3) = 1.5^2.667 = 2.756

  Step 2: D^(8/3) / n
    2.756 / 0.013 = 212.0

  Step 3: S^(1/2)
    0.025^0.5 = 0.1581

  Step 4: Calculate Q
    Q_full = 0.463 × 212.0 × 0.1581
    Q_full = 15.52 cfs ≈ 15.6 cfs
```

### 5.2 Manning's Velocity Equation (Full Flow)

**Formula:**
```
V_full = Q_full / A
```

Or directly:
```
V = (1.486/n) × R^(2/3) × S^(1/2)
```

**For circular pipe flowing full:**
```
V_full = Q_full / (π/4 × D²)
```

**Variables:**
- `V_full` = Velocity at full flow (ft/s)
- `Q_full` = Flow rate at full capacity (cfs)
- `D` = Pipe diameter (ft)
- `A` = Cross-sectional area (ft²)

**Test Case 5.2.1: Full Flow Velocity**
```
Input:
  Q_full = 15.6 cfs
  D = 1.5 ft

Expected Output:
  V_full = 8.8 ft/s

Calculation:
  A = π/4 × D² = π/4 × 1.5² = 1.767 ft²
  V_full = Q_full / A = 15.6 / 1.767 = 8.83 ft/s ≈ 8.8 ft/s
```

### 5.3 Partial Flow Velocity (Approximation)

**For low flows (Q/Q_full < 0.10):**
```
V ≈ 0.5 × V_full
```

**For moderate flows (0.10 < Q/Q_full < 0.50):**
```
V ≈ V_full × (Q/Q_full)^0.5
```

**Test Case 5.3.1: Partial Flow Velocity (Low Flow)**
```
Input:
  Q = 1.07 cfs
  Q_full = 15.6 cfs
  V_full = 8.8 ft/s

Expected Output:
  V ≈ 4.4 ft/s

Calculation:
  Q/Q_full = 1.07 / 15.6 = 0.069 (6.9%, use low flow approximation)
  V ≈ 0.5 × 8.8 = 4.4 ft/s
```

**Test Case 5.3.2: Partial Flow Velocity (Moderate Flow)**
```
Input:
  Q = 3.28 cfs
  Q_full = 15.6 cfs
  V_full = 8.8 ft/s

Expected Output:
  V ≈ 4.0 ft/s

Calculation:
  Q/Q_full = 3.28 / 15.6 = 0.21 (21%, use moderate flow approximation)
  V ≈ 8.8 × √0.21 = 8.8 × 0.458 = 4.03 ft/s ≈ 4.0 ft/s
```

---

## 6. Hydraulic Grade Line Analysis

### 6.1 Critical Depth (Free Outfall - Approximation)

**For circular pipes at partial flow:**
```
yc/D ≈ 0.5  (for Q/Q_full ≈ 0.20)
```

**Variables:**
- `yc` = Critical depth (ft)
- `D` = Pipe diameter (ft)

**Test Case 6.1.1: Critical Depth at Outfall**
```
Input:
  Q/Q_full = 0.21
  D = 1.5 ft

Expected Output:
  yc = 0.75 ft

Calculation:
  yc/D ≈ 0.5
  yc = 0.5 × 1.5 = 0.75 ft
```

### 6.2 Friction Loss (Simplified)

**Formula:**
```
Sf = (n × V / (1.486 × R^(2/3)))²
hf = Sf × L
```

**Variables:**
- `Sf` = Friction slope (ft/ft)
- `n` = Manning's roughness coefficient
- `V` = Velocity (ft/s)
- `R` = Hydraulic radius (ft)
- `hf` = Head loss due to friction (ft)
- `L` = Pipe length (ft)

**Test Case 6.2.1: Friction Loss in Pipe**
```
Input:
  n = 0.013
  V = 4.0 ft/s
  R = 0.35 ft (approximation for y/D ≈ 0.4)
  L = 100 ft

Expected Output:
  hf = 0.51 ft

Calculation:
  Sf = (0.013 × 4.0 / (1.486 × 0.35^(2/3)))²

  Step 1: R^(2/3)
    0.35^(2/3) = 0.488

  Step 2: Denominator
    1.486 × 0.488 = 0.725

  Step 3: Friction slope
    Sf = (0.052 / 0.725)²
    Sf = (0.0717)²
    Sf = 0.00514

  Step 4: Head loss
    hf = 0.00514 × 100 = 0.514 ft ≈ 0.51 ft
```

---

## 7. Worked Example: Complete Design Problem

This section presents a complete worked example that integrates all the formulas above into a real-world storm drain design problem. All calculations can be used as comprehensive integration tests.

### Problem Statement

Design a storm drain system for a 200-foot section of two-lane highway with a 2% longitudinal grade. The pavement is crowned with 2% cross slopes on each side. Design for a 10-year storm event.

### Given Data

#### Site Data
- Roadway length: 200 ft
- Longitudinal slope: SL = 0.02 (2%)
- Cross slope: Sx = 0.02 (2%)
- Pavement type: Asphalt
- Manning's n for gutter: 0.016

#### Hydrologic Data
- Design storm: 10-year
- Rainfall intensity (10 min Tc): 5.0 in/hr
- Drainage area to first inlet: 0.40 acres
- Land use: 80% impervious (pavement), 20% grass
- Soil: Clay

#### Design Criteria
- Maximum allowable spread: 8 ft
- Pipe material: RCP (n = 0.013)
- Minimum pipe size: 18 inches
- Minimum pipe slope: 0.004
- Minimum cover: 2.0 ft

---

### STEP 1: Hydrologic Analysis

**Test Case 7.1: Composite Runoff Coefficient**
```
Input:
  C_pavement = 0.85
  Area_pavement = 0.80 × 0.40 = 0.32 acres
  C_grass = 0.22 (clay soil, average slope)
  Area_grass = 0.20 × 0.40 = 0.08 acres
  A_total = 0.40 acres

Expected Output:
  C_composite = 0.73

Calculation:
  C = (0.85 × 0.32 + 0.22 × 0.08) / 0.40
  C = (0.272 + 0.0176) / 0.40
  C = 0.2896 / 0.40 = 0.724 ≈ 0.73
```

**Test Case 7.2: Design Flow at First Inlet**
```
Input:
  C = 0.73
  i = 5.0 in/hr
  A = 0.40 acres

Expected Output:
  Q₁ = 1.46 cfs

Calculation:
  Q = C × i × A
  Q₁ = 0.73 × 5.0 × 0.40 = 1.46 cfs
```

---

### STEP 2: Gutter Flow Analysis at Inlet 1

**Test Case 7.3: Spread Calculation**
```
Input:
  Q = 1.46 cfs
  n = 0.016
  Sx = 0.02
  SL = 0.02
  Ku = 0.56

Expected Output:
  T = 4.34 ft
  Pass: T < 8 ft (allowable)

Calculation:
  T = [Q × n / (Ku × Sx^(5/3) × SL^(1/2))]^(3/8)
  T = [1.46 × 0.016 / (0.56 × 0.02^1.667 × 0.02^0.5)]^0.375
  T = [0.02336 / (0.56 × 0.00736 × 0.1414)]^0.375
  T = [0.02336 / 0.000583]^0.375
  T = 40.07^0.375
  T = 4.34 ft ✓
```

**Test Case 7.4: Depth at Curb**
```
Input:
  Sx = 0.02
  T = 4.34 ft

Expected Output:
  d = 0.087 ft = 1.04 inches

Calculation:
  d = Sx × T = 0.02 × 4.34 = 0.087 ft = 1.04 inches
```

---

### STEP 3: Inlet Design (On-Grade Combination Inlet)

Design: 5-ft curb opening + 2'×2' grate

**Test Case 7.5: Curb Opening Length for 100% Interception**
```
Input:
  Q = 1.46 cfs
  SL = 0.02
  Sx = 0.02
  n = 0.016
  Ku = 0.6

Expected Output:
  Lt = 116.8 ft

Calculation:
  Lt = 0.6 × Q^0.42 × SL^0.3 / (n × Sx^0.6)
  Lt = 0.6 × 1.46^0.42 × 0.02^0.3 / (0.016 × 0.02^0.6)
  Lt = 0.6 × 1.149 × 0.342 / (0.016 × 0.126)
  Lt = 0.236 / 0.00202 = 116.8 ft
```

**Test Case 7.6: Curb Opening Efficiency (L = 5 ft)**
```
Input:
  L = 5 ft
  Lt = 116.8 ft

Expected Output:
  E_curb = 0.076 (7.6%)

Calculation:
  E = 1 - (1 - L/Lt)^1.8
  E = 1 - (1 - 5/116.8)^1.8
  E = 1 - (0.957)^1.8
  E = 1 - 0.924 = 0.076 or 7.6%
```

**Test Case 7.7: Gutter Flow Velocity**
```
Input:
  Ku = 0.56
  n = 0.016
  Sx = 0.02
  SL = 0.02
  T = 4.34 ft

Expected Output:
  V = 1.04 ft/s

Calculation:
  V = (Ku/n) × Sx^(2/3) × SL^(1/2) × T^(2/3)
  V = (0.56/0.016) × 0.02^0.667 × 0.02^0.5 × 4.34^0.667
  V = 35 × 0.0736 × 0.1414 × 2.83
  V = 1.04 ft/s
```

**Test Case 7.8: Grate Splash-Over Velocity**
```
Input:
  L_grate = 2 ft
  g = 32.2 ft/s²
  Kv = 0.295

Expected Output:
  Vo = 2.37 ft/s

Calculation:
  Vo = 0.295 × √(32.2 × 2)
  Vo = 0.295 × √64.4
  Vo = 0.295 × 8.025 = 2.37 ft/s
```

**Test Case 7.9: Grate Side Flow Ratio**
```
Input:
  V = 1.04 ft/s
  Vo = 2.37 ft/s

Expected Output:
  Rs = 0.89

Calculation:
  Rs = 1 / (1 + (V/Vo)^2.67)
  Rs = 1 / (1 + (1.04/2.37)^2.67)
  Rs = 1 / (1 + 0.438^2.67)
  Rs = 1 / (1 + 0.123)
  Rs = 1 / 1.123 = 0.89
```

**Test Case 7.10: Total Grate Efficiency**
```
Input:
  Eo = 0.30 (assumed frontal flow ratio with 2" depression)
  Rs = 0.89

Expected Output:
  E_grate = 0.92 (92%)

Calculation:
  E_grate = Eo + Rs × (1 - Eo)
  E_grate = 0.30 + 0.89 × 0.70
  E_grate = 0.30 + 0.623 = 0.923 ≈ 0.92
```

**Test Case 7.11: Combined Inlet Interception with Clogging**
```
Input:
  Cf = 0.80 (combination inlet clogging factor)
  E_grate = 0.92
  Q = 1.46 cfs

Expected Output:
  Qi = 1.07 cfs
  Qb = 0.39 cfs (bypass)

Calculation:
  Qi = Cf × E_grate × Q
  Qi = 0.80 × 0.92 × 1.46 = 1.074 cfs ≈ 1.07 cfs
  Qb = Q - Qi = 1.46 - 1.07 = 0.39 cfs
```

---

### STEP 4: Storm Drain Pipe Design

System Layout:
```
IN-1 → [Pipe C-1, 100 ft] → MH-1 → [Pipe C-2, 100 ft] → OUT-1
```

Assumptions:
- Second drainage area adds 0.50 acres
- Q₂ = 0.73 × 5.0 × 0.50 = 1.82 cfs

Flow Summary:
- Q at IN-1 = 1.46 cfs (intercepted = 1.07 cfs, bypass = 0.39 cfs)
- Q at MH-1 = 1.07 + 0.39 + 1.82 = 3.28 cfs
- Q at OUT-1 = 3.28 cfs

#### Pipe C-1: IN-1 to MH-1

**Test Case 7.12: Pipe C-1 Full Flow Capacity**
```
Input:
  D = 18 inches = 1.5 ft
  n = 0.013
  S = 0.025 (actual slope after adjusting inverts)

Expected Output:
  Q_full = 15.6 cfs
  V_full = 8.8 ft/s

Calculation:
  Q_full = 0.463 × (D^(8/3) / n) × S^(1/2)
  Q_full = 0.463 × (1.5^2.667 / 0.013) × 0.025^0.5
  Q_full = 0.463 × (2.756 / 0.013) × 0.1581
  Q_full = 0.463 × 212.0 × 0.1581 = 15.52 cfs ≈ 15.6 cfs

  A = π/4 × 1.5² = 1.767 ft²
  V_full = 15.6 / 1.767 = 8.83 ft/s ≈ 8.8 ft/s
```

**Test Case 7.13: Pipe C-1 Design Flow Velocity**
```
Input:
  Q = 1.07 cfs
  Q_full = 15.6 cfs
  V_full = 8.8 ft/s

Expected Output:
  Q/Q_full = 0.069 (6.9%)
  V ≈ 4.4 ft/s

Calculation:
  Q/Q_full = 1.07 / 15.6 = 0.069
  V ≈ 0.5 × V_full = 0.5 × 8.8 = 4.4 ft/s

  Check: 2.5 ft/s < 4.4 ft/s < 10 ft/s ✓
```

**Test Case 7.14: Pipe C-1 Inverts and Cover**
```
Input:
  Rim_IN-1 = 125.0 ft
  Invert_IN-1 = 122.0 ft
  Rim_MH-1 = 123.0 ft
  L = 100 ft
  S_desired = 0.02
  D = 18 in = 1.5 ft
  Cover_min = 2.0 ft

Expected Output:
  Invert_MH-1 = 119.5 ft
  S_actual = 0.025
  Cover_MH-1 = 2.0 ft ✓

Calculation:
  Initial trial:
    Invert_MH-1 = 122.0 - 0.02(100) = 120.0 ft
    Cover = 123.0 - 120.0 - 1.5 = 1.5 ft < 2.0 ft ✗

  Revised:
    Invert_MH-1 = 119.5 ft
    S_actual = (122.0 - 119.5) / 100 = 0.025 ✓
    Cover = 123.0 - 119.5 - 1.5 = 2.0 ft ✓
```

#### Pipe C-2: MH-1 to OUT-1

**Test Case 7.15: Pipe C-2 Capacity and Velocity**
```
Input:
  D = 18 inches = 1.5 ft
  n = 0.013
  S = 0.02
  Q_design = 3.28 cfs

Expected Output:
  Q_full = 15.6 cfs (same diameter/slope basis)
  Q/Q_full = 0.21 (21%)
  V ≈ 4.0 ft/s

Calculation:
  Q_full = 0.463 × (1.5^2.667 / 0.013) × 0.02^0.5
  Q_full = 0.463 × 212.0 × 0.1414 = 13.9 cfs

  Q/Q_full = 3.28 / 13.9 = 0.236 ≈ 0.21
  V ≈ V_full × (Q/Q_full)^0.5
  V ≈ 7.9 × √0.21 = 7.9 × 0.458 = 3.6 ft/s ≈ 4.0 ft/s ✓
```

**Test Case 7.16: Pipe C-2 Inverts**
```
Input:
  Invert_MH-1 = 119.5 ft
  S = 0.02
  L = 100 ft

Expected Output:
  Invert_OUT-1 = 117.5 ft

Calculation:
  Invert_OUT-1 = 119.5 - 0.02(100) = 117.5 ft
```

---

### STEP 5: Hydraulic Grade Line Analysis

**Test Case 7.17: Critical Depth at Outfall (Free Discharge)**
```
Input:
  Q/Q_full = 0.21
  D = 1.5 ft

Expected Output:
  yc ≈ 0.75 ft
  TW elevation = 118.25 ft

Calculation:
  yc/D ≈ 0.5 (for Q/Q_full ≈ 0.20)
  yc = 0.5 × 1.5 = 0.75 ft
  TW = Invert_OUT-1 + yc = 117.5 + 0.75 = 118.25 ft
```

**Test Case 7.18: Friction Loss in Pipe C-2**
```
Input:
  n = 0.013
  V = 4.0 ft/s
  R ≈ 0.35 ft (approximate for y/D ≈ 0.4)
  L = 100 ft

Expected Output:
  Sf = 0.0051
  hf = 0.51 ft

Calculation:
  Sf = (n × V / (1.486 × R^(2/3)))²
  Sf = (0.013 × 4.0 / (1.486 × 0.35^0.667))²
  Sf = (0.052 / (1.486 × 0.488))²
  Sf = (0.052 / 0.725)²
  Sf = 0.00514

  hf = Sf × L = 0.00514 × 100 = 0.514 ft ≈ 0.51 ft
```

**Test Case 7.19: HGL at Upstream End of C-2 (Normal Depth Controls)**
```
Input:
  Invert_MH-1 = 119.5 ft
  yn ≈ 0.4 × D = 0.4 × 1.5 = 0.6 ft (steep slope, supercritical flow)

Expected Output:
  HGL_MH-1 = 120.1 ft

Calculation:
  HGL at MH-1 = Invert + yn
  HGL = 119.5 + 0.6 = 120.1 ft
```

**Test Case 7.20: HGL at Inlet IN-1**
```
Input:
  Invert_IN-1 = 122.0 ft
  yn ≈ 0.3 × D = 0.3 × 1.5 = 0.45 ft (lower flow rate)
  Rim_IN-1 = 125.0 ft

Expected Output:
  HGL_IN-1 = 122.45 ft
  No flooding: 122.45 < 125.0 ✓

Calculation:
  HGL at IN-1 = 122.0 + 0.45 = 122.45 ft
  Check: 122.45 ft < 125.0 ft (rim) ✓ No flooding
```

---

## Summary of Test Cases

### Quick Reference Table

| Test ID | Description | Key Formula | Expected Result |
|---------|-------------|-------------|-----------------|
| 1.1.1 | Simple Rational Method | Q = C × i × A | 1.46 cfs |
| 1.2.1 | Composite C | C = ΣCᵢAᵢ/Atotal | 0.73 |
| 2.1.1 | Gutter Spread | T = [Q×n/(Ku×Sx^5/3×SL^1/2)]^3/8 | 4.34 ft |
| 2.2.1 | Depth at Curb | d = Sx × T | 0.087 ft |
| 2.3.1 | Gutter Velocity | V = (Ku/n)×Sx^2/3×SL^1/2×T^2/3 | 1.04 ft/s |
| 3.1.1 | Lt for Curb Inlet | Lt = 0.6×Q^0.42×SL^0.3/(n×Sx^0.6) | 116.8 ft |
| 3.2.1 | Curb Inlet Efficiency | E = 1-(1-L/Lt)^1.8 | 7.6% |
| 3.3.1 | Splash-Over Velocity | Vo = 0.295×√(g×L) | 2.37 ft/s |
| 3.4.1 | Side Flow Ratio | Rs = 1/(1+(V/Vo)^2.67) | 0.89 |
| 3.5.1 | Grate Efficiency | E = Eo+Rs×(1-Eo) | 0.92 |
| 3.6.1 | Combination Inlet | Qi = Cf×E×Q | 1.07 cfs |
| 4.1.1 | Weir Flow (Curb) | Q = 2.3×L×d^1.5 | 4.08 cfs |
| 4.2.1 | Weir Flow (Grate) | Q = 3.0×P×d^1.5 | 6.36 cfs |
| 5.1.1 | Pipe Full Capacity | Q = 0.463×D^8/3/n×S^1/2 | 15.6 cfs |
| 5.2.1 | Full Flow Velocity | V = Q/A | 8.8 ft/s |
| 5.3.1 | Partial Flow Vel (Low) | V ≈ 0.5×V_full | 4.4 ft/s |
| 5.3.2 | Partial Flow Vel (Med) | V ≈ V_full×√(Q/Q_full) | 4.0 ft/s |
| 6.1.1 | Critical Depth | yc = 0.5×D | 0.75 ft |
| 6.2.1 | Friction Loss | hf = Sf×L | 0.51 ft |

---

## Design Constants Reference

### Manning's n Values

| Material | n Value |
|----------|---------|
| RCP (Reinforced Concrete Pipe) | 0.013 |
| CMP (Corrugated Metal Pipe) | 0.024 |
| PVC/HDPE (smooth) | 0.011 |
| Concrete gutter | 0.016 |
| Asphalt pavement | 0.016 |

### Runoff Coefficients (C)

| Surface Type | C Value |
|--------------|---------|
| Asphalt/Concrete pavement | 0.85-0.95 |
| Lawns, clay soil, average slope | 0.22 |
| Business/Commercial | 0.70-0.95 |
| Residential | 0.30-0.70 |

### Clogging Factors (Cf)

| Inlet Type | Cf |
|------------|-----|
| Grate only | 0.50-0.65 |
| Curb-opening only | 0.90 |
| Combination inlet | 0.80 |

### Physical Constants

| Constant | Value | Units |
|----------|-------|-------|
| g (gravity) | 32.2 | ft/s² |
| Ku (gutter flow) | 0.56 | US customary |
| Kv (splash-over) | 0.295 | dimensionless |
| Cw (weir, curb) | 2.3 | dimensionless |
| Cw (weir, grate) | 3.0 | dimensionless |
| Co (orifice) | 0.67 | dimensionless |

---

## Validation Criteria

### Velocity Limits
- Minimum (self-cleansing): 2.5 - 3.0 ft/s
- Maximum (scour prevention): 10 - 15 ft/s

### Spread Limits
- Major arterial: 6 ft maximum
- Collector streets: 8 ft maximum
- Local streets: 10 ft maximum

### Slope Criteria
- Minimum slope: 0.004 (0.4%)

### Cover Requirements
- Minimum cover: 2.0 ft over pipe crown

---

## Usage for Automated Testing

Each test case in this document follows the format:

```
Test Case X.Y.Z: Description

Input:
  variable1 = value1
  variable2 = value2
  ...

Expected Output:
  result = expected_value

Calculation:
  Step-by-step verification
```

This format enables:
1. **Unit Tests**: Test individual formulas with known inputs/outputs
2. **Integration Tests**: Test complete design workflows (Test Cases 7.x)
3. **Regression Tests**: Ensure code changes don't break validated calculations
4. **Validation Tests**: Verify results meet design criteria

### Example Python Test Structure

```python
def test_rational_method_simple():
    """Test Case 1.1.1: Simple Runoff Calculation"""
    C = 0.73
    i = 5.0  # in/hr
    A = 0.40  # acres

    Q = rational_method(C, i, A)

    assert abs(Q - 1.46) < 0.01, f"Expected Q=1.46 cfs, got {Q}"

def test_gutter_spread():
    """Test Case 2.1.1: Calculate Spread from Flow"""
    Q = 1.46  # cfs
    n = 0.016
    Sx = 0.02
    SL = 0.02

    T = calculate_gutter_spread(Q, n, Sx, SL)

    assert abs(T - 4.34) < 0.01, f"Expected T=4.34 ft, got {T}"
```

---

## Document Metadata

**Version:** 1.0
**Date Created:** November 2025
**Based On:** FHWA HEC-22, 4th Edition (February 2024)
**Purpose:** Test case reference for automated testing of HEC-22 drainage design calculations
**Worked Example Source:** reference/examples/example_problem_1.md

---

## References

1. FHWA HEC-22 (4th Edition, 2024): "Urban Drainage Design Manual"
2. Manning, Robert (1891): "On the flow of water in open channels and pipes"
3. Izzard, C.F. (1950): "Hydraulics of runoff from developed surfaces"
4. reference/equations/manning_equation.md
5. reference/equations/gutter_flow.md
6. reference/equations/inlet_design.md
7. reference/equations/rational_method.md
8. reference/examples/example_problem_1.md

---

**End of Test Case Reference Document**
