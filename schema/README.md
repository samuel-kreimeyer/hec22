# HEC-22 Drainage Network JSON Schema

## Overview

This directory contains the JSON schema definition for the HEC-22 drainage network model. The schema is designed to be:

- **Language-agnostic**: Works equally well with Rust, Go, Python, or any language with JSON support
- **Validation-ready**: Supports JSON Schema Draft 2020-12 for automated validation
- **Extensible**: Easy to add new features without breaking existing implementations
- **Standards-compliant**: Follows FHWA HEC-22 methodology and terminology

## Files

- `drainage-network.schema.json` - Main schema definition
- `examples/simple-network.json` - Example of a passing design
- `examples/network-with-violations.json` - Example showing design violations
- `README.md` - This file

## Design Philosophy

### 1. Separation of Input and Output

The schema separates **design input** from **analysis results**:

- **Input sections** (required): `project`, `network`, `drainageAreas`, `rainfall`, `designCriteria`
- **Output section** (optional): `analysis` - populated by solver software

This allows the same file to serve as:
- Input to analysis tools (without `analysis` section)
- Complete record of analysis (with `analysis` section populated)

### 2. Graph-Based Network Model

The drainage network uses a **node-edge graph** representation:

**Nodes** represent:
- Inlets (collection points)
- Junctions/Manholes (connection points)
- Outfalls (discharge points)

**Conduits** represent:
- Pipes (circular, rectangular, etc.)
- Gutters (surface flow)
- Open channels (natural or constructed)

This matches the mental model used in Civil3D, StormCAD, and other hydraulic modeling software.

### 3. Type-Specific Properties

Each node type and conduit type has its own property object:

```json
{
  "type": "inlet",
  "inlet": {
    "inletType": "combination",
    "grate": { ... },
    "curbOpening": { ... }
  }
}
```

This approach:
- Keeps the schema clean and organized
- Makes validation straightforward
- Allows type-specific properties without conflicts
- Simplifies code generation for strongly-typed languages (Rust, Go)

### 4. Flexible Units System

The schema requires explicit unit specification in `project.units`:

```json
{
  "units": {
    "system": "US",
    "length": "ft",
    "elevation": "ft",
    "flow": "cfs",
    "area": "acres"
  }
}
```

This enables:
- International usage (US customary and SI)
- Mixed-unit workflows (e.g., elevation in feet, pipe diameter in inches)
- Clear documentation of what units are expected

### 5. Design Criteria as Data

Design constraints are stored as data, not hardcoded:

```json
{
  "designCriteria": {
    "gutterSpread": {
      "maxSpread": 10.0
    },
    "hglCriteria": {
      "maxHglBelowRim": 1.0
    }
  }
}
```

Benefits:
- Different criteria for different jurisdictions
- Easy to run "what-if" scenarios
- Criteria violations can be automatically detected

### 6. Rich Violation Reporting

The `analysis.violations` array provides structured error reporting:

```json
{
  "violations": [
    {
      "type": "hgl",
      "severity": "error",
      "elementId": "MH-001",
      "message": "HGL at 110.85 ft is 0.65 ft above rim elevation",
      "value": 110.85,
      "limit": 109.20
    }
  ]
}
```

This enables:
- Automated design validation
- Clear reporting for engineers
- Programmatic filtering by severity or type
- Traceable violations to specific elements

## Use Cases

### Use Case 1: Manual Entry and Validation

**Scenario**: "I have a spreadsheet, validate the model meets gutter spread limits and no junctions have an HGL above the structure"

**Workflow**:

1. **Convert spreadsheet to JSON**:
   ```bash
   # Future utility
   hec22-convert --from csv --to json drainage_design.csv > network.json
   ```

2. **Validate schema compliance**:
   ```bash
   # Any JSON schema validator
   jsonschema -i network.json drainage-network.schema.json
   ```

3. **Run hydraulic analysis**:
   ```bash
   # Solver populates the analysis section
   hec22-solve network.json > network-analyzed.json
   ```

4. **Check for violations**:
   ```bash
   # Extract violations
   jq '.analysis.violations[] | select(.type == "hgl" or .type == "spread")' network-analyzed.json
   ```

### Use Case 2: Automated Design

**Scenario**: Future goal to support automated drainage design

**Workflow**:

