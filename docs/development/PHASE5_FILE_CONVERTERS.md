# Phase 5: File Format Converters - Requirements & Planning

**Status**: Planning Phase
**Target**: Interoperability with existing drainage analysis tools
**Document Version**: 1.0
**Last Updated**: 2026-01-01

## Overview

Phase 5 focuses on building file format converters to enable hec22 to import/export data from other drainage analysis programs. This addresses a critical need: engineers have existing projects in SWMM, Civil3D, HydroCAD, and other tools, and need a migration path.

## Objectives

### Import Formats (Priority Order)
1. **SWMM .inp** - EPA Storm Water Management Model files (HIGH PRIORITY)
2. **Civil3D XML** - Autodesk Civil3D pipe network export (MEDIUM)
3. **HydroCAD** - HydroCAD project files (MEDIUM)
4. **Excel templates** - Pre-formatted spreadsheet layouts (LOW - CSV already supported)
5. **GIS shapefiles** - Import network geometry from ArcGIS/QGIS (LOW)

### Export Formats (Priority Order)
1. **GeoJSON** - For web mapping and GIS integration (HIGH PRIORITY)
2. **SWMM .inp** - Export to EPA SWMM for dynamic modeling (MEDIUM)
3. **PDF reports** - Professional calculation packages (LOW - separate feature)
4. **DXF/DWG** - CAD drawing exchange (LOW)

## Use Cases

**Primary Use Case**: "Convert my 20 SWMM models to hec22 format"
**Secondary Use Case**: "Export results to GIS for visualization"
**Tertiary Use Case**: "Round-trip between hec22 and SWMM for specialized analysis"

## Current State Analysis

### Internal Data Model

hec22's internal representation is well-structured for conversion:

**Core Types**:
- `Network`: Collection of nodes and conduits
  - `Node`: Inlet, Junction, or Outfall with type-specific properties
  - `Conduit`: Pipe, Gutter, or Channel with geometry and hydraulics
- `DrainageArea`: Subcatchments with runoff parameters
- `Project`: Metadata, units, location
- `Rainfall`: IDF curves and design storms

**Serialization**:
- Full Serde support with JSON schema
- CSV import already implemented in `src/csv.rs`
- JSON is the canonical interchange format

### Existing I/O Capabilities

**Current Input**: CSV files for nodes, conduits, drainage areas, IDF curves
**Current Output**: Text reports, JSON, CSV, SVG visualizations, HTML viewers
**Template**: `src/csv.rs` provides a good example of parser structure

## Architecture Design

### Module Structure

```
src/
  converters/
    mod.rs              # Public API and converter trait
    swmm/
      mod.rs            # SWMM module
      import.rs         # SWMM → hec22
      export.rs         # hec22 → SWMM
      sections.rs       # SWMM section parsers
    civil3d/
      mod.rs            # Civil3D module
      import.rs         # Civil3D XML → hec22
    geojson/
      mod.rs            # GeoJSON module
      export.rs         # hec22 → GeoJSON
    common.rs           # Shared utilities (unit conversion, validation)
    error.rs            # Conversion error types
```

### Converter Trait Design

```rust
/// Trait for file format converters
pub trait Converter {
    /// Check if this converter can handle the file
    fn can_import(&self, path: &Path) -> bool;

    /// Import from external format to hec22 Network
    fn import(&self, path: &Path) -> Result<ConversionContext, ConversionError>;

    /// Export from hec22 Network to external format
    fn export(&self, context: &ConversionContext, path: &Path) -> Result<(), ConversionError>;

    /// Get converter metadata
    fn metadata(&self) -> ConverterMetadata;
}

/// Conversion context with warnings and metadata
pub struct ConversionContext {
    pub network: Network,
    pub project: Project,
    pub drainage_areas: Option<Vec<DrainageArea>>,
    pub rainfall: Option<Rainfall>,
    pub warnings: Vec<ConversionWarning>,
    pub metadata: HashMap<String, String>,
}

/// Warning about data loss or assumptions during conversion
pub struct ConversionWarning {
    pub severity: WarningSeverity,
    pub category: WarningCategory,
    pub message: String,
    pub element_id: Option<String>,
    pub suggestion: Option<String>,
}

pub enum WarningSeverity {
    Info,      // Informational (e.g., "Used default Manning's n")
    Warning,   // Data transformation (e.g., "Rounded diameter to nearest inch")
    Critical,  // Possible data loss (e.g., "Feature not supported, omitted")
}

pub enum WarningCategory {
    MissingData,
    UnitConversion,
    UnsupportedFeature,
    Assumption,
    DataLoss,
}
```

### CLI Integration

