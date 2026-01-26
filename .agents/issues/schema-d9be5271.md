# Issue: Missing README Section: Documentation

**Type:** schema
**Severity:** warning
**Tool:** check-schema
**Detected:** 2026-01-10T16:22:45.218802Z

## Summary
README.md is missing required section: `## Documentation`

## Evidence
Expected section header: `## Documentation`
Section not found in README.md.

## Impact
The `## Documentation` section is required by the WorkBench schema. This section provides important context for understanding the project.

## Recommended Action
Add the missing section to README.md:

```markdown
## Documentation

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