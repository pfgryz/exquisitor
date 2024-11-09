use crate::searching::organism::OrganismFound;
use std::collections::{HashMap, HashSet};

fn calculate_search_quality(
    reference: Vec<OrganismFound>,
    probe: Vec<OrganismFound>,
) -> (f64, f64) {
    let reference_map: HashMap<String, f64> = reference.into_iter().map(|s| s.to_tuple()).collect();

    let probe_map: HashMap<String, f64> = probe.into_iter().map(|s| s.to_tuple()).collect();

    // Positive
    let mut positive = 0f64;

    for (name, quality) in &reference_map {
        let probe_quality = probe_map.get(name).unwrap_or(&0.0);
        positive += f64::min(*quality, *probe_quality) / quality;
    }

    positive /= reference_map.len() as f64;

    // Negative
    let unique: HashSet<String> = reference_map
        .keys()
        .chain(probe_map.keys())
        .cloned()
        .collect();
    let mut negative = 0f64;

    for name in &unique {
        let reference_quality = *reference_map.get(name).unwrap_or(&0.0);
        let probe_quality = *probe_map.get(name).unwrap_or(&0.0);

        let common = f64::min(reference_quality, probe_quality);
        let sum = f64::max(reference_quality, probe_quality);
        negative += common / sum;
    }

    negative /= unique.len() as f64;

    (positive, negative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_calculate_search_quality_both() {
        let reference = vec![
            OrganismFound::new("A".into(), 2f64),
            OrganismFound::new("B".into(), 2f64),
        ];
        let probe = vec![
            OrganismFound::new("A".into(), 1f64),
            OrganismFound::new("B".into(), 2f64),
        ];

        let (positive, negative) = calculate_search_quality(reference, probe);

        assert_approx_eq!(f64, positive, 0.75);
        assert_approx_eq!(f64, negative, 0.75);
    }

    #[test]
    fn test_calculate_search_quality_missing_in_probe() {
        let reference = vec![
            OrganismFound::new("A".into(), 2f64),
            OrganismFound::new("B".into(), 2f64),
        ];
        let probe = vec![OrganismFound::new("B".into(), 1f64)];

        let (positive, negative) = calculate_search_quality(reference, probe);

        assert_approx_eq!(f64, positive, 0.25);
        assert_approx_eq!(f64, negative, 0.25);
    }

    #[test]
    fn test_calculate_search_quality_missing_in_reference() {
        let reference = vec![OrganismFound::new("A".into(), 2f64)];
        let probe = vec![
            OrganismFound::new("A".into(), 1f64),
            OrganismFound::new("B".into(), 2f64),
        ];

        let (positive, negative) = calculate_search_quality(reference, probe);

        assert_approx_eq!(f64, positive, 0.5);
        assert_approx_eq!(f64, negative, 0.25);
    }
}
