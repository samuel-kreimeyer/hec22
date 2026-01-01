# Pump Station Equations

## Reference
Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, Chapter 12

## Total Dynamic Head (TDH)

### System Curve Equation

```
TDH = Hs + Hf + Hv + Hl
```

**Where:**
- TDH = Total dynamic head (ft)
- Hs = Static head (ft)
- Hf = Friction head loss (ft)
- Hv = Velocity head (ft)
- Hl = Minor losses through fittings, valves, etc. (ft)

**Reference:** Equation 12.1

### Static Head

```
Hs = (Elevation_outlet - Elevation_inlet)
```

**Where:**
- Elevation_outlet = Water surface elevation at discharge point (ft)
- Elevation_inlet = Water surface elevation in wet well (ft)

**Notes:**
- Varies with storage water level in wet well
- May vary if outlet elevation fluctuates (tidal, backwater)
- Measure between actual water surfaces, not pipe inverts

### Friction Head Loss

**Using Manning's Equation:**
```
Hf = (n² × L × V²) / (2.22 × R^(4/3))
```

**Where:**
- Hf = Friction loss (ft)
- n = Manning's roughness coefficient
- L = Pipe length (ft)
- V = Flow velocity (ft/s)
- R = Hydraulic radius (ft)

**Using Darcy-Weisbach Equation:**
```
Hf = f × (L/D) × (V²/(2g))
```

**Where:**
- f = Darcy friction factor (dimensionless)
- L = Pipe length (ft)
- D = Pipe diameter (ft)
- V = Flow velocity (ft/s)
- g = 32.2 ft/s²

**Notes:**
- Friction loss varies with Q² (flow squared)
- Changes with pipe size, length, roughness
- Use consistent units throughout calculation

### Velocity Head

```
Hv = V²/(2g)
```

**Where:**
- Hv = Velocity head (ft)
- V = Flow velocity in discharge pipe (ft/s)
- g = Gravitational acceleration (32.2 ft/s²)

**Notes:**
- Usually small component of TDH
- Included for completeness
- Most significant at high velocities

### Minor Losses

```
Hl = Σ(K × V²/(2g))
```

**Where:**
- Hl = Total minor losses (ft)
- K = Loss coefficient for each fitting/valve
- V = Flow velocity (ft/s)
- g = 32.2 ft/s²
- Σ = Sum of all minor loss components

**Common K Values:**

| Component | K Value |
|-----------|---------|
| Check valve (swing type) | 2.0 - 2.5 |
| Check valve (ball type) | 10.0 |
| Gate valve (fully open) | 0.2 |
| Gate valve (3/4 open) | 1.0 |
| Gate valve (1/2 open) | 5.6 |
| 90° elbow (standard) | 0.9 |
| 90° elbow (long radius) | 0.6 |
| 45° elbow | 0.4 |
| Tee (flow through run) | 0.6 |
| Tee (flow through branch) | 1.8 |
| Entrance (sharp-edged) | 0.5 |
| Entrance (well-rounded) | 0.05 |
| Exit | 1.0 |
| Sudden expansion | (1 - A₁/A₂)² |
| Sudden contraction | 0.5(1 - A₁/A₂) |

**Note:** Check valve K values can be significantly higher than theoretical - consult manufacturer data.

## Pump Performance Parameters

### Specific Speed

```
Ns = (N × √Q) / H^(3/4)
```

**Where:**
- Ns = Specific speed (dimensionless)
- N = Pump rotational speed (rpm)
- Q = Discharge at best efficiency point (gpm)
- H = Total head at best efficiency point (ft)

**Pump Type Selection:**
- **Axial flow:** Ns > 9,000 (high flow, low head)
- **Mixed flow:** Ns = 4,000 - 9,000 (medium flow and head)
- **Radial flow:** Ns < 4,000 (high head, low flow)

### Pump Efficiency

```
Efficiency (η) = (Water horsepower / Brake horsepower) × 100%
```

**Water Horsepower:**
```
WHP = (Q × TDH × ρ) / 550
```

**Where:**
- WHP = Water horsepower
- Q = Discharge (ft³/s)
- TDH = Total dynamic head (ft)
- ρ = Water density (62.4 lb/ft³ at 60°F)
- 550 = Conversion factor (ft-lb/s per hp)

