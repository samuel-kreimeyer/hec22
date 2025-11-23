# Chapter 10 - Detention and Retention Design Notes

## Implementation Guide for HEC-22 Stormwater Storage Facilities

---

## 1. Overview

This document provides design notes and implementation guidance for detention and retention facilities based on HEC-22 Chapter 10. These notes are intended to guide software development and computational implementation of detention/retention basin design procedures.

---

## 2. Core Concepts

### 2.1 Detention vs. Retention

**Detention Facilities (Dry Ponds)**
- Temporarily store runoff and release through controlled outlet
- No permanent pool between storm events
- Primary function: peak flow attenuation
- Secondary benefit: some water quality improvement

**Retention Facilities (Wet Ponds)**
- Maintain permanent pool elevation
- Treat both quantity and quality
- Release through evaporation, infiltration, and surface outlets
- Require water budget analysis

### 2.2 Key Design Parameters

| Parameter | Description | Typical Values |
|-----------|-------------|----------------|
| Design AEP | Annual Exceedance Probability | 0.5, 0.1, 0.01 (multi-stage) |
| Target Release Rate | Pre-development peak (typical) | Site-specific |
| Freeboard | Safety margin above max storage | 1-2 ft minimum |
| Side Slopes | Basin wall slopes | 3:1 to 5:1 (H:V) |
| Minimum Velocity | Self-cleansing in outlet pipe | 2.5-3.0 ft/s |

---

## 3. Design Workflow

### 3.1 Overall Process

```
1. Determine Design Objectives
   ├─ Identify design storm(s) (AEP)
   ├─ Calculate pre-development peak flow
   ├─ Calculate post-development peak flow
   └─ Set target release rate

2. Preliminary Sizing
   ├─ Estimate required storage volume
   ├─ Select basin location and geometry
   └─ Estimate outlet structure type

3. Develop Stage-Storage Relationship
   ├─ Survey or obtain topography
   ├─ Determine basin geometry
   └─ Calculate volume at incremental stages

4. Develop Stage-Discharge Relationship
   ├─ Size outlet pipe (HDS-5 procedures)
   ├─ Size orifices and/or weirs
   └─ Account for emergency spillway

5. Perform Storage Routing
   ├─ Route design hydrograph through facility
   ├─ Check peak outflow vs. target
   └─ Iterate on outlet structure sizing

6. Final Design Checks
   ├─ Verify freeboard requirements
   ├─ Check safety features
   ├─ Confirm maintenance access
   └─ Water budget (for retention)
```

---

## 4. Preliminary Storage Volume Estimation

### 4.1 Method Selection

| Method | Best Use Case | Accuracy | Complexity |
|--------|---------------|----------|------------|
| Loss-of-Natural-Storage | Retention volumes | Conservative | Low |
| Actual Inflow/Estimated Release | Quick estimates | Moderate | Low |
| Triangular Hydrograph | Rational Method designs | Good | Low |
| NRCS TR-55 | NRCS hydrographs | Moderate | Low-Medium |

### 4.2 Implementation Notes

**Loss-of-Natural-Storage Method (Eq. 10.1-10.3)**
```
Q_s = Q_a - Q_b
V_s = α * A * Q_s

where:
  Q_s = storage needed (inches of depth)
  Q_a = post-development runoff depth
  Q_b = pre-development runoff depth
  α = 3,630 (US customary units)
  A = drainage area (acres)
```

**Critical Implementation Details:**
- This gives preliminary estimate only
- Must verify with routing
- More conservative for retention than detention
- Use consistent time durations for Q_a and Q_b

**Triangular Hydrograph Method (Eq. 10.4)**
```
V_s = 0.5 * t_i * (q_i - q_o)

where:
  t_i = duration of basin inflow = 2 * T_c
  q_i = peak inflow rate
  q_o = peak outflow rate (target)
```

**Critical Implementation Details:**
- Works best with Rational Method
- Requires time of concentration
- Simple and reasonably accurate
- Good for preliminary design

---

## 5. Stage-Storage Relationships

### 5.1 Basin Geometry Types

#### Rectangular Basins (Eq. 10.7-10.9)

**Horizontal Bottom:**
```python
V = L * W * D

# Example implementation
def rectangular_storage(length, width, depth):
    """Calculate storage for rectangular basin with horizontal bottom"""
    return length * width * depth
```

