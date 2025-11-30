# Tributary Flow Routing

## Overview

This document explains how tributary flows are routed through stormwater drainage networks in HEC-22, ensuring that each conduit carries only its upstream tributary flows, with flows combining only at junction points.

## Flow Routing Algorithm

The flow routing implementation uses **Kahn's topological sorting algorithm** to process nodes in the correct order from upstream to downstream. This ensures that all upstream flows are calculated before being combined at downstream junctions.

### Algorithm Steps

1. **Initialize Node Inflows**: Direct inflows from drainage areas are computed using the rational method (Q = C × i × A)

2. **Topological Sort**: Nodes are sorted from upstream to downstream using Kahn's algorithm:
   - Calculate in-degree for each node (count of upstream conduits)
   - Start with nodes having in-degree = 0 (sources/inlets)
   - Process nodes in order, decrementing downstream in-degrees
   - Add nodes to queue when in-degree becomes 0

3. **Route Flows**: For each node in topological order:
   - Get total flow at the node (direct inflow + upstream contributions)
   - Distribute flow to downstream conduits
   - Add flow to downstream node totals

4. **Flow Accumulation**: At each node, flows accumulate from all upstream conduits:
   ```
   node_total_flow = direct_inflow + sum(upstream_conduit_flows)
   ```

## Tributary Flow Isolation

### Key Principle

**Each conduit carries ONLY the flow from its upstream tributary area.**

Flows from parallel branches remain isolated until they reach a common junction point where they physically converge.

### Example: Simple Branching Network

```
Network topology:
  Branch A: Inlet-A (5 cfs) → Conduit-A ↘
                                          Junction → Trunk → Outfall
  Branch B: Inlet-B (3 cfs) → Conduit-B ↗

Flow distribution:
  - Conduit-A: 5 cfs (only from Branch A)
  - Conduit-B: 3 cfs (only from Branch B)
  - Trunk: 8 cfs (combined at junction: 5 + 3)
  - Outfall: 8 cfs (total system flow)
```

### Example: Multi-Level Branching Network

```
Network topology:
                 Inlet-A1 (2 cfs) ↘
                                    Conduit-A1 → Junction-A (4 cfs) ↘
                 Inlet-A2 (2 cfs) ↗                                  ↘
                                                                       Trunk → Outfall (9 cfs)
                 Inlet-B1 (3 cfs) ↘                                  ↗
                                    Conduit-B1 → Junction-B (5 cfs) ↗
                 Inlet-B2 (2 cfs) ↗

Flow distribution:
  Sub-branch A:
    - Pipe-A1a: 2 cfs (Inlet-A1 tributary only)
    - Pipe-A2a: 2 cfs (Inlet-A2 tributary only)
    - Conduit-A: 4 cfs (combined at Junction-A: 2 + 2)

  Sub-branch B:
    - Pipe-B1b: 3 cfs (Inlet-B1 tributary only)
    - Pipe-B2b: 2 cfs (Inlet-B2 tributary only)
    - Conduit-B: 5 cfs (combined at Junction-B: 3 + 2)

  Main trunk:
    - Trunk: 9 cfs (combined at Main-Junction: 4 + 5)
```

## Implementation Details

### Code Location

The flow routing logic is implemented in `/src/solver.rs`:

- **`route_flows()`** (lines 836-874): Basic flow routing through the network
- **`topological_sort_upstream_to_downstream()`** (lines 741-801): Kahn's algorithm implementation
- **`compute_rational_flows()`** (lines 808-823): Rational method flow calculations

### Key Data Structures

```rust
// Node inflows from drainage areas
HashMap<String, f64> node_inflows;

// Conduit flows (result of routing)
HashMap<String, f64> conduit_flows;

// Total accumulated flow at each node
HashMap<String, f64> node_total_flows;
```

### Flow Routing Process

