//! Integration test: storage routing from JSON-defined storageFacilities.

use hec22::DrainageNetwork;

#[test]
fn test_storage_routing_from_json_facility_data() {
    let json = r#"
{
  "version": "1.0.0",
  "project": {
    "name": "Storage Routing JSON Test",
    "units": { "system": "US" }
  },
  "network": {
    "nodes": [],
    "conduits": []
  },
  "storageFacilities": [
    {
      "id": "DET-JSON-1",
      "type": "detention",
      "stageStorage": [
        { "stage": 0.0, "storage": 0.0 },
        { "stage": 1.0, "storage": 1000.0 },
        { "stage": 2.0, "storage": 2000.0 }
      ],
      "stageDischarge": [
        { "stage": 0.0, "discharge": 0.0 },
        { "stage": 1.0, "discharge": 10.0 },
        { "stage": 2.0, "discharge": 20.0 }
      ],
      "inflowHydrograph": [
        { "timeSeconds": 0.0, "inflow": 10.0 },
        { "timeSeconds": 60.0, "inflow": 20.0 },
        { "timeSeconds": 120.0, "inflow": 10.0 }
      ],
      "initialStage": 1.0
    }
  ]
}
"#;

    let mut network = DrainageNetwork::from_json(json).expect("JSON should parse");
    network
        .run_storage_routing()
        .expect("Storage routing should succeed");

    let analysis = network
        .analysis
        .as_ref()
        .expect("Analysis should be populated");
    let storage_results = analysis
        .storage_results
        .as_ref()
        .expect("Storage results should be present");

    assert_eq!(storage_results.len(), 1);
    assert_eq!(storage_results[0].facility_id, "DET-JSON-1");
    assert!(storage_results[0].max_stage > 0.0);
    assert!(storage_results[0].max_storage > 0.0);
    assert!(storage_results[0].max_outflow > 0.0);
}
