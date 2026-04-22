use std::path::PathBuf;

/// Returns the path to the icons directory
fn icons_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icons")
}

/// Reads an SVG icon from disk given its ID
pub fn load_icon_svg(icon_id: u64) -> Result<String, std::io::Error> {
    let icon_path = icons_dir().join(format!("{}.svg", icon_id));
    std::fs::read_to_string(icon_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_existing_icon() {
        // Icon 0 should exist
        let result = load_icon_svg(0);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("<svg"));
    }

    #[test]
    fn test_load_nonexistent_icon() {
        // Icon 999999 should not exist
        let result = load_icon_svg(999999);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_common_icons() {
        // Test some commonly used icons
        assert!(load_icon_svg(296).is_ok()); // JS
        assert!(load_icon_svg(633).is_ok()); // TS
        assert!(load_icon_svg(525).is_ok()); // RS
    }
}
