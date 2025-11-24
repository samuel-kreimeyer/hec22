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

# Chapter 11 - Urban Stormwater Quality Design Notes

## Implementation Guide for HEC-22 Stormwater Quality Management

---

## 1. Overview

This document provides design notes and implementation guidance for urban stormwater quality management based on HEC-22 Chapter 11. These notes are intended to guide software development and computational implementation of Best Management Practices (BMPs) for highway and urban drainage applications.

---

## 2. Core Concepts

### 2.1 Best Management Practices (BMPs)

**Definition**: Structural or non-structural measures to mitigate adverse impacts of development by managing stormwater quantity, quality, and pollution sources.

**Three Categories**:
1. **Quantity Control**: Attenuate urbanized peak flows and store runoff volumes
2. **Quality Control**: Reduce pollutant loads
3. **Source Control**: Prevent or reduce introduction of pollutants (non-structural)

### 2.2 Water Quality Volume (WQV)

**First Flush Concept**: Initial runoff carries most significant non-point pollutant loads

**Common WQV Estimation Methods**:
- First 0.5 inch of runoff from impervious area
- First 0.5 inch of runoff from entire catchment
- First 1.0 inch of rainfall resulting in runoff from entire catchment

**Important Note**: Treating volumes >1.0 inch provides only minor improvement in pollutant removal efficiency

### 2.3 Key Design Parameters

| Parameter | Description | Typical Values |
|-----------|-------------|----------------|
| WQV | Water quality volume | 0.5-1.0 inch rainfall equivalent |
| Detention Time | Extended detention duration | 24-48 hours |
| Infiltration Rate | Soil permeability | Site-specific (in/hr) |
| Pollutant Removal | Efficiency by pollutant type | Varies by BMP and pollutant |

---

## 3. Pollutant Load Estimation

### 3.1 Simple Method (Eq. 11.1)

```
L = c × R × C × A

where:
  L = Average annual loading (lb or billion colonies)
  c = Unit conversion factor (0.226 for chemicals, 10³ for bacteria)
  R = Annual runoff (inch)
  C = Pollutant concentration (mg/L or 1000/mL)
  A = Area (acres)
```

**Implementation:**
```python
def simple_method_loading(runoff_depth, concentration, area, pollutant_type='chemical'):
    """Calculate average annual pollutant loading using Simple Method

    Args:
        runoff_depth: annual runoff depth (inches)
        concentration: pollutant concentration (mg/L for chemicals, 1000/mL for bacteria)
        area: drainage area (acres)
        pollutant_type: 'chemical' or 'bacteria'

    Returns:
        loading: average annual loading (lb for chemicals, billion colonies for bacteria)
    """
    conversion_factor = 0.226 if pollutant_type == 'chemical' else 1000
    loading = conversion_factor * runoff_depth * concentration * area
    return loading
```

**Applicability**: Sites less than 1 mi² (640 acres)

### 3.2 Advanced Methods

**FHWA Highway Runoff Database (HRDB)**:
- Characterizes stormwater runoff pollutant loads from highways
- Provides statistical data for various constituents

**Stochastic Empirical Loading and Dilution Model (SELDM)**:
- Estimates and simulates stormflow volumes, concentrations, and loads
- Assesses risk of adverse effects on receiving waters
- Evaluates potential effectiveness of mitigation measures

**Other Software Tools**:
- SWMM (Stormwater Management Model) - USEPA
- STORM (Storage, Treatment, Overflow, Runoff Model) - USACE
- HSPF (Hydrologic Simulation Program, Fortran) - USEPA
- STEPL (Spreadsheet Tool for Estimating Pollutant Loads) - USEPA

---

## 4. Structural BMPs: Storage-Based

### 4.1 Extended Detention Dry Ponds

**Function**: Temporarily store runoff for 24-48 hours to allow settling

**Key Design Features**:
- No permanent pool between events
- Hydraulic control structure (riser with hood)
- Low flow channel
- Emergency spillway

**Pollutant Removal**:
- Particulates: up to 90% (with 24+ hour detention)
- Soluble phosphorus/nitrogen: slight reduction

**Design Considerations**:
```python
class ExtendedDetentionPond:
    def __init__(self):
        self.detention_time = 24  # hours (minimum for good performance)
        self.side_slopes = 3.0  # 3:1 (H:V)
        self.freeboard = 1.0  # ft

    def estimate_volume(self, drainage_area, runoff_coef, rainfall_depth):
        """Estimate required storage volume

        Args:
            drainage_area: watershed area (acres)
            runoff_coef: weighted runoff coefficient
            rainfall_depth: design storm depth (inches)

        Returns:
            volume: required storage (ft³)
        """
        runoff_depth = runoff_coef * rainfall_depth
        volume = 3630 * drainage_area * runoff_depth
        return volume
```

**Advantages**:
- Cost-effective (typically <10% more than conventional dry ponds)
- Creates wildlife habitat
- Provides some downstream protection

**Disadvantages**:
- Occasional nuisance problems (odor, debris, weeds)
- Moderate to high maintenance requirements
- Eventual sediment removal needed

### 4.2 Wet Ponds (Retention Ponds)

**Function**: Dual purpose - control runoff volume and treat for pollutant removal

**Key Features**:
- Permanent pool during dry weather
- Multi-stage hydraulic outlets
- Sediment forebay
- Safety benching (2.5-8 ft depth, 10 ft minimum width)

**Design Components**:
- Pool depth: 2.5-8 feet (optimal)
- Side slopes: 3:1 minimum (safety)
- Safety/vegetated ledge: 10 ft wide minimum
- Embankment
- Multi-stage outlets

**Pollutant Removal**:
- Sediment: High
- BOD: High
- Organic nutrients: High
- Trace metals: High
- Soluble nutrients (nitrate, ortho-phosphorus): Moderate (biological processes)

**Water Budget Analysis**:
```python
def wet_pond_water_budget(drainage_area, pool_surface_area, runoff_coef,
                          annual_rainfall, annual_evap, infiltration_rate):
    """Calculate annual water budget for wet pond

    Args:
        drainage_area: watershed area (acres)
        pool_surface_area: pond surface area (acres)
        runoff_coef: weighted runoff coefficient
        annual_rainfall: average annual rainfall (inches)
        annual_evap: average annual evaporation (inches)
        infiltration_rate: average infiltration rate (in/hr)

    Returns:
        net_budget: net annual volume (ft³)
        maintains_pool: boolean indicating if permanent pool persists
    """
    # Runoff inflow
    runoff_volume = 3630 * drainage_area * runoff_coef * annual_rainfall

    # Evaporation loss
    evap_volume = 3630 * pool_surface_area * annual_evap

    # Infiltration loss
    hours_per_year = 24 * 365
    infiltration_volume = infiltration_rate * hours_per_year * pool_surface_area * 3630 / 12

    # Net budget
    net_budget = runoff_volume - evap_volume - infiltration_volume
    maintains_pool = net_budget > 0

    return net_budget, maintains_pool
```

