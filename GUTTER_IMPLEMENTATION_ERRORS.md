# Gutter Flow Implementation Errors - Chapter 5

## Critical Errors Found in `/src/gutter.rs`

### Error 1: Equation 5.2 - Incorrect Exponents (Lines 96-99)

**Current Implementation:**
```rust
(k / self.manning_n)
    * self.cross_slope.powf(5.0 / 3.0)      // WRONG: 5/3 = 1.6667
    * self.longitudinal_slope.sqrt()
    * spread.powf(8.0 / 3.0)                 // WRONG: 8/3 = 2.6667
```

**HEC-22 Chapter 5, Equation 5.2:**
```
Q = (Ku/n) Sx^1.67 SL^0.5 T^2.67
```

**Correct Implementation:**
```rust
(k / self.manning_n)
    * self.cross_slope.powf(1.67)           // CORRECT
    * self.longitudinal_slope.powf(0.5)
    * spread.powf(2.67)                      // CORRECT
```

**Impact:** This error affects ALL gutter flow calculations throughout the codebase.

---

### Error 2: Equation 5.4 - Spread Calculation (Lines 110-112)

**Current Implementation:**
```rust
(numerator / denominator).powf(3.0 / 8.0)  // WRONG: Uses 3/8
```

**HEC-22 Chapter 5, Equation 5.4:**
```
T = [(Qn)/(Ku Sx^1.67 SL^0.5)]^0.375
```

**Note:** 3/8 = 0.375, so this is technically correct, but should use 0.375 for consistency with HEC-22 notation.

**Impact:** Minor - numerically equivalent but notation doesn't match source material.

---

### Error 3: Equation 5.7 - Composite Gutter Eo Ratio (Lines 204-207)

**Current Implementation:**
```rust
fn flow_efficiency_ratio(&self, sx_prime: f64) -> f64 {
    let ratio = self.roadway_slope / sx_prime;
    let term = (1.0 + ratio).powf(8.0 / 3.0);
    term / (1.0 + ratio.powf(8.0 / 3.0))
}
```

**HEC-22 Chapter 5, Equation 5.7:**
```
Eo = 1 / [1 + (Sw/Sx) / [(1 + (Sw/Sx)/(T/W-1))^2.67 - 1]]
```

**Where:**
- Sw = cross slope in depressed section
- Sx = cross slope of pavement
- T = total spread
- W = width of depressed section

**Correct Implementation:**
```rust
fn flow_efficiency_ratio(&self, sw: f64, sx: f64, t: f64, w: f64) -> f64 {
    let sw_over_sx = sw / sx;
    let t_over_w = t / w;

    let denominator_term = (1.0 + sw_over_sx / (t_over_w - 1.0)).powf(2.67) - 1.0;
    let eo = 1.0 / (1.0 + sw_over_sx / denominator_term);
    eo
}
```

**Impact:** CRITICAL - This completely changes the flow distribution between depressed section and roadway.

---

### Error 4: Composite Gutter Flow Capacity (Lines 253-258)

**Current Implementation:**
```rust
let q_total = (k / self.manning_n)
    * sx_prime.powf(5.0 / 3.0)  // Wrong exponent
    * self.longitudinal_slope.sqrt()
    * spread.powf(8.0 / 3.0)     // Wrong exponent
    * (1.0 + sw_over_sx.powf(8.0 / 3.0) - (w_over_t).powf(8.0 / 3.0) * sw_over_sx.powf(8.0 / 3.0));
```

**HEC-22 Chapter 5, Example 5.2 Algorithm:**

The composite gutter calculation should NOT use a single formula. Instead, it requires:

1. Split flow into Qw (depressed section) and Qs (side section)
2. For Qs: Use Equation 5.2 with Ts = T - W
3. For Qw/Q ratio: Use Equation 5.7 (Eo)
4. Total Q = Qs / (1 - Eo)

**Impact:** CRITICAL - Wrong algorithm, doesn't match HEC-22 methodology.

---

### Error 5: Missing Equation 5.8 - Depressed Section Cross Slope

**HEC-22 Equation 5.8:**
```
Sw = Sx + a/W
```

**Where:**
- Sw = Cross slope in depressed section
- Sx = Pavement cross slope
- a = Gutter depression depth (ft)
- W = Width of depressed section (ft)