```rust
// Initialize with direct inflows
for (node_id, &flow) in node_inflows {
    node_total_flows.insert(node_id.clone(), flow);
}

// Process nodes in topological order
for node_id in sorted_nodes {
    // Get total flow at this node
    let node_flow = node_total_flows.get(&node_id).cloned().unwrap_or(0.0);

    // Route to downstream conduits
    let downstream_conduits = network.downstream_conduits(&node_id);
    let flow_per_conduit = node_flow / downstream_conduits.len() as f64;

    for conduit in downstream_conduits {
        // Assign flow to conduit
        conduit_flows.insert(conduit.id.clone(), flow_per_conduit);

        // Add to downstream node's total
        let downstream_flow = node_total_flows
            .entry(conduit.to_node.clone())
            .or_insert(0.0);
        *downstream_flow += flow_per_conduit;
    }
}
```

## Testing

### Test Coverage

The tributary flow isolation behavior is verified by comprehensive tests in `/tests/`:

1. **`tributary_flow_test.rs`**: Tests simple parallel branches
   - Two branches (5 cfs and 3 cfs) converging at a junction
   - Verifies each branch carries only its tributary flow
   - Verifies combined flow at junction (8 cfs)

2. **`multi_level_tributary_flow_test.rs`**: Tests multi-level branching
   - Four inlets in two sub-branches
   - Verifies flow accumulation at multiple levels
   - Tests complex topological sorting scenarios

3. **`network_integration_test.rs`**: Integration tests
   - `test_simple_linear_network`: Linear network with series inlets
   - `test_branching_network`: Y-shaped branching network
   - `test_unbalanced_branching_network`: Asymmetric branch lengths

### Running Tests

```bash
# Run all tributary flow tests
cargo test tributary_flow

# Run specific test with output
cargo test test_tributary_flow_isolation_in_parallel_branches -- --nocapture

# Run multi-level test
cargo test test_multi_level_tributary_flow_isolation -- --nocapture
```

## Hydraulic Calculations

After flow routing is complete, hydraulic calculations (HGL/EGL solver) use the routed flows:

1. **Manning's Equation**: Calculate pipe capacity and normal depth
2. **Energy Loss Calculations**:
   - Friction loss along conduits
   - Entrance/exit losses
   - Bend losses
   - Junction losses (FHWA Access Hole Method)
3. **HGL/EGL Propagation**: Calculate water surface elevations upstream from outfall

The junction loss calculations correctly account for multiple inflow pipes with different flows using the FHWA Access Hole Method (HEC-22 Equations 9.11-9.31).

## Design Implications

### Proper Pipe Sizing

Each pipe must be sized for its tributary flow only:

- **Upstream pipes**: Size for local tributary area flows
- **Downstream pipes**: Size for accumulated flows from all upstream tributaries
- **Trunk pipes**: Size for total system flow

### Flow Concentration

Flows concentrate at junctions where multiple branches converge. The hydraulic grade line will reflect this increased flow in downstream conduits.

### Time of Concentration

The rational method assumes all tributary flows arrive simultaneously (peak flow condition). The time of concentration should reflect the longest flow path in the tributary area.

## Common Pitfalls (Avoided by This Implementation)

❌ **Wrong**: Adding parallel branch flows to each conduit
- Would incorrectly assign 8 cfs to both Conduit-A and Conduit-B

✅ **Correct**: Each conduit carries only its upstream tributary flow
- Conduit-A: 5 cfs, Conduit-B: 3 cfs, Trunk: 8 cfs

❌ **Wrong**: Processing nodes before all upstream flows are known
- Could result in incorrect flow accumulation

✅ **Correct**: Topological sorting ensures upstream flows are calculated first
- Kahn's algorithm processes nodes in the correct order

## References

- HEC-22: Urban Drainage Design Manual (FHWA, 2013)
- Chapter 4: Rational Method for Peak Flow Estimation
- Chapter 9: Storm Drain System Hydraulics
- Kahn's Algorithm: L. Kahn (1962), "Topological sorting of large networks"

## Related Documentation

- [Hydraulic Calculations](hydraulic_calculations.md)
- [Junction Loss Methods](junction_loss.md)
- [Network Topology](network_topology.md)
