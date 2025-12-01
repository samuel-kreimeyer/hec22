# Inlet Hydraulics Review Report

**Date**: 2025-12-01
**Reviewer**: Claude Code
**Reference**: HEC-22 (4th Edition, 2024) Chapter 7 - Inlets

## Executive Summary

This report provides a thorough review of the inlet hydraulic calculations implemented in the `hec22` library, specifically examining traceability to HEC-22 Chapter 7 equations. The review focuses on curb-opening inlets (Section 7.2.2), required equations (7.10, 7.11), and Example 7.2 procedures.

**Key Findings**:
- ✅ Core inlet equations are mathematically correct
- ⚠️ **CRITICAL**: Equation numbering errors in code comments (Eq 7.10 mislabeled as 7-11)
- ⚠️ **CRITICAL**: Equation 7.11 (effective cross slope Se) not properly implemented for curb-opening inlets
- ⚠️ Missing HEC-22 equation number references in many functions
- ✅ Sag inlet design follows HEC-22 Example 7.5 procedure correctly
- ⚠️ Example 7.2 procedure not fully implemented for depressed curb openings

---

## 1. Equation Traceability Analysis

### 1.1 Equation 7.10: Curb-Opening Length for Total Interception

**HEC-22 Reference**: Chapter 7, Section 7.2.2, Equation 7.10

**Equation**:
```
L_T = K_u × Q^0.42 × S_L^0.3 / (n × S_x^0.6)
```

Where:
- L_T = Required length for 100% interception (ft)
- K_u = 0.6 (US customary units)
- Q = Flow rate (cfs)
- S_L = Longitudinal slope (ft/ft)
- S_x = Cross slope (ft/ft)
- n = Manning's roughness coefficient

**Implementation Location**: `src/inlet.rs:223-235`

**Code**:
```rust
/// Calculate required length for 100% interception
///
/// L_T = 0.6 × Q^0.42 × S_L^0.3 / (n × S_x^0.6)
///
/// HEC-22 Equation 7-11  // ❌ INCORRECT LABEL
pub fn length_for_total_interception(
    flow: f64,
    manning_n: f64,
    cross_slope: f64,
    longitudinal_slope: f64,
) -> f64 {
    0.6 * flow.powf(0.42) * longitudinal_slope.powf(0.3)
        / (manning_n * cross_slope.powf(0.6))
}
```

**Status**: ⚠️ **ISSUE FOUND**
- Mathematical implementation: ✅ CORRECT
- Equation reference: ❌ **INCORRECT** - Comment states "HEC-22 Equation 7-11" but should be "HEC-22 Equation 7.10"
- Function is used by `GrateInletOnGrade` but should also be available for curb-opening inlet calculations

**Impact**: Medium - Correct calculation but confusing documentation

**Recommendation**:
1. Correct comment to reference Equation 7.10
2. Note that this function is also called from line 227 for grate inlets, which is not typical

---

### 1.2 Equation 7.11: Effective Cross Slope (Se)

**HEC-22 Reference**: Chapter 7, Section 7.2.2, Equation 7.11

**Equation**:
```
S_e = S_x + S'_w × E_o
```

Where:
- S_e = Effective cross slope (ft/ft)
- S_x = Pavement cross slope (ft/ft)
- S'_w = a/W = Depression slope (ft/ft)
- a = Depression depth (ft)
- W = Gutter width (ft)
- E_o = Ratio of frontal flow to total flow

**Purpose**: When a curb-opening inlet has a local depression, the effective cross slope increases, which reduces the required length for interception.

**Implementation Analysis**:

**Partial Implementation Found**: `src/gutter.rs:193-199`
```rust
fn equivalent_cross_slope(&self, depression_ft: f64) -> f64 {
    self.gutter_slope + (depression_ft / self.gutter_width)
}
```

This calculates `S_x + a/W`, which is the first two terms of Equation 7.11, but it does NOT include the `× E_o` term.

**Status**: ❌ **CRITICAL ISSUE**
- Full Equation 7.11 is NOT implemented
- The `equivalent_cross_slope()` function in `CompositeGutter` calculates only part of the equation
- This function is NOT used in curb-opening inlet efficiency calculations (`src/inlet.rs:279-315`)
- **Impact**: Curb-opening inlets with local depressions may have incorrect interception efficiency

