# HEC-22 Chapter 9 - Storm Drain Conduits: Complete Equation Reference

## Document Overview
This document catalogs all equations from HEC-22 4th Edition, Chapter 9: Storm Drain Conduits. Each equation includes the equation number, all variables with definitions, and context for intended use.

---

## Section 9.1.3 - Hydraulic Capacity

### Equation 9.1: Mean Velocity in Full Flow Pipe (Manning's Equation)

```
V = (K_V/n)D^0.67 S_o^0.5
```

**Variables:**
- V = Mean velocity, ft/s (m/s)
- K_V = Unit conversion constant, 0.59 in CU (0.397 in SI)
- n = Manning's roughness coefficient
- D = Storm drain diameter, ft (m)
- S_o = Slope of the energy grade line, ft/ft (m/m)

**Context:** This is Manning's equation adapted for circular storm drains flowing full. Used for computing the capacity for roadside and median channels. The equation relates pipe velocity to diameter, roughness, and slope.

---

### Equation 9.2: Flow Rate in Full Flow Pipe (Manning's Equation)

```
Q = (K_Q/n)D^2.67 S_o^0.5
```

**Variables:**
- Q = Rate of flow, ft³/s (m³/s)
- K_Q = Unit conversion constant, 0.46 in CU (0.312 in SI)
- n = Manning's roughness coefficient
- D = Storm drain diameter, ft (m)
- S_o = Slope of the energy grade line, ft/ft (m/m)

**Context:** Companion to Equation 9.1, this calculates discharge directly. Critical for sizing storm drains. Demonstrates that doubling diameter increases capacity by factor of 6.35, doubling slope increases capacity by 1.4, and doubling roughness reduces capacity by 50%.

---

## Section 9.1.6.1 - Pipe Friction Losses

### Equation 9.3: Head Loss Due to Friction

```
h_f = S_f L
```

**Variables:**
- h_f = Friction loss, ft (m)
- S_f = Friction slope, ft/ft (m/m)
- L = Length of pipe, ft (m)

**Context:** Friction or boundary shear loss represents the major loss in storm drainage systems. The friction slope is the slope of the hydraulic gradient for a pipe run. For steady uniform flow, friction slope equals pipe slope for partially full flow.

---

### Equation 9.4: Friction Slope for Full Flow

```
S_f = (h_f/L) = (Qn/(K_Q D^2.67))^2
```

**Variables:**
- S_f = Friction slope, ft/ft (m/m)
- h_f = Friction loss, ft (m)
- L = Length of pipe, ft (m)
- Q = Rate of flow, ft³/s (m³/s)
- n = Manning's roughness coefficient
- K_Q = Unit conversion constant, 0.46 in CU (0.312 in SI)
- D = Storm drain diameter, ft (m)

**Context:** Used to calculate pipe friction loss for full flow in circular pipes. Derived from rearranging Manning's equation. Essential for determining energy losses in pressure flow conditions.

---

## Section 9.1.6.2 - Exit Losses

### Equation 9.5: Exit Loss at Storm Drain Outlet

```
H_o = 1.0[V_o^2/2g - V_d^2/2g]
```

**Variables:**
- H_o = Exit loss, ft (m)
- V_o = Average outlet velocity, ft/s (m/s)
- V_d = Channel velocity downstream of outlet in direction of pipe flow, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Exit loss from storm drain outlet is function of change in velocity at outlet. For sudden expansion at end wall, coefficient is 1.0. When V_d = 0 (reservoir), exit loss equals one velocity head. For part full flow where pipe outlets in channel with water moving same direction, exit loss is virtually zero.

---

## Section 9.1.6.3 - Bend Losses

### Equation 9.6: Bend Loss Coefficient

```
H_b = 0.0033(Δ)V^2/2g
```

**Variables:**
- H_b = Bend loss, ft (m)
- Δ = Angle of bend, degrees
- V = Velocity in pipe, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Estimates bend loss coefficient for storm drain design for bends in pipe run (not in access hole structure). Coefficient 0.0033 is derived from AASHTO 2014. Used when pipes change direction without access holes.

---

## Section 9.1.6.4 - Transition Losses

### Equation 9.7: Expansion Loss

```
H_e = K_e[V_2^2/2g - V_1^2/2g]
```

