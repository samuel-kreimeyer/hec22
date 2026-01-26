# Issue: Missing README Section: Quick Start

**Type:** schema
**Severity:** warning
**Tool:** check-schema
**Detected:** 2026-01-10T16:22:45.218800Z

## Summary
README.md is missing required section: `## Quick Start`

## Evidence
Expected section header: `## Quick Start`
Section not found in README.md.

## Impact
The `## Quick Start` section is required by the WorkBench schema. This section provides important context for understanding the project.

## Recommended Action
Add the missing section to README.md:

```markdown
## Quick Start

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