**Current Implementation (Lines 197-199):**
```rust
fn equivalent_cross_slope(&self, depression_ft: f64) -> f64 {
    self.gutter_slope + (depression_ft / self.gutter_width)
}
```

**Status:** This is actually CORRECT, but:
1. Variable name `gutter_slope` should be `pavement_slope` or `cross_slope` for clarity
2. Missing equation reference in docstring
3. Should reference Equation 5.8

---

### Error 6: Composite Gutter Iterative Algorithm Not Implemented

**HEC-22 Example 5.2, Part B shows the required algorithm:**

```
Step B1. Select an initial estimate of Qs.
Step B2. Compute Qw = Q - Qs.
Step B3. Determine W/T ratio using Equation 5.7.
Step B4. Compute spread T = W(T/W).
Step B5. Compute Ts = T - W.
Step B6. Use Equation 5.2 to compute Qs for Ts.
Step B7. Compare computed Qs with assumed Qs.
Step B8. If not close, assume new Qs and repeat.
```

**Current Implementation (Lines 263-290):**
Uses bisection on total spread T, which is NOT the algorithm shown in HEC-22.

**Impact:** CRITICAL - Doesn't follow the documented HEC-22 methodology.

---

### Error 7: Missing Equation References in Docstrings

**Current State:** No equation numbers referenced anywhere in the code.

**Required:** All docstrings should reference equation numbers like:
- "HEC-22 Equation 5.2"
- "Based on Equation 5.7"
- "Example 5.1 verification"

**Impact:** Makes it difficult to trace implementation to source material.

---

### Error 8: Velocity Calculation Method

**Current Implementation (Line 131):**
```rust
let velocity = if area > 0.0 { flow / area } else { 0.0 };
```

**HEC-22 Equation 5.3:**
```
V = (2Ku/n) Sx^0.67 SL^0.5 T^0.67
```

**Status:** Current implementation is CORRECT (Q/A = V), but should also provide the direct formula option and reference Equation 5.3.

---

## Summary of Required Fixes

### Priority 1 (Critical - Affects Accuracy):
1. Fix Equation 5.2 exponents: 5/3 → 1.67, 8/3 → 2.67
2. Fix Equation 5.7 Eo calculation (completely wrong)
3. Reimplement composite gutter flow using correct algorithm from Example 5.2
4. Implement proper iterative method from Example 5.2 Part B

### Priority 2 (Important - Affects Usability):
5. Add equation number references to all docstrings
6. Rename variables to match HEC-22 notation (Sw, Sx, etc.)
7. Add Equation 5.3 velocity formula option

### Priority 3 (Documentation):
8. Create unit tests based on all worked examples (5.1-5.6)
9. Verify against HEC-22 example problems
10. Add algorithm documentation for iterative methods

---

## Test Cases from HEC-22 Worked Examples

### Example 5.1: Triangular Gutter
**Given:** SL=0.01, Sx=0.02, n=0.016
**Part A:** Q=1.8 cfs → Expected T=9.0 ft
**Part B:** T=8.2 ft → Expected Q=1.4 cfs

### Example 5.2: Composite Gutter
**Given:** W=2 ft, SL=0.01, Sx=0.02, n=0.016, a=2 inches
**Part A:** T=8.2 ft → Expected Q=2.3 cfs
**Part B:** Q=4.2 cfs → Expected T=11.1 ft (requires iteration)

### Example 5.3: V-Shaped Gutter
**Given:** SL=0.01, Sx1=0.25, Sx2=0.04, Sx3=0.02, TBC=2.0, n=0.016
**Q=1.77 cfs** → Expected T=8.31 ft

### Example 5.4: V-Shaped Median
**Given:** TAB=TBC=3.28, SL=0.01, Sx1=Sx2=0.25, Sx3=0.04, n=0.016
**Part A:** Q=24.7 cfs → Expected T=13.12 ft
**Part B:** T=23.0 ft → Expected Q=49 cfs

### Example 5.5: Circular Channel
**Given:** D=4.92, SL=0.01, n=0.016, Q=17.6 cfs
**Expected:** d=0.98 ft, Tw=3.93 ft

### Example 5.6: Gutter Flow Time
**Given:** T1=3.28, T2=9.84, SL=0.03, Sx=0.02, n=0.016, L=330 ft
**Expected:** t=1.7 minutes