**Variables:**
- H_e = Expansion loss, ft (m)
- K_e = Expansion coefficient (see Table 9.3)
- V_1 = Velocity upstream of transition, ft/s (m/s)
- V_2 = Velocity downstream of transition, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Energy losses in expansions in open channel flow expressed in terms of kinetic energy at two ends. Typically designers use access holes when pipe size increases. K_e values depend on D2/D1 ratio and angle of cone (Table 9.3).

---

### Equation 9.8: Contraction Loss

```
H_c = K_c[V_2^2/2g - V_1^2/2g]
```

**Variables:**
- H_c = Contraction loss, ft (m)
- K_c = Contraction coefficient
- V_1 = Velocity upstream of transition, ft/s (m/s)
- V_2 = Velocity downstream of transition, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Analogous energy loss for contractions. However, designers don't use contractions in storm drains because of potential for clogging and safety hazards when transitioning to smaller pipe size.

---

## Section 9.1.6.5 - Junction Losses

### Equation 9.9: Junction Loss (Momentum Equation)

```
H_j = [(Q_o V_o) - (Q_i V_i) - (Q_l V_l cos θ_j)] / [0.5g(A_o + A_i)] + h_i - h_o
```

**Variables:**
- H_j = Junction loss, ft (m)
- Q_o = Outlet flow, ft³/s (m³/s)
- Q_i = Inlet flow, ft³/s (m³/s)
- Q_l = Lateral flow, ft³/s (m³/s)
- V_o = Outlet velocity, ft/s (m/s)
- V_i = Inlet velocity, ft/s (m/s)
- V_l = Lateral velocity, ft/s (m/s)
- h_o = Outlet velocity head, ft (m)
- h_i = Inlet velocity head, ft (m)
- A_o = Outlet cross-sectional area, ft² (m²)
- A_i = Inlet cross-sectional area, ft² (m²)
- θ_j = Angle between inflow trunk pipe and inflow lateral pipe
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Minor loss equation for pipe junction connecting lateral pipe to larger trunk pipe without access hole structure. Form of momentum equation. Underground junctions present debris clogging hazard; designers prefer junction boxes with access.

---

## Section 9.1.6.6 - Approximate Method for Access Hole Energy Loss

### Equation 9.10: Approximate Access Hole Loss

```
H_ah = K_ah V_o^2/2g
```

**Variables:**
- H_ah = Head loss across access hole, ft (m)
- K_ah = Head loss coefficient (see Table 9.4)
- V_o = Outlet pipe velocity, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Simplest method for preliminary design only. Estimates losses across access hole by multiplying velocity head of outflow pipe by coefficient. Coefficients from Table 9.4 vary with structure configuration and angle. Does not apply to EGL calculations - preliminary estimate only for establishing initial pipe invert elevations.

---

## Section 9.1.6.7.1 - FHWA Access Hole Method: Outflow Pipe Energy

### Equation 9.11: Total Energy Head Components

```
E_i = y + (P/γ) + (V^2/2g)
```

**Variables:**
- E_i = Outflow pipe energy head, ft (m)
- y = Outflow pipe depth (potential head), ft (m)
- P/γ = Outflow pipe pressure head, ft (m)
- V²/2g = Outflow pipe velocity head, ft (m)

**Context:** Defines total outflow pipe energy head as sum of potential, pressure, and velocity head components. Solving this directly can be problematic for certain conditions where pressure cannot be assumed atmospheric.

---

### Equation 9.12: Energy Head from EGL and Invert

```
E_i = EGL_i - Z_i
```

**Variables:**
- E_i = Outflow pipe energy head, ft (m)
- EGL_i = Outflow pipe energy grade line, ft (m)
- Z_i = Outflow pipe invert elevation, ft (m)

**Context:** Alternative method to determine E_i by subtracting outflow pipe invert elevation from energy grade line (both known values). Avoids problems with solving Equation 9.11 directly. Serves as check on the method.

---

### Equation 9.13: Initial Access Hole Energy Level

```
E_ai = max(E_aio, E_ais, E_aiu)
```

**Variables:**
- E_ai = Initial access hole energy level, ft (m)
- E_aio = Estimated access hole energy level for outlet control (full/partially full), ft (m)
- E_ais = Estimated access hole energy level for inlet control (submerged), ft (m)
- E_aiu = Estimated access hole energy level for inlet control (unsubmerged), ft (m)

**Context:** Initial estimate of access hole energy level is maximum of three possible conditions determining hydraulic regime: outlet control (full and partially full flow), inlet control submerged (orifice), and inlet control unsubmerged (weir).

