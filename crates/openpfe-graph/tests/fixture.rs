//! Shared PFE seed fixture ([spike/program.md](../../../.dev/crates/openpfe-graph/spike/program.md)).

use openpfe_graph::{GrafeoGraphStore, GraphStore, Result};
use serde_json::json;

pub mod ids {
    pub const CLUSTER: &str = "c1111111-1111-4111-8111-111111111111";
    pub const CLUSTER_B: &str = "c2222222-2222-4222-8222-222222222222";
    pub const P1: &str = "p1111111-1111-4111-8111-111111111111";
    pub const P2: &str = "p2222222-2222-4222-8222-222222222222";
    pub const P3: &str = "p3333333-3333-4333-8333-333333333333";
    pub const P_DUP: &str = "p4444444-4444-4444-8444-444444444444";
    pub const P_UNRELATED: &str = "p5555555-5555-4555-8555-555555555555";
    pub const P_LEX_1: &str = "p6666666-6666-4666-8666-666666666666";
    pub const P_LEX_2: &str = "p7777777-7777-4777-8777-777777777777";
    pub const P_LEX_3: &str = "p8888888-8888-4888-8888-888888888888";
    pub const P_SEM_1: &str = "p9999999-9999-4999-8999-999999999999";
    pub const P_STRUCT_1: &str = "paaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
}

pub fn seed_core(store: &mut GrafeoGraphStore) -> Result<()> {
    store.upsert_node(
        "cluster",
        ids::CLUSTER,
        json!({ "title": "Core cluster", "status": "open" })
            .as_object()
            .cloned()
            .expect("object"),
    )?;
    store.upsert_node(
        "cluster",
        ids::CLUSTER_B,
        json!({ "title": "Peer cluster", "status": "open" })
            .as_object()
            .cloned()
            .expect("object"),
    )?;

    for (id, title, desc) in [
        (ids::P1, "Auth gateway", "First problem in chain"),
        (ids::P2, "Session store", "Second problem"),
        (ids::P3, "Token rotation", "Third problem"),
    ] {
        store.upsert_node(
            "problem",
            id,
            json!({
                "title": title,
                "description": desc,
                "status": "open",
                "cluster_id": ids::CLUSTER
            })
            .as_object()
            .cloned()
            .expect("object"),
        )?;
        store.create_edge(id, ids::CLUSTER, "member_of", Default::default())?;
    }

    store.create_edge(ids::P1, ids::P2, "depends_on", Default::default())?;
    store.create_edge(ids::P2, ids::P3, "depends_on", Default::default())?;

    store.create_edge(
        ids::CLUSTER,
        ids::CLUSTER_B,
        "interfaces",
        json!({
            "contract_body": "POST /api/v1/auth",
            "version": "1.0.0",
            "consumer_id": ids::CLUSTER,
            "provider_id": ids::CLUSTER_B
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;

    Ok(())
}

pub fn seed_s6_lexical(store: &mut GrafeoGraphStore) -> Result<()> {
    store.upsert_node(
        "problem",
        ids::P_DUP,
        json!({
            "title": "Auth gateway",
            "description": "Duplicate title for search",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_DUP, ids::CLUSTER, "member_of", Default::default())?;

    store.upsert_node(
        "problem",
        ids::P_UNRELATED,
        json!({
            "title": "Completely unrelated billing export",
            "description": "Should not match auth queries",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_UNRELATED, ids::CLUSTER, "member_of", Default::default())?;
    Ok(())
}

/// S6+ fixture nodes per [spike/program.md](../../../.dev/crates/openpfe-graph/spike/program.md).
pub fn seed_s6_plus(store: &mut GrafeoGraphStore) -> Result<()> {
    const LEX_DESC: &str =
        "Ensure API clients authenticate users before granting access to protected resources";

    store.upsert_node(
        "problem",
        ids::P_LEX_1,
        json!({
            "title": "Authenticate API users",
            "description": LEX_DESC,
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_LEX_1, ids::CLUSTER, "member_of", Default::default())?;
    store.create_edge(ids::P_LEX_1, ids::P2, "depends_on", Default::default())?;

    store.upsert_node(
        "problem",
        ids::P_LEX_2,
        json!({
            "title": "Authenticate API users",
            "description": "Duplicate title for lexical ranking",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_LEX_2, ids::CLUSTER, "member_of", Default::default())?;

    store.upsert_node(
        "problem",
        ids::P_LEX_3,
        json!({
            "title": "Quarterly billing reconciliation export",
            "description": "Unrelated to authentication",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_LEX_3, ids::CLUSTER, "member_of", Default::default())?;

    store.upsert_node(
        "problem",
        ids::P_SEM_1,
        json!({
            "title": "API user authentication",
            "description": "Verify user identity for API clients prior to allowing access to secured endpoints",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_SEM_1, ids::CLUSTER, "member_of", Default::default())?;

    store.upsert_node(
        "problem",
        ids::P_STRUCT_1,
        json!({
            "title": "Opaque widget serialization",
            "description": "Lexically unrelated; shares dependency hub",
            "status": "open",
            "cluster_id": ids::CLUSTER
        })
        .as_object()
        .cloned()
        .expect("object"),
    )?;
    store.create_edge(ids::P_STRUCT_1, ids::CLUSTER, "member_of", Default::default())?;
    store.create_edge(ids::P_STRUCT_1, ids::P2, "depends_on", Default::default())?;

    Ok(())
}

/// Synthetic `problem` nodes for S4 subgraph cap smoke (~800 in integration tests).
pub fn seed_bulk(store: &mut GrafeoGraphStore, n: usize) -> Result<()> {
    for i in 0..n {
        let id = format!("pbulk{i:04}-0000-4000-8000-{i:012x}");
        store.upsert_node(
            "problem",
            &id,
            json!({
                "title": format!("Synthetic problem {i}"),
                "description": format!("Bulk fixture node {i} for subgraph caps"),
                "status": "open",
                "cluster_id": ids::CLUSTER
            })
            .as_object()
            .cloned()
            .expect("object"),
        )?;
        store.create_edge(&id, ids::CLUSTER, "member_of", Default::default())?;
        if i > 0 {
            let prev = format!("pbulk{:04}-0000-4000-8000-{:012x}", i - 1, i - 1);
            store.create_edge(&id, &prev, "depends_on", Default::default())?;
        }
    }
    Ok(())
}
