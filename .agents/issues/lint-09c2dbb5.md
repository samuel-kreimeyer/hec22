# Issue: Clippy: unnecessary closure used to substitute value for `...

**Type:** lint
**Severity:** error
**Tool:** lint
**Detected:** 2026-01-10T16:22:53.748422Z

## Summary
Linting issue on line 527.

## Evidence
File: src/solver.rs
Line: 527

clippy output:
```
unnecessary closure used to substitute value for `Option::None`
```

## Impact
Code quality issues detected by clippy.

## Recommended Action
Fix the issue according to clippy's suggestion.

## Automation
- Detectable: yes
- Auto-fixable: no

## Metadata
```json
{
  "files": ["src/solver.rs"],
  "lines": [527]
}
```