**Sloped Bottom:**
```python
V = (L / tan(θ)) * h² + (L * W) * h

# Example implementation
def rectangular_sloped_storage(length, width, depth, theta):
    """Calculate storage for rectangular basin with sloped bottom"""
    import math
    term1 = (length / math.tan(theta)) * depth**2
    term2 = (length * width) * depth
    return term1 + term2
```

#### Trapezoidal Basins (Eq. 10.10-10.11)

```python
V = L*W*D + (L + W)*Z*D² + (4/3)*Z²*D³

where:
  L, W = base dimensions
  D = depth
  Z = side slope (horizontal:vertical ratio)

# Example implementation
def trapezoidal_storage(length, width, depth, z):
    """Calculate storage for trapezoidal basin

    Args:
        length: length of basin at base (ft)
        width: width of basin at base (ft)
        depth: depth of ponding (ft)
        z: side slope factor (H:V), e.g., 3.0 for 3:1 slope
    """
    term1 = length * width * depth
    term2 = (length + width) * z * depth**2
    term3 = (4.0/3.0) * z**2 * depth**3
    return term1 + term2 + term3
```

#### Irregular Basins (Eq. 10.18-10.19)

**Average-End Area Method:**
```python
V = ((A1 + A2) / 2) * Δh

# Incremental implementation
def irregular_storage_incremental(elevations, areas):
    """Calculate cumulative storage from contour data"""
    storage = [0]
    for i in range(1, len(elevations)):
        dh = elevations[i] - elevations[i-1]
        avg_area = (areas[i] + areas[i-1]) / 2
        dV = avg_area * dh
        storage.append(storage[-1] + dV)
    return storage
```

**Conic Section Method (More Accurate):**
```python
V = (Δh / 3) * (A1 + sqrt(A1*A2) + A2)

# Implementation
def conic_storage_incremental(elevations, areas):
    """Calculate cumulative storage using conic approximation"""
    import math
    storage = [0]
    for i in range(1, len(elevations)):
        dh = elevations[i] - elevations[i-1]
        a1, a2 = areas[i-1], areas[i]
        dV = (dh / 3.0) * (a1 + math.sqrt(a1 * a2) + a2)
        storage.append(storage[-1] + dV)
    return storage
```

### 5.2 Implementation Considerations

**Data Structures:**
```python
class StageStorage:
    def __init__(self):
        self.elevations = []  # ft
        self.storage = []     # ft³
        self.areas = []       # ft² (optional, for reference)

    def interpolate_storage(self, elevation):
        """Linear interpolation for storage at given elevation"""
        # Implementation using numpy.interp or custom interpolation
        pass

    def interpolate_elevation(self, storage):
        """Linear interpolation for elevation at given storage"""
        pass
```

---

## 6. Stage-Discharge Relationships

### 6.1 Orifice Flow (Eq. 10.23)

```python
Q = C_o * A_o * sqrt(2 * g * h_o)

where:
  C_o = discharge coefficient (typically 0.6)
  A_o = orifice area (ft²)
  g = gravitational acceleration (32.2 ft/s²)
  h_o = effective head on orifice (ft)
```

**Implementation:**
```python
def orifice_discharge(area, head, coef=0.6, g=32.2):
    """Calculate discharge through an orifice

    Args:
        area: orifice area (ft²)
        head: effective head from centroid of opening (ft)
        coef: discharge coefficient (dimensionless)
        g: gravitational acceleration (ft/s²)

    Returns:
        discharge (ft³/s)
    """
    import math
    if head <= 0:
        return 0.0
    return coef * area * math.sqrt(2 * g * head)
```

**Critical Details:**
- Head measured from centroid of opening
- For submerged orifice, h = difference in water surface elevations
- Use C_o = 0.6 for square-edged openings
- Use C_o = 0.4 for ragged edges (torch-cut)
- Pipes < 1 ft diameter: treat as orifice when h_o/D > 1.5
- Pipes > 1 ft diameter: use HDS-5 culvert hydraulics

### 6.2 Weir Flow

#### Sharp-Crested Weir (Eq. 10.32-10.33)

**No End Contractions:**
```python
Q = C_w * sqrt(2*g) * L * h^1.5

where:
  C_w = weir coefficient (typically 0.37)
  L = weir length (ft)
  h = head above weir crest (ft)
```

**With End Contractions:**
```python
Q = C_w * sqrt(2*g) * (L - 0.2*h) * h^1.5

where:
  C_w = 0.415 for h/h_c < 0.3
```

