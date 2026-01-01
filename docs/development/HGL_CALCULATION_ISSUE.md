# HGL Calculation Issue - Critical Bug Investigation

## Date
2025-12-31

## Severity
**CRITICAL** - Hydraulic grade line calculations are producing physically impossible results

## Problem Statement

The hydraulic solver is reporting HGL values significantly above rim elevations, indicating severe flooding at nodes, yet simultaneously reporting that conduits are flowing at very low capacity (often <5% full). This is physically impossible.

### Key Contradiction

**If conduits were under pressure (surcharged), they would be flowing full (100% capacity).**

The current analysis shows:
- Conduits flowing at 0.9% - 20% capacity (nearly empty)
- HGL values 4-10 ft above rim elevations (severe flooding/surcharge)

These two conditions cannot coexist. Either:
1. The pipes are nearly empty → HGL should be near invert based on normal depth
2. The pipes are surcharged → Pipes should be 100% full with pressurized flow

## Evidence

### Example 1: Node 115+42 LT (Sag Grate Inlet)

**Node Properties:**
- Invert Elevation: 1275.82 ft
- Rim Elevation: 1279.40 ft
- Structure Height: 3.58 ft

**Analysis Results:**
- **HGL: 1284.35 ft** (4.95 ft ABOVE rim - severe flooding!)
- EGL: 1284.89 ft
- Depth: 8.53 ft

**Outlet Conduit (115+42 LT_to_115+24 LT):**
- Flow: 0.30 cfs
- Diameter: 18 inches
- Length: 17 ft
- Slope: 10.1% (very steep)
- **Capacity Used: 0.9%** (essentially empty!)
- Flow Regime: Supercritical

**Physical Impossibility:**
A pipe that is 99.1% empty cannot create a water surface 5 feet above the rim. The water would simply flow out through the nearly empty pipe.

### Example 2: Node 113+78 LT (Grate Inlet)

**Node Properties:**
- Invert: 1276.0 ft
- Rim: 1280.39 ft
- Structure Height: 4.39 ft

**Analysis Results:**
- **HGL: 1284.97 ft** (4.58 ft above rim)
- Depth: 8.97 ft

**Outlet Conduit (113+78 LT_to_208+65 RT):**
- Flow: 0.38 cfs
- Diameter: 18 inches
- Length: 35 ft
- Slope: 0.71%
- **Capacity Used: 4.3%** (nearly empty)
- Flow Regime: Supercritical

### Example 3: Node 116+25 LT

**Node Properties:**
- Invert: 1276.31 ft
- Rim: 1281.22 ft

**Analysis Results:**
- **HGL: 1286.09 ft** (4.87 ft above rim)
- Depth: 9.78 ft

**Outlet Conduit (116+25 LT_to_115+66 LT):**
- Flow: 2.98 cfs
- Diameter: 18 inches
- Length: 57 ft
- Slope: 2.07%
- **Capacity Used: 19.7%** (only 1/5 full)
- Flow Regime: Supercritical

### Outfalls (Control Points)

The outfalls themselves show reasonable values:

**200+19 RT:**
- Invert: 1272.67 ft
- HGL: 1273.51 ft (0.84 ft above invert - reasonable for free outfall)

**203+48 RT:**
- Invert: 1271.20 ft
- HGL: 1273.33 ft (2.13 ft above invert - reasonable)

**216+90 RT:**
- Invert: 1285.00 ft
- HGL: 1287.79 ft (2.79 ft above invert - reasonable)

**Observation:** The problem develops as the solver propagates upstream from the outfalls.

## Current Implementation Analysis

### HGL Calculation in `src/solver.rs`

#### 1. Outfall Initialization (Lines 140-199)

For free outfalls, tailwater is calculated as:
```rust
// Tailwater depth = average of critical depth and pipe diameter
let tailwater_depth = (critical_depth + diameter) / 2.0;
// HGL at outfall
let hgl = outfall.invert_elevation + tailwater_depth;
```

This appears reasonable for free outfall boundary conditions.

#### 2. Pipe Solving (Lines 628-738)

For each conduit going upstream:

```rust
// Calculate flow properties based on normal depth
let yn = self.mannings.normal_depth(flow, diameter, slope, manning_n, gravity);

// Calculate energy losses
let friction_loss = self.energy_loss.friction_loss(...);
let entrance_loss = self.energy_loss.entrance_loss(...);
let exit_loss = self.energy_loss.exit_loss(...);
let bend_loss = ...;
let total_loss = friction_loss + entrance_loss + exit_loss + bend_loss;

// Calculate upstream HGL/EGL
let downstream_egl = downstream_hgl + flow_result.velocity_head;  // ← LINE 735
let upstream_egl = downstream_egl + total_loss;                    // ← LINE 736
let upstream_hgl = upstream_egl - flow_result.velocity_head;       // ← LINE 737
```

