mod fixture;

use fixture::ids;
use openpfe_graph::{GrafeoGraphStore, GraphError, GraphStore, NodeFilter, SubgraphLimits};
use serde_json::json;
use tempfile::TempDir;

fn store_dir(dir: &TempDir) -> std::path::PathBuf {
    dir.path().join(".openpfe").join("graph").join("store")
}

#[test]
fn s1_lifecycle_and_reopen_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = store_dir(&dir);

    {
        let mut store = GrafeoGraphStore::open(&path).expect("open");
        fixture::seed_core(&mut store).expect("seed");
        assert!(store.get_node(ids::P1).expect("get").is_some());
        store.close().expect("close");
    }

    let store = GrafeoGraphStore::open(&path).expect("reopen");
    let p1 = store.get_node(ids::P1).expect("get").expect("p1");
    assert_eq!(p1.node_type, "problem");
    assert_eq!(
        p1.properties.get("title").and_then(|v| v.as_str()),
        Some("Auth gateway")
    );
    assert_eq!(store.node_count(), 5);
}

#[test]
fn s3_interfaces_contract_in_subgraph() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");

    let sg = store
        .subgraph(ids::CLUSTER, SubgraphLimits::default())
        .expect("subgraph");
    let iface = sg
        .edges
        .iter()
        .find(|e| e.edge_type == "interfaces")
        .expect("interfaces edge");
    assert_eq!(
        iface.properties.get("contract_body").and_then(|v| v.as_str()),
        Some("POST /api/v1/auth")
    );
}

#[test]
fn s6_bm25_search_duplicate_title() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");
    fixture::seed_s6_lexical(&mut store).expect("s6 seed");

    let hits = store.search_problems("Auth gateway", 5).expect("search");
    let top_ids: Vec<_> = hits.iter().map(|h| h.node_id.as_str()).collect();
    assert!(
        top_ids.iter().take(3).any(|id| *id == ids::P_DUP),
        "duplicate title in top 3: {top_ids:?}"
    );
    assert!(
        !top_ids.iter().take(2).any(|id| *id == ids::P_UNRELATED),
        "unrelated not in top 2"
    );
}

#[test]
fn s2_curation_update_and_delete() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");

    store
        .upsert_node(
            "problem",
            ids::P1,
            json!({
                "title": "Auth gateway v2",
                "description": "Updated",
                "status": "refined",
                "cluster_id": ids::CLUSTER
            })
            .as_object()
            .cloned()
            .unwrap(),
        )
        .expect("update");

    let updated = store.get_node(ids::P1).expect("get").expect("node");
    assert_eq!(
        updated.properties.get("status").and_then(|v| v.as_str()),
        Some("refined")
    );

    assert!(store.delete_node(ids::P3).expect("delete"));
    assert!(store.get_node(ids::P3).expect("get").is_none());
}

#[test]
fn s4_subgraph_respects_caps() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");
    fixture::seed_bulk(&mut store, 800).expect("bulk");

    let limits = SubgraphLimits::default();
    let sg = store
        .subgraph(ids::CLUSTER, limits)
        .expect("subgraph");

    assert!(sg.nodes.len() <= limits.max_nodes);
    assert!(sg.nodes.len() < 500);
}

#[test]
fn s5_acyclic_validation() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");

    let clean = store.validate_acyclic_deps().expect("validate");
    assert!(clean.is_empty());

    store
        .create_edge(ids::P3, ids::P1, "depends_on", Default::default())
        .expect("inject cycle");
    let cycles = store.validate_acyclic_deps().expect("validate cycle");
    assert!(!cycles.is_empty());
    assert!(
        cycles.iter().flatten().any(|id| id == ids::P1),
        "cycle path contains p1: {cycles:?}"
    );
}

#[test]
fn durability_close_reopen_counts() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = store_dir(&dir);

    let count = {
        let mut store = GrafeoGraphStore::open(&path).expect("open");
        fixture::seed_core(&mut store).expect("seed");
        fixture::seed_s6_lexical(&mut store).expect("s6");
        let count = store.node_count();
        store.close().expect("close");
        count
    };

    let store = GrafeoGraphStore::open(&path).expect("reopen");
    assert_eq!(store.node_count(), count);
}

#[test]
fn backup_full_and_directory_copy() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = store_dir(&dir);
    let backup_root = dir.path().join("backup");
    let copy_root = dir.path().join("copy");

    {
        let mut store = GrafeoGraphStore::open(&path).expect("open");
        fixture::seed_core(&mut store).expect("seed");
        store.backup_full(&backup_root).expect("backup");
        store.close().expect("close");
    }

    fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let dest = dst.join(entry.file_name());
            if ty.is_dir() {
                copy_dir_all(&entry.path(), &dest)?;
            } else {
                std::fs::copy(entry.path(), dest)?;
            }
        }
        Ok(())
    }

    copy_dir_all(&path, &copy_root).expect("copy store dir");

    let store = GrafeoGraphStore::open(&copy_root).expect("reopen copy");
    let p1 = store.get_node(ids::P1).expect("get").expect("p1");
    assert_eq!(
        p1.properties.get("title").and_then(|v| v.as_str()),
        Some("Auth gateway")
    );
}

#[test]
fn list_nodes_by_type_and_cluster() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");
    fixture::seed_s6_lexical(&mut store).expect("s6");

    let problems = store
        .list_nodes(&NodeFilter {
            node_type: Some("problem".into()),
            cluster_id: Some(ids::CLUSTER.into()),
            limit: Some(10),
        })
        .expect("list");
    assert!(problems.len() >= 3);
    assert!(problems.iter().all(|n| n.node_type == "problem"));
    assert!(problems
        .iter()
        .all(|n| n.properties.get("cluster_id").and_then(|v| v.as_str()) == Some(ids::CLUSTER)));
}

#[test]
fn neighbors_outgoing_depends_on() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");

    let out = store
        .neighbors(ids::P1, Some(&["depends_on"]), true)
        .expect("neighbors");
    assert_eq!(out, vec![ids::P2.to_string()]);
}

#[test]
fn member_of_rejects_non_cluster_target() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed");

    let err = store
        .create_edge(ids::P1, ids::P2, "member_of", Default::default())
        .expect_err("member_of to problem");
    assert!(matches!(err, GraphError::Message(_)));
}