**Implementation:**
```python
def sharp_weir_discharge(length, head, coef=0.37, end_contractions=False, g=32.2):
    """Calculate discharge over sharp-crested weir

    Args:
        length: weir length (ft)
        head: head above weir crest (ft)
        coef: weir coefficient (dimensionless)
        end_contractions: True if end contractions present
        g: gravitational acceleration (ft/s²)

    Returns:
        discharge (ft³/s)
    """
    import math
    if head <= 0:
        return 0.0

    effective_length = length
    if end_contractions:
        effective_length = length - 0.2 * head
        coef = 0.415  # for h/hc < 0.3

    return coef * math.sqrt(2 * g) * effective_length * head**1.5
```

#### Broad-Crested Weir (Table 10.7)

**Implementation:**
```python
def broad_weir_coefficient(head, breadth):
    """Get broad-crested weir coefficient from lookup table

    Args:
        head: head above weir (ft)
        breadth: breadth of weir crest (ft)

    Returns:
        coefficient (dimensionless)
    """
    # Implement table lookup or interpolation
    # C_w ranges from 0.29 to 0.41
    # Maximum is 0.41 for well-rounded upstream edge
    # Minimum is 0.29 for sharp corners
    pass
```

#### V-Notch Weir (Eq. 10.35)

```python
Q = C_w * sqrt(2*g) * tan(θ/2) * h^2.5

where:
  θ = angle of v-notch (degrees)
  C_w = 0.31 (typical)
```

#### Proportional Weir (Eq. 10.36-10.37)

**Geometry:**
```python
x/b = 1 - 0.315 * arctan((y/a)^0.5)

Q = C_w * sqrt(2*g) * a^0.5 * b * (h - a/3)
```

**Note:** Proportional weirs provide linear head-discharge relationship, reducing required storage but more complex to construct.

### 6.3 Composite Stage-Discharge

**Implementation Strategy:**
```python
class OutletStructure:
    def __init__(self):
        self.components = []  # List of outlet components

    def add_orifice(self, invert_elev, area, coef=0.6):
        """Add orifice to outlet structure"""
        self.components.append({
            'type': 'orifice',
            'invert': invert_elev,
            'area': area,
            'coef': coef
        })

    def add_weir(self, crest_elev, length, coef=0.37):
        """Add weir to outlet structure"""
        self.components.append({
            'type': 'weir',
            'crest': crest_elev,
            'length': length,
            'coef': coef
        })

    def add_spillway(self, crest_elev, length, coef=0.35):
        """Add emergency spillway"""
        self.components.append({
            'type': 'spillway',
            'crest': crest_elev,
            'length': length,
            'coef': coef
        })

    def calculate_discharge(self, stage):
        """Calculate total discharge at given stage"""
        total_q = 0.0

        for comp in self.components:
            if comp['type'] == 'orifice':
                head = stage - comp['invert'] - (comp['area']**0.5 / 2)
                if head > 0:
                    total_q += orifice_discharge(comp['area'], head, comp['coef'])

            elif comp['type'] == 'weir':
                head = stage - comp['crest']
                if head > 0:
                    total_q += sharp_weir_discharge(comp['length'], head, comp['coef'])

            elif comp['type'] == 'spillway':
                head = stage - comp['crest']
                if head > 0:
                    total_q += sharp_weir_discharge(comp['length'], head, comp['coef'])

        return total_q
```

---

## 7. Storage Routing

### 7.1 Modified Puls Method (Eq. 10.45)

**Continuity Equation:**
```
(I1 + I2)/2 - (O1 + O2)/2 = ΔS/Δt

Rearranged:
(2S2/Δt + O2) = (I1 + I2) + (2S1/Δt - O1)
```