**Current Behavior**:
- `CurbOpeningInletOnGrade::interception()` (line 283) does NOT account for local depression effects on cross slope
- The function calculates length efficiency but does not adjust for Se

**Missing from Code**:
1. No calculation of Se = Sx + S'w × Eo
2. No use of Se in place of Sx when calculating L_T for depressed inlets
3. No method to pass depression geometry to curb-opening inlets

**Example 7.2 Verification**: Example 7.2 in HEC-22 demonstrates:
- Step 1: Calculate L_T without depression using Equation 7.10 with Sx
- Step 2: Calculate L_T with depression using Equation 7.10 with **Se** from Equation 7.11
- Step 3: Compare to show depression reduces required length

**Current Implementation**: Does NOT follow Example 7.2 procedure for depressed curb openings

**Recommendation**:
1. **HIGH PRIORITY**: Add Se calculation to `CurbOpeningInletOnGrade`
2. Add depression parameters (depth, width) to `CurbOpeningInletOnGrade` struct
3. Modify `length_for_total_interception()` to accept either Sx or Se
4. Implement Example 7.2 procedure in a test or example

---

### 1.3 Equation 7.13: Curb-Opening Inlet Efficiency

**HEC-22 Reference**: Chapter 7, Equation 7.13

**Equation**:
```
E = 1 - (1 - L/L_T)^1.8
```

Where:
- E = Interception efficiency (0.0 to 1.0)
- L = Actual inlet length (ft)
- L_T = Required length for 100% interception (ft)

**Implementation Location**: `src/inlet.rs:295-299`

**Code**:
```rust
let l_t = Self::length_for_total_interception(approach_flow, velocity);
let efficiency_gross = if self.length >= l_t {
    1.0
} else {
    1.0 - (1.0 - self.length / l_t).powf(1.8)
};
```

**Status**: ✅ **CORRECT**
- Mathematical implementation: ✅ CORRECT
- Missing equation reference in comment: ⚠️ Should add "// HEC-22 Equation 7.13"

---

### 1.4 Equations 7.14 & 7.15: Sag Inlet Capacity

**HEC-22 Reference**: Chapter 7, Equations 7.14 (weir) and 7.15 (orifice)

**Equation 7.14 (Weir Flow)**:
```
Q_i = C_w × P × d^1.5
```

**Equation 7.15 (Orifice Flow)**:
```
Q_i = C_o × A_g × √(2gd)
```

**Implementation Location**: `src/inlet.rs:419-450`

**Code**:
```rust
// Weir flow (low head)
let cw = 3.0; // Weir coefficient
let q_weir = cw * perimeter * ponding_depth.powf(1.5);

// Orifice flow (high head)
let co = 0.67; // Orifice coefficient
let g = 32.17; // ft/s²
let q_orifice = co * net_area * (2.0 * g * ponding_depth).sqrt();

// Capacity is minimum of weir and orifice
q_weir.min(q_orifice)
```

**Status**: ✅ **CORRECT**
- Mathematical implementation: ✅ CORRECT
- Missing equation references: ⚠️ Should add "// HEC-22 Equation 7.14" and "// HEC-22 Equation 7.15"
- Function documentation mentions equations in prose but not by number

---

## 2. HEC-22 Example Verification

### 2.1 Example 7.2: Curb-Opening Inlet on Grade

**Example Description**: Demonstrates curb-opening inlet interception with and without local depression.

**Example Parameters** (from HEC-22 Chapter 7, pages 97-98):
- Q = 3.5 cfs
- S_L = 0.03 (longitudinal slope)
- S_x = 0.02 (cross slope)
- n = 0.016
- L = 5 ft (inlet length)
- Depression: a = 2 inches = 0.167 ft, W = 2 ft

**Example Steps**:

#### Case 1: No Depression
1. Calculate L_T using Equation 7.10 with Sx = 0.02
2. Calculate E using Equation 7.13
3. Result: E ≈ 20% interception

#### Case 2: With Depression
1. Calculate Eo (frontal flow ratio) based on gutter geometry
2. Calculate Se using Equation 7.11: Se = Sx + (a/W) × Eo
3. Calculate L_T using Equation 7.10 with **Se instead of Sx**
4. Calculate E using Equation 7.13
5. Result: E ≈ 50% interception (improvement due to depression)