---

### Equation 9.14: Outlet Control Energy Level

```
E_aio = E_i + H_i
```

**Variables:**
- E_aio = Access hole energy level for outlet control, ft (m)
- E_i = Outflow pipe energy head, ft (m)
- H_i = Entrance loss assuming outlet control, ft (m)

**Context:** In outlet control condition, downstream storm drain system limits discharge such that outflow pipe flows full or partially full in subcritical flow. Adds entrance loss to outflow pipe energy.

---

### Equation 9.15: Entrance Loss Coefficient

```
H_i = K_i V^2/2g
```

**Variables:**
- H_i = Entrance loss, ft (m)
- K_i = Entrance loss coefficient = 0.2 (Kerenyi et al. 2006)
- V = Velocity in pipe, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Entrance loss for outlet control condition. Coefficient of 0.2 is from FHWA research (Kerenyi et al. 2006).

---

### Equation 9.16: Discharge Intensity

```
DI = Q / [A(gD_o)^0.5]
```

**Variables:**
- DI = Discharge intensity (dimensionless)
- Q = Discharge, ft³/s (m³/s)
- A = Area of outflow pipe, ft² (m²)
- D_o = Diameter of outflow pipe, ft (m)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Dimensionless ratio adapted from culvert analysis. Describes discharge intensity as ratio of discharge to pipe dimensions. Key parameter for inlet control calculations. Reduces reliance on velocity head which can be unreliable.

---

### Equation 9.17: Submerged Inlet Control (Orifice)

```
E_ais = D_o(DI)^2
```

**Variables:**
- E_ais = Access hole energy level for inlet control (submerged), ft (m)
- D_o = Diameter of outflow pipe, ft (m)
- DI = Discharge intensity (dimensionless)

**Context:** Submerged inlet control uses orifice analogy. Applies when opening in access hole structure to outlet pipe is limiting and water depth is sufficiently high. Derived from laboratory data with DI ≤ 1.6.

---

### Equation 9.18: Unsubmerged Inlet Control (Weir)

```
E_aiu = 1.6D_o(DI)^0.67
```

**Variables:**
- E_aiu = Access hole energy level for inlet control (unsubmerged), ft (m)
- D_o = Diameter of outflow pipe, ft (m)
- DI = Discharge intensity (dimensionless)

**Context:** Unsubmerged inlet control uses weir analogy. Applies when flow control is limited by opening but water level involves treating opening as weir. Laboratory data shows DI range of 0.0 to 0.5, though equation not limited to this range.

---

## Section 9.1.6.7.2 - Adjustments for Access Hole Losses

### Equation 9.19: Revised Access Hole Energy Level

```
E_a = E_ai + H_B + H_θ + H_P
```

**Variables:**
- E_a = Revised access hole energy level, ft (m)
- E_ai = Initial access hole energy level, ft (m)
- H_B = Additional energy loss for benching (floor configuration), ft (m)
- H_θ = Additional energy loss for angled inflows, ft (m)
- H_P = Additional energy loss for plunging flows, ft (m)

**Context:** Adjusts initial access hole energy level for benching, inflow angles, and plunging flows using superposition principle. Avoids extreme values from single multiplicative coefficient. If E_a < E_i, set E_a = E_i.

---

### Equation 9.20: Benching Energy Loss

```
H_B = C_B(E_ai - E_i)
```

**Variables:**
- H_B = Additional energy loss for benching, ft (m)
- C_B = Energy loss coefficient for benching (see Table 9.5)
- E_ai = Initial access hole energy level, ft (m)
- E_i = Outflow pipe energy head, ft (m)

**Context:** Benching tends to direct flow through access hole, reducing energy losses. Coefficients from Table 9.5 vary from flat (level) to improved benching. Negative values indicate water depth reduction. Depends on whether bench is submerged (E_ai/D_o > 2.5) or unsubmerged (E_ai/D_o < 1.0).

---

### Equation 9.21: Flow-Weighted Angle

```
θ_w = Σ(Q_j θ_j) / ΣQ_j
```

**Variables:**
- θ_w = Flow-weighted angle, degrees
- Q_j = Contributing flow from inflow pipe j, ft³/s (m³/s)
- θ_j = Angle measured from outlet pipe (180° is straight pipe), degrees

**Context:** Addresses effect of skewed inflows by considering momentum vectors. Contribution of all non-plunging inflows with hydraulic connection resolves into single flow-weighted angle. If all flows plunging, set θ_w = 180°.

