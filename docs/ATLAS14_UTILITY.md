# ATLAS14 Rainfall Data Utility

## Overview

The `atlas14_fetch` utility fetches precipitation frequency data from NOAA ATLAS14 and generates IDF (Intensity-Duration-Frequency) curves in CSV format compatible with the HEC-22 hydraulic analysis tool.

## Installation

Build the utility using Cargo:

```bash
cargo build --release --bin atlas14_fetch
```

The binary will be located at `target/release/atlas14_fetch`.

## Usage

### Basic Usage

Fetch IDF data for a specific location using latitude and longitude:

```bash
atlas14_fetch --lat 40.7128 --lon -74.0060 --output nyc_idf.csv
```

### Command-Line Options

- `--lat, -y <LAT>`: Latitude in decimal degrees (required)
- `--lon, -x <LON>`: Longitude in decimal degrees (required)
- `--output, -o <FILE>`: Output CSV file path (required)
- `--units, -u <UNITS>`: Unit system - "english" or "metric" (default: "english")
- `--return-periods, -r <PERIODS>`: Comma-separated return periods in years (default: "2,5,10,25,50,100")
- `--durations, -d <DURATIONS>`: Comma-separated durations in minutes (default: "5,10,15,30,60,120")

### Examples

#### Example 1: New York City (English units)
```bash
atlas14_fetch --lat 40.7128 --lon -74.0060 --output nyc_idf.csv
```

#### Example 2: Custom return periods and durations
```bash
atlas14_fetch \
  --lat 34.0522 \
  --lon -118.2437 \
  --output la_idf.csv \
  --return-periods "1,2,5,10,25,50,100,500" \
  --durations "5,10,15,30,45,60,120,180,360,720,1440"
```

#### Example 3: Metric units
```bash
atlas14_fetch \
  --lat 51.5074 \
  --lon -0.1278 \
  --output london_idf.csv \
  --units metric
```

## Output Format

The utility generates a CSV file with three columns compatible with HEC-22:

```csv
return_period,duration,intensity
2,5,6.82
2,10,5.49
2,15,4.75
2,30,3.54
...
```

Where:
- `return_period`: Return period in years
- `duration`: Storm duration in minutes
- `intensity`: Rainfall intensity (in/hr for English units, mm/hr for metric units)

## Using the Output with HEC-22

The generated IDF CSV can be used directly with the HEC-22 analysis tools:

```bash
# Use the IDF curve in your hydraulic analysis
hec22 \
  --nodes nodes.csv \
  --conduits conduits.csv \
  --drainage-areas drainage.csv \
  --idf-curve nyc_idf.csv \
  --return-period 10 \
  --output results.json
```

## IDF Curve Interpolation

The HEC-22 library automatically interpolates between IDF curve points using linear interpolation. If your time of concentration falls between two duration values (e.g., 12 minutes when you have 10 and 15 minute data points), the intensity will be calculated as:

```
i(12) = i(10) + (12-10)/(15-10) * (i(15) - i(10))
```

This is implemented in `src/rainfall.rs` in the `IdfCurve::get_intensity()` method.

## Finding Coordinates

### Method 1: Google Maps
1. Right-click on a location in Google Maps
2. Select the coordinates to copy them
3. Format: latitude, longitude (e.g., 40.7128, -74.0060)

### Method 2: NOAA ATLAS14 Website
1. Visit https://hdsc.nws.noaa.gov/pfds/
2. Click on your state
3. Click on the map to see the coordinates for that location

### Method 3: Using a geocoding service
You can use online geocoding services to convert city names to coordinates:
- https://www.latlong.net/
- https://nominatim.openstreetmap.org/

## Technical Details

### Data Source

The utility fetches **real** precipitation frequency data from the NOAA HDSC (Hydrometeorological Design Studies Center) Precipitation Frequency Data Server API:

- **API Endpoint**: `https://hdsc.nws.noaa.gov/cgi-bin/new/fe_text_lwr.csv`
- **Method**: HTTP GET
- **Parameters**:
  - `lat`: Latitude in decimal degrees
  - `lon`: Longitude in decimal degrees
  - `data`: `intensity` (rainfall intensity in in/hr or mm/hr)
  - `units`: `english` or `metric`
  - `series`: `pds` (partial duration series) for design applications

### Data Quality

The utility fetches **official NOAA ATLAS14 precipitation frequency estimates** directly from NOAA's servers. This is the same authoritative data used by professional engineers and included in NOAA Atlas 14 publications.

**Coverage**: NOAA Atlas 14 currently covers most of the United States. Coverage may vary by region:
- Contiguous US: Complete coverage
- Alaska: Limited coverage
- Hawaii and US territories: May have limited or no coverage

For locations outside ATLAS14 coverage, the API will return an error.

### Data Format

NOAA returns CSV data with:
- **Rows**: Different return periods (1, 2, 5, 10, 25, 50, 100, 200, 500, 1000 years)
- **Columns**: Different storm durations (5-min, 10-min, 15-min, 30-min, 1-hr, 2-hr, 3-hr, 6-hr, 12-hr, 24-hr, etc.)
- **Values**: Precipitation intensity (in/hr) or depth (inches) depending on the `data` parameter

The utility automatically parses this CSV and extracts only the requested return periods and durations.

## Troubleshooting

### "Latitude must be between -90 and 90 degrees"
Ensure your latitude coordinate is valid. Remember:
- Positive = North
- Negative = South

### "Longitude must be between -180 and 180 degrees"
Ensure your longitude coordinate is valid. Remember:
- Positive = East
- Negative = West

### Network Errors
The utility requires internet access to query the NOAA ATLAS14 database. Check your internet connection and firewall settings.

## Future Enhancements

Planned features for future versions:
- [ ] Support for city/address geocoding (no manual coordinates needed)
- [ ] Confidence interval data (upper/lower bounds)
- [ ] Temporal distribution patterns (SCS Type I, IA, II, III)
- [ ] Batch processing of multiple locations
- [ ] Caching of fetched data to minimize API calls
- [ ] Support for precipitation depth values in addition to intensity

## References

- NOAA Atlas 14: https://hdsc.nws.noaa.gov/pfds/
- HEC-22 Urban Drainage Design Manual: https://www.fhwa.dot.gov/engineering/hydraulics/pubs/10009/
- IDF Curve Theory: https://www.weather.gov/media/owp/oh/hdsc/docs/Atlas14_Volume2.pdf