**Algorithm:**
```python
def storage_routing(inflow_hydro, stage_storage, stage_discharge, dt):
    """Route hydrograph through detention basin

    Args:
        inflow_hydro: list of (time, inflow) tuples
        stage_storage: StageStorage object
        stage_discharge: OutletStructure object
        dt: time step (seconds)

    Returns:
        outflow_hydro: list of (time, outflow, stage) tuples
    """
    # Initialize
    outflow_hydro = []
    S1 = 0.0  # initial storage
    O1 = 0.0  # initial outflow

    # Calculate initial value
    lhs1 = 2*S1/dt - O1

    for i in range(1, len(inflow_hydro)):
        time, I2 = inflow_hydro[i]
        I1 = inflow_hydro[i-1][1]

        # Right-hand side of routing equation
        rhs = (I1 + I2) + lhs1

        # Solve for S2 and O2 iteratively
        # This requires finding stage where:
        # 2*S(stage)/dt + O(stage) = rhs

        stage2 = solve_routing_equation(rhs, stage_storage, stage_discharge, dt)
        S2 = stage_storage.interpolate_storage(stage2)
        O2 = stage_discharge.calculate_discharge(stage2)

        outflow_hydro.append((time, O2, stage2))

        # Update for next time step
        lhs1 = 2*S2/dt + O2
        S1 = S2
        O1 = O2

    return outflow_hydro

def solve_routing_equation(target, stage_storage, stage_discharge, dt):
    """Solve routing equation for stage

    Find stage where: 2*S(stage)/dt + O(stage) = target
    """
    # Use bisection, Newton-Raphson, or interpolation
    # Implementation depends on available data structures
    pass
```

### 7.2 Time Step Selection

**Guidelines:**
- Δt should be ≤ time to peak / 5
- Typical values: 0.05 to 0.2 hours (3 to 12 minutes)
- Smaller Δt increases accuracy but computation time
- Check sensitivity to Δt

---

## 8. Design Iterations

### 8.1 Single-Stage Riser Design

**Iterative Process:**
```
1. Size outlet pipe using HDS-5 procedures
   - Peak discharge = target release rate
   - Set invert elevation
   - Ensure headwater doesn't submerge lowest opening

2. Estimate preliminary storage (Section 10.3.1)

3. Set dead storage elevation (if wet pond)
   - Obtain Vd from stage-storage curve

4. Calculate total storage: Vt = Vs + Vd

5. Get maximum elevation E1 from stage-storage curve

6. Size orifice or weir:
   For orifice (Eq. 10.39):
     A_o = Q_target / (C_o * sqrt(2*g*(E1-E_o-H_o/2)))

   For weir (Eq. 10.41):
     L_w = Q_target / (C_w * sqrt(2*g) * (E1-E_o)^1.5)

7. Perform storage routing

8. Compare peak outflow to target:
   - If too high: reduce orifice area or weir length
   - If too low: may increase (reduces storage)

9. Iterate until peak ≈ target
```

### 8.2 Multi-Stage Riser Design

**For Two-Stage Example:**
```
1. Design low-stage opening (as single-stage riser)
   - Controls frequent storm (e.g., 0.5 AEP)

2. Estimate high-stage storage (Vs2)

3. Calculate total high-stage storage: Vt2 = Vd + Vs2

4. Get maximum elevation E2

5. Calculate discharge through low-stage opening at E2
   - This contributes to overall discharge

6. Set high-stage weir invert = E1 (max from low-stage event)

7. Size high-stage weir:
   - Must pass: Q_target_high - Q_low_stage_at_E2

8. Perform routing for both design storms

9. Iterate as needed
```

---

## 9. Water Budget Analysis (Retention/Wet Ponds)

### 9.1 Components

```python
def water_budget_annual(drainage_area, pool_surface_area, pool_bottom_area,
                       runoff_coef, annual_rainfall, annual_evap, infiltration_rate):
    """Calculate annual water budget for retention pond

    Args:
        drainage_area: watershed area (acres)
        pool_surface_area: pond surface area (acres)
        pool_bottom_area: pond bottom area (acres)
        runoff_coef: weighted runoff coefficient
        annual_rainfall: average annual rainfall (inches)
        annual_evap: average annual evaporation (inches)
        infiltration_rate: average infiltration rate (in/hr)

    Returns:
        net_budget: net annual volume (ft³)
        maintains_pool: boolean indicating if permanent pool persists
    """
    # Runoff inflow
    runoff_depth = runoff_coef * annual_rainfall  # inches
    runoff_volume = 3630 * drainage_area * runoff_depth  # ft³

    # Evaporation loss
    evap_volume = 3630 * pool_surface_area * annual_evap  # ft³

    # Infiltration loss
    hours_per_year = 24 * 365
    infiltration_volume = (infiltration_rate * hours_per_year *
                          pool_bottom_area * 43560 / 12)  # ft³

    # Net budget
    net_budget = runoff_volume - evap_volume - infiltration_volume
    maintains_pool = net_budget > 0

    return net_budget, maintains_pool
```

### 9.2 Design Considerations

