//! Placeholder library for the Rust framework bootstrap template.
//!
//! Generated repositories replace this package identity and source during
//! bootstrap. The template itself owns no product semantics.

#[cfg(test)]
mod workflow_source_freshness_canary {
    #[test]
    fn source_freshness_marker_v1() {
        let marker = "workflow-source-freshness-marker-v1";
        assert!(marker.ends_with("v1"));
    }
}