---

### Equation 9.22: Angled Inflow Coefficient

```
C_θ = 4.5(ΣQ_j/Q_o)cos(θ_w/2)
```

**Variables:**
- C_θ = Angled inflow coefficient (dimensionless)
- ΣQ_j = Sum of non-plunging contributing flows, ft³/s (m³/s)
- Q_o = Flow in outflow pipe, ft³/s (m³/s)
- θ_w = Flow-weighted angle, degrees

**Context:** Coefficient approaches zero as θ_w approaches 180° and as relative inflow approaches zero. Accounts for turbulence from non-aligned pipe junctions.

---

### Equation 9.23: Angled Inflow Energy Loss

```
H_θ = C_θ(E_ai - E_i)
```

**Variables:**
- H_θ = Additional energy loss for angled inflows, ft (m)
- C_θ = Angled inflow coefficient (dimensionless)
- E_ai = Initial access hole energy level, ft (m)
- E_i = Outflow pipe energy head, ft (m)

**Context:** Additional angle inflow energy loss based on coefficient from Equation 9.22. Applied when pipes enter structure at angles other than 180°.

---

### Equation 9.24: Relative Plunge Height

```
h_k = (z_k - E_ai) / D_o
```

**Variables:**
- h_k = Relative plunge height (dimensionless)
- z_k = Difference between access hole invert and inflow pipe k invert, ft (m)
- E_ai = Initial access hole energy level, ft (m)
- D_o = Diameter of outflow pipe, ft (m)

**Context:** Plunging inflow occurs where inflow pipe invert (z_k) is greater than estimated structure water depth (approximated by E_ai). Only applies when z_k < 10D_o; if z_k > 10D_o, set it to 10D_o.

---

### Equation 9.25: Plunging Flow Coefficient

```
C_P = Σ(Q_k h_k) / Q_o
```

**Variables:**
- C_P = Plunging flow coefficient (dimensionless)
- Q_k = Flow from plunging pipe k, ft³/s (m³/s)
- h_k = Relative plunge height for pipe k (dimensionless)
- Q_o = Flow in outflow pipe, ft³/s (m³/s)

**Context:** As proportion of plunging flows approaches zero, C_P approaches zero. Accounts for energy dissipation from water falling into access hole.

---

### Equation 9.26: Plunging Inflow Energy Loss

```
H_P = C_P(E_ai - E_i)
```

**Variables:**
- H_P = Additional energy loss for plunging flows, ft (m)
- C_P = Plunging flow coefficient (dimensionless)
- E_ai = Initial access hole energy level, ft (m)
- E_i = Outflow pipe energy head, ft (m)

**Context:** Additional plunging inflow energy loss. Applies to flows entering structure from inlets or elevated incoming pipes above water depth in access hole.

---

### Equation 9.27: Combined Additional Loss (Alternative Form)

```
H_a = (C_B + C_θ + C_P)(E_ai - E_i)
```

**Variables:**
- H_a = Total additional loss, ft (m)
- C_B = Energy loss coefficient for benching (dimensionless)
- C_θ = Angled inflow coefficient (dimensionless)
- C_P = Plunging flow coefficient (dimensionless)
- E_ai = Initial access hole energy level, ft (m)
- E_i = Outflow pipe energy head, ft (m)

**Context:** Algebraic rearrangement of benching, inflow angle, and plunging equations to compute total additional loss. Value should always be positive; if negative, set H_a = 0.

---

### Equation 9.28: Final Access Hole Energy Level

```
E_a = E_ai + H_a
```

**Variables:**
- E_a = Revised access hole energy level, ft (m)
- E_ai = Initial access hole energy level, ft (m)
- H_a = Total additional loss, ft (m)

**Context:** Final revised access hole energy level from initial estimate plus additional losses. If computed E_a < E_i, use higher value (E_i).

---

### Equation 9.29: Access Hole Energy Grade Line

```
EGL_a = E_a + Z_a
```

**Variables:**
- EGL_a = Access hole energy grade line elevation, ft (m)
- E_a = Revised access hole energy level, ft (m)
- Z_a = Access hole invert elevation (same as outflow pipe invert), ft (m)

**Context:** Determines access hole energy grade line elevation. Assumes access hole invert has same elevation as outflow pipe invert. Can use EGL_a as comparison elevation to check for potential surcharging.

---

