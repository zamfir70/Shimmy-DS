#[cfg(test)]
mod tests {
    use crate::profile::{ProfileMesh, TasteVector, MistakePattern};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_profile_mesh_default() {
        let mesh = ProfileMesh::default();

        assert_eq!(mesh.taste.curiosity, 0.05);
        assert_eq!(mesh.taste.coherence_pleasure, 0.05);
        assert_eq!(mesh.preferred_complexity, 0.5);
        assert!(mesh.common_mistakes.is_empty());
    }

    #[test]
    fn test_profile_mesh_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_mesh.json");
        let path_str = path.to_str().unwrap();

        let mut mesh = ProfileMesh::default();
        mesh.taste.curiosity = 0.8;
        mesh.preferred_complexity = 0.7;
        mesh.register_mistake("Inconsistent character motivation");

        // Save
        mesh.save(path_str).unwrap();

        // Load
        let loaded_mesh = ProfileMesh::load(path_str);

        assert_eq!(loaded_mesh.taste.curiosity, 0.8);
        assert_eq!(loaded_mesh.preferred_complexity, 0.7);
        assert_eq!(loaded_mesh.common_mistakes.len(), 1);
        assert_eq!(loaded_mesh.common_mistakes[0].description, "Inconsistent character motivation");
        assert_eq!(loaded_mesh.common_mistakes[0].frequency, 1);
    }

    #[test]
    fn test_profile_mesh_load_nonexistent() {
        let mesh = ProfileMesh::load("nonexistent_file.json");

        // Should return default values
        assert_eq!(mesh.taste.curiosity, 0.05);
        assert_eq!(mesh.preferred_complexity, 0.5);
        assert!(mesh.common_mistakes.is_empty());
    }

    #[test]
    fn test_taste_update_and_clamping() {
        let mut mesh = ProfileMesh::default();

        let delta = TasteVector {
            curiosity: 2.0, // Should be clamped to 1.0
            coherence_pleasure: -2.0, // Should be clamped to -1.0
            delight: 0.3,
            awe: 0.0,
            unease: 0.0,
            boredom: 0.0,
        };

        mesh.update_taste(delta);

        assert_eq!(mesh.taste.curiosity, 1.0); // Clamped
        assert_eq!(mesh.taste.coherence_pleasure, -1.0); // Clamped
        assert_eq!(mesh.taste.delight, 0.3);
    }

    #[test]
    fn test_mistake_registration_and_frequency() {
        let mut mesh = ProfileMesh::default();

        mesh.register_mistake("Plot hole in chapter 3");
        mesh.register_mistake("Character inconsistency");
        mesh.register_mistake("Plot hole in chapter 3"); // Duplicate

        assert_eq!(mesh.common_mistakes.len(), 2);

        let plot_hole = mesh.common_mistakes.iter()
            .find(|m| m.description == "Plot hole in chapter 3")
            .unwrap();
        assert_eq!(plot_hole.frequency, 2);

        let char_issue = mesh.common_mistakes.iter()
            .find(|m| m.description == "Character inconsistency")
            .unwrap();
        assert_eq!(char_issue.frequency, 1);
    }

    #[test]
    fn test_mistake_weight() {
        let mut mesh = ProfileMesh::default();

        mesh.register_mistake("Common error");
        mesh.register_mistake("Common error");
        mesh.register_mistake("Common error");

        let weight = mesh.get_mistake_weight("Common error");
        assert_eq!(weight, 3.0 / 100.0);

        let no_weight = mesh.get_mistake_weight("Unknown error");
        assert_eq!(no_weight, 0.0);
    }

    #[test]
    fn test_complexity_bias() {
        let mut mesh = ProfileMesh::default();

        // Default complexity (0.5) should give 0 bias
        assert_eq!(mesh.complexity_bias(), 0.0);

        // High complexity preference
        mesh.preferred_complexity = 0.75;
        assert_eq!(mesh.complexity_bias(), 0.5);

        // Low complexity preference
        mesh.preferred_complexity = 0.25;
        assert_eq!(mesh.complexity_bias(), -0.5);
    }

    #[test]
    fn test_mistake_truncation() {
        let mut mesh = ProfileMesh::default();

        // Add more than 20 mistakes
        for i in 0..25 {
            for _ in 0..=i { // Different frequencies
                mesh.register_mistake(&format!("Error {}", i));
            }
        }

        // Should be truncated to 20 most frequent
        assert_eq!(mesh.common_mistakes.len(), 20);

        // Should be sorted by frequency (descending)
        assert!(mesh.common_mistakes[0].frequency >= mesh.common_mistakes[1].frequency);
    }
}