**Advantages**:
- Effective water quality treatment
- Creates wildlife habitat
- Higher property values
- Recreation and landscape amenities

**Disadvantages**:
- Possible habitat degradation (upstream/downstream)
- Downstream sediment imbalance
- Safety hazards
- Nuisance problems (odor, algae, debris)
- Costly sediment removal

---

## 5. Structural BMPs: Infiltration-Based

### 5.1 Infiltration/Exfiltration Trenches

**Function**: Underground reservoir for runoff infiltration or diversion

**Three System Types**:

**1. Complete Exfiltration System**:
- No pipe outlet from trench
- All water exits through soil
- Total peak flow, volume, and quality control
- Overflow for excess runoff (berm or dike)

**2. Partial Exfiltration System**:
- Perforated pipe collects water
- Pipe near top: small storms completely exfiltrate
- Pipe at bottom: acts as short-term detention

**3. Water Quality Exfiltration System (Eq. 11.2)**:
```
WQv = Q × A = (Rv × P) × A

where:
  WQv = Water quality volume
  Q = Depth of runoff (inch)
  Rv = Volumetric runoff coefficient
  P = Rainfall depth (inch)
  A = Drainage area (ac)
```

**Implementation:**
```python
def water_quality_volume(volumetric_runoff_coef, rainfall_depth, area):
    """Calculate water quality volume for exfiltration trench

    Args:
        volumetric_runoff_coef: volumetric runoff coefficient
        rainfall_depth: design rainfall depth (inches)
        area: drainage area (acres)

    Returns:
        wqv: water quality volume (ft³)
    """
    runoff_depth = volumetric_runoff_coef * rainfall_depth
    wqv = 3630 * area * runoff_depth
    return wqv
```

**Design Components**:
- Backfill material (coarse stone)
- Observation wells
- Permeable filter fabric
- Overflow pipes
- Emergency overflow berms
- Vegetated buffer strip

**Soil Requirements**:
- Permeable soils required
- Groundwater table below trench bottom
- Minimum permeability: site-specific

**Advantages**:
- Preserves natural groundwater recharge
- Fits into margins and perimeters
- Good for small sites/infill development

**Disadvantages**:
- Sediment control during construction difficult
- Frequent clogging (high maintenance)
- Possible groundwater contamination risk
- Need careful construction and maintenance

### 5.2 Infiltration Basins

**Function**: Impound and exfiltrate stormwater through permeable basin floor

**Design Types**:
- Combined exfiltration/detention facilities
- Simple infiltration basins

**Drainage Area**: Up to 50 acres

**Components**:
- Stabilized inlet
- Riprap settling basin
- 3:1 side slopes
- Embankment
- Riser with hood
- Valved backup underdrain
- Emergency spillway

**Advantages**:
- Preserves natural water balance
- Serves larger developments
- Useful as sediment basin during construction
- Cost-effective compared to other BMPs

**Disadvantages**:
- High failure rate due to unsuitable soils
- Frequent maintenance required
- Nuisance problems (odors, mosquitoes, soggy ground)

**Failure Prevention**:
- Regular inspection for standing water
- Ensure proper soil conditions
- Adequate pretreatment

### 5.3 Sand Filters

**Function**: Filter first flush runoff through sand bed before discharge

**Types**:
1. **Sand Filter Compartment** (two-chamber system):
   - Sedimentation chamber
   - Sand filter layer (24 inches)
   - Washed gravel layers
   - Underdrain (6-inch PVC)

2. **Peat-Sand Filter** (multi-layer system):
   - Grass cover crop
   - Peat layer (12-18 inches)
   - 50/50 Peat/Sand mix (4 inches)
   - Fine-medium grain sand (20-24 inches)
   - Washed gravel (6 inches)
   - Perforated PVC underdrain

**Pollutant Removal**:
- Sediment: High
- Trace metals: High
- Soluble pollutants: Moderate

**Advantages**:
- Adaptable to thin soils, low infiltration, limited space
- High removal for sediment and metals
- Low failure rate
- Can use where infiltration not feasible

**Disadvantages**:
- Frequent maintenance required
- Unattractive surfaces
- Odor problems

---

## 6. Green Infrastructure and Low Impact Development (LID)

### 6.1 Green Infrastructure Principles

**Definition**: Stormwater management designed to capture rainwater near where it falls by slowing runoff and promoting infiltration to mimic natural processes

**Examples**:
- Green roofs
- Rain gardens (bioretention)
- Grass paver parking lots
- Infiltration trenches
- Permeable pavements
- Bioswales
- Planter boxes
- Rainwater harvesting
- Stormwater tree systems

**Benefits**:
- Ecological improvements
- Economic benefits
- Social benefits
- Climate resilience

### 6.2 Low Impact Development (LID) Strategies

**Principle**: Mimic pre-development hydrology through infiltration, interception, and evapotranspiration

**Common LID Practices**:

**1. Bioretention Areas (Rain Gardens)**:
- Shallow surface depression
- Native vegetation
- Bioretention soil media
- Gravel reservoir
- Underdrain (optional)

**2. Bioretention Swales (Bioswales)**:
- Parabolic or trapezoidal depression
- Bioretention soil media
- Vegetation for infiltration and filtration
- Promotes sedimentation and pollutant removal

**3. Stormwater Curb Extensions (Bump Outs)**:
- Extend curb into roadway
- Reduce traffic speed
- Capture roadway and sidewalk runoff

**4. Stormwater Planters**:
- Narrow, flat-bottomed rectangular areas
- Vertical walls
- Captures urban runoff

**5. Stormwater Tree Systems**:
- Tree or shrub
- Bioretention soil media
- Gravel reservoir
- Intercepts and captures stormwater

**6. Permeable Pavements**:
- Porous asphalt
- Pervious concrete
- Permeable pavers
- Allow infiltration through void spaces

### 6.3 Pollutant Removal Efficiencies

**Green Infrastructure Performance** (from Table 11.1):

| Practice | TSS | TN | TP | Fecal Coliform | Zn | Cu | Pb |
|----------|-----|-----|-----|----------------|-----|-----|-----|
| Bioretention | ● | ○ | ● | - | ● | - | ● |
| Bioswale | ● | ○ | ○ | ○ | - | - | - |
| Curb Extension | ● | ○ | ● | - | ● | - | ● |
| Planter | ● | ○ | ● | - | ● | - | ● |
| Street Trees | ● | ● | ● | ● | ● | ● | ● |
| Infiltration Trench | ● | ○ | ● | ● | ● | - | - |
| Subsurface Infiltration | ● | ● | ● | ● | ● | ● | ● |
| Permeable Pavement | ● | - | ● | - | ● | ● | ● |

**Legend**: ○ = 0-30%; ● = 31-65%; ● = >65%; - = no data

### 6.4 Grassed Swales

**Function**: Convey and filter stormwater runoff

**Design Features**:
- Check dams (reduce velocity)
- Level spreaders (perpendicular excavated depressions)
- Biofiltration swales (increased hydraulic residence time)

