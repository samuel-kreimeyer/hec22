🌧️ Proposed Software Architecture for HEC-22 Framework

1. Core Domain Model

The foundation should be a network graph:

DrainageNetwork
├── Nodes (structures)
│   ├── Junction
│   ├── Inlet
│   ├── Outfall
│   └── Special structures (future: pump station, detention basin, drop structure)
│
└── Edges (conduits)
    ├── Pipe
    ├── Gutter (roadway channel)
    ├── OpenChannel (ditch)
    └── Culvert (future)

Each class should define only physical, geometric, and hydraulic parameters. Computation will be done by service modules that operate ON the network.

Example (Python-like):

class Node:
    id: str
    elevation: float
    x: float
    y: float

class Inlet(Node):
    inlet_type: str  # e.g., "curb_and_gutter", "grate"
    flow_capture_params: dict

class Conduit:
    id: str
    from_node: str
    to_node: str
    geometry: dict  # slope, length, shape, diameter, roughness
    surface_type: Optional[str]  # gutter type, Manning n of gutter


⸻

2. Input Abstraction Layer

You’re right to support multiple sources—but architecture-wise, treat them as adapters that generate the same internal network representation.

Input Adapters
├── Tabular (CSV / Excel)  ← *first*
├── SWMM .inp
├── IFC (future)
└── API or JSON (future)

Priority 1: Tabular input → internal graph.
Everything else plugs into same API later.

❗Recommendation: Define a data contract (network_schema.json or equivalent) that specify expected input columns before writing code.

⸻

3. Hydrology Input

Since hydrology varies, don’t embed storm design inside the core. Instead:

HydroProvider
├── Atlas14Table
├── ConstantFlow (manual)
├── TimeSeries / Hydrograph (future)

Then each Node or Subcatchment simply receives “design discharge” from HydroProvider.

⚠️ SWMM compatibility becomes easy later because SWMM .inp already defines contributing area/hydro.

⸻

4. Compute Architecture (Strategy Pattern)

Computation Engine
├── Network Build
├── Flow Routing (steady state, rational method)
├── Inlet Capture (HEC-22 Chapter 4/5)
├── Gutter Spread Calculations
└── Hydraulic Grade Line (energy grade check – Chapter 8)

Each module independent, in execution order.

Allow different solvers in the future (steady state vs dynamic, normally just rational method).

⸻

5. Output Layer

Similar to input:

Output Adapters
├── Tabular CSV / Excel  ← *first*
├── PDF Report (future)
├── JSON/GeoJSON
├── IFC Export (future)
└── SWMM .inp (back-export)

Priority: Tabular report mapping to each edge and node, e.g.:

Component	Q (cfs)	Velocity (fps)	HGL Elev.	Spread (ft)	Surcharge?



⸻

6. Integration Strategy & MVP Scope

Feature	MVP	Phase 2	Phase 3
Tabular CSV input/output	✔️		
Node/Edge model	✔️		
Flow routing (rational)	✔️		
Gutter flow/spread	✔️		
HGL/EGL check	✔️		
SWMM .inp import/export		✔️	
IFC support			✔️
Pump stations			✔️


⸻

7. Implementation-Ready Object Model Outline

class DrainageNetwork:
    nodes: Dict[str, Node]
    conduits: Dict[str, Conduit]
    def validate(self): ...
    def connect(self): ...

class FlowCalculator:
    def compute_flows(network, hydrology): ...

class HydraulicCalculator:
    def compute_hgl(network): ...


⸻

8. Why this works

🚗 Mimics Civil3D/StormCAD mental model (nodes & pipes)
📦 Modular — easy to integrate IFC, SWMM, dynamic solvers later
🧮 Focus on equilibrium-based design first, not full simulation
📑 Supports clear, engineer-friendly tabular output first
🔌 Aligns well with domain-driven design and adapter pattern

⸻

🔥 Next Steps

I recommend this implementation roadmap:

Step 1 — Define JSON/CSV schema for tabular input

🔹 I can draft this file next (network_schema.json) and a matching example CSV.

Step 2 — Write class stubs per above (no logic yet)

🔹 I can write a PR draft with these classes.

Step 3 — Define expected output format and unit test placeholders

🔹 Prepare test harness BEFORE writing logic.
