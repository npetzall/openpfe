mod fixture;

use fixture::ids;
use openpfe_graph::{
    schema::DEFAULT_EMBEDDING_DIMENSIONS, FindSimilarDraft, GrafeoGraphStore, GraphStore,
};
use serde_json::json;
use tempfile::TempDir;

fn store_dir(dir: &TempDir) -> std::path::PathBuf {
    dir.path().join(".openpfe").join("graph").join("store")
}

fn open_seeded() -> (TempDir, GrafeoGraphStore) {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed core");
    fixture::seed_s6_plus(&mut store).expect("seed s6+");
    (dir, store)
}

#[test]
fn s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5() {
    let (_dir, store) = open_seeded();

    let draft = FindSimilarDraft {
        title: "Authenticate API users".into(),
        description: "Ensure API clients authenticate users before granting access".into(),
        cluster_id: Some(ids::CLUSTER.into()),
        embedding: None,
    };
    let hits = store.find_similar(&draft, 5).expect("find_similar");
    let top_ids: Vec<_> = hits.iter().map(|h| h.node_id.as_str()).collect();

    assert!(
        top_ids.iter().take(3).any(|id| *id == ids::P_LEX_2),
        "P-lex-2 in top 3: {top_ids:?}"
    );
    assert!(
        !top_ids.iter().take(5).any(|id| *id == ids::P_LEX_3),
        "P-lex-3 absent from top 5: {top_ids:?}"
    );
}

#[test]
fn s6plus_semantic_paraphrase_in_top5() {
    let (_dir, store) = open_seeded();

    let draft = FindSimilarDraft {
        title: "New auth concern".into(),
        description: "Verify user identity for API clients prior to allowing access to secured endpoints"
            .into(),
        cluster_id: None,
        embedding: None,
    };
    let hits = store.find_similar(&draft, 5).expect("find_similar");
    let top_ids: Vec<_> = hits.iter().map(|h| h.node_id.as_str()).collect();

    assert!(
        top_ids.iter().take(5).any(|id| *id == ids::P_SEM_1),
        "P-sem-1 in top 5: {top_ids:?}"
    );
}

#[test]
fn s6plus_structural_shared_deps_when_lexical_weak() {
    let (_dir, store) = open_seeded();

    let draft = FindSimilarDraft {
        title: "zzzz unrelated noise".into(),
        description: "xyzzy completely different tokens".into(),
        cluster_id: Some(ids::CLUSTER.into()),
        embedding: None,
    };
    let hits = store.find_similar(&draft, 5).expect("find_similar");
    let top_ids: Vec<_> = hits.iter().map(|h| h.node_id.as_str()).collect();

    assert!(
        top_ids.iter().take(5).any(|id| *id == ids::P_STRUCT_1),
        "P-struct-1 in top 5 via structural leg: {top_ids:?}"
    );
    let struct_hit = hits
        .iter()
        .find(|h| h.node_id == ids::P_STRUCT_1)
        .expect("struct hit");
    assert!(
        struct_hit.match_kinds.iter().any(|k| k == "structural"),
        "structural kind set: {:?}",
        struct_hit.match_kinds
    );
}

#[test]
fn s6plus_merged_hit_carries_multiple_kinds() {
    let (_dir, store) = open_seeded();

    let draft = FindSimilarDraft {
        title: "Authenticate API users".into(),
        description: "Ensure API clients authenticate users before granting access to protected resources"
            .into(),
        cluster_id: Some(ids::CLUSTER.into()),
        embedding: None,
    };
    let hits = store.find_similar(&draft, 10).expect("find_similar");

    let multi = hits
        .iter()
        .find(|h| h.match_kinds.len() >= 2)
        .expect("hit with multiple match_kinds");
    assert!(
        multi.snippet.is_some(),
        "snippet populated: {:?}",
        multi.snippet
    );
    assert!(multi.score > 0.0);
}

#[test]
fn s6plus_hybrid_when_embedding_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut store = GrafeoGraphStore::open(store_dir(&dir)).expect("open");
    fixture::seed_core(&mut store).expect("seed core");

    const P_VEC: &str = "pbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    let embedding: Vec<f32> = (0..DEFAULT_EMBEDDING_DIMENSIONS)
        .map(|i| if i % 7 == 0 { 1.0 } else { 0.01 })
        .collect();

    store
        .upsert_node(
            "problem",
            P_VEC,
            json!({
                "title": "Vector indexed authentication gateway",
                "description": "Hybrid search target with stored embedding",
                "status": "open",
                "cluster_id": ids::CLUSTER,
                "embedding": embedding
            })
            .as_object()
            .cloned()
            .expect("object"),
        )
        .expect("upsert vector node");
    store
        .create_edge(P_VEC, ids::CLUSTER, "member_of", Default::default())
        .expect("member_of");
    store.rebuild_vector_index().expect("rebuild vector");

    let draft = FindSimilarDraft {
        title: "Vector indexed authentication gateway".into(),
        description: "Hybrid search target with stored embedding".into(),
        cluster_id: Some(ids::CLUSTER.into()),
        embedding: Some(embedding),
    };
    let hits = store.find_similar(&draft, 5).expect("find_similar");
    let vec_hit = hits
        .iter()
        .find(|h| h.node_id == P_VEC)
        .expect("vector node in hits");

    assert!(
        vec_hit.match_kinds.iter().any(|k| k == "lexical"),
        "lexical kind: {:?}",
        vec_hit.match_kinds
    );
    assert!(
        vec_hit.match_kinds.iter().any(|k| k == "semantic"),
        "semantic kind: {:?}",
        vec_hit.match_kinds
    );
}