**Pollutant Removal**:
- Particulates: Moderate to high (under proper conditions)
- Soluble pollutants: Low

**Design Considerations**:
- Typically cost less than curb and gutter
- Limited capacity for large storms
- Often lead into storm drain inlets
- Refer to HEC-15 for design guidance

**Biofiltration Swale Design**:
- Maximize hydraulic residence time
- Filtration, infiltration, adsorption
- Biological uptake
- Refer to Washington State DOT Highway Runoff Manual

### 6.5 Filter Strips

**Function**: Accept overland sheet flow for filtration and infiltration

**Requirements for Proper Function**:
1. Level spreading device
2. Dense vegetation (erosion-resistant species)
3. Uniform, even, low slope
4. Length ≥ contributing runoff area

**Design Criteria**:
- Use HEC-15 for permissible shear stresses
- High removal of particulate pollutants
- Little data on soluble pollutant removal

**Advantages**:
- Low cost to establish
- Minimal maintenance if preserved before development
- Wildlife habitat
- Stream protection
- Riparian zone preservation

**Limitations**:
- Does not provide significant storage or peak flow reduction
- Tendency for flow concentration (short-circuiting)

### 6.6 Constructed Wetlands

**Function**: Remove pollutants from highway and urban runoff through natural wetland processes

**Design Approach**:
- Often used with detention basin upstream
- Detention basin allows heavy particulates to settle
- Minimizes disturbance to wetland soils and vegetation

**Performance**:
- Comparable to detention basins for monitored pollutants
- Better for some indicators

**Design Considerations**:
- Vegetation must withstand runoff without dislodging
- May not be effective where water's edge is unstable or heavily used
- Flood-prone areas may affect marsh vegetation effectiveness

---

## 7. Ultra-Urban BMPs

### 7.1 Concept

**Definition**: Treatment BMPs installed underground with small footprints for densely developed areas with limited right-of-way

**Applications**:
- Retrofitting urban areas
- New urban development
- Beneath parking lots, garages, rooftops
- Integrated into streetscape

### 7.2 Water Quality Inlets (Pre-cast Storm Drain Inlets)

**Function**: Remove sediment, oil/grease, and large particulates before reaching storm drainage or infiltration BMPs

**Three-Chamber System** (typical):
1. **Sediment Trapping Chamber**:
   - Settles grit and sediment
   - Traps floating debris
   - Permanent pool

2. **Oil Separation Chamber**:
   - Permanent pool
   - Inverted elbow connection
   - Separates oils and hydrocarbons

3. **Final Chamber**:
   - Connected to outlet
   - Trash rack protected orifice

**Applications**:
- Gas stations
- Vehicle repair facilities
- Loading areas
- Areas with high vehicle wastes

**Advantages**:
- Compatible with storm drain network
- Easy to access
- Pretreats runoff before infiltration BMPs
- Unobtrusive

**Disadvantages**:
- Limited pollutant removal capability
- Frequent cleaning required (and not always assured)
- Sediment disposal challenges
- Cost

### 7.3 Other Ultra-Urban Applications

**Filter Inserts**:
- Bag or basket type
- Small openings for low flow
- Overflow for larger flows

**Hydrodynamic Devices**:
- Baffles, vortex mechanisms, settling components
- Separate sediment and pollutants
- Inserted between inlets and pipes

**Sumps**:
- Bottom of access holes
- Below pipe flow lines
- Sediment and debris deposition
- Weep holes release stormwater

---

## 8. Non-Structural BMPs

### 8.1 Common Practices

**1. Storm Drain Cleaning**:
- Remove sediment and debris from pipes and inlets
- Minor water quality improvement
- Most efficient for suspended solids removal

**2. Street Sweeping**:
- Remove sediment from paved surfaces
- Modest water quality benefits
- Sediment, debris, trash/litter removal

**3. Efficient Landscaping Practices**:
- Minimize/eliminate pollutants (fertilizers)
- Avoid excessive irrigation

**4. Trash Management Practices**:
- Minimize public littering
- Minimize windblown trash

**5. Elimination of Groundwater Inflow**:
- Watertight joints
- Elevate pipes above groundwater table
- Prevents perpetual low flows with pollutants

**6. Slope and Channel Stabilization**:
- Vegetating, lining, or reconfiguring
- Reduce erosion

**7. Winter Maintenance**:
- Proper use of deicing chemicals and abrasives
- Post-winter cleanup

**8. Irrigation Runoff Reduction**:
- Maintain landscaped areas
- Reduce overwatering
- Mitigate excess runoff with high pollutant concentrations

---

## 9. BMP Selection Matrix

### 9.1 Selection Criteria

**Physical Conditions**:
- Site topography
- Soil type and permeability
- Groundwater table depth
- Bedrock depth
- Available space

**Watershed Area**:
- Small sites (<1 acre)
- Medium sites (1-10 acres)
- Large sites (>10 acres)

**Stormwater Quantity Objectives**:
- Peak flow attenuation
- Volume reduction
- Groundwater recharge

**Water Quality Objectives**:
- Particulate removal
- Soluble pollutant removal
- Bacteria/pathogen removal
- Metals removal
- Nutrient removal

### 9.2 Implementation Considerations

**Cost Factors**:
- Construction cost
- Operation and maintenance cost
- Life-cycle cost
- Land cost (if applicable)

**Performance Factors**:
- Pollutant removal efficiency
- Reliability
- Failure rate
- Maintenance requirements

**Regulatory Factors**:
- NPDES requirements
- Local ordinances
- State environmental regulations
- Stormwater management requirements

**Site-Specific Factors**:
- Available space
- Aesthetics
- Safety
- Access for maintenance
- Integration with project design

---

## 10. Computational Implementation Roadmap

### 10.1 Phase 1: Pollutant Loading Models

**Priority Components**:
1. Simple Method calculator (Eq. 11.1)
2. Water quality volume calculator (Eq. 11.2)
3. Runoff coefficient database
4. Pollutant concentration database

**Data Structures**:
```python
class PollutantLoader:
    def __init__(self):
        self.method = 'simple'  # or 'hrdb', 'seldm'
        self.pollutant_concentrations = {}
        self.annual_runoff = 0
        self.drainage_area = 0

    def calculate_loading(self, pollutant_type):
        """Calculate pollutant loading"""
        pass
```

### 10.2 Phase 2: BMP Design Tools

**Storage-Based BMPs**:
```python
class BMPDesigner:
    def __init__(self, bmp_type):
        self.bmp_type = bmp_type  # 'extended_detention', 'wet_pond', etc.
        self.drainage_area = 0
        self.removal_efficiencies = {}

    def size_facility(self, design_storm, wqv):
        """Size BMP facility"""
        pass

    def estimate_removal(self, pollutant):
        """Estimate pollutant removal efficiency"""
        pass
```

**Infiltration-Based BMPs**:
```python
class InfiltrationBMP:
    def __init__(self, system_type):
        self.system_type = system_type  # 'trench', 'basin', 'sand_filter'
        self.soil_permeability = 0
        self.groundwater_depth = 0

    def check_feasibility(self):
        """Check if site is suitable"""
        pass

    def size_system(self, wqv):
        """Size infiltration system"""
        pass
```

