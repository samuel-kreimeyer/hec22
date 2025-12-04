# HEC-22 Chapter 5 - Roadway Pavement Drainage Equations

## Section 5.2.2 - Longitudinal Grade

### Equation 5.1: Vertical Curve Constant
```
K = L / (G₂ - G₁)
```
**Variables:**
- K = Vertical curve constant, ft/percent (m/percent)
- L = Horizontal length of curve, ft (m)
- G₁, G₂ = Grade of roadway, percent

**Application:** To provide adequate drainage in sag vertical curves, maintain K ≤ 167 (50 in SI)

---

## Section 5.3.1 - Capacity Relationship (Izzard's Modified Manning's Equation)

### Equation 5.2: Gutter Flow Rate
```
Q = (Kᵤ/n) Sₓ^1.67 Sₗ^0.5 T^2.67
```
**Variables:**
- Kᵤ = Unit conversion constant, 0.56 in CU (0.376 in SI)
- n = Manning's coefficient
- Q = Flow rate, ft³/s (m³/s)
- T = Width of flow (spread), ft (m)
- Sₓ = Cross slope, ft/ft (m/m)
- Sₗ = Longitudinal grade, ft/ft (m/m)

**Notes:** Neglects resistance of curb face since negligible

### Equation 5.3: Gutter Velocity
```
V = (2Kᵤ/n) Sₓ^0.67 Sₗ^0.5 T^0.67
```
**Variables:** Same as Equation 5.2 plus:
- V = Velocity, ft/s (m/s)

### Equation 5.4: Spread Given Flow
```
T = [(Qn)/(Kᵤ Sₓ^1.67 Sₗ^0.5)]^0.375
```
**Variables:** Same as Equation 5.2

**Application:** Used to estimate spread width given a flow rate

---

## Section 5.3.2.2 - Composite Gutters

### Equation 5.5: Flow in Depressed Section
```
Qw = Q Eₒ
```
**Variables:**
- Qw = Flow in the depressed section of the gutter, ft³/s (m³/s)
- Q = Total gutter flow rate, ft³/s (m³/s)
- Eₒ = Ratio of flow in the depressed section to total gutter flow

### Equation 5.6: Flow in Side Section
```
Qs = Q (1 - Eₒ)
```
**Variables:**
- Qs = Flow in the gutter section above the depressed section, ft³/s (m³/s)

### Equation 5.7: Ratio of Flow in Depressed Section
```
Eₒ = 1 / [1 + (Sw/Sx) / [(1 + (Sw/Sx)/(T/W-1))^2.67 - 1]]
```
**Variables:**
- Sw = Cross slope in the depressed section, ft/ft (m/m)
- Sx = Cross slope of pavement, ft/ft (m/m)
- T = Total spread, ft (m)
- W = Width of depressed section, ft (m)

**Algorithm:** This is an implicit equation requiring iterative solution when solving for T given Q

### Equation 5.8: Depressed Section Cross Slope
```
Sw = Sx + a/W
```
**Variables:**
- a = Gutter depression depth, ft (m)
- W = Width of depressed section, ft (m)

---

## Section 5.3.3.1 - V-Sections

### Equation 5.9: Equivalent Cross Slope for V-Shaped Sections
```
Sx = (Sx1 × Sx2) / (Sx1 + Sx2)
```
**Variables:**
- Sx1, Sx2 = Cross slopes of the two sides of the V-section, ft/ft (m/m)
- Sx = Equivalent cross slope, ft/ft (m/m)

**Application:** Use this equivalent Sx in Equation 5.2 to compute flow in V-shaped gutters

---

## Section 5.3.3.2 - Circular Sections

### Equation 5.10: Circular Gutter Depth-Discharge Relationship
```
d/D = Kᵤ [(Qn)/(D^2.67 Sₗ^0.5)]^0.488
```
**Variables:**
- d = Depth of flow in circular gutter, ft (m)
- D = Diameter of circular gutter, ft (m)
- Kᵤ = Unit conversion constant, 0.972 in CU (1.179 in SI)
- Q = Flow rate, ft³/s (m³/s)
- n = Manning's coefficient
- Sₗ = Longitudinal grade, ft/ft (m/m)

### Equation 5.11: Circular Gutter Top Width
```
Tw = 2[r² - (r - d)²]^0.5
```
**Variables:**
- Tw = Width of circular gutter section (chord of arc), ft (m)
- r = Radius of circular gutter, ft (m)
- d = Depth of flow, ft (m)

---

## Section 5.3.5 - Relative Flow Capacities

### Equation 5.12: Relative Effect of Cross Slope on Capacity
```
(Sx1/Sx2)^1.67 = Q1/Q2
```
**Application:** Shows cross slope has significant effect on capacity

### Equation 5.13: Relative Effect of Longitudinal Slope on Capacity
```
(SL1/SL2)^0.5 = Q1/Q2
```
**Application:** Shows longitudinal slope has moderate effect on capacity

### Equation 5.14: Relative Effect of Spread on Capacity
```
(T1/T2)^2.67 = Q1/Q2
```
**Application:** Shows spread has very significant effect on capacity

---

## Section 5.3.6 - Gutter Flow Time