1. **Define constraints** (in `designCriteria`)
2. **Run optimization**:
   ```bash
   hec22-optimize --objective minimize-cost network.json > optimized.json
   ```
3. **Solver iterates**:
   - Sizes pipes to meet HGL criteria
   - Adjusts inlet spacing to meet spread limits
   - Minimizes total pipe length/cost

### Use Case 3: File Format Conversion

**Scenario**: Convert between common drainage file formats

**Examples**:
```bash
# SWMM .inp to JSON
hec22-convert --from swmm drainage.inp > network.json

# JSON to Civil3D pipe network XML
hec22-convert --from json --to civil3d network.json > civil3d-import.xml

# JSON to HydroCAD format
hec22-convert --from json --to hydrocad network.json > project.hcd
```

## Language-Specific Implementation Notes

### Rust

**Recommended approach**: Use `serde` for JSON serialization/deserialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DrainageNetwork {
    version: String,
    project: Project,
    network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    rainfall: Option<Rainfall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drainage_areas: Option<Vec<DrainageArea>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    design_criteria: Option<DesignCriteria>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis: Option<Analysis>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Node {
    Junction {
        id: String,
        invert_elevation: f64,
        #[serde(flatten)]
        junction: JunctionProperties,
    },
    Inlet {
        id: String,
        invert_elevation: f64,
        #[serde(flatten)]
        inlet: InletProperties,
    },
    Outfall {
        id: String,
        invert_elevation: f64,
        #[serde(flatten)]
        outfall: OutfallProperties,
    },
}
```

**Benefits**:
- Compile-time type safety
- Zero-cost abstractions
- Excellent performance for large networks

### Go

**Recommended approach**: Use `encoding/json` with struct tags

```go
package hec22

type DrainageNetwork struct {
    Version        string          `json:"version"`
    Project        Project         `json:"project"`
    Network        Network         `json:"network"`
    Rainfall       *Rainfall       `json:"rainfall,omitempty"`
    DrainageAreas  []DrainageArea  `json:"drainageAreas,omitempty"`
    DesignCriteria *DesignCriteria `json:"designCriteria,omitempty"`
    Analysis       *Analysis       `json:"analysis,omitempty"`
}

type Node struct {
    ID              string   `json:"id"`
    Type            NodeType `json:"type"`
    InvertElevation float64  `json:"invertElevation"`
    RimElevation    float64  `json:"rimElevation,omitempty"`

    // Type-specific properties (only one should be populated)
    Junction *JunctionProperties `json:"junction,omitempty"`
    Inlet    *InletProperties    `json:"inlet,omitempty"`
    Outfall  *OutfallProperties  `json:"outfall,omitempty"`
}
```

**Benefits**:
- Simple, idiomatic Go
- Good performance
- Built-in JSON support

### Python

**Recommended approach**: Use Pydantic for validation

```python
from pydantic import BaseModel, Field
from typing import Optional, List, Literal
from enum import Enum

class NodeType(str, Enum):
    JUNCTION = "junction"
    INLET = "inlet"
    OUTFALL = "outfall"

class Node(BaseModel):
    id: str
    type: NodeType
    invert_elevation: float = Field(alias="invertElevation")
    rim_elevation: Optional[float] = Field(None, alias="rimElevation")

    junction: Optional[JunctionProperties] = None
    inlet: Optional[InletProperties] = None
    outfall: Optional[OutfallProperties] = None

class DrainageNetwork(BaseModel):
    version: str
    project: Project
    network: Network
    rainfall: Optional[Rainfall] = None
    drainage_areas: Optional[List[DrainageArea]] = Field(None, alias="drainageAreas")
    design_criteria: Optional[DesignCriteria] = Field(None, alias="designCriteria")
    analysis: Optional[Analysis] = None
```

**Benefits**:
- Automatic validation
- IDE autocomplete support
- Easy to generate from schema

## Validation

### JSON Schema Validation

Validate files against the schema using any JSON Schema validator:

**Python**:
```python
import jsonschema
import json

with open('drainage-network.schema.json') as f:
    schema = json.load(f)

with open('network.json') as f:
    data = json.load(f)

jsonschema.validate(instance=data, schema=schema)
```

**Node.js**:
```javascript
const Ajv = require('ajv');
const ajv = new Ajv();

const schema = require('./drainage-network.schema.json');
const data = require('./network.json');

const validate = ajv.compile(schema);
const valid = validate(data);

if (!valid) console.log(validate.errors);
```

**Rust**:
```rust
use jsonschema::JSONSchema;
use serde_json;

let schema = serde_json::from_str(include_str!("drainage-network.schema.json"))?;
let instance = serde_json::from_str(&json_data)?;

let compiled = JSONSchema::compile(&schema)?;
let result = compiled.validate(&instance);
```

## Schema Versioning

The schema follows semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes (e.g., removing required fields, changing field types)
- **MINOR**: Backward-compatible additions (e.g., new optional fields)
- **PATCH**: Clarifications and bug fixes to schema documentation

Current version: **1.0.0**

### Upgrade Path

When the schema version changes:

1. **Patch updates**: No action required
2. **Minor updates**: New features available, existing files still valid
3. **Major updates**: Migration utility will be provided

## Extending the Schema

### Adding New Node Types

To add a new node type (e.g., "pump-station"):

1. Add to `node.type` enum:
   ```json
   "enum": ["junction", "inlet", "outfall", "pump-station"]
   ```

2. Add type-specific properties:
   ```json
   "pumpStation": {
     "type": "object",
     "properties": {
       "pumpCurve": { "type": "string" },
       "wetWellVolume": { "type": "number" }
     }
   }
   ```

3. Update examples and documentation

### Adding New Analysis Results

To add new computed values:

1. Add to `analysis.nodeResults` or `analysis.conduitResults`:
   ```json
   "shearStress": {
     "type": "number",
     "description": "Bed shear stress (lb/sqft)"
   }
   ```

2. Update solver to compute new values
3. Update documentation

### Adding Custom Fields

For project-specific extensions, use a `custom` object:

```json
{
  "id": "MH-001",
  "type": "junction",
  "custom": {
    "assetId": "CITY-MH-2024-001",
    "lastInspection": "2024-11-01",
    "condition": "good"
  }
}
```

The schema allows `additionalProperties` in specific locations for this purpose.

## File Conversion Utilities (Future)

Planned conversion utilities:

### Input Formats
- **CSV**: Tabular node/conduit tables
- **SWMM .inp**: EPA Storm Water Management Model
- **Civil3D**: Autodesk Civil3D pipe network XML
- **HydroCAD**: HydroCAD project files
- **Excel**: Spreadsheet templates

### Output Formats
- **JSON**: This schema
- **GeoJSON**: For GIS integration
- **IFC**: Industry Foundation Classes (future)
- **PDF**: Analysis reports
- **CSV**: Results export

## Testing and Validation

### Test Suite

Create test files for:

1. **Minimal valid network**: Simplest possible valid file
2. **Complete network**: All optional fields populated
3. **Violations**: Known design violations
4. **Edge cases**: Boundary conditions (zero slope, very large areas, etc.)

### Continuous Validation

For automated testing:

```bash
# Validate all examples
for file in examples/*.json; do
  echo "Validating $file..."
  jsonschema -i "$file" drainage-network.schema.json
done
```

## Performance Considerations

### Large Networks

For networks with 1000+ nodes:

1. **Streaming**: Process node/conduit arrays in chunks
2. **Indexing**: Build lookup tables by ID for O(1) access
3. **Lazy loading**: Only parse `analysis` section when needed
4. **Compression**: Use gzip for storage (JSON compresses well)

### Memory-Efficient Approaches

**Rust**:
- Use `serde_json::from_reader()` for streaming
- Use `Arc<T>` for shared data structures

**Go**:
- Use `json.Decoder` for streaming
- Use sync.Map for concurrent access

**Python**:
- Use `ijson` for iterative parsing
- Use generators for large result sets

## License and Attribution

This schema is designed for the HEC-22 Urban Drainage Analysis System, following FHWA HEC-22 (4th Edition, 2024) methodology.

## Questions and Feedback

For questions or suggestions about the schema:
- Open an issue in the repository
- Review the HEC-22 documentation in `/reference`
- Check worked examples in `/reference/examples`

## References

- FHWA HEC-22: Urban Drainage Design Manual (4th Edition, 2024)
- JSON Schema Specification: https://json-schema.org/
- GeoJSON Specification: https://geojson.org/
- EPA SWMM Documentation: https://www.epa.gov/water-research/storm-water-management-model-swmm