- Perform analysis for average, wet, and dry years
- Account for seasonal variations
- Infiltration rates critical - require field testing
- Consider groundwater table effects
- May need liner if infiltration too high

---

## 10. Safety and Maintenance Features

### 10.1 Safety Checklist

- [ ] Fencing required for deep basins or near public areas
- [ ] Anti-vortex devices on riser inlets
- [ ] Trash racks on outlet structures (removable for maintenance)
- [ ] Mild side slopes (3:1 to 5:1) if public access possible
- [ ] Safety benching below water line for wet ponds
- [ ] Location away from roads/intersections if possible
- [ ] Velocity control at discharge points
- [ ] Warning signage

### 10.2 Maintenance Requirements

**Inspection Schedule:**
- Initial: First few months after construction
- Routine: Annual basis
- Event-based: During and after major storms

**Maintenance Activities:**
- Mowing: At least twice per year
- Sediment removal: As needed (typically every 5-10 years)
- Debris/trash removal: Twice per year minimum
- Outlet structure inspection/repair: Annual
- Vegetation control: Prevent woody growth
- Embankment inspection: Check for erosion, piping

---

## 11. Computational Implementation Roadmap

### 11.1 Phase 1: Core Hydraulics

**Priority Components:**
1. Stage-storage calculators for standard geometries
2. Orifice discharge function
3. Weir discharge functions (sharp-crested, broad-crested)
4. Basic storage routing algorithm

**Data Structures:**
```python
# Suggested class hierarchy
Basin
├── geometry_type: str
├── dimensions: dict
├── stage_storage: StageStorage
└── outlet_structure: OutletStructure

OutletStructure
├── components: list
├── calculate_discharge(stage): float
└── add_component(component): void

Component (base class)
├── Orifice
├── Weir
│   ├── SharpCrestedWeir
│   ├── BroadCrestedWeir
│   └── VNotchWeir
└── Spillway
```

### 11.2 Phase 2: Design Tools

**Iterative Design Engine:**
```python
class DetentionDesigner:
    def __init__(self, drainage_area, target_discharge, inflow_hydro):
        self.drainage_area = drainage_area
        self.target_discharge = target_discharge
        self.inflow_hydro = inflow_hydro

    def estimate_preliminary_volume(self, method='triangular'):
        """Estimate initial storage volume"""
        pass

    def size_outlet_pipe(self):
        """Size outlet pipe using HDS-5 procedures"""
        pass

    def size_outlet_opening(self, opening_type='orifice'):
        """Size orifice or weir for target discharge"""
        pass

    def optimize_design(self, tolerance=0.05):
        """Iterate design until peak outflow within tolerance of target"""
        pass
```

### 11.3 Phase 3: Advanced Features

- Multi-stage riser optimization
- Water quality volume calculations
- Real-time control strategies
- Climate change scenario analysis
- Cost optimization
- GIS integration for basin siting

### 11.4 Testing Strategy

**Unit Tests:**
- Individual hydraulic equations against hand calculations
- Example problems from HEC-22 Chapter 10
- Edge cases (zero flow, very low head, etc.)

**Integration Tests:**
- Complete design examples (Examples 10.1-10.14)
- Comparison with commercial software (StormCAD, HydroCAD)
- Sensitivity analysis

**Validation Data:**
- Use worked examples from Chapter 10
- Create test suite with known inputs/outputs
- Verify routing against analytical solutions for simple cases

---

## 12. Common Pitfalls and Best Practices

### 12.1 Common Errors

1. **Incorrect head measurement for orifices**
   - Must measure from centroid, not top or bottom
   - For circular orifice, subtract D/2 from stage

2. **Forgetting end contractions on weirs**
   - Changes both coefficient and effective length

3. **Submerged vs. unsubmerged flow**
   - Check if tailwater affects outlet control
   - Apply appropriate equations

4. **Inadequate time step in routing**
   - Can cause numerical instability
   - Peak outflow sensitivity check

5. **Single-stage design for multi-storm criteria**
   - May not effectively control all design storms
   - Consider multi-stage outlet

### 12.2 Best Practices

1. **Always route to verify preliminary estimates**
   - Preliminary methods are approximations only

2. **Check outlet pipe capacity**
   - Must be able to convey combined flow from all openings

3. **Provide adequate freeboard**
   - Minimum 1 ft for small basins
   - 2 ft preferred for safety

4. **Design for maintenance**
   - Accessible outlet structures
   - Removable trash racks
   - Positive drainage for dry ponds