**Simplified form:**
```
WHP = (Q × TDH) / 8.8
```

Where Q is in cfs and TDH is in feet.

**Brake Horsepower:**
```
BHP = WHP / η
```

**Where:**
- BHP = Brake horsepower (input power to pump)
- η = Pump efficiency (decimal, e.g., 0.75 for 75%)

### Net Positive Suction Head (NPSH)

**NPSH Available:**
```
NPSH_a = Pa + Hs - Hf_s - Hvp
```

**Where:**
- NPSH_a = Net positive suction head available (ft)
- Pa = Atmospheric pressure head (ft) ≈ 34 ft at sea level
- Hs = Static suction head (ft), positive if above pump, negative if below
- Hf_s = Friction loss in suction pipe (ft)
- Hvp = Vapor pressure head of water (ft) ≈ 0.8 ft at 60°F

**Design Requirement:**
```
NPSH_a ≥ NPSH_r + Safety Factor
```

**Where:**
- NPSH_r = NPSH required (from pump manufacturer)
- Safety Factor = 2-3 ft typical

**Notes:**
- Insufficient NPSH causes cavitation
- NPSH_r varies with flow rate
- Check at all operating points
- Altitude affects Pa significantly

## Storage and Cycling

### Minimum Storage Volume for Pump Cycling

```
V_min = (Q × t_min) / (alternation_factor)
```

**Where:**
- V_min = Minimum storage between start and stop elevations (ft³)
- Q = Pump discharge rate (ft³/s)
- t_min = Minimum cycle time (seconds)
- alternation_factor = Depends on number of pumps and operation strategy

**For single pump:**
```
V_min = Q × t_min
```

**For two equal pumps with alternation:**
```
V_min = 0.5 × Q × t_min
```

**Typical Minimum Cycle Times:**
- Small motors (<10 hp): 10 minutes between starts
- Medium motors (10-50 hp): 15 minutes between starts
- Large motors (>50 hp): 20 minutes between starts
- Maximum starts per hour: 4-6 (varies by manufacturer)

**Note:** Consult motor manufacturer for specific requirements.

### Stage-Storage Relationship

For wet well with vertical walls:
```
V = A_base × h
```

**For cylindrical wet well:**
```
V = π × r² × h
```

**Where:**
- V = Storage volume (ft³)
- A_base = Base area (ft²)
- r = Radius (ft)
- h = Depth of storage (ft)

### Pump Start/Stop Elevations

**Minimum wet well depth:**
```
D_min = D_suction + NPSH_r + S_min
```

**Where:**
- D_min = Minimum wet well depth (ft)
- D_suction = Submergence required for suction inlet (ft)
- NPSH_r = NPSH required by pump (ft)
- S_min = Minimum clearance/safety margin (ft, typically 0.5-1.0 ft)

**Pump start elevation (for lead pump):**
```
El_start1 = El_min + (V_min / A_wetwell)
```

**Where:**
- El_start1 = Start elevation for lead pump (ft)
- El_min = Minimum operating elevation (ft)
- V_min = Minimum cycling volume (ft³)
- A_wetwell = Surface area of wet well (ft²)

## Mass Curve Routing

### Cumulative Inflow Volume

```
V_in(t) = Σ[(Q_i-1 + Q_i) / 2] × Δt
```

**Where:**
- V_in(t) = Cumulative inflow volume at time t (ft³)
- Q_i = Inflow rate at time step i (ft³/s)
- Δt = Time step (seconds)
- Σ = Cumulative sum from start to time t

### Storage at Any Time

```
S(t) = V_in(t) - V_out(t)
```

**Where:**
- S(t) = Storage at time t (ft³)
- V_in(t) = Cumulative inflow (ft³)
- V_out(t) = Cumulative outflow (ft³)

**Maximum storage required:**
```
S_max = max[V_in(t) - V_out(t)]
```

For all time steps during the design storm event.

### Pump Station Drawdown Time

```
t_drawdown = V_stored / Q_pump
```

**Where:**
- t_drawdown = Time to empty storage (seconds)
- V_stored = Volume of stored water (ft³)
- Q_pump = Pump discharge rate (ft³/s)

## Design Checks

### Velocity in Discharge Pipe

```
V = Q / A = Q / (π × D²/4)
```