## Section 9.1.6.7.3 - Inflow Pipe Exit Losses

### Equation 9.30: Inflow Pipe Energy Head (Non-Plunging)

```
EGL_o = E_a + H_o
```

**Variables:**
- EGL_o = Inflow pipe energy head, ft (m)
- E_a = Revised access hole energy grade line, ft (m)
- H_o = Inflow pipe exit loss, ft (m)

**Context:** For non-plunging inflow pipes with hydraulic connection to water in access hole (when E_a > inflow pipe invert). Exit loss calculated using inflow pipe velocity head since supercritical flow not a concern on inflow pipe.

---

### Equation 9.31: Inflow Pipe Exit Loss

```
H_o = K_o V^2/2g
```

**Variables:**
- H_o = Inflow pipe exit loss, ft (m)
- K_o = Exit loss coefficient = 0.4 (Kerenyi et al. 2006)
- V = Velocity in inflow pipe, ft/s (m/s)
- g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)

**Context:** Exit loss calculated in traditional manner using inflow pipe velocity head. Coefficient 0.4 from FHWA research. For plunging pipes, take EGL_o from inflow pipe hydraulics calculations instead - it doesn't depend on access hole conditions.

---

## Section 9.2.2 - Time of Concentration

### Equation 9.32: Contributing Area for Shorter Time of Concentration

```
A_c = A(t_c1 / t_c2)
```

**Variables:**
- A_c = Part of larger primary area contributing during shorter time of concentration, ac (ha)
- A = Area of larger primary area, ac (ha)
- t_c1 = Time of concentration of smaller, less pervious area, min
- t_c2 = Time of concentration of larger primary area, min

**Context:** Used when highly impervious sub-area might dominate design flow. Estimates portion of area relevant to shorter time of concentration. Second calculation uses weighted C value combining smaller less pervious area and area A_c. Designer uses larger of two discharge calculations.

---

## Section 9.2.3 - Minimum Velocity and Grades

### Equation 9.33: Minimum Slope for Design Velocity

```
S = K_u[nV / D^0.67]^2
```

**Variables:**
- S = Minimum slope, ft/ft (m/m)
- K_u = Unit conversion constant, 2.87 in CU (6.35 in SI)
- n = Manning's roughness coefficient
- V = Design velocity (typically 3 ft/s minimum), ft/s (m/s)
- D = Diameter, ft (m)

**Context:** Maintains self-cleaning velocity in storm drain system to prevent sediment deposition and capacity loss. Typically develop storm drains to maintain full flow velocities of 3 ft/s or greater. Computed from Manning's equation rearranged to solve for slope.

---

## Summary Statistics

**Total Equations Documented:** 33

**Sections Covered:**
- Hydraulic Capacity (Manning's Equations): 2 equations
- Pipe Friction Losses: 2 equations
- Exit Losses: 1 equation
- Bend Losses: 1 equation
- Transition Losses: 2 equations
- Junction Losses: 1 equation
- Approximate Access Hole Method: 1 equation
- FHWA Access Hole Method (Initial Energy): 8 equations
- FHWA Access Hole Method (Adjustments): 11 equations
- FHWA Access Hole Method (Exit Losses): 2 equations
- Time of Concentration: 1 equation
- Minimum Velocity/Grades: 1 equation

---

## Implementation Notes

1. **Units:** All equations provided with both Customary Units (CU) and SI conversions where applicable.

2. **Critical Constants:**
   - g = 32.2 ft/s² (9.81 m/s²)
   - K_V = 0.59 (CU), 0.397 (SI)
   - K_Q = 0.46 (CU), 0.312 (SI)
   - K_u = 2.87 (CU), 6.35 (SI)

3. **Key Coefficients:**
   - K_i (entrance loss) = 0.2
   - K_o (exit loss) = 0.4
   - Coefficients for K_ah, C_B vary by configuration (see Tables 9.4, 9.5)

4. **Design Philosophy:** Chapter emphasizes open channel flow design over pressure flow for safety margin, conservative approach given inexact runoff estimation methods and difficulty/cost of replacing storm drains.

5. **Computational Sequence:** Equations designed for upstream-to-downstream preliminary sizing (Sections 9.3), then downstream-to-upstream energy analysis (Section 9.4) for final verification.

---

*Document compiled from HEC-22 4th Edition, Chapter 9: Storm Drain Conduits*
*Date: 2025-11-28*
*All equation numbers, variables, and context preserved from source material*
