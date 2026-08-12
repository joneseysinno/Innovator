//! Persist unit tests — roundtrip + no resolved/rect keys.

use super::save::capture_overrides;
use super::types::*;
use super::load_layout;
use hyper_ui::{ContainerId, Overrides, SizeClass};

#[test]
fn persisted_container_json_has_no_resolved_or_rect() {
    let c = PersistedContainer {
        id: 1,
        label: "Nav".into(),
        icon: "N".into(),
        intent: PersistedVisibility::Shown,
        extent: PersistedExtent {
            min: 280.0,
            ideal: 360.0,
            weight: 0.0,
        },
    };
    let json = serde_json::to_string(&c).unwrap();
    assert!(!json.contains("resolved"));
    assert!(!json.contains("rect"));
    assert!(json.contains("intent"));
    assert!(json.contains("extent"));
}

#[test]
fn session_json_forbids_resolved_and_rect_keys() {
    let session = PersistedSession {
        version: PERSIST_VERSION,
        next_workspace_id: 9,
        workspaces: vec![PersistedWorkspace {
            open_id: "home".into(),
            state: PersistedContainer {
                id: 1,
                label: "Home".into(),
                icon: "H".into(),
                intent: PersistedVisibility::Shown,
                extent: PersistedExtent {
                    min: 320.0,
                    ideal: 1280.0,
                    weight: 1.0,
                },
            },
            focused_page: None,
            page_tree: None,
            page_overrides: PersistedOverrides::default(),
            page_templates: None,
            pod_templates: None,
            next_page_id: None,
            stub_ios: None,
        }],
    };
    let value = serde_json::to_value(&session).unwrap();
    assert_no_forbidden_keys(&value);
}

#[test]
fn collapse_overrides_roundtrip_scoped_by_size_class() {
    let mut o = Overrides::new();
    o.set_collapse(ContainerId(10), SizeClass::Compact, true);
    o.set_collapse(ContainerId(10), SizeClass::Large, false);
    let persisted = capture_overrides(&o);
    let restored = super::apply::restore_overrides(&persisted);
    assert_eq!(
        restored.get_collapse(ContainerId(10), SizeClass::Compact),
        Some(true)
    );
    assert_eq!(
        restored.get_collapse(ContainerId(10), SizeClass::Large),
        Some(false)
    );
    assert!(restored.get_collapse(ContainerId(10), SizeClass::Medium).is_none());
}

#[test]
fn overrides_roundtrip_scoped_by_size_class() {
    let mut o = Overrides::new();
    o.set(ContainerId(10), SizeClass::Large, 0.5);
    o.set(ContainerId(10), SizeClass::Compact, 0.2);
    let persisted = capture_overrides(&o);
    let restored = super::apply::restore_overrides(&persisted);
    assert!((restored.get(ContainerId(10), SizeClass::Large).unwrap() - 0.5).abs() < 1e-6);
    assert!((restored.get(ContainerId(10), SizeClass::Compact).unwrap() - 0.2).abs() < 1e-6);
    // Large entry does not affect Medium.
    assert!(restored.get(ContainerId(10), SizeClass::Medium).is_none());
}

#[test]
fn save_load_roundtrip_file() {
    let dir = std::env::temp_dir().join("innovator_persist_test");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("layout.json");
    let _ = std::fs::remove_file(&path);

    let session = PersistedSession {
        version: PERSIST_VERSION,
        next_workspace_id: 3,
        workspaces: vec![PersistedWorkspace {
            open_id: "structural_analysis".into(),
            state: PersistedContainer {
                id: 2,
                label: "Structural".into(),
                icon: "S".into(),
                intent: PersistedVisibility::Hidden,
                extent: PersistedExtent {
                    min: 320.0,
                    ideal: 1280.0,
                    weight: 1.0,
                },
            },
            focused_page: Some(0),
            page_tree: None,
            page_overrides: PersistedOverrides {
                entries: vec![PersistedOverrideEntry {
                    id: 1,
                    class: PersistedSizeClass::Large,
                    fraction: 0.55,
                }],
                ..PersistedOverrides::default()
            },
            page_templates: None,
            pod_templates: None,
            next_page_id: Some(3),
            stub_ios: None,
        }],
    };
    let json = serde_json::to_string_pretty(&session).unwrap();
    std::fs::write(&path, &json).unwrap();
    let loaded = load_layout(&path).expect("load");
    assert_eq!(loaded.version, PERSIST_VERSION);
    assert_eq!(loaded.workspaces[0].open_id, "structural_analysis");
    assert!((loaded.workspaces[0].page_overrides.entries[0].fraction - 0.55).abs() < 1e-6);
    assert_no_forbidden_keys(&serde_json::from_str(&json).unwrap());
    let _ = std::fs::remove_file(&path);
}

fn assert_no_forbidden_keys(value: &serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                assert_ne!(k, "resolved", "forbidden key in JSON");
                assert_ne!(k, "rect", "forbidden key in JSON");
                assert_no_forbidden_keys(v);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                assert_no_forbidden_keys(item);
            }
        }
        _ => {}
    }
}
