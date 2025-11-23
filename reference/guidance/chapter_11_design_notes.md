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