```bash
# Convert SWMM to hec22 JSON
hec22 convert --from swmm --to json drainage.inp network.json

# Convert hec22 to GeoJSON
hec22 convert --from json --to geojson network.json network.geojson

# Convert with verbose warnings
hec22 convert --from swmm --to json --verbose drainage.inp network.json

# List supported formats
hec22 convert --list-formats
```

## Key Design Decisions

### 1. Handling Lossy Conversions

**Decision**: Warn the user and degrade gracefully with opportunities to insert default values or enter missing data.

**Implementation Strategy**:
- Collect warnings during parsing
- Report warnings at end with severity levels
- Provide suggestions for missing data
- Allow interactive mode for critical missing values
- Generate template sections for unsupported features

**Example**:
```
WARNING: SWMM inlet 'IN-001' has no HEC-22 inlet type specified
  Suggestion: Add inlet_type to hec22 nodes.csv
  Default: Assumed 'combination' inlet type based on geometry

CRITICAL: HydroCAD pond routing cannot be directly converted
  Suggestion: Review detention basin parameters manually
  Action: Created placeholder detention node 'POND-01'
```

### 2. Unit Conversions

**Decision**: Unit conversions should be automatic, but we must be certain of units.

**Implementation Strategy**:
- Always require explicit unit specification in source format
- If units are ambiguous, error (don't assume)
- Track units through entire conversion pipeline
- Validate dimensional consistency after conversion
- Log all unit conversions in warnings

**SWMM Example**:
```
[OPTIONS]
FLOW_UNITS  CFS
```
If missing → ERROR: "Unit specification required. Add FLOW_UNITS to [OPTIONS] section"

**Civil3D Example**:
Units in XML schema attributes - parse and validate

### 3. Validation Strategy

**Decision**: Parse, don't validate. (Parse leniently, validate separately)

**Implementation Approach**:
- **During Import**:
  - Parse all data successfully if possible
  - Generate warnings for questionable values
  - Don't reject files for validation failures
  - Build a complete network structure

- **After Import** (separate validation pass):
  - Check connectivity
  - Verify slope consistency
  - Validate hydraulic parameters
  - Report violations but don't fail

**Rationale**:
- Import should succeed even if network has design errors
- Engineers may want to import partially complete designs
- Validation is a separate concern from parsing
- Users can fix validation errors in hec22

### 4. Coordinate System Handling

**Decision**: GIS coordinate transforms are out of scope.

**Implementation**:
- Import coordinates as-is (no reprojection)
- Store original coordinate system metadata if available
- Document required coordinate system in exports
- For visualization, assume consistent coordinate system
- If mixing coordinate systems → user's responsibility

**Acceptable**:
- Reading State Plane coordinates from shapefile
- Storing lat/lon from SWMM if provided
- Preserving x/y from Civil3D

**Out of Scope**:
- Converting between State Plane zones
- Lat/lon ↔ State Plane conversion
- Datum transformations (NAD27 → NAD83)

### 5. Round-Tripping

**Decision**: Round-tripping will not be possible as schemas don't match.

**Implications**:
- SWMM → hec22 → SWMM will lose SWMM-specific features
- Focus on one-way migration paths initially
- Document which features survive round-trip
- Preserve original file as metadata if possible

**Strategy**:
- Store original format metadata in `custom` fields
- Generate warnings about data loss on export
- Provide "best effort" export with clear limitations
- Recommend keeping original files as source of truth

## Format Requirements

### SWMM (.inp) - HIGH PRIORITY

**Why Start Here**:
- Well-documented text-based format (easy to parse)
- Similar domain model (urban drainage)
- Large user base with many example files
- Good alignment with hec22's node-conduit model

**Required Sections**:
```
[JUNCTIONS]     → hec22 Junction nodes
[OUTFALLS]      → hec22 Outfall nodes
[CONDUITS]      → hec22 Pipe conduits
[SUBCATCHMENTS] → hec22 DrainageArea
[RAINGAGES]     → hec22 Rainfall (partial)
[XSECTIONS]     → Pipe geometry
[LOSSES]        → Entrance/exit/bend losses
```

**Optional Sections**:
```
[CURVES]        → Stage-discharge curves
[TIMESERIES]    → Rainfall time series (may skip)
[INLETS]        → SWMM-INLET extension (if present)
```

**Mapping Strategy**:
- Parse line-by-line with section headers
- Build lookup tables for cross-references
- Handle both EPA SWMM and PCSWMM formats
- Support comments and blank lines

**What to Collect**:
- SWMM 5.2 Reference Manual (EPA-600/R-22/030)
- Sample .inp files from real projects (3-5 files of varying complexity)
- SWMM-INLET documentation if supporting inlet extensions

### Civil3D XML - MEDIUM PRIORITY

**Format**: XML export from Autodesk Civil3D Pipe Network

**Required Elements**:
- Pipe network structure
- Part references (pipes, structures)
- Invert elevations and slopes
- Coordinate geometry

**Challenges**:
- References to part catalogs (need to resolve or require catalog)
- Multiple coordinate systems possible
- Proprietary extensions

**What to Collect**:
- Civil3D Pipe Network XML schema documentation
- Example exports from Civil3D 2022+ (at least 3 examples)
- Understanding of part catalog structure
- Coordinate system metadata format

### HydroCAD - MEDIUM PRIORITY

**Format**: HydroCAD project files (.hcd or exports)

**Notes**:
- Determine if XML-based or proprietary binary
- May focus on export formats rather than native .hcd
- Strong for detention/retention (HEC-22 Chapter 10)

**What to Collect**:
- HydroCAD file format documentation
- Sample export files (3-5 examples)
- Clarification on current HydroCAD version format

### GeoJSON - HIGH PRIORITY (Export)

**Purpose**: GIS integration and web mapping

**Structure**:
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {"type": "Point", "coordinates": [x, y]},
      "properties": {
        "id": "IN-001",
        "type": "inlet",
        "invert_elev": 120.5,
        "rim_elev": 125.0
      }
    }
  ]
}
```

**Layers**:
- Nodes (Point features)
- Conduits (LineString features)
- Drainage areas (Polygon features, if geometry available)

**Coordinate Handling**:
- GeoJSON requires WGS84 (EPSG:4326) longitude/latitude
- If hec22 has x/y in State Plane → document as-is or skip coordinates
- If hec22 has lat/lon → use directly

**What to Collect**:
- GeoJSON specification (RFC 7946)
- Examples of drainage networks in GeoJSON
- Determine attribute schema for properties

## Dependencies

### Required Crates

```toml
[dependencies]
# Already have:
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.3"

# New for converters:
regex = "1.10"              # Pattern matching in SWMM text format
quick-xml = "0.31"          # Civil3D XML parsing
geojson = "0.24"            # GeoJSON output
encoding_rs = "0.8"         # Handle legacy file encodings (SWMM files may be non-UTF8)

# Optional/future:
shapefile = "0.5"           # Shapefile I/O
gdal = "0.16"               # Advanced GIS operations (only if needed)
```

## Testing Strategy

### Test Corpus Structure

```
tests/converters/
  swmm/
    fixtures/
      simple.inp              # Minimal valid SWMM file (3 nodes, 2 pipes)
      complex.inp             # Full-featured SWMM project
      bentonville.inp         # Port of existing hec22 example
      with_inlets.inp         # SWMM with inlet extension
    expected/
      simple_expected.json    # Expected hec22 JSON output
      complex_expected.json
    swmm_import_test.rs
    swmm_export_test.rs
  civil3d/
    fixtures/
      drainage_export.xml
    expected/
      drainage_expected.json
    civil3d_import_test.rs
  geojson/
    expected/
      bentonville.geojson
    geojson_export_test.rs
```

### Test Categories

**1. Unit Tests** (per converter):
- Section parsers
- Unit conversion functions
- Error handling for malformed input

**2. Integration Tests**:
- Full file import
- Validate against expected output
- Round-trip where possible
- Warning generation

**3. Validation Tests**:
- Network connectivity after import
- Hydraulic parameter ranges
- Coordinate validity

**4. Regression Tests**:
- Keep known-good conversions as fixtures
- Detect breaking changes

## Implementation Roadmap

### Phase 5.1: Foundation (Week 1-2)

**Goals**:
- [ ] Set up converter module structure
- [ ] Define `Converter` trait and error types
- [ ] Implement `ConversionContext` and warning system
- [ ] Add CLI subcommand: `hec22 convert`
- [ ] Write design documentation

**Deliverables**:
- `src/converters/mod.rs` with trait definitions
- `src/converters/error.rs` with error types
- CLI integration in `src/main.rs`
- Tests for warning system

### Phase 5.2: SWMM Import (Week 3-5)

**Goals**:
- [ ] Implement SWMM .inp parser
- [ ] Map SWMM sections to hec22 types
- [ ] Handle unit conversions (CFS/GPM/MGD/CMS/LPS)
- [ ] Generate warnings for unsupported features
- [ ] Test with 5+ real SWMM files

**Deliverables**:
- `src/converters/swmm/import.rs` fully functional
- Test suite with fixtures
- Documentation of SWMM mapping
- Known limitations documented

**Critical Sections**:
1. `[JUNCTIONS]` → `Node::Junction`
2. `[OUTFALLS]` → `Node::Outfall`
3. `[CONDUITS]` + `[XSECTIONS]` → `Conduit::Pipe`
4. `[SUBCATCHMENTS]` → `DrainageArea`
5. `[OPTIONS]` → `Project` metadata and units

**Stretch Goals**:
- Support SWMM-INLET extension for inlet nodes
- Import `[CURVES]` for stage-discharge

### Phase 5.3: GeoJSON Export (Week 6-7)

**Goals**:
- [ ] Implement GeoJSON export for nodes and conduits
- [ ] Handle coordinate systems appropriately
- [ ] Include all relevant properties
- [ ] Test with GIS software (QGIS)

**Deliverables**:
- `src/converters/geojson/export.rs`
- Sample GeoJSON outputs
- Documentation for GIS users

**Feature Layers**:
- Nodes (Point)
- Conduits (LineString)
- Drainage areas (Polygon, if geometry exists)

### Phase 5.4: Additional Formats (Week 8+)

**Civil3D Import**:
- [ ] Research Civil3D XML schema
- [ ] Implement XML parser
- [ ] Map pipe network elements
- [ ] Test with Civil3D exports

**SWMM Export** (optional):
- [ ] Implement hec22 → SWMM export
- [ ] Document limitations and data loss
- [ ] Provide warnings for unsupported features

## Preparation Checklist

### Before Starting Implementation

**Research & Collection**:
- [ ] Download SWMM 5.2 Reference Manual (EPA-600/R-22/030)
- [ ] Collect 5+ sample SWMM .inp files of varying complexity
  - [ ] Simple network (3-5 nodes)
  - [ ] Medium network (20-50 nodes)
  - [ ] Complex network (100+ nodes)
  - [ ] Network with inlets (if SWMM-INLET extension available)
  - [ ] Bentonville example converted to SWMM (if possible)
- [ ] Collect 3+ Civil3D XML export examples
- [ ] Research HydroCAD file format (determine feasibility)
- [ ] Review GeoJSON RFC 7946 specification
- [ ] Identify coordinate system handling requirements

**Design Documentation**:
- [ ] Document SWMM → hec22 mapping strategy (create `docs/reference/file_formats/swmm_mapping.md`)
- [ ] Document unit conversion strategy (add to `docs/reference/constants/`)
- [ ] Create converter error taxonomy
- [ ] Design warning message templates

**Infrastructure**:
- [ ] Set up test corpus directory structure
- [ ] Add converter dependencies to Cargo.toml
- [ ] Create example conversion commands for README
- [ ] Plan CLI help text and error messages

**Validation**:
- [ ] Identify validation rules for imported networks
- [ ] Design validation report format
- [ ] Plan interactive mode for resolving missing data

### Success Criteria

**Phase 5 is considered complete when**:
- [ ] Can import SWMM files and produce valid hec22 JSON
- [ ] Can export hec22 JSON to GeoJSON for GIS use
- [ ] Comprehensive test suite with real-world examples
- [ ] Clear documentation of supported features and limitations
- [ ] Warning system provides actionable guidance
- [ ] CLI is intuitive for non-programmers

## Known Limitations & Future Work

### Current Limitations

1. **No Coordinate Transforms**: Users must provide data in consistent coordinate systems
2. **No Round-Trip Guarantee**: Converting A → B → A may lose information
3. **Limited Dynamic Modeling**: SWMM's time-series routing not supported
4. **Manual Inlet Mapping**: SWMM inlets may require manual classification

### Future Enhancements

- **Interactive Mode**: Prompt for missing critical values during import
- **Batch Conversion**: Convert entire directories of files
- **Validation Reports**: Generate detailed validation reports after import
- **Format Auto-Detection**: Automatically detect file format from content
- **Partial Imports**: Import only specific sections (e.g., just network geometry)
- **Excel Templates**: Pre-formatted Excel sheets with VBA macros
- **PDF Export**: Generate professional calculation reports (separate phase)

## References

### Specifications & Standards
- EPA SWMM 5.2 Reference Manual (EPA-600/R-22/030)
- GeoJSON Format Specification (RFC 7946)
- FHWA HEC-22 (4th Edition, 2024)

### Example Tools
- SWMM-GUI (EPA)
- PCSWMM (CHI Water)
- Autodesk Civil3D
- HydroCAD
- StormCAD (Bentley)

### Similar Projects
- swmmr (R package for SWMM files)
- swmmio (Python library for SWMM)
- pyswmm (Python wrapper for SWMM engine)

## Document History

| Version | Date       | Author | Changes                           |
|---------|------------|--------|-----------------------------------|
| 1.0     | 2026-01-01 | System | Initial requirements document     |

---

**Next Steps**: Collect sample files and begin Phase 5.1 (Foundation) implementation.
