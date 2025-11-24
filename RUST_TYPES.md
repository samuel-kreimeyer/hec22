# Rust Type Definitions for HEC-22 Drainage Networks

This document describes the Rust implementation of the HEC-22 drainage network data model.

## Overview

The Rust types provide a strongly-typed, compile-time validated representation of drainage networks that corresponds directly to the JSON schema. All types implement `Serialize` and `Deserialize` from `serde`, enabling seamless conversion between Rust structs and JSON.

## Module Organization

```
hec22/
├── src/
│   ├── lib.rs              # Root module and DrainageNetwork type
│   ├── project.rs          # Project metadata and units
│   ├── node.rs             # Node types (Junction, Inlet, Outfall)
│   ├── conduit.rs          # Conduit types (Pipe, Gutter, Channel)
│   ├── network.rs          # Network topology
│   ├── drainage.rs         # Drainage areas and subcatchments
│   ├── rainfall.rs         # Rainfall events and IDF curves
│   └── analysis.rs         # Analysis results and violations
├── examples/
│   ├── build_network.rs    # Programmatic network construction
│   └── load_json.rs        # Load and analyze JSON files
└── tests/
    └── json_schema_tests.rs # Integration tests
```

## Quick Start

### Adding to Your Project

Add to `Cargo.toml`:

```toml
[dependencies]
hec22 = { path = "../hec22" }
```

### Loading a Network from JSON

```rust
use hec22::DrainageNetwork;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from file
    let json = fs::read_to_string("network.json")?;
    let network: DrainageNetwork = serde_json::from_str(&json)?;

    // Access network components
    println!("Project: {}", network.project.name);
    println!("Nodes: {}", network.network.nodes.len());
    println!("Conduits: {}", network.network.conduits.len());

    Ok(())
}
```

### Building a Network Programmatically

```rust
use hec22::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create project
    let project = project::Project {
        name: "My Project".to_string(),
        units: project::Units::us_customary(),
        // ... other fields
    };

    // Create nodes
    let outfall = node::Node::new_outfall(
        "OUT-1".to_string(),
        100.0,
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::Free,
            tailwater_elevation: None,
            tidal_curve: None,
        },
    );

    // Create conduits
    let pipe = conduit::Conduit::new_pipe(
        "P-1".to_string(),
        "N1".to_string(),
        "OUT-1".to_string(),
        150.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0),
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            // ... other fields
        },
    );

    // Build network
    let mut network = network::Network::new();
    network.add_node(outfall);
    network.add_conduit(pipe);

    // Create complete drainage network
    let drainage_network = DrainageNetwork::new(project, network);

    // Serialize to JSON
    let json = drainage_network.to_json()?;
    println!("{}", json);

    Ok(())
}
```

## Core Types

### DrainageNetwork

Root-level type containing all components:

```rust
pub struct DrainageNetwork {
    pub version: String,
    pub project: Project,
    pub network: Network,
    pub rainfall: Option<Rainfall>,
    pub drainage_areas: Option<Vec<DrainageArea>>,
    pub design_criteria: Option<DesignCriteria>,
    pub analysis: Option<Analysis>,
}
```

Key methods:
- `new(project, network)` - Create new network
- `from_json(json)` - Parse from JSON string
- `to_json()` - Serialize to JSON string
- `find_node(id)` - Find node by ID
- `find_conduit(id)` - Find conduit by ID
- `upstream_conduits(node_id)` - Get upstream conduits
- `downstream_conduits(node_id)` - Get downstream conduits

### Node Types

Three node variants with type-specific properties:

```rust
// Junction/Manhole
let junction = Node::new_junction(
    "MH-001".to_string(),
    invert_elevation,
    rim_elevation,
    JunctionProperties {
        diameter: Some(4.0),
        loss_coefficient: Some(0.15),
        benching: Some(true),
        // ...
    },
);

// Inlet
let inlet = Node::new_inlet(
    "IN-001".to_string(),
    invert_elevation,
    rim_elevation,
    InletProperties {
        inlet_type: InletType::Combination,
        location: InletLocation::OnGrade,
        // ...
    },
);

// Outfall
let outfall = Node::new_outfall(
    "OUT-001".to_string(),
    invert_elevation,
    OutfallProperties {
        boundary_condition: BoundaryCondition::NormalDepth,
        // ...
    },
);
```

