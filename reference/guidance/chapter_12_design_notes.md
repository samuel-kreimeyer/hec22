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