**Implementation Status**: ❌ **NOT IMPLEMENTED**

The current `CurbOpeningInletOnGrade::interception()` function does NOT:
- Accept depression parameters
- Calculate Se per Equation 7.11
- Use Se in place of Sx for depressed inlets
- Follow the Example 7.2 procedure

**Test Coverage**: No test exists for Example 7.2

**Recommendation**:
1. Create test: `tests/curb_opening_inlet_test.rs`
2. Implement Example 7.2 as `test_example_7_2_curb_opening_with_depression()`
3. Extend `CurbOpeningInletOnGrade` to support depression geometry

---

### 2.2 Example 7.5: Grate Inlet in Sag

**Example Description**: Demonstrates grate sizing for sag condition using weir and orifice equations.

**Implementation**: `src/inlet.rs:513-625` - `GrateInletSag::design_for_sag()`

**Status**: ✅ **CORRECT**
- Function documentation explicitly references Example 7.5
- Implementation follows 7-step procedure from HEC-22:
  1. Calculate required perimeter for weir (Eq 7.14)
  2. Apply clogging factor
  3. Verify depth with selected grate
  4. Calculate required area for orifice (Eq 7.15)
  5. Apply clogging and opening ratio
  6. Verify depth with selected grate
  7. Use more conservative result (higher depth)

**Test Coverage**: ✅ Verified in `tests/grate_sizing_test.rs:25-159`

---

## 3. Code Structure Analysis

### 3.1 Inlet Types Implementation

| Inlet Type | On-Grade | Sag | HEC-22 Section |
|------------|----------|-----|----------------|
| Grate | ✅ `GrateInletOnGrade` | ✅ `GrateInletSag` | 7.3, 7.4 |
| Curb Opening | ⚠️ `CurbOpeningInletOnGrade` | ✅ `CurbOpeningInletSag` | 7.2.2, 7.5 |
| Combination | ✅ `CombinationInletOnGrade` | ❌ Missing | 7.6 |

**Issues**:
- `CurbOpeningInletOnGrade` missing depression support (Equation 7.11)
- No `CombinationInletSag` structure (combination inlets in sag are less common but mentioned in HEC-22)

---

### 3.2 Missing Equation References

**Functions lacking HEC-22 equation number references**:

| Function | Location | Missing Reference |
|----------|----------|-------------------|
| `CurbOpeningInletOnGrade::length_for_total_interception()` | inlet.rs:322 | Equation 7.10 (currently says 7-15, incorrect formula) |
| `CurbOpeningInletOnGrade::interception()` efficiency calculation | inlet.rs:298 | Equation 7.13 |
| `GrateInletSag::capacity_with_opening_ratio()` weir | inlet.rs:433 | Equation 7.14 |
| `GrateInletSag::capacity_with_opening_ratio()` orifice | inlet.rs:438 | Equation 7.15 |
| `CurbOpeningInletSag::capacity()` weir | inlet.rs:664 | Equation 7.14 (curb opening variant) |
| `CurbOpeningInletSag::capacity()` orifice | inlet.rs:670 | Equation 7.15 (curb opening variant) |

---

## 4. Documentation Issues

### 4.1 Incorrect Equation Reference

**Location**: `src/inlet.rs:226`

**Current**:
```rust
/// HEC-22 Equation 7-11
pub fn length_for_total_interception(
```

**Should Be**:
```rust
/// HEC-22 Equation 7.10
pub fn length_for_total_interception(
```

---

### 4.2 Module-Level Documentation

**Location**: `src/inlet.rs:1-16`

**Current**: States "This module implements inlet design procedures from HEC-22 Chapter 7"

**Issue**: Should explicitly list which equations are implemented:
- Equation 7.10: Curb-opening length (Lt)
- Equation 7.13: Curb-opening efficiency (E)
- Equation 7.14: Sag inlet weir flow
- Equation 7.15: Sag inlet orifice flow

---

### 4.3 Reference Document Issue

**File**: `reference/equations/inlet_design.md:4`

**Current**: "Based on FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual, **Chapter 4**"

**Issue**: Should be Chapter 7, not Chapter 4. Chapter 4 covers gutter flow, Chapter 7 covers inlets.

---

## 5. Recommendations

### Priority 1: Critical Corrections

