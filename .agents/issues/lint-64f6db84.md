# Issue: Clippy: very complex type used. Consider factoring parts i...

**Type:** lint
**Severity:** error
**Tool:** lint
**Detected:** 2026-01-10T16:22:53.748432Z

## Summary
Linting issue on line 142.

## Evidence
File: src/visualization/profile.rs
Line: 142

clippy output:
```
very complex type used. Consider factoring parts into `type` definitions
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
  "files": ["src/visualization/profile.rs"],
  "lines": [142]
}
```