5. **Document assumptions**
   - Discharge coefficients used
   - Roughness values
   - Design storm parameters

6. **Sensitivity analysis**
   - Vary coefficients ±10%
   - Check impact on peak outflow

---

## 13. Key Equations Reference

### Storage Volume Estimation

| Equation | Description | Reference |
|----------|-------------|-----------|
| Q_s = Q_a - Q_b | Loss of natural storage | Eq. 10.1 |
| V_s = αAQ_s | Storage from runoff depth | Eq. 10.2 |
| V_s = 0.5t_i(q_i-q_o) | Triangular hydrograph | Eq. 10.4 |

### Basin Geometry

| Equation | Description | Reference |
|----------|-------------|-----------|
| V = LWD | Rectangular basin | Eq. 10.7 |
| V = LWD + (L+W)ZD² + (4/3)Z²D³ | Trapezoidal basin | Eq. 10.10 |
| V = [(A1+A2)/2]d | Average-end area | Eq. 10.18 |

### Outlet Hydraulics

| Equation | Description | Reference |
|----------|-------------|-----------|
| Q = C_o A_o √(2gh_o) | Orifice discharge | Eq. 10.23 |
| Q = C_w √(2g) L h^1.5 | Weir discharge | Eq. 10.32 |

### Routing

| Equation | Description | Reference |
|----------|-------------|-----------|
| ΔS/Δt = (I1+I2)/2 - (O1+O2)/2 | Storage routing | Eq. 10.45 |

---

## 14. References and Resources

### HEC-22 Sections
- Section 10.1: Design Objectives and Challenges
- Section 10.2: Storage Facility Types
- Section 10.3: Design Information (storage, stage-storage, stage-discharge)
- Section 10.4: Water Budgets
- Section 10.5: Storage Routing
- Section 10.6: Design Procedure

### Related HEC Documents
- HDS-5: Hydraulic Design of Highway Culverts (for outlet pipe sizing)
- HEC-HMS: Hydrologic Modeling System (for hydrograph generation)
- HEC-RAS: For complex hydraulic analysis if needed

### Additional Standards
- AASHTO Highway Drainage Guidelines
- Local stormwater management ordinances
- State environmental regulations

---

## Appendix A: Implementation Checklist

### Minimum Viable Product (MVP)

- [ ] Triangular hydrograph preliminary volume estimation
- [ ] Trapezoidal basin stage-storage calculation
- [ ] Orifice discharge calculation
- [ ] Sharp-crested weir discharge calculation
- [ ] Basic storage routing (Modified Puls)
- [ ] Single-stage riser design iteration

### Enhanced Version

- [ ] All preliminary volume methods
- [ ] All basin geometry types
- [ ] All weir types
- [ ] Multi-stage riser design
- [ ] Water budget calculator
- [ ] Optimization algorithms
- [ ] Graphical output (hydrographs, stage-storage curves)

### Production Ready

- [ ] Input validation and error handling
- [ ] Comprehensive unit tests
- [ ] User documentation
- [ ] Example problems with solutions
- [ ] Performance optimization
- [ ] API documentation
- [ ] Integration with GIS/CAD

---

## Appendix B: Sample Validation Problems

### Problem 1: Simple Detention Basin

**Given:**
- Drainage area: 38 acres
- Pre-development peak: 50 ft³/s
- Post-development peak: 131 ft³/s
- Design storm: 0.1 AEP

**Expected Results:**
- Preliminary storage: ~28,000-32,000 ft³
- Weir length: ~1.6 ft (after iteration)
- Peak outflow: ~50 ft³/s
- Maximum stage: ~4.5 ft

*See Example 10.14 for complete solution*

### Problem 2: Water Budget Check

**Given:**
- Drainage area: 100 acres
- Pool surface: 3 acres
- Pool bottom: 2 acres
- Runoff coefficient: 0.3
- Annual rainfall: 50 inches
- Annual evaporation: 35 inches
- Infiltration rate: 0.1 in/hr

**Expected Results:**
- Annual runoff: 5,445,000 ft³
- Annual evaporation: 381,150 ft³
- Annual infiltration: 6,359,760 ft³
- Net budget: -1,295,910 ft³ (does not maintain pool)

*See Example 10.13 for complete solution*

---

**Document Version:** 1.0
**Last Updated:** November 2025
**Based on:** HEC-22 4th Edition, Chapter 10