1. **Implement Equation 7.11 (Se calculation)**
   - Add depression parameters to `CurbOpeningInletOnGrade` struct
   - Implement `calculate_effective_cross_slope()` method
   - Modify `length_for_total_interception()` to use Se when depression exists
   - Add test for Example 7.2

2. **Fix Equation 7.10 Reference**
   - Change "HEC-22 Equation 7-11" to "HEC-22 Equation 7.10" in inlet.rs:226

3. **Correct Reference Document**
   - Fix chapter number in `reference/equations/inlet_design.md`

### Priority 2: Documentation Improvements

4. **Add Equation Number Comments**
   - Add "// HEC-22 Equation X.XX" comments to all formula implementations
   - Add references to specific sections (e.g., "Section 7.2.2")

5. **Expand Module Documentation**
   - List all implemented equations explicitly
   - Add cross-references to test files that verify examples

6. **Add Example 7.2 Test**
   - Create comprehensive test showing depression effects
   - Verify both cases (with/without depression)

### Priority 3: Enhancements

7. **Add Combination Inlet for Sag**
   - Implement `CombinationInletSag` if needed for completeness
   - HEC-22 Section 7.6 covers combination inlets

8. **Enhance Curb Opening Calculations**
   - Current `length_for_total_interception` uses simplified formula (line 322-326)
   - Should account for throat type (Horizontal, Inclined, Vertical)
   - Different Ku values apply per throat configuration

---

## 6. Summary Table: Equation Compliance

| Equation | Description | Status | Location | Issue |
|----------|-------------|--------|----------|-------|
| 7.10 | Curb-opening length (Lt) | ✅ Implemented | inlet.rs:233 | Wrong label (says 7-11) |
| 7.11 | Effective cross slope (Se) | ❌ Missing | - | Not implemented for curb openings |
| 7.13 | Curb-opening efficiency (E) | ✅ Implemented | inlet.rs:298 | Missing comment |
| 7.14 | Sag weir flow | ✅ Implemented | inlet.rs:433, 664 | Missing comment |
| 7.15 | Sag orifice flow | ✅ Implemented | inlet.rs:438, 670 | Missing comment |

---

## 7. Test Coverage Analysis

| Test File | Coverage | Missing |
|-----------|----------|---------|
| `tests/grate_sizing_test.rs` | ✅ Example 7.5 | - |
| `tests/inlet_bypass_test.rs` | ✅ Bypass behavior | - |
| Curb opening test | ❌ Missing | Example 7.2 |
| Depression effects | ❌ Missing | Equation 7.11 verification |

---

## 8. Conclusion

The inlet hydraulics implementation is **mathematically sound** for the equations that are implemented, but has **critical traceability issues**:

### Strengths:
- Core formulas (7.10, 7.13, 7.14, 7.15) are correct
- Sag inlet design follows HEC-22 Example 7.5 procedure
- Good test coverage for grate sizing
- Well-structured code with clear separation of inlet types

### Critical Issues:
- **Equation 7.11 not implemented** - curb-opening inlets with depressions incorrectly sized
- **Incorrect equation label** - Equation 7.10 labeled as 7-11
- **Example 7.2 not implemented** - key validation case missing
- **Missing equation references** - poor traceability

### Action Items:
1. Implement Se calculation (Equation 7.11) for depressed curb openings - **HIGH PRIORITY**
2. Fix equation 7.10 comment label - **HIGH PRIORITY**
3. Add equation number comments throughout - **MEDIUM PRIORITY**
4. Create Example 7.2 test - **MEDIUM PRIORITY**
5. Fix reference document chapter number - **LOW PRIORITY**

---

## References

1. FHWA HEC-22 (4th Edition, 2024) - Urban Drainage Design Manual
2. Chapter 7: Inlets
   - Section 7.2.2: Curb-Opening Inlets on Grade
   - Example 7.2: Curb-Opening Inlet with Depression (pages 97-98)
   - Example 7.5: Grate Inlet in Sag (pages 110-113)
3. Implementation: `/home/user/hec22/src/inlet.rs`
4. Tests: `/home/user/hec22/tests/grate_sizing_test.rs`, `/home/user/hec22/tests/inlet_bypass_test.rs`

---

**Report Status**: Complete
**Next Steps**: Address Priority 1 items (Equation 7.11 implementation and reference corrections)
