# Issue: Missing README Section: Purpose

**Type:** schema
**Severity:** warning
**Tool:** check-schema
**Detected:** 2026-01-10T16:22:45.218792Z

## Summary
README.md is missing required section: `## Purpose`

## Evidence
Expected section header: `## Purpose`
Section not found in README.md.

## Impact
The `## Purpose` section is required by the WorkBench schema. This section provides important context for understanding the project.

## Recommended Action
Add the missing section to README.md:

```markdown
## Purpose

[Content here]
```

## Automation
- Detectable: yes
- Auto-fixable: no

## Metadata
```json
{
  "files": ["/home/sam/Projects/hec22/README.md"]
}
```