### 10.3 Phase 3: Advanced Features

- BMP selection optimization
- Cost-benefit analysis
- Life-cycle cost comparison
- Multiple BMP treatment train design
- Integration with SWMM or other models
- Climate change impact assessment
- Adaptive management strategies

### 10.4 Testing Strategy

**Unit Tests**:
- Pollutant loading equations
- WQV calculations
- BMP sizing algorithms
- Removal efficiency models

**Integration Tests**:
- Complete BMP design workflows
- Treatment train performance
- Comparison with published examples

**Validation Data**:
- FHWA HRDB data
- State DOT BMP performance monitoring
- Published research studies
- Manufacturer performance data

---

## 11. Key Equations Reference

### Pollutant Loading

| Equation | Description | Reference |
|----------|-------------|-----------|
| L = c × R × C × A | Simple Method | Eq. 11.1 |

### Water Quality Volume

| Equation | Description | Reference |
|----------|-------------|-----------|
| WQv = Q × A = (Rv × P) × A | Water quality volume | Eq. 11.2 |

---

## 12. Design Standards and Guidelines

### 12.1 Water Quality Volume

**Typical Values**:
- 0.5 inch from impervious area
- 0.5 inch from entire catchment
- 1.0 inch rainfall from entire catchment

### 12.2 Detention Times

| BMP Type | Detention Time |
|----------|----------------|
| Extended Detention | 24-48 hours |
| Wet Pond | Permanent pool |
| Bioretention | Hours to days |

### 12.3 Side Slopes

| Application | Slope (H:V) |
|-------------|-------------|
| Safety requirement | 3:1 minimum |
| Typical ponds | 3:1 to 5:1 |
| Mowing access | 4:1 or flatter |

---

## 13. References and Resources

### HEC-22 Sections
- Section 11.1: BMP Alternatives and Selection
- Section 11.2: Pollutant Loads
- Section 11.3: Structural BMPs
- Section 11.4: Green Infrastructure
- Section 11.5: Ultra-Urban BMPs
- Section 11.6: Non-Structural BMPs

### Related Standards
- NPDES requirements (Clean Water Act)
- State and local stormwater ordinances
- AASHTO Highway Drainage Guidelines
- EPA Stormwater Management Model (SWMM)

### Additional Resources
- FHWA Highway Runoff Database (HRDB)
- SELDM (Stochastic Empirical Loading and Dilution Model)
- EPA Green Streets Handbook
- State DOT BMP design manuals
- Schueler (1987) - Controlling Urban Runoff
- NASEM reports on BMPs

---

**Document Version:** 1.0
**Last Updated:** November 2025
**Based on:** HEC-22 4th Edition, Chapter 11

# Chapter 12 - Pump Stations Design Notes

## Implementation Guide for HEC-22 Stormwater Pump Station Design

---

## 1. Overview

This document provides design notes and implementation guidance for stormwater pump stations based on HEC-22 Chapter 12. These notes are intended to guide software development and computational implementation of pump station design procedures for highway applications.

**Critical Note**: Pump stations should only be considered where gravity flow systems are not feasible due to high life-cycle costs and operational complexity.

---

## 2. Core Concepts

### 2.1 When to Use Pump Stations

**Primary Indication**: Elevation and topography prohibit gravity flow

**Gravity Alternatives to Consider First**:
- Deep conduit trenches
- Tunnels
- Siphons
- Groundwater recharge basins

### 2.2 Unique Design Challenges

**Multidisciplinary Requirements**:
- Hydraulic engineering
- Electrical engineering
- Mechanical engineering
- Structural/building design
- Control systems

**Operational Challenges**:
- High life-cycle costs
- Maintenance requirements
- Power reliability
- Equipment failure risks
- Safety considerations

### 2.3 Key Design Parameters

| Parameter | Description | Typical Values |
|-----------|-------------|----------------|
| Design AEP | Annual Exceedance Probability | 0.02 (major highways) |
| Total Dynamic Head (TDH) | Total head required | Site-specific |
| Pump Cycle Time | Minimum time between starts | Motor-dependent |
| Storage Volume | Volume between start/stop | Calculated from routing |
| NPSH Required | Net Positive Suction Head | Pump-specific |

---

## 3. Pump Station Types

### 3.1 Wet-Pit Stations

**Configuration**:
- Pumps submerged in wet well or sump
- Motors and controls located overhead
- Water pumped vertically through riser pipe

**Drive Shaft Design**:
- Motor connects to pump via drive shaft
- Shaft located in center of riser pipe
- Longer maintenance requirements

**Submersible Pump Design**:
- Motor and pump combined unit
- Less maintenance (no long drive shaft)
- Easier removal with rail systems
- Can remove without entering wet well

**Advantages**:
- Lower cost than dry-pit
- Commonly used for stormwater
- Space-efficient

**Disadvantages**:
- More difficult maintenance access
- Equipment exposed to water
- Limited storage adaptability

### 3.2 Dry-Pit Stations

**Configuration**:
- Separate wet well (storage) and dry well (pumps)
- Horizontal suction pipe connects wet to dry well
- Pumps housed in dry well
- Radial flow pumps typically used

**Advantages**:
- Ease of access for maintenance and repair
- Equipment protection from fire and explosion
- Adaptable for storage volume

**Disadvantages**:
- Higher cost than wet-pit
- Requires more space
- More complex construction

### 3.3 Selection Criteria

**Use Wet-Pit When**:
- Budget constraints exist
- Space is limited within ROW
- Stormwater pumping (low hazard)
- Simpler design acceptable

**Use Dry-Pit When**:
- Maintenance accessibility is critical
- Equipment protection required
- Space is available
- Budget allows higher cost

---

## 4. Pump Types

### 4.1 Axial Flow Pumps

**Operation Principle**:
- Water moves along axis of rotation
- Propeller-like impeller
- Open water operation

**Performance Characteristics**:
- Best for: Low head, high discharge
- Efficiency: High in design range

**Advantages**:
- Large volume capacity
- Good efficiency for low head

**Disadvantages**:
- Poor debris handling (propeller damage)
- Fibrous material wrapping

**Applications**:
- Low lift stations
- High flow rates
- Clean water (minimal debris)

### 4.2 Radial Flow Pumps

**Operation Principle**:
- Water enters along axis
- Impeller "flings" water outward (perpendicular to axis)
- Centrifugal force increases head
- Scroll-shaped casing

**Performance Characteristics**:
- Best for: High head applications
- Efficiency: Good across range, excellent at high head

**Debris Handling**:
- Single vane, open impeller: Best
- Multiple vanes: Reduced opening size
- Decreasing vanes = better debris handling

**Applications**:
- High lift stations
- Variable head conditions
- Stormwater with debris

### 4.3 Mixed Flow Pumps

**Operation Principle**:
- Transition between axial and radial
- Flow direction changes at angle (not perpendicular)
- Often multi-stage design
- Water redirected back along axis

**Multi-Stage Configuration**:
- Multiple impellers on common shaft
- Progressive energy addition
- Each stage adds more head

**Performance Characteristics**:
- Best for: Intermediate head and discharge
- Efficiency: Good for wide range

**Advantages**:
- Better debris shedding than axial
- Multi-stage capability
- Most submersible pumps use this type

**Applications**:
- Medium lift stations
- Submersible installations
- Moderate debris conditions

### 4.4 Pump Selection by Specific Speed

**Specific Speed Ranges**:
- Axial flow: High specific speed
- Mixed flow: Medium specific speed
- Radial flow: Low specific speed

**Selection Process**:
1. Review existing pump station designs
2. Consult manufacturer performance curves
3. Calculate pump specific speed
4. Match to appropriate pump type

---

## 5. Pump Selection and Sizing

### 5.1 System Curve (Eq. 12.1)

```
TDH = Hs + Hf + Hv + Hl

where:
  TDH = Total dynamic head (ft)
  Hs = Static head (ft)
  Hf = Friction head loss (ft)
  Hv = Velocity head (ft)
  Hl = Losses through fittings, valves, etc. (ft)
```

**Implementation:**
```python
def calculate_tdh(static_head, friction_loss, velocity_head, minor_losses):
    """Calculate total dynamic head for pump system

    Args:
        static_head: vertical lift required (ft)
        friction_loss: pipe friction losses (ft)
        velocity_head: velocity head (ft)
        minor_losses: losses through fittings, valves, etc. (ft)

    Returns:
        tdh: total dynamic head (ft)
    """
    tdh = static_head + friction_loss + velocity_head + minor_losses
    return tdh
```

**Static Head**:
- Vertical lift required
- Difference between outlet and inlet water surface
- Varies with storage water levels
- May vary if outlet elevation fluctuates

**Friction Head**:
- Pipe friction losses (Manning's or Darcy-Weisbach)
- Depends on pipe size, length, roughness
- Varies with flow rate (Q²)

**Velocity Head**:
- V²/(2g)
- Usually small component
- Included for completeness

**Minor Losses**:
- Valves (check, gate, air/vacuum)
- Bends and fittings
- Pipe expansions/contractions
- Entrance/exit losses

**Design Considerations**:
- Carefully select discharge line size
- Match or exceed pump outlet size
- Balance cost vs. head loss
- Minimize valves and fittings
- Include expansion loss if pipe larger than outlet

### 5.2 Pump Performance Curve

**Manufacturer-Provided Information**:
- TDH vs. pump capacity (discharge)
- Efficiency curves
- Horsepower requirements
- NPSH required

**Operating Point**:
- Intersection of system curve and pump curve
- Actual pump performance
- Varies with water level (changing TDH)

**Design Points to Specify**:
1. Near highest head (lowest water level)
2. At design head (design water level)
3. At lowest head (highest water level)

**Efficiency Considerations**:
- Select pump for best efficiency at design point
- Design point corresponds to design water level
- Stormwater pump efficiency varies by type

### 5.3 Number of Pumps

**Recommended Minimum**: 2-3 pumps

**Two-Pump System**:
- Each pump: 66-100% of required discharge
- Provides redundancy for pump failure
- Simpler operation and maintenance

**Three-Pump System**:
- Each pump: 50% of design flow
- Two pumps operating provides 100% capacity
- Better redundancy
- More operational flexibility

**Equal Size Benefits**:
- Free alternation into service
- Even load distribution
- Simplified maintenance scheduling
- Interchangeable parts
- Automatic alternation system recommended

**Automatic Alternation**:
- Rotate lead and lag pump after each cycle
- Equalizes wear
- Reduces cycling storage requirements
- Hour and start meters aid maintenance scheduling

**Sizing Limits**:
- Power unit size limitations
- Practical operation and maintenance constraints
- Damage assessment if one pump fails

### 5.4 Net Positive Suction Head (NPSH)

**Definition**: Head above vapor pressure required to prevent cavitation at impeller

**Cavitation Prevention**:
- Maintain sufficient water depth above pump inlet
- Ensure available NPSH > required NPSH
- Prevent vortex formation

**NPSH Factors**:
- Pump type and speed
- Ambient atmospheric pressure (altitude)
- Water temperature
- Submergence depth

**Design Requirement**:
- Manufacturer provides NPSH required (lab testing)
- Designer calculates NPSH available
- Must ensure: NPSH available > NPSH required + safety factor

---

## 6. Pump Station Components

### 6.1 Water-Level Sensors

**Purpose**: Automatic pump operation without human intervention

**Sensor Types**:
- Float switches
- Electronic probes
- Ultrasonic devices
- Mercury switches
- Air pressure switches

**Critical Function**:
- Control starting and stopping of pump motors
- Must prevent excessive cycling
- Set to achieve minimum cycle time

**Minimum Cycle Time**:
- Prevents motor/engine damage
- Typically specified by manufacturer
- Requires sufficient storage volume between start/stop

**Storage Volume Between Start/Stop**:
```python
def calculate_cycling_storage(pump_flow, min_cycle_time):
    """Calculate minimum storage between pump start and stop elevations

    Args:
        pump_flow: pump discharge rate (ft³/s)
        min_cycle_time: minimum time between starts (seconds)

    Returns:
        storage: required storage volume (ft³)
    """
    storage = pump_flow * min_cycle_time
    return storage
```

### 6.2 Power

**Electric Motors** (Most Common):
- Least maintenance
- Least oversight required
- Lowest cost (when available)
- Most reliable

**Fuel-Driven Engines**:
- Gasoline
- Diesel
- Natural gas
- Considerations:
  - Reliable fuel storage (minimize leakage)
  - Fuel perishability
  - Periodic maintenance required
  - Must start reliably without oversight

**Selection Factors**:
- Future energy costs
- Station reliability
- Maintenance requirements
- Capital cost
- Availability of utility power

**Backup Power**:
- Consider for critical installations
- Two independent electrical feeds with automatic transfer switch
- Mobile generators (trailer-mounted for multiple stations)
- Permanent backup generator
- Evaluate consequences of failure vs. backup cost

**Note**: Storms requiring pumping often cause power outages - backup power often necessary

### 6.3 Discharge System

**Preferred Design**:
- Lift vertically
- Discharge through individual lines
- Connect to gravity storm drain quickly
- Minimize piping complexity

**Frost Depth Considerations**:
- Frozen discharge pipes can damage pumps
- Bury below frost depth
- Provide drainage/freeze protection

**Force Main Design** (long discharge lines):
- May combine lines from multiple pump stations
- Requires check valves (prevent backflow)
- Gate valves for isolation during repair
- Cost analysis for optimal length and type

**Check Valves**:
- Prevent backflow to wet well
- Prevent pump restart from backflow
- Prevent pump direction reversal
- Preferably in horizontal lines
- Spring-assisted "non-slam" type to prevent water hammer

**Gate Valves**:
- Shut-off for pump or valve removal
- Either fully open or fully closed (not for throttling)
- Minimize number to reduce cost, maintenance, head loss

**Air/Vacuum Valves**:
- Allow trapped air to escape during pump start
- Prevent vacuum damage during pump stop
- Especially important for large diameter pipes
- Not needed if discharge open to atmosphere
- Combination valves at high points in force mains

**Force Main Drainage**:
- Prevent corrosive/hazardous anaerobic conditions
- Provide drainage or removal of stored water
- Water remaining after pumping event becomes nuisance

### 6.4 Flap Gates and Valving

**Flap Gates**:
- Restrict backflow into discharge pipe
- Discourage entry into outfall line
- Not watertight
- Set elevation above normal receiving water level
- May eliminate need for check valves

**Check Valves**:
- Watertight backflow prevention
- Prevent pump restart from backflow
- Prevent motor rotation reversal
- Prevent return flow prolonging operation
- Types: swing, ball, dashpot, electric

**Gate Valves**:
- Shut-off device for maintenance
- Allow pump or valve removal
- Should not throttle flow
- Fully open or fully closed only

**Air/Vacuum Release Valves**:
- Evacuate trapped air during filling
- Allow air entry during drainage
- Critical at high points in force mains

### 6.5 Trash Racks and Grit Chambers

**Trash Racks**:
- At entrance to wet well (if large debris anticipated)
- Simple inclined steel bar screens
- Standardized modules for easy replacement
- Emergency overflow if relatively small

**Surface Screening Alternative**:
- Screen at surface inlets
- Prevents debris entry into system
- Facilitates maintenance
- Improves hygiene

**Grit Chambers**:
- Capture settleable solids
- Reduce wear on impellers and cases
- Easily accessible location
- Mechanical removal (backhoe, vacuum truck) preferred over manual

### 6.6 Monitoring Systems and Maintenance

**Traditional Monitoring**:
- Onsite warning lights
- Remote alarms
- Status indication

**Modern ITS Integration**:
- Video surveillance
- Electronic message signs
- Cellular/wireless communications
- Remote status transmission
- Real-time data collection

**Monitored Functions**:
- Power status
- Pump operations (start/stop, run time)
- Unauthorized entry
- Explosive fumes
- High water levels
- Inflow/outflow data

**Data Collection Opportunities**:
- Electronic weather monitoring
- Temperature and rainfall measurements
- Operation records (start/stop times, water levels)
- Multi-year performance data
- Valuable for future design improvements
- Real-time data for traffic management

**Maintenance Program**:
- Regular schedule by trained personnel
- Hour meters and start counters
- Preventive maintenance
- Periodic testing of equipment
- Inspection for vandalism, deterioration
- Protection from pests (fire ants, bats, raccoons, snakes)

---

## 7. Site Planning and Hydrology

### 7.1 Location

**Typical Location**: Near low point in drainage system

**Access**:
- Adjacent frontage road or overpass
- High ground if possible (accessible during highway flooding)

**Site Investigation**:
- Soil borings for bearing capacity
- Identify potential problems

**Aesthetic Considerations**:
- Architecturally pleasing modern design
- Screening walls for exterior equipment
- Landscaping and plantings
- Underground placement (if necessary/desirable)
- Unobtrusive parking and work areas
- Community integration

**Construction Considerations**:
- Caisson construction vs. open-pit
- Soil conditions impact method selection
- Construction cost vs. life-cycle cost
- Feedback from construction personnel
- "As-built" drawings for changes

### 7.2 Hydrology

**Design Storm**:
- Major highways: 0.02 AEP typical
- Validate for 0.01 AEP
- Determine flooding extent and risk

**Drainage Area**:
- Keep small to reduce station size
- Minimize impacts of malfunction
- Anticipate future development

**Storage Considerations**:
- High peak discharges occur over short duration
- Additional storage greatly reduces peak pumping rate
- Economic analysis for optimum storage/pumping balance
- Storage typically low-cost compared to pumping capacity
- Refer to Chapter 10 for storage routing procedures

### 7.3 Collection Systems

**Pipe Grades**:
- Typically mild due to topography and depth constraints
- ~3 ft/s velocity when flowing full (avoid siltation)
- Minimum 2% grade in storage units

**Pipe Depth**:
- Minimum cover requirements
- Construction clearance
- Local head requirements
- Uppermost inlets often governed by these factors

**Inflow Distribution**:
- Baffles to ensure equal distribution to all pumps
- Refer to Hydraulic Institute for pump station layout

**Forebay or Storage Box**:
- Collectors terminate at structure
- Or discharge directly into station
- Check collector capacity and storage volume
- Ensure adequate cycling time

**Storage in Collection System**:
- Extensive systems can have significant pipe storage
- Consider when designing collection system
- Especially near pump station

**Debris Screening**:
- At surface: Facilitates maintenance and removal
- In wet well/storage: Alternative location
- Consider accessibility and maintenance convenience

**Hazardous Materials Spill Protection**:
- Isolate pump station from main collection system
- Open forebay or closed box with ventilation
- Prevent gasoline, oils, chemicals from reaching pump station
- Vent volatile gases

---

## 8. Storage and Mass Curve Routing

### 8.1 Storage Volume Considerations

**Balance**: Pump rates vs. storage volume
- More storage = smaller pumps
- Less storage = larger pumps
- Iterative procedure with economic optimization

**Life-Cycle Cost**:
- Construction cost
- Operating cost (minimal for stormwater - infrequent operation)
- Maintenance cost
- For stormwater: construction cost usually dominant

### 8.2 Pump Station Operation Sequence

**Typical Event Sequence**:
1. Water level rises to first pump start elevation
2. First pump starts
3. If inflow > pump rate: level continues rising
4. Second pump starts at its start elevation
5. Continue until inflow subsides or all pumps operating
6. After pumping rate > inflow: level recedes
7. Pumps stop sequentially at stop elevations
8. May cycle again if inflow continues at rate < one pump

**Static Condition**:
- Final water level between lowest NPSH elevation and first pump start

### 8.3 Inflow Mass Curve

**Development**:
1. Divide inflow hydrograph into uniform time increments
2. Compute inflow volume over each time step
3. Sum inflow volumes cumulatively
4. Plot cumulative volume vs. time

**Implementation:**
```python
def create_mass_curve(inflow_hydrograph, time_step):
    """Create cumulative inflow mass curve

    Args:
        inflow_hydrograph: list of (time, flow) tuples
        time_step: time increment (seconds)

    Returns:
        mass_curve: list of (time, cumulative_volume) tuples
    """
    mass_curve = [(0, 0)]
    cumulative_volume = 0

    for i in range(1, len(inflow_hydrograph)):
        # Average flow over time step
        avg_flow = (inflow_hydrograph[i-1][1] + inflow_hydrograph[i][1]) / 2
        # Volume for this time step
        volume = avg_flow * time_step
        cumulative_volume += volume
        mass_curve.append((inflow_hydrograph[i][0], cumulative_volume))

    return mass_curve
```

### 8.4 Mass Curve Routing Procedure

**Required Information**:
1. Inflow hydrograph (from hydrologic evaluation)
2. Stage-storage curve (from physical geometry)
3. Stage-discharge curve (from pump curves and start/stop elevations)

**Routing Process**:
- Plot cumulative inflow (mass curve)
- Plot cumulative outflow (based on pump operation)
- Vertical distance = storage at any time
- Maximum vertical distance = required storage

**Pump Operation Events** (from Figure 12.6):
- Point A: First pump starts
- Point B: Storage empties, pump stops
- Point C: Storage refills, lead pump starts
- Point D: Second pump starts
- Point E: Second pump stops
- Point F: Lead pump stops, storage empties

**Iteration Process**:
1. Try different pump start elevations
2. Find set that minimizes required storage
3. Verify pump cycle time requirements
4. Check against available storage volume

**Spreadsheet Implementation**:
```python
def mass_curve_routing(inflow_hydro, stage_storage, stage_discharge, dt):
    """Route hydrograph through pump station storage

    Args:
        inflow_hydro: list of (time, inflow) tuples
        stage_storage: function or lookup table (elevation -> storage)
        stage_discharge: function (elevation -> discharge)
        dt: time step (seconds)

    Returns:
        results: list of (time, storage, elevation, discharge) tuples
    """
    results = []
    current_storage = 0
    current_elevation = stage_storage.inverse(current_storage)

    for time, inflow in inflow_hydro:
        # Calculate discharge at current elevation
        outflow = stage_discharge(current_elevation)

        # Update storage
        net_inflow = inflow - outflow
        current_storage += net_inflow * dt

        # Find new elevation
        current_elevation = stage_storage.inverse(current_storage)

        results.append((time, current_storage, current_elevation, outflow))

    return results
```

**Time Step Selection**:
- Δt ≤ time to peak / 5
- Typical: 0.05 to 0.2 hours (3 to 12 minutes)
- Smaller Δt: more accurate, more computation
- Check sensitivity to time step

---

## 9. Design Procedure

### 9.1 Overall Design Process

```
1. Site Selection and Planning
   ├─ Determine location
   ├─ Investigate site conditions (soils, access)
   ├─ Consider aesthetics and landscaping
   └─ Plan for construction method

2. Hydrologic Analysis
   ├─ Delineate drainage area
   ├─ Determine design storm (typically 0.02 AEP)
   ├─ Develop inflow hydrograph
   └─ Consider future development

3. Preliminary Pump Sizing
   ├─ Estimate required pump discharge
   ├─ Calculate static head
   ├─ Estimate friction and minor losses
   ├─ Determine preliminary TDH
   └─ Review similar existing stations

4. Storage Evaluation
   ├─ Estimate preliminary storage volume
   ├─ Develop stage-storage curve
   ├─ Balance storage vs. pump capacity
   └─ Economic analysis of alternatives

5. System Curve Development
   ├─ Calculate TDH for range of flows
   ├─ Include all losses
   ├─ Plot system curve
   └─ Check at multiple water levels

6. Pump Selection
   ├─ Obtain manufacturer performance curves
   ├─ Match pump type to system requirements
   ├─ Check efficiency at design point
   ├─ Verify NPSH available > NPSH required
   └─ Determine number of pumps

7. Stage-Discharge Curve
   ├─ Set pump start/stop elevations
   ├─ Create composite curve for multiple pumps
   ├─ Account for cycling time requirements
   └─ Include high water alarm elevation

8. Mass Curve Routing
   ├─ Route inflow hydrograph through system
   ├─ Verify storage adequacy
   ├─ Check pump cycling
   ├─ Iterate on start/stop elevations
   └─ Confirm maximum stage acceptable

9. Component Selection
   ├─ Discharge piping and valves
   ├─ Water level sensors
   ├─ Power supply and backup
   ├─ Monitoring and alarms
   └─ Safety features

10. Final Design and Documentation
    ├─ Prepare plans and specifications
    ├─ Include operation and maintenance manual
    ├─ Specify testing requirements
    ├─ Provide as-built drawings
    └─ Train maintenance personnel
```

### 9.2 Economic Analysis

**Capital Costs**:
- Excavation and construction
- Pump equipment
- Electrical/mechanical systems
- Control systems
- Building/structure
- Force main piping

**Operating Costs** (often minimal for stormwater):
- Energy consumption
- Routine maintenance
- Repairs
- Monitoring

**Life-Cycle Cost Comparison**:
- More storage + smaller pumps
- Less storage + larger pumps
- Different pump types
- Wet-pit vs. dry-pit
- Number of pumps

**Sensitivity Analysis**:
- Future energy costs
- Equipment replacement costs
- Maintenance costs
- Climate change impacts

---

## 10. Safety Considerations

### 10.1 Design for Safe Operation

**Access and Egress**:
- Ladders and stairwells
- Safe access for maintenance personnel
- Emergency escape routes
- Adequate space for equipment operation and maintenance

**Guarding**:
- Moving components (drive shafts)
- Proper lighting
- Air testing equipment for confined space entry

**Ventilation**:
- Proper ventilation essential
- Prevent accumulation of hazardous gases
- Air quality testing before entry

**Security**:
- Prevent unauthorized entry
- Minimize windows
- Locked access points
- Intrusion alarms

**Confined Space Requirements**:
- Pump stations likely classified as confined spaces
- Appropriate access requirements
- Safety equipment
- Entry permits and procedures

### 10.2 Operational Safety

**High Water Alarms**:
- Warn of impending flooding
- Set above second pump start elevation
- Remote notification

**Emergency Procedures**:
- Documented procedures for equipment failure
- Backup power operation
- Flood response
- Traffic management coordination

**Hazardous Materials**:
- Protection from spills in highway corridor
- Isolation of pump station from collection system
- Ventilation for volatile fumes
- Fire protection

---

## 11. Computational Implementation Roadmap

### 11.1 Phase 1: System Curve and Hydraulics

**Priority Components**:
1. TDH calculator (Eq. 12.1)
2. Friction loss calculator (Manning's or Darcy-Weisbach)
3. Minor loss coefficient library
4. System curve generator

**Data Structures**:
```python
class PumpSystem:
    def __init__(self):
        self.static_head = 0
        self.discharge_pipe = {}  # diameter, length, roughness
        self.fittings = []  # list of fittings with K values
        self.valves = []  # list of valves with K values

    def calculate_tdh(self, flow):
        """Calculate total dynamic head at given flow"""
        pass

    def create_system_curve(self, flow_range):
        """Generate system curve for range of flows"""
        pass
```

### 11.2 Phase 2: Pump Selection Tools

**Pump Performance Database**:
```python
class PumpPerformance:
    def __init__(self, pump_id):
        self.pump_id = pump_id
        self.pump_type = ''  # axial, radial, mixed
        self.performance_curve = {}  # flow -> head
        self.efficiency_curve = {}  # flow -> efficiency
        self.power_curve = {}  # flow -> horsepower
        self.npsh_required = {}  # flow -> NPSH

    def get_operating_point(self, system_curve):
        """Find intersection of pump and system curves"""
        pass

    def get_efficiency(self, flow):
        """Get pump efficiency at given flow"""
        pass
```

**Selection Wizard**:
```python
class PumpSelector:
    def __init__(self):
        self.required_flow = 0
        self.tdh_design = 0
        self.pump_database = []

    def recommend_pump_type(self, specific_speed):
        """Recommend pump type based on specific speed"""
        pass

    def find_suitable_pumps(self, criteria):
        """Search database for pumps meeting criteria"""
        pass
```

### 11.3 Phase 3: Storage and Routing

**Stage-Storage Relationship**:
```python
class StageStorage:
    def __init__(self):
        self.elevations = []
        self.storage_volumes = []
        self.geometry_type = ''  # rectangular, circular, irregular

    def interpolate_storage(self, elevation):
        """Get storage at given elevation"""
        pass

    def interpolate_elevation(self, storage):
        """Get elevation for given storage"""
        pass

    def calculate_from_geometry(self, geometry_params):
        """Calculate stage-storage from geometry"""
        pass
```

**Mass Curve Routing Engine**:
```python
class MassCurveRouter:
    def __init__(self):
        self.inflow_hydrograph = []
        self.stage_storage = None
        self.stage_discharge = None
        self.time_step = 300  # seconds

    def route_hydrograph(self):
        """Route inflow through pump station"""
        pass

    def optimize_start_stops(self, n_pumps, cycle_time_min):
        """Find optimal pump start/stop elevations"""
        pass

    def generate_mass_curve(self):
        """Create cumulative inflow curve"""
        pass
```

### 11.4 Phase 4: Complete Design System

**Integrated Design Tool**:
- Input: drainage area, hydrograph, site conditions
- Process: system curve, pump selection, routing
- Output: complete pump station design
- Cost estimation
- Equipment specifications
- Operation and maintenance plan

**Optimization**:
- Number of pumps
- Pump sizes
- Storage volume
- Start/stop elevations
- Life-cycle cost minimization

**Reporting**:
- Design calculations
- Equipment schedules
- Operation and maintenance manual
- Testing and acceptance procedures

### 11.5 Testing Strategy

**Unit Tests**:
- TDH calculations
- Friction loss formulas
- Mass curve generation
- Stage-storage interpolation

**Integration Tests**:
- Complete pump station design workflows
- Comparison with HEC-24 examples
- Validation against existing installations

**Validation Data**:
- HEC-24 example problems
- State DOT pump station designs
- Manufacturer performance data
- Field measurements from operating stations

---

## 12. Key Equations Reference

### Total Dynamic Head

| Equation | Description | Reference |
|----------|-------------|-----------|
| TDH = Hs + Hf + Hv + Hl | Total dynamic head | Eq. 12.1 |

### Friction Losses

Use Manning's equation or Darcy-Weisbach equation for pipe friction losses.

### Minor Losses

| Component | K Value Range |
|-----------|---------------|
| Check valve | 2.0 - 10 |
| Gate valve (open) | 0.2 |
| 90° elbow | 0.9 |
| Tee (flow through run) | 0.6 |
| Entrance (sharp) | 0.5 |
| Exit | 1.0 |

---

## 13. Design Standards and Guidelines

### 13.1 Design Storm

| Facility Type | Typical AEP |
|---------------|-------------|
| Major controlled-access highway | 0.02 |
| Arterial streets | 0.02 |
| Check flooding extent | 0.01 |

### 13.2 Minimum Cycle Time

Varies by motor/engine type - consult manufacturer

Typical range: 4-6 starts per hour maximum

### 13.3 Minimum Number of Pumps

| System Size | Recommended |
|-------------|-------------|
| Small discharge, limited growth | 2 pumps |
| Most applications | 2-3 pumps |
| Large systems | 3+ pumps |

### 13.4 Station Depth

Minimize to reduce cost:
- Only depth needed for pump submergence
- NPSH requirements
- Hydraulic clearance below inlet invert

---

## 14. References and Resources

### HEC-22 Sections
- Section 12.1: Pump Station Types and Pumps
- Section 12.2: Pump Station Components
- Section 12.3: Site Planning and Hydrology
- Section 12.4: Storage and Mass Curve Routing

### Related HEC Documents
- HEC-24: Highway Stormwater Pump Station Design (detailed procedures)
- HEC-22 Chapter 10: Detention and Retention (storage routing)
- HDS-5: Hydraulic Design of Highway Culverts (outlet pipe sizing)

### Additional Standards
- Hydraulic Institute Standards
- AASHTO Drainage Manual
- State DOT design manuals
- Pump manufacturer technical literature
- Electrical and mechanical codes

---

## Appendix A: Implementation Checklist

### Minimum Viable Product (MVP)

- [ ] TDH calculator with all loss components
- [ ] System curve generator
- [ ] Simple pump selection tool
- [ ] Stage-storage calculator for basic geometries
- [ ] Mass curve generator
- [ ] Basic routing algorithm

### Enhanced Version

- [ ] Pump performance database
- [ ] Multiple pump optimization
- [ ] Start/stop elevation optimizer
- [ ] Economic analysis tools
- [ ] Component selection guides
- [ ] Report generation

### Production Ready

- [ ] Comprehensive pump database
- [ ] Multiple station types
- [ ] Life-cycle cost analysis
- [ ] Sensitivity analysis tools
- [ ] Integration with CAD systems
- [ ] Operation and maintenance manual generation
- [ ] Equipment specification templates
- [ ] Compliance checking (codes, standards)

---

## Appendix B: Common Pitfalls and Best Practices

### Common Errors

1. **Inadequate storage between start/stop**
   - Results in excessive pump cycling
   - Damages motors and equipment

2. **Insufficient NPSH**
   - Causes cavitation
   - Pump damage

3. **Undersized discharge piping**
   - Excessive friction losses
   - Higher TDH required
   - Reduced pump efficiency

4. **No backup power**
   - Pump station failure during storms (when needed most)
   - Flooding of highway

5. **Poor access for maintenance**
   - Difficult repairs
   - Safety hazards
   - Higher maintenance costs

### Best Practices

1. **Maximize storage to minimize pump size**
   - Lower capital cost
   - Lower operating cost
   - Improved reliability

2. **Use equal-sized pumps**
   - Interchangeable parts
   - Simplified maintenance
   - Automatic alternation
   - Even wear distribution

3. **Minimize piping complexity**
   - Fewer valves and fittings
   - Lower head losses
   - Reduced maintenance
   - Lower cost

4. **Plan for backup power**
   - Critical for highway safety
   - Consider dual feeds or generator
   - Automatic transfer switch

5. **Design for maintenance**
   - Adequate access
   - Equipment removal capability
   - Safe working environment
   - Monitoring systems

6. **Consider ITS integration**
   - Remote monitoring
   - Real-time data
   - Automated alarms
   - Performance tracking

---

**Document Version:** 1.0
**Last Updated:** November 2025
**Based on:** HEC-22 4th Edition, Chapter 12