**Critical Issue Identified:**

Line 735 converts HGL to EGL using `flow_result.velocity_head`, which is the velocity head **in the current pipe** based on normal depth.

Line 737 converts back to HGL by subtracting the same velocity head.

**This approach assumes:**
1. The downstream HGL represents a still water surface (no velocity)
2. Energy losses accumulate through the system
3. The water surface rises to match the energy grade line minus velocity head

**This is the correct approach for PRESSURIZED FLOW** but **incorrect for OPEN CHANNEL FLOW**.

#### 3. Junction Loss Application (Lines 254-324)

After solving all conduits:

```rust
// Apply junction loss to upstream EGL
for inlet in &upstream_conduits {
    if let Some(upstream_egl) = node_egls.get_mut(&inlet.from_node) {
        *upstream_egl += junction_head_loss;  // ← Adds to EGL only
    }
}
```

**Additional Issue:** Junction losses are added to EGL but **not to HGL**. The HGL displayed in results doesn't reflect junction losses.

## Root Cause Analysis

### The Fundamental Problem

The solver is using a **pressurized flow energy equation** approach for pipes that are clearly flowing as **open channel flow**.

#### Pressurized Flow (Pipes Running Full)
- Water fills the entire pipe cross-section
- Flow is driven by pressure gradient
- Energy equation: Upstream HGL = Downstream HGL + friction losses + minor losses + elevation change
- HGL can be above the pipe crown (system under pressure)

#### Open Channel Flow (Pipes Partially Full)
- Water flows with a free surface
- Flow is driven by gravity
- Water surface profile controlled by normal depth, critical depth, and hydraulic jumps
- HGL ≈ water surface elevation (plus small velocity head)
- For normal/uniform flow: depth = normal depth based on Manning's equation
- For gradually varied flow: use standard step method or direct step method

### What's Happening

The current implementation:
1. Starts with reasonable tailwater at outfalls
2. For each upstream pipe, calculates normal depth (correct!)
3. Calculates energy losses (reasonable approach)
4. **Adds all energy losses to the water surface** as if the system were pressurized (WRONG!)

Result: The HGL accumulates head as it goes upstream, even though the pipes are flowing nearly empty in open channel mode.

### What Should Happen

For open channel flow in storm sewers:
1. Calculate normal depth for the given flow, slope, and roughness
2. Water surface elevation = invert + normal depth
3. HGL ≈ water surface + velocity head (usually small for partially full pipes)
4. Check for control sections (critical depth at sudden slope changes)
5. Check for hydraulic jumps (supercritical to subcritical transitions)
6. Only use energy equation approach when pipes are actually surcharged (>100% of capacity)

## Expected Behavior

### Example Calculation: Node 115+42 LT

**Given:**
- Flow: 0.30 cfs
- Pipe: 18" diameter = 1.5 ft
- Slope: 10.1% = 0.101 ft/ft (very steep - expect supercritical flow)
- Manning's n: 0.013 (RCP)

**Expected Normal Depth Calculation:**

Using Manning's equation for circular pipe, the normal depth for 0.30 cfs in an 18" pipe at 10% slope would be approximately **0.1-0.15 ft** (about 10% of diameter).

**Expected HGL:**
- Invert at upstream end: 1275.82 + (17 × 0.101) = ~1277.54 ft
- Normal depth: ~0.15 ft
- **Expected water surface: ~1277.69 ft**
- Velocity head (for steep slope, shallow flow): ~0.05-0.10 ft
- **Expected HGL: ~1277.74 - 1277.79 ft**

**Actual HGL from solver:** 1284.35 ft

**Error:** 1284.35 - 1277.79 = **6.56 ft too high!**

## HEC-22 Chapter 9 Findings

**Reference:** FHWA HEC-22 Chapter 9, Section 9.4 - "Hydraulic and Energy Grade Line Evaluation"

### Key Methodology from HEC-22

HEC-22 Section 9.4 provides a detailed 8-step procedure for evaluating HGL and EGL in storm sewer systems.

**Critical Discovery: Table 9.7 - Flow Conditions at Upstream End of Conduit**

HEC-22 defines **four flow conditions** based on the HGL position relative to the pipe:

| Condition | Criterion | Flow Type | Calculation Method |
|-----------|-----------|-----------|-------------------|
| **A** | HGL_i ≥ TOC_i | **Full flow (surcharge)** | Use energy equation with all losses |
| **B** | TOC_i ≥ HGL_i > BOC_i + yn | **Downstream-controlled partial flow** | Use energy equation with all losses |
| **C** | BOC_i + yn ≥ HGL_i > BOC_i + yc | **Subcritical partial flow** | Use energy equation with all losses |
| **D** | BOC_i + yc ≥ HGL_i | **SUPERCRITICAL partial flow** | **Pipe losses NOT carried upstream!** |

Where:
- HGL_i = Hydraulic grade line at upstream end
- TOC_i = Top of conduit at upstream end (invert + diameter)
- BOC_i = Bottom of conduit at upstream end (invert elevation)
- yn = Normal depth for the flow rate
- yc = Critical depth for the flow rate

### The Critical Fix: Supercritical Flow (Condition D)

**From HEC-22 Example 9.2, Pipe 41-42:**

> "BOCi + yc (354.07 + 0.87 = 354.94 ft) ≥ HGLi (346.51 ft), therefore, **supercritical partial flow conditions (condition D). Pipe losses not carried upstream. Recompute HGLi and EGLi.**"

The example then recalculates:
```
HGL_i = BOC_i + yn + V²/2g
```

**This means: For supercritical flow, the upstream HGL is set based on NORMAL DEPTH, not accumulated energy losses!**

### Why Our Current Implementation Fails

Looking at our analysis results, most conduits show:
- **Flow Regime: Supercritical** (Froude number > 1)
- **Low capacity usage** (4-20% full)

These are **Condition D** pipes according to HEC-22. For these pipes:

**HEC-22 says:** Set HGL = Invert + Normal Depth + Velocity Head (don't accumulate losses)

**Our code does:** Accumulates all energy losses upstream (treats like Condition A/B/C)

**Result:** Massively inflated HGL values

### Example from Our Analysis

**Pipe 115+42 LT_to_115+24 LT:**
- Flow: 0.30 cfs
- Diameter: 18"
- Slope: 10.1% (steep - guarantees supercritical)
- Regime: Supercritical ✓
- Capacity: 0.9% full

**What our code does:**
- Takes downstream HGL
- Adds velocity head to get EGL
- Adds friction + entrance + exit losses
- Subtracts velocity head
- Result: HGL = 1284.35 ft (way too high!)

**What HEC-22 says to do (Condition D):**
- Calculate normal depth: yn ≈ 0.15 ft
- HGL_i = BOC_i + yn + V²/2g
- HGL_i ≈ 1277.54 + 0.15 + 0.05 = 1277.74 ft
- **Difference: 6.6 ft error!**

### Flow Regime Classification

Our code correctly calculates Froude number and identifies supercritical flow, but then **ignores this information** when computing HGL!

From our analysis output:
```
Conduit                      Flow    Velocity  Depth   Capacity  Froude  Regime
113+78 LT_to_208+65 RT       0.38    2.51      0.21    4.3%      1.16    Supercritical
115+42 LT_to_115+24 LT       0.30    5.89      0.10    0.9%      3.97    Supercritical
116+25 LT_to_115+66 LT       2.98    6.65      0.45    19.7%     2.06    Supercritical
```

**All of these should use Condition D (normal depth), not energy accumulation!**

### 2. Review HEC-22 Chapter 6: Storm Drain Inlets

Chapter 6 discusses on-grade and sag inlets and their interaction with storm sewer hydraulics.

### 3. Compare with Other References

- **HEC-RAS Storm Sewer Extension** - How does HEC-RAS handle this?
- **SWMM (EPA Storm Water Management Model)** - Dynamic wave routing vs. kinematic wave
- **Bentley StormCAD/FlowMaster** - Implementation approach

## Required Fix: Implement HEC-22 Table 9.7 Flow Conditions

Based on HEC-22 Chapter 9 Section 9.4, the solver must classify each pipe's flow condition and apply the appropriate HGL calculation method.

### Implementation Steps

**1. At upstream end of each pipe, determine flow condition:**

```rust
let boc_i = upstream_invert;  // Bottom of conduit at upstream end
let toc_i = upstream_invert + diameter;  // Top of conduit at upstream end

// Calculate normal depth and critical depth
let yn = normal_depth(flow, diameter, slope, manning_n);
let yc = critical_depth(flow, diameter);

// Classify flow condition based on preliminary HGL from energy equation
let preliminary_hgl = /* from energy equation as currently calculated */;

let flow_condition = if preliminary_hgl >= toc_i {
    FlowCondition::A_Surcharge  // Full flow
} else if preliminary_hgl >= (boc_i + yn) {
    FlowCondition::B_DownstreamControlled  // Partial flow, downstream controlled
} else if preliminary_hgl >= (boc_i + yc) {
    FlowCondition::C_Subcritical  // Subcritical partial flow
} else {
    FlowCondition::D_Supercritical  // Supercritical partial flow
};
```

**2. Apply appropriate HGL calculation:**

```rust
let (upstream_hgl, upstream_egl) = match flow_condition {
    FlowCondition::A_Surcharge |
    FlowCondition::B_DownstreamControlled |
    FlowCondition::C_Subcritical => {
        // Use energy equation with all losses (current implementation)
        let downstream_egl = downstream_hgl + downstream_velocity_head;
        let upstream_egl = downstream_egl + total_losses;
        let upstream_hgl = upstream_egl - upstream_velocity_head;
        (upstream_hgl, upstream_egl)
    }

    FlowCondition::D_Supercritical => {
        // *** FIX: For supercritical flow, use normal depth! ***
        // Do NOT accumulate energy losses
        let upstream_hgl = boc_i + yn + velocity_head;
        let upstream_egl = upstream_hgl + velocity_head;
        (upstream_hgl, upstream_egl)
    }
};
```

**3. Update `solve_pipe()` function in `src/solver.rs`:**

Modify lines 628-738 to:
- Calculate both normal depth (yn) and critical depth (yc)
- Determine flow condition based on Table 9.7 criteria
- Apply condition-specific HGL calculation
- Return flow condition in ConduitResult for reporting

**4. Handle subcritical/supercritical transitions:**

Special cases to handle:
- Hydraulic jumps (supercritical → subcritical)
- Critical depth at slope changes
- Surcharge transitions (partial → full flow)

### Simplified First Implementation

For initial fix, use simplified approach:

```rust
// After calculating preliminary HGL from energy equation
if froude_number > 1.0 {
    // Supercritical flow - use normal depth instead
    upstream_hgl = upstream_invert + normal_depth + velocity_head;
    upstream_egl = upstream_hgl + velocity_head;
} else {
    // Subcritical/surcharged - use energy equation (current method)
    // ... existing code ...
}
```

This will immediately fix the majority of incorrect HGL values since most pipes are flowing supercritical.

### 5. Test Cases Needed

Create validation tests against:
- HEC-22 Chapter 9 examples
- Known storm sewer benchmark problems
- Hand calculations for simple cases

## Current Status

**Impact:** All hydraulic analysis results showing flooding are **unreliable** and likely **overestimated**.

**Root Cause Identified:** ✅
- HEC-22 Chapter 9, Section 9.4, Table 9.7 defines four flow conditions
- Supercritical flow (Condition D) should NOT accumulate energy losses
- Our solver treats all pipes the same way (accumulates losses)
- Result: HGL values 4-10 ft too high for supercritical pipes

**Action Items:**
1. ✅ Document the issue (this file)
2. ✅ Review HEC-22 Chapter 9 hydraulic calculation methodology
3. ✅ Determine correct approach for open channel vs. pressurized flow
4. ⬜ Design solver modifications (outlined above)
5. ⬜ Implement HEC-22 Table 9.7 flow condition classification
6. ⬜ Modify solve_pipe() to use normal depth for supercritical flow
7. ⬜ Create validation tests against HEC-22 Example 9.2
8. ⬜ Regenerate analysis with corrected hydraulics

## Summary

**The Fix:**
For pipes flowing supercritical (Froude > 1), the HGL at the upstream end should be calculated as:
```
HGL = Invert + Normal_Depth + Velocity_Head
```

NOT by accumulating energy losses from downstream.

This aligns with HEC-22 Chapter 9, Section 9.4, Table 9.7, Condition D, and is demonstrated in HEC-22 Example 9.2.

**Expected Impact:**
- HGL values will drop by 4-10 ft for most nodes
- Flooding indicators will be dramatically reduced
- Results will match HEC-22 methodology
- System will correctly identify actual surcharge/flooding locations

## References

- FHWA HEC-22 Chapter 9: Storm Sewer Systems
- FHWA HEC-22 Chapter 6: Storm Drain Inlets
- Open Channel Hydraulics (Chow, 1959)
- Modern approach: HEC-RAS storm sewer extension documentation

## Notes

The inlet interception calculations (Chapter 7) are unaffected by this issue - they correctly use gutter flow calculations based on approach flow and spread. The issue is isolated to the pipe hydraulics and HGL propagation through the network.

However, the incorrect HGL values would affect:
- Flooding assessment
- Surcharge analysis
- Energy grade line slopes
- System adequacy evaluation

## Related Files

- `src/solver.rs` - Lines 628-738 (solve_pipe function)
- `src/solver.rs` - Lines 254-324 (junction loss application)
- `src/hydraulics.rs` - Manning's equation implementations
- `src/energy.rs` - Energy loss calculations
