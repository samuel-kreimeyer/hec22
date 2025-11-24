# JSON Schema Quick Reference

## Minimal Valid Network

```json
{
  "version": "1.0.0",
  "project": {
    "name": "My Project",
    "units": {
      "system": "US"
    }
  },
  "network": {
    "nodes": [
      {
        "id": "OUT-1",
        "type": "outfall",
        "invertElevation": 100.0
      }
    ],
    "conduits": []
  }
}
```

## Key Data Structures

### Node Types

```json
// Junction/Manhole
{
  "id": "MH-001",
  "type": "junction",
  "invertElevation": 120.5,
  "rimElevation": 125.0,
  "junction": {
    "diameter": 4.0,
    "lossCoefficient": 0.15
  }
}

// Inlet
{
  "id": "IN-001",
  "type": "inlet",
  "invertElevation": 124.5,
  "rimElevation": 128.0,
  "inlet": {
    "inletType": "combination",
    "location": "on-grade",
    "cloggingFactor": 0.15
  }
}

// Outfall
{
  "id": "OUT-001",
  "type": "outfall",
  "invertElevation": 115.0,
  "outfall": {
    "boundaryCondition": "normal-depth"
  }
}
```

### Conduit Types

```json
// Pipe
{
  "id": "P-101",
  "type": "pipe",
  "fromNode": "IN-001",
  "toNode": "MH-001",
  "length": 150,
  "slope": 0.01,
  "pipe": {
    "shape": "circular",
    "diameter": 18,
    "material": "RCP",
    "manningN": 0.013
  }
}

// Gutter
{
  "id": "G-101",
  "type": "gutter",
  "fromNode": "IN-001",
  "toNode": "IN-001",
  "length": 400,
  "slope": 0.015,
  "gutter": {
    "crossSlope": 0.02,
    "longitudinalSlope": 0.015,
    "width": 12,
    "manningN": 0.016
  }
}
```

### Drainage Area

```json
{
  "id": "DA-001",
  "area": 1.25,
  "outlet": "IN-101",
  "runoffCoefficient": 0.82,
  "timeOfConcentration": 12.5
}
```

### Design Storm

```json
{
  "id": "storm-10yr",
  "name": "10-Year Design Storm",
  "returnPeriod": 10,
  "peakIntensity": 3.8
}
```

### Design Criteria

```json
{
  "designCriteria": {
    "gutterSpread": {
      "maxSpread": 10.0
    },
    "hglCriteria": {
      "maxHglBelowRim": 1.0,
      "allowSurcharge": false
    },
    "velocity": {
      "minVelocity": 2.5,
      "maxVelocity": 15.0
    }
  }
}
```

### Analysis Results (Output)

```json
{
  "analysis": {
    "nodeResults": [
      {
        "nodeId": "MH-001",
        "hgl": 120.10,
        "egl": 120.65,
        "flooding": false
      }
    ],
    "violations": [
      {
        "type": "hgl",
        "severity": "error",
        "elementId": "IN-001",
        "message": "HGL exceeds rim elevation",
        "value": 112.45,
        "limit": 112.00
      }
    ]
  }
}
```

## Common Patterns

### Reading a Network (Rust)

```rust
use serde_json;
use std::fs;

let json_str = fs::read_to_string("network.json")?;
let network: DrainageNetwork = serde_json::from_str(&json_str)?;

// Access nodes
for node in &network.network.nodes {
    println!("Node {} at elevation {}", node.id, node.invert_elevation);
}
```

### Writing Analysis Results (Go)

```go
// Read input
data, _ := ioutil.ReadFile("network.json")
var network DrainageNetwork
json.Unmarshal(data, &network)

// Run analysis
results := RunHydraulicAnalysis(&network)

// Add results
network.Analysis = &results

// Write output
output, _ := json.MarshalIndent(network, "", "  ")
ioutil.WriteFile("network-analyzed.json", output, 0644)
```

### Filtering Violations (Python)

```python
import json

with open('network-analyzed.json') as f:
    network = json.load(f)

# Find all HGL violations
hgl_violations = [
    v for v in network['analysis']['violations']
    if v['type'] == 'hgl'
]

# Print errors only
errors = [v for v in hgl_violations if v['severity'] == 'error']
for err in errors:
    print(f"{err['elementId']}: {err['message']}")
```

## Field Name Conventions

- **camelCase**: Used in JSON (e.g., `invertElevation`, `fromNode`)
- **snake_case**: Use in Rust/Python code (e.g., `invert_elevation`, `from_node`)
- **PascalCase**: Use in Go types (e.g., `InvertElevation`, `FromNode`)

## Required vs Optional Fields

### Always Required
- `version` (root)
- `project.name`
- `project.units.system`
- `network` (root)
- `network.nodes` (array, can be empty)
- `network.conduits` (array, can be empty)
- Node: `id`, `type`, `invertElevation`
- Conduit: `id`, `type`, `fromNode`, `toNode`, `length`

### Commonly Optional
- `rainfall` (only if analyzing hydrology)
- `drainageAreas` (only if computing runoff)
- `designCriteria` (only if validating design)
- `analysis` (only after running solver)
- `rimElevation` (but needed for HGL checks)
- Type-specific properties (e.g., `junction`, `inlet`, `pipe`, `gutter`)

## Validation Checklist

Before running analysis, verify:

- [ ] Schema version is specified
- [ ] Units system is defined
- [ ] All node IDs are unique
- [ ] All conduit IDs are unique
- [ ] `fromNode` and `toNode` reference existing node IDs
- [ ] Drainage area `outlet` references existing node ID
- [ ] Pipe slopes are positive or zero
- [ ] Manning's n values are positive
- [ ] Areas and flows are non-negative

## Common Pitfalls

1. **Missing rim elevation**: Required for HGL flooding checks
2. **Mismatched node references**: `fromNode`/`toNode` must exist in nodes array
3. **Unit inconsistency**: Mixing feet and meters without conversion
4. **Negative slopes**: Should be positive (or use inverts to define slope)
5. **Missing type-specific properties**: e.g., defining a "pipe" without `pipe` object
6. **Invalid enum values**: Check allowed values (e.g., `material: "RCP"` not `"Concrete Pipe"`)

## Tool Ecosystem (Planned)

```bash
# Validate schema
hec22 validate network.json

# Run hydraulic analysis
hec22 solve network.json -o results.json

# Generate report
hec22 report results.json -o report.pdf

# Convert from SWMM
hec22 convert --from swmm drainage.inp -o network.json

# Check design criteria
hec22 check results.json --criteria spread,hgl

# Optimize design
hec22 optimize network.json --objective cost
```

## Schema URL

When referencing the schema in JSON files:

```json
{
  "$schema": "https://hec22.dev/schemas/drainage-network.schema.json",
  "version": "1.0.0",
  ...
}
```

(Note: URL is placeholder - update when schema is published)
