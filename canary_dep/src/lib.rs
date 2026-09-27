#![forbid(unsafe_code)]

pub fn unchanged_compiler_cache_neighbor() -> &'static str {
    "sccache-unchanged-neighbor-v1"
}

#[cfg(test)]
mod tests {
    use super::unchanged_compiler_cache_neighbor;

    #[test]
    fn unchanged_neighbor_is_stable() {
        assert_eq!(
            unchanged_compiler_cache_neighbor(),
            "sccache-unchanged-neighbor-v1"
        );
    }
}