### Equation 5.15: Average Velocity Between Two Spreads
```
Va = Kᵤ KG [(T₂^2.67 - T₁^2.67) / (T₂² - T₁²)]
```
**Variables:**
- Va = Average velocity in the gutter section between T₁ and T₂ locations, ft/s (m/s)
- T₁ = Upstream spread, ft (m)
- T₂ = Downstream spread, ft (m)
- KG = Gutter geometry parameter
- Kᵤ = Unit conversion constant, 0.84 in CU (0.564 in SI)

**Derivation:** Obtained by integration of Manning's equation for spatially varied flow (see Appendix B)

### Equation 5.16: Gutter Geometry Parameter
```
KG = (Sₗ^0.5 Sₓ^0.67) / n
```
**Variables:**
- Sₗ = Longitudinal grade, ft/ft (m/m)
- Sₓ = Cross slope, ft/ft (m/m)
- n = Manning's coefficient

**Application:** Used in Equation 5.15 to compute average velocity

---

## Worked Examples (Basis for Unit Tests)

### Example 5.1: Computation of Triangular Gutter Flow
**Given:**
- Sₗ = 0.010 ft/ft
- Sₓ = 0.020 ft/ft
- n = 0.016

**Part A:** Find spread T for Q = 1.8 ft³/s
- **Solution:** T = 9.0 ft

**Part B:** Find flow Q for T = 8.2 ft
- **Solution:** Q = 1.4 ft³/s

### Example 5.2: Composite Gutter Flow (ITERATIVE METHOD)
**Given:**
- W = 2 ft (depressed section width)
- Sₗ = 0.01 ft/ft
- Sₓ = 0.02 ft/ft
- n = 0.016
- a = 2 inches (gutter depression)

**Part A:** Find flow Q for T = 8.2 ft
- **Algorithm:**
  1. Compute Sw = a/W + Sx
  2. Compute Ts = T - W
  3. Compute Qs using Equation 5.2
  4. Compute Eₒ using Equation 5.7
  5. Compute Q = Qs / (1 - Eₒ)
- **Solution:** Q = 2.3 ft³/s

**Part B:** Find spread T for Q = 4.2 ft³/s (REQUIRES ITERATION)
- **Algorithm:**
  1. Assume initial Qs
  2. Compute Qw = Q - Qs
  3. Compute Eₒ = Qw / Q
  4. Solve Equation 5.7 for T/W
  5. Compute T = W(T/W)
  6. Compute Ts = T - W
  7. Compute Qs from Equation 5.2 using Ts
  8. Compare computed Qs with assumed Qs
  9. If not close, assume new Qs and repeat
- **Solution:** T = 11.1 ft (after iteration)

### Example 5.3: V-Shaped Roadside Shoulder Gutter (ITERATIVE METHOD)
**Given:**
- Sₗ = 0.01 ft/ft
- Sx1 = 0.25 ft/ft
- Sx3 = 0.02 ft/ft
- Sx2 = 0.04 ft/ft
- TBC = 2.0 ft
- n = 0.016
- Q = 1.77 ft³/s

**Algorithm:**
1. Calculate Sx using Equation 5.9 assuming flow entirely in V-section (Sx1 and Sx2)
2. Find hypothetical spread T' using Equation 5.4
3. Check if T' is within V-section
4. If outside, use iterative weighted slope method
5. Develop weighted slope for composite section
6. Repeat until computed T matches assumed T

**Solution:** T = 8.31 ft

### Example 5.4: V-Shaped Median Shallow Swale (ITERATIVE METHOD)
**Given:**
- TAB = TBC = 3.28 ft
- Sₗ = 0.01 ft/ft
- n = 0.016
- Sx1 = Sx2 = 0.25 ft/ft
- Sx3 = 0.04 ft/ft

**Part A:** Find spread for Q = 24.7 ft³/s
- **Algorithm:** Treat half-section as composite, iterate to find Qs
- **Solution:** T = 13.12 ft

**Part B:** Find flow for T = 23.0 ft
- **Solution:** Q = 49 ft³/s

### Example 5.5: Circular Channels
**Given:**
- D = 4.92 ft
- Sₗ = 0.01 ft/ft
- n = 0.016
- Q = 17.6 ft³/s

**Algorithm:**
1. Use Equation 5.10 to find d/D
2. Calculate d = D(d/D)
3. Use Equation 5.11 to find Tw

**Solution:** d = 0.98 ft, Tw = 3.93 ft

### Example 5.6: Gutter Flow Time
**Given:**
- T₁ = 3.28 ft
- T₂ = 9.84 ft
- Sₗ = 0.03 ft/ft
- Sₓ = 0.02 ft/ft
- n = 0.016
- L = 330 ft (inlet spacing)

**Algorithm:**
1. Compute KG using Equation 5.16
2. Compute Va using Equation 5.15
3. Compute time = L/Va

**Solution:** t = 1.7 minutes

---

## Key Implementation Notes

1. **Iterative Methods Required:** Examples 5.2, 5.3, and 5.4 demonstrate that solving for spread given flow in composite sections requires iteration
2. **Convergence Criteria:** Iterations should continue until computed value matches assumed value within acceptable tolerance
3. **Initial Guesses:** Good initial guesses speed convergence
4. **Equation References:** All docstrings should reference equation numbers (e.g., "5.2", "5.7")
5. **Unit Conversion Constants:** Must use correct Kᵤ values for CU vs SI units