**Where:**
- V = Velocity (ft/s)
- Q = Flow rate (ft³/s)
- D = Pipe diameter (ft)

**Typical Ranges:**
- Minimum velocity: 2.5-3.0 ft/s (self-cleaning)
- Maximum velocity: 5-10 ft/s (limit erosion/water hammer)
- Optimal range: 3-6 ft/s

### Discharge Pipe Sizing

**Minimum diameter:**
```
D_min ≥ √(4Q / (π × V_max))
```

**Recommended:**
```
D_discharge ≥ D_pump_outlet
```

Match or exceed pump discharge flange size to minimize losses.

### Power Requirements

**Motor Power:**
```
Motor_HP = (BHP × SF) / Motor_efficiency
```

**Where:**
- Motor_HP = Required motor horsepower
- BHP = Brake horsepower (from pump curve)
- SF = Service factor (typically 1.15-1.25)
- Motor_efficiency = Motor efficiency (typically 0.85-0.95)

**Operating Cost (Annual):**
```
Cost = (kW × hours × $/kWh)
```

**Where:**
```
kW = BHP × 0.746
```

**Note:** For stormwater pump stations, operating hours are typically low (infrequent operation).

## Example Calculation

**Given:**
- Design discharge: Q = 10 ft³/s
- Static head: Hs = 15 ft
- Discharge pipe: 12-inch diameter, 200 ft long
- Pipe material: PVC (n = 0.011)
- Fittings: 2 elbows (K=0.9 each), 1 check valve (K=2.5), 1 gate valve (K=0.2)
- Exit loss: K = 1.0

**Step 1: Calculate velocity**
```
D = 12 in = 1.0 ft
A = π × (1.0)² / 4 = 0.785 ft²
V = Q / A = 10 / 0.785 = 12.7 ft/s
```

**Step 2: Calculate friction loss (Manning's)**
```
R = D/4 = 0.25 ft
Hf = (0.011² × 200 × 12.7²) / (2.22 × 0.25^(4/3))
Hf = (0.000121 × 200 × 161.3) / (2.22 × 0.354)
Hf = 3.90 / 0.786
Hf = 4.96 ft
```

**Step 3: Calculate velocity head**
```
Hv = V² / (2g) = 12.7² / (2 × 32.2)
Hv = 161.3 / 64.4
Hv = 2.5 ft
```

**Step 4: Calculate minor losses**
```
K_total = 2(0.9) + 2.5 + 0.2 + 1.0 = 5.5
Hl = 5.5 × (12.7² / 64.4)
Hl = 5.5 × 2.5
Hl = 13.8 ft
```

**Step 5: Calculate TDH**
```
TDH = Hs + Hf + Hv + Hl
TDH = 15 + 4.96 + 2.5 + 13.8
TDH = 36.3 ft
```

**Step 6: Calculate required horsepower**
```
WHP = (Q × TDH) / 8.8
WHP = (10 × 36.3) / 8.8
WHP = 41.3 hp
```

Assuming 75% pump efficiency:
```
BHP = 41.3 / 0.75 = 55 hp
```

Select 60 hp motor (next standard size).

## Design Considerations

### Number of Pumps

**Two-pump system:**
- Each pump: 66-100% of design capacity
- One pump provides redundancy

**Three-pump system:**
- Each pump: 50% of design capacity
- Two pumps operating = 100% capacity
- Better redundancy and flexibility

**Advantages of equal-sized pumps:**
- Automatic alternation
- Interchangeable parts
- Simplified maintenance
- Even wear distribution

### Time Step for Routing

**Recommended:**
```
Δt ≤ (Time to peak) / 5
```

**Typical values:**
- 3-12 minutes (0.05-0.2 hours)
- Smaller Δt = more accuracy, more computation
- Check sensitivity to Δt selection

## SI Units Conversion

For SI units (metric):
- TDH, H, all heads in meters
- Q in m³/s
- V in m/s
- g = 9.81 m/s²
- Power in kW = (Q × TDH × 9.81) / η

## References

- HEC-22 Chapter 12 (FHWA 2024): Pump Stations
- HEC-24: Highway Stormwater Pump Station Design (detailed procedures)
- Hydraulic Institute Standards
- Pump manufacturer technical data
- ASCE/EWRI standards for pump station design