### Conduit Types

Three conduit variants:

```rust
// Pipe
let pipe = Conduit::new_pipe(
    id,
    from_node,
    to_node,
    length,
    PipeProperties {
        shape: PipeShape::Circular,
        diameter: Some(18.0),
        material: Some(PipeMaterial::RCP),
        manning_n: 0.013,
        // ...
    },
);

// Gutter
let gutter = Conduit::new_gutter(
    id,
    from_node,
    to_node,
    length,
    GutterProperties {
        cross_slope: 0.02,
        longitudinal_slope: 0.015,
        manning_n: 0.016,
        // ...
    },
);

// Channel
let channel = Conduit::new_channel(
    id,
    from_node,
    to_node,
    length,
    ChannelProperties {
        shape: ChannelShape::Trapezoidal,
        bottom_width: Some(5.0),
        side_slope: Some(2.0),
        manning_n: 0.035,
    },
);
```

### Drainage Areas

```rust
let drainage_area = DrainageArea {
    id: "DA-001".to_string(),
    area: 1.25,
    outlet: "IN-001".to_string(),
    runoff_coefficient: Some(0.82),
    time_of_concentration: Some(12.5),
    // ...
};

// Calculate runoff using Rational Method
let flow = drainage_area.rational_method_runoff(intensity);
```

### Rainfall

```rust
// Design storm
let storm = DesignStorm::uniform(
    "storm-10yr".to_string(),
    "10-Year Storm".to_string(),
    10.0,     // return period
    3.8,      // intensity
);

// IDF curve
let idf = IdfCurve {
    return_period: 10.0,
    points: vec![
        IdfPoint { duration: 5.0, intensity: 6.5 },
        IdfPoint { duration: 10.0, intensity: 5.2 },
        // ...
    ],
};

// Interpolate intensity
let intensity = idf.get_intensity(7.5)?;
```

### Analysis Results

```rust
// Create analysis
let mut analysis = Analysis::new(
    AnalysisMethod::Rational,
    "storm-10yr".to_string(),
);

// Add violation
let violation = Violation::hgl_violation(
    "MH-001".to_string(),
    125.5,  // actual HGL
    125.0,  // rim elevation
    Severity::Error,
);
analysis.add_violation(violation);

// Query violations
let errors = analysis.get_errors();
let hgl_violations = analysis.get_violations_by_type(ViolationType::Hgl);
```

## Field Naming Conventions

### JSON ↔ Rust Mapping

JSON uses `camelCase`, Rust uses `snake_case`:

| JSON               | Rust               |
|--------------------|--------------------|
| `invertElevation`  | `invert_elevation` |
| `fromNode`         | `from_node`        |
| `manningN`         | `manning_n`        |
| `runoffCoefficient`| `runoff_coefficient`|

Serde handles this automatically with `#[serde(rename = "...")]` attributes.

### Enums

JSON uses specific formats, Rust enums match:

```rust
// JSON: "type": "junction"
NodeType::Junction

// JSON: "inletType": "combination"
InletType::Combination

// JSON: "material": "RCP"
PipeMaterial::RCP

// JSON: "flowRegime": "subcritical"
FlowRegime::Subcritical
```

## Type Safety Features

### Compile-Time Validation

```rust
// Won't compile - missing required fields
let project = Project {
    name: "Test".to_string(),
    // ERROR: missing field `units`
};

// Type system prevents invalid values
let boundary = BoundaryCondition::Free; // OK
let boundary = "free";                  // ERROR: type mismatch
```

### Optional Fields

```rust
// Optional fields use Option<T>
pub struct Node {
    pub rim_elevation: Option<f64>,  // May be None
    pub coordinates: Option<Coordinates>,
    // ...
}

// Access with pattern matching
if let Some(rim) = node.rim_elevation {
    println!("Rim: {}", rim);
}

// Or with unwrap_or
let rim = node.rim_elevation.unwrap_or(0.0);
```

