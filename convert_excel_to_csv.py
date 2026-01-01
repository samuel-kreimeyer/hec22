#!/usr/bin/env python3
"""
Convert storm sewer system Excel file to HEC-22 CSV format
"""

import pandas as pd
import numpy as np

# Read the Excel file
xl_file = "storm sewer system.xlsx"
inlets_df = pd.read_excel(xl_file, sheet_name='Inlets')
pipes_df = pd.read_excel(xl_file, sheet_name='Pipes')

# ============================================================================
# Create Nodes CSV
# ============================================================================
nodes_data = []

for idx, row in inlets_df.iterrows():
    station = str(row['Station']).strip().rstrip('.')
    inlet_type = str(row['Type']).strip()
    location = str(row['Location']).strip() if pd.notna(row['Location']) else 'on grade'
    invert = row['Invert']
    rim = row['Rim']

    # Identify outfall nodes (these are terminal discharge points)
    outfall_nodes = ['200+19 RT', '203+48 RT', '216+90 RT']

    # Determine node type
    if station in outfall_nodes:
        node_type = 'outfall'
        inlet_type_value = ''
        inlet_location = ''
    elif 'Manhole' in inlet_type:
        node_type = 'junction'
        inlet_type_value = ''
        inlet_location = ''
    else:
        node_type = 'inlet'
        # Map inlet types
        if 'Curb' in inlet_type:
            inlet_type_value = 'curb'
        elif 'Grate' in inlet_type:
            inlet_type_value = 'grate'
        else:
            inlet_type_value = 'combination'

        # Map location
        if 'sag' in location.lower():
            inlet_location = 'sag'
        else:
            inlet_location = 'on_grade'

    # Get structure dimensions
    shape = str(row['Structure Shape']).strip() if pd.notna(row['Structure Shape']) else 'Circular'
    diameter = row['Structure Length'] if pd.notna(row['Structure Length']) else None

    # Get grate dimensions
    grate_length = row['Grate Length'] if pd.notna(row['Grate Length']) and row['Grate Length'] > 0 else None
    grate_width = row['Grate Width'] if pd.notna(row['Grate Width']) and row['Grate Width'] > 0 else None

    # Get curb opening dimensions
    curb_length = row['Curb Length'] if pd.notna(row['Curb Length']) and row['Curb Length'] > 0 else None
    curb_height = row['Curb Height'] if pd.notna(row['Curb Height']) and row['Curb Height'] > 0 else None

    # Get depression
    depression = row['Depression'] if pd.notna(row['Depression']) and row['Depression'] > 0 else None

    # Get bypass routing (where bypass flow goes)
    bypass_to = None
    if pd.notna(row['Bypass']) and node_type == 'inlet':
        bypass_raw = str(row['Bypass']).strip().rstrip('.')
        # Apply same fixes as for station names
        if bypass_raw == "208+22 RT":
            bypass_to = "208+17 RT"
        elif bypass_raw == "413+36 23' LT":
            bypass_to = "413+36 @ 23' LT"
        else:
            bypass_to = bypass_raw

    nodes_data.append({
        'id': station,
        'type': node_type,
        'invert_elev': invert,
        'rim_elev': rim if node_type != 'outfall' else '',  # Outfalls don't need rim elevation
        'x': None,  # No coordinates in spreadsheet
        'y': None,
        'shape': shape if node_type == 'junction' else '',
        'diameter': diameter if node_type == 'junction' else '',
        'width': '',
        'height': '',
        'inlet_type': inlet_type_value,
        'inlet_location': inlet_location,
        'grate_length': grate_length if node_type == 'inlet' else '',
        'grate_width': grate_width if node_type == 'inlet' else '',
        'bar_configuration': 'perpendicular' if grate_length else '',
        'curb_opening_length': curb_length if node_type == 'inlet' else '',
        'curb_opening_height': curb_height if node_type == 'inlet' else '',
        'throat_type': 'horizontal' if curb_length else '',
        'local_depression': depression if node_type == 'inlet' else '',
        'clogging_factor': 0.15 if node_type == 'inlet' else '',
        'grate_count': 1 if grate_length else '',
        'boundary_condition': 'free' if node_type == 'outfall' else '',
        'bypass_to': bypass_to if node_type == 'inlet' else ''
    })

nodes_csv = pd.DataFrame(nodes_data)
nodes_csv.to_csv('nodes.csv', index=False)

# Count node types
inlet_count = len([n for n in nodes_data if n['type'] == 'inlet'])
junction_count = len([n for n in nodes_data if n['type'] == 'junction'])
outfall_count = len([n for n in nodes_data if n['type'] == 'outfall'])
print(f"Created nodes.csv with {len(nodes_csv)} nodes ({inlet_count} inlets, {junction_count} junctions, {outfall_count} outfalls)")

# ============================================================================
# Create Conduits CSV
# ============================================================================

# Fix known data inconsistencies in the spreadsheet
def fix_station_name(station):
    """Fix known inconsistencies in station names"""
    station = station.strip().rstrip('.')
    # Fix known typos/inconsistencies
    if station == "208+22 RT":
        return "208+17 RT"  # Typo in spreadsheet
    if station == "413+36 23' LT":
        return "413+36 @ 23' LT"  # Missing @ symbol
    return station

conduits_data = []

# Define outfall nodes
outfall_nodes = ['200+19 RT', '203+48 RT', '216+90 RT']

for idx, row in pipes_df.iterrows():
    upstream = fix_station_name(str(row['Upstream']))
    downstream = fix_station_name(str(row['Downstream']))

    # Skip pipes that go from an outfall to "Outfall" - these shouldn't exist
    # since the upstream node IS the outfall
    if downstream == 'Outfall' and upstream in outfall_nodes:
        continue

    # Determine pipe shape
    pipe_type_str = str(row['Type.1']).strip() if pd.notna(row['Type.1']) else 'Circular'
    if 'Arch' in pipe_type_str:
        shape = 'arch'
        diameter = row['Rise'] if pd.notna(row['Rise']) else 18
        width = row['Span'] if pd.notna(row['Span']) else diameter
    elif 'Rectangular' in pipe_type_str:
        shape = 'rectangular'
        diameter = ''
        width = row['Span'] if pd.notna(row['Span']) else 24
        height = row['Rise'] if pd.notna(row['Rise']) else 24
    else:
        shape = 'circular'
        diameter = row['Rise'] if pd.notna(row['Rise']) else 18
        width = ''
        height = ''

    # Get other properties
    length = row['Length'] if pd.notna(row['Length']) else 100
    slope = row['Slope'] if pd.notna(row['Slope']) else 0.01
    manning_n = row['n Value'] if pd.notna(row['n Value']) else 0.013

    conduits_data.append({
        'id': f"{upstream}_to_{downstream}",
        'type': 'pipe',
        'from_node': upstream,
        'to_node': downstream,
        'shape': shape,
        'diameter': diameter if shape == 'circular' or shape == 'arch' else '',
        'width': width if shape == 'arch' or shape == 'rectangular' else '',
        'height': height if shape == 'rectangular' else '',
        'length': length,
        'slope': slope,
        'manning_n': manning_n,
        'material': 'RCP',
        'cross_slope': '',
        'long_slope': ''
    })

conduits_csv = pd.DataFrame(conduits_data)
conduits_csv.to_csv('conduits.csv', index=False)
print(f"Created conduits.csv with {len(conduits_csv)} conduits")

# ============================================================================
# Create Drainage Areas CSV
# ============================================================================
drainage_areas_data = []

for idx, row in pipes_df.iterrows():
    upstream = fix_station_name(str(row['Upstream']))

    # Get drainage area and runoff coefficient
    area = row['Drainage Area'] if pd.notna(row['Drainage Area']) and row['Drainage Area'] > 0 else None
    runoff_coef = row['Runoff Coefficient'] if pd.notna(row['Runoff Coefficient']) else 0.8
    time_of_conc = row['Time of Concentration'] if pd.notna(row['Time of Concentration']) else 5.0

    if area and area > 0:
        drainage_areas_data.append({
            'id': f"DA_{upstream}",
            'area': area,
            'runoff_coef': runoff_coef,
            'time_of_conc': time_of_conc,
            'outlet_node': upstream,
            'land_use': 'Commercial',
            'design_storm': ''
        })

drainage_areas_csv = pd.DataFrame(drainage_areas_data)
drainage_areas_csv.to_csv('drainage_areas.csv', index=False)
print(f"Created drainage_areas.csv with {len(drainage_areas_csv)} drainage areas")

# ============================================================================
# Create IDF Curves CSV (example values - should be replaced with real data)
# ============================================================================
idf_data = []

# Example IDF curve for 10-year storm (typical urban area)
# These are placeholder values - user should provide real IDF data for their location
durations = [5, 10, 15, 30, 60, 120]  # minutes
intensities_10yr = [6.5, 5.2, 4.5, 3.8, 2.8, 2.0]  # in/hr

for duration, intensity in zip(durations, intensities_10yr):
    idf_data.append({
        'return_period': 10,
        'duration': duration,
        'intensity': intensity
    })

idf_csv = pd.DataFrame(idf_data)
idf_csv.to_csv('idf_curves.csv', index=False)
print(f"Created idf_curves.csv with {len(idf_csv)} data points")

print("\n=== Conversion Complete ===")
print("Created files:")
print("  - nodes.csv")
print("  - conduits.csv")
print("  - drainage_areas.csv")
print("  - idf_curves.csv")
print("\nYou can now run the analysis with:")
print("  cargo run --release -- --nodes nodes.csv --conduits conduits.csv --drainage-areas drainage_areas.csv --idf-curves idf_curves.csv --return-period 10 --output analysis_results.txt")
