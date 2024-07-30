#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

use super::*;

/// Example Bundle resource (search result) compatible with STU3, R4B and R5
/// (due to hacky duplicate information / additional keys).
const BUNDLE_SEARCH_RESULT_EXAMPLE: &str = r#"
	{
		"resourceType" : "Bundle",
		"id" : "bundle-example",
		"meta" : {
			"lastUpdated" : "2014-08-18T01:43:30Z"
		},
		"type" : "searchset",
		"total" : 3,
		"link" : [{
			"relation" : "self",
			"url" : "https://example.com/base/MedicationRequest?patient=347&_include=MedicationRequest.medication&_count=2"
		},
		{
			"relation" : "next",
			"url" : "https://example.com/base/MedicationRequest?patient=347&searchId=ff15fd40-ff71-4b48-b366-09c706bed9d0&page=2"
		}],
		"entry" : [{
			"fullUrl" : "https://example.com/base/MedicationRequest/3123",
			"resource" : {
				"resourceType" : "MedicationRequest",
				"id" : "3123",
				"status" : "unknown",
				"intent" : "order",
				"medication" : {
					"reference" : {
						"reference" : "Medication/example"
					}
				},
				"medicationReference": {
					"reference": "Medication/example"
				},
				"subject" : {
					"reference" : "Patient/347"
				}
			},
			"search" : {
				"mode" : "match",
				"score" : 1
			}
		},
		{
			"fullUrl" : "https://example.com/base/Medication/example",
			"resource" : {
				"resourceType" : "Medication",
				"id" : "example"
			},
			"search" : {
				"mode" : "include"
			}
		}]
	}
	"#;

macro_rules! test_bundle_ext {
	($version:ident) => {
		mod $version {
			use $crate::$version::resources::Bundle;

			use super::*;

			fn search_result() -> Bundle {
				serde_json::from_str(BUNDLE_SEARCH_RESULT_EXAMPLE).unwrap()
			}

			#[test]
			fn bundle_ext_next_page_url() {
				let bundle = search_result();

				let next_url = bundle.next_page_url().unwrap();
				assert_eq!(*next_url, "https://example.com/base/MedicationRequest?patient=347&searchId=ff15fd40-ff71-4b48-b366-09c706bed9d0&page=2");
			}
		}
	};
}

for_all_versions!(test_bundle_ext);