### Validation Methods

```rust
// Network connectivity validation
network.validate_connectivity()?;

// Type checks
if node.is_inlet() {
    // Handle inlet-specific logic
}

if conduit.is_pipe() {
    // Access pipe properties
    if let Some(props) = conduit.pipe {
        println!("Diameter: {:?}", props.diameter);
    }
}
```

## Utility Methods

### Network Queries

```rust
// Find elements by ID
let node = network.find_node("MH-001")?;
let conduit = network.find_conduit("P-101")?;

// Get nodes by type
let inlets = network.nodes_by_type(NodeType::Inlet);
let junctions = network.network.junctions();
let outfalls = network.network.outfalls();

// Traverse network
let upstream = network.upstream_conduits("MH-001");
let downstream = network.downstream_conduits("IN-001");
```

### Calculations

```rust
// Conduit slope
let slope = conduit.calculate_slope();
let effective = conduit.effective_slope();

// Drainage area runoff
let q = drainage_area.rational_method_runoff(intensity)?;

// Time of concentration
let tc = drainage_area.calculate_total_tc()?;

// IDF interpolation
let intensity = idf_curve.get_intensity(duration)?;

// Typical Manning's n
let n = PipeMaterial::RCP.typical_manning_n(); // 0.013
```

## Running Examples

```bash
# Build network programmatically
cargo run --example build_network

# Load and analyze JSON
cargo run --example load_json

# Run tests
cargo test

# Run specific test
cargo test test_load_simple_network

# Run with output
cargo test -- --nocapture
```

## Integration Testing

The project includes comprehensive tests that validate:

- ✓ JSON schema examples load correctly
- ✓ Roundtrip serialization (Rust → JSON → Rust)
- ✓ Network connectivity validation
- ✓ Node and conduit property access
- ✓ Drainage area calculations
- ✓ IDF curve interpolation
- ✓ Violation filtering and querying
- ✓ Upstream/downstream traversal

Run all tests:

```bash
cargo test
```

## Performance Considerations

### Large Networks

For networks with 1000+ nodes:

```rust
use std::collections::HashMap;

// Build index for O(1) lookups
let node_map: HashMap<_, _> = network.network.nodes
    .iter()
    .map(|n| (n.id.as_str(), n))
    .collect();

let node = node_map.get("MH-001");
```

### Streaming JSON

For very large files:

```rust
use serde_json::Deserializer;

let file = File::open("large-network.json")?;
let reader = BufReader::new(file);
let network: DrainageNetwork = serde_json::from_reader(reader)?;
```

## Error Handling

All I/O and parsing operations return `Result` types:

```rust
use std::error::Error;

fn load_network(path: &str) -> Result<DrainageNetwork, Box<dyn Error>> {
    let json = std::fs::read_to_string(path)?;
    let network = serde_json::from_str(&json)?;
    network.network.validate_connectivity()?;
    Ok(network)
}
```

## Future Enhancements

Planned additions to the Rust types:

- [ ] Builder pattern for complex types
- [ ] Validation traits for design criteria checking
- [ ] Graph algorithms (shortest path, flow routing)
- [ ] Hydraulic calculation methods (Manning's equation, etc.)
- [ ] GeoJSON geometry support
- [ ] SWMM .inp parser
- [ ] Profile plot generation

## Contributing

When adding new types:

1. Add type definition in appropriate module
2. Implement `Serialize` and `Deserialize` (use `#[derive]`)
3. Use `#[serde(rename = "...")]` for JSON field names
4. Mark optional fields with `Option<T>`
5. Add helper methods for common operations
6. Write tests verifying JSON roundtrip
7. Update this documentation

## Resources

- [Serde documentation](https://serde.rs/)
- [JSON Schema specification](https://json-schema.org/)
- [HEC-22 Schema README](schema/README.md)
- [HEC-22 Quick Reference](schema/QUICK_REFERENCE.md)
