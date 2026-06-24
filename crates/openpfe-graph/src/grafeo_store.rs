//! Grafeo-backed [`GraphStore`](crate::graph_store::GraphStore).

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use grafeo::{EdgeId, GrafeoDB, NodeId, Value};
use serde_json::Map as JsonMap;

use crate::error::{GraphError, Result};
use crate::graph_store::GraphStore;
use crate::schema::{
    self, label, prop, rel, MIN_BM25_SCORE, STRUCTURAL_SCORE_SCALE, SUBGRAPH_EDGE_TYPES,
};
use crate::types::{
    Edge, FindSimilarDraft, Node, NodeFilter, SearchHit, SimilarHit, SubgraphLimits,
    SubgraphResult,
};

pub struct GrafeoGraphStore {
    db: GrafeoDB,
    db_path: PathBuf,
    id_to_node: HashMap<String, NodeId>,
    node_to_id: HashMap<NodeId, String>,
    problem_indexes_ready: bool,
}

impl GrafeoGraphStore {
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn node_count(&self) -> usize {
        self.db.node_count()
    }

    fn rebuild_id_index(&mut self) {
        self.id_to_node.clear();
        self.node_to_id.clear();
        for node in self.db.iter_nodes() {
            let uuid = node.properties.iter().find_map(|(key, value)| {
                if key.as_str() == prop::ID {
                    value_as_str(value).map(str::to_string)
                } else {
                    None
                }
            });
            if let Some(uuid) = uuid {
                self.id_to_node.insert(uuid.clone(), node.id);
                self.node_to_id.insert(node.id, uuid);
            }
        }
    }

    /// Create BM25 / HNSW indexes when the first `problem` nodes exist.
    fn ensure_problem_indexes(&mut self) -> Result<()> {
        if self.problem_indexes_ready {
            return Ok(());
        }
        let has_problem = self.db.iter_nodes().any(|n| {
            n.labels
                .iter()
                .any(|lb| lb.as_str() == label::PROBLEM)
        });
        if !has_problem {
            return Ok(());
        }

        let _ = self.db.create_text_index(label::PROBLEM, prop::TITLE);
        let _ = self.db.create_text_index(label::PROBLEM, prop::DESCRIPTION);
        let _ = self.db.create_vector_index(
            label::PROBLEM,
            prop::EMBEDDING,
            Some(schema::DEFAULT_EMBEDDING_DIMENSIONS),
            Some("cosine"),
            None,
            None,
            None,
        );

        self.problem_indexes_ready = true;
        Ok(())
    }

    fn build_undirected_adj(&self) -> HashMap<NodeId, Vec<(NodeId, EdgeId)>> {
        let allowed: HashSet<&str> = SUBGRAPH_EDGE_TYPES.iter().copied().collect();
        let mut adj: HashMap<NodeId, Vec<(NodeId, EdgeId)>> = HashMap::new();
        for edge in self.db.iter_edges() {
            if !allowed.contains(edge.edge_type.as_str()) {
                continue;
            }
            adj.entry(edge.src).or_default().push((edge.dst, edge.id));
            adj.entry(edge.dst).or_default().push((edge.src, edge.id));
        }
        adj
    }

    fn merge_text_scores(
        &self,
        query: &str,
        k: usize,
    ) -> Result<HashMap<String, f64>> {
        let mut by_id: HashMap<String, f64> = HashMap::new();
        for property in [prop::TITLE, prop::DESCRIPTION] {
            let hits = self.db.text_search(label::PROBLEM, property, query, k)?;
            for (nid, score) in hits {
                if score < MIN_BM25_SCORE {
                    continue;
                }
                if let Some(uuid) = self.node_to_id.get(&nid) {
                    by_id
                        .entry(uuid.clone())
                        .and_modify(|s| *s = s.max(score))
                        .or_insert(score);
                }
            }
        }
        Ok(by_id)
    }

    fn hybrid_scores(
        &self,
        query_text: &str,
        query_vector: Option<&[f32]>,
        k: usize,
    ) -> Result<HashMap<String, f64>> {
        let hits = self.db.hybrid_search(
            label::PROBLEM,
            prop::TITLE,
            prop::EMBEDDING,
            query_text,
            query_vector,
            k,
            None,
        )?;
        let mut out = HashMap::new();
        for (nid, score) in hits {
            if score < MIN_BM25_SCORE && query_vector.is_none() {
                continue;
            }
            if let Some(uuid) = self.node_to_id.get(&nid) {
                out.insert(uuid.clone(), score);
            }
        }
        Ok(out)
    }

    fn node_by_id(&self, nid: NodeId) -> Option<Node> {
        let node = self.db.get_node(nid)?;
        let node_type = node.labels.first()?.to_string();
        let id = self.node_to_id.get(&nid)?.clone();
        let mut properties = JsonMap::new();
        for (key, value) in &node.properties {
            let name = key.to_string();
            if name == prop::ID {
                continue;
            }
            properties.insert(name, value_to_json(value));
        }
        Some(Node {
            id,
            node_type,
            properties,
        })
    }

    fn edge_by_id(&self, eid: EdgeId) -> Option<Edge> {
        let edge = self.db.get_edge(eid)?;
        let src_id = self.node_to_id.get(&edge.src)?.clone();
        let dst_id = self.node_to_id.get(&edge.dst)?.clone();
        let mut properties = JsonMap::new();
        for (key, value) in &edge.properties {
            properties.insert(key.to_string(), value_to_json(value));
        }
        Some(Edge {
            src_id,
            dst_id,
            edge_type: edge.edge_type.to_string(),
            properties,
        })
    }
}

impl GraphStore for GrafeoGraphStore {
    /// Open or create the store under `store_dir` (`./.openpfe/graph/store/`).
    ///
    /// Creates `store_dir` when missing and opens `{store_dir}/store.grafeo`.
    fn open(store_dir: impl AsRef<Path>) -> Result<Self> {
        let store_dir = store_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&store_dir).map_err(|e| GraphError::msg(e.to_string()))?;
        let db_path = store_dir.join("store.grafeo");
        let db = GrafeoDB::open(&db_path)?;
        let mut store = Self {
            db,
            db_path,
            id_to_node: HashMap::new(),
            node_to_id: HashMap::new(),
            problem_indexes_ready: false,
        };
        store.rebuild_id_index();
        store.ensure_problem_indexes()?;
        Ok(store)
    }

    fn close(self) -> Result<()> {
        self.db.close()?;
        Ok(())
    }

    /// Upsert a node by openpfe UUID. Injects property `id`; label = `node_type`.
    ///
    /// v1 does not enforce `cluster_id` ↔ `member_of` consistency on upsert — callers
    /// should keep denormalized `cluster_id` aligned with graph edges.
    fn upsert_node(
        &mut self,
        node_type: &str,
        id: &str,
        mut properties: JsonMap<String, serde_json::Value>,
    ) -> Result<()> {
        properties.insert(
            prop::ID.to_string(),
            serde_json::Value::String(id.to_string()),
        );
        if let Some(&nid) = self.id_to_node.get(id) {
            for (key, json) in properties {
                if key == prop::ID {
                    continue;
                }
                self.db
                    .set_node_property(nid, &key, json_to_value(&json));
            }
        } else {
            let props: Vec<_> = properties
                .into_iter()
                .map(|(k, v)| (k, json_to_value(&v)))
                .collect();
            let nid = self.db.create_node_with_props(&[node_type], props);
            self.id_to_node.insert(id.to_string(), nid);
            self.node_to_id.insert(nid, id.to_string());
        }

        if node_type == label::PROBLEM {
            self.ensure_problem_indexes()?;
        }
        Ok(())
    }

    fn get_node(&self, id: &str) -> Result<Option<Node>> {
        let Some(&nid) = self.id_to_node.get(id) else {
            return Ok(None);
        };
        Ok(self.node_by_id(nid))
    }

    fn list_nodes(&self, filter: &NodeFilter) -> Result<Vec<Node>> {
        let label_filter = filter.node_type.as_deref();
        let limit = filter.limit.unwrap_or(usize::MAX);
        let nodes: Vec<_> = match label_filter {
            Some(l) => self
                .db
                .iter_nodes()
                .filter(|n| n.labels.iter().any(|lb| lb.as_str() == l))
                .collect(),
            None => self.db.iter_nodes().collect(),
        };
        let mut out = Vec::new();
        for node in nodes {
            if let Some(pfe) = self.node_by_id(node.id) {
                if let Some(ref cid) = filter.cluster_id
                    && pfe.properties.get(prop::CLUSTER_ID).and_then(|v| v.as_str())
                        != Some(cid.as_str())
                {
                    continue;
                }
                out.push(pfe);
                if out.len() >= limit {
                    break;
                }
            }
        }
        Ok(out)
    }

    fn delete_node(&mut self, id: &str) -> Result<bool> {
        let Some(nid) = self.id_to_node.remove(id) else {
            return Ok(false);
        };
        self.node_to_id.remove(&nid);
        Ok(self.db.delete_node(nid))
    }

    fn create_edge(
        &mut self,
        src_id: &str,
        dst_id: &str,
        edge_type: &str,
        properties: JsonMap<String, serde_json::Value>,
    ) -> Result<()> {
        let src = *self
            .id_to_node
            .get(src_id)
            .ok_or_else(|| GraphError::msg(format!("unknown node {src_id}")))?;
        let dst = *self
            .id_to_node
            .get(dst_id)
            .ok_or_else(|| GraphError::msg(format!("unknown node {dst_id}")))?;
        if edge_type == rel::MEMBER_OF {
            let node = self
                .db
                .get_node(dst)
                .ok_or_else(|| GraphError::msg(format!("unknown node {dst_id}")))?;
            let is_cluster = node
                .labels
                .iter()
                .any(|lb| lb.as_str() == label::CLUSTER);
            if !is_cluster {
                return Err(GraphError::msg(format!(
                    "member_of target must be type cluster, got {:?}",
                    node.labels.iter().map(|l| l.as_str()).collect::<Vec<_>>()
                )));
            }
        }
        if properties.is_empty() {
            self.db.create_edge(src, dst, edge_type);
        } else {
            let props: Vec<_> = properties
                .into_iter()
                .map(|(k, v)| (k, json_to_value(&v)))
                .collect();
            self.db.create_edge_with_props(src, dst, edge_type, props);
        }
        Ok(())
    }

    fn delete_edge(&mut self, src_id: &str, dst_id: &str, edge_type: &str) -> Result<bool> {
        let src = *self
            .id_to_node
            .get(src_id)
            .ok_or_else(|| GraphError::msg(format!("unknown node {src_id}")))?;
        let dst = *self
            .id_to_node
            .get(dst_id)
            .ok_or_else(|| GraphError::msg(format!("unknown node {dst_id}")))?;
        for edge in self.db.iter_edges() {
            if edge.src == src
                && edge.dst == dst
                && edge.edge_type.as_str() == edge_type
                && self.db.delete_edge(edge.id)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn neighbors(
        &self,
        id: &str,
        edge_types: Option<&[&str]>,
        outgoing: bool,
    ) -> Result<Vec<String>> {
        let nid = *self
            .id_to_node
            .get(id)
            .ok_or_else(|| GraphError::msg(format!("unknown node {id}")))?;
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for edge in self.db.iter_edges() {
            let other = if outgoing && edge.src == nid {
                Some(edge.dst)
            } else if !outgoing && edge.dst == nid {
                Some(edge.src)
            } else {
                None
            };
            let Some(other) = other else { continue };
            if let Some(types) = edge_types
                && !types.contains(&edge.edge_type.as_str())
            {
                continue;
            }
            if let Some(uuid) = self.node_to_id.get(&other)
                && seen.insert(uuid.clone())
            {
                out.push(uuid.clone());
            }
        }
        Ok(out)
    }

    fn subgraph(&self, cluster_id: &str, limits: SubgraphLimits) -> Result<SubgraphResult> {
        let root = *self
            .id_to_node
            .get(cluster_id)
            .ok_or_else(|| GraphError::msg(format!("unknown cluster {cluster_id}")))?;

        let adj = self.build_undirected_adj();
        let mut node_ids: HashSet<NodeId> = HashSet::new();
        let mut edge_ids: HashSet<EdgeId> = HashSet::new();
        let mut queue: VecDeque<(NodeId, u32)> = VecDeque::new();
        node_ids.insert(root);
        queue.push_back((root, 0));

        while let Some((current, depth)) = queue.pop_front() {
            if node_ids.len() >= limits.max_nodes {
                break;
            }
            if depth >= limits.max_depth {
                continue;
            }
            for &(next, eid) in adj.get(&current).into_iter().flatten() {
                if node_ids.len() >= limits.max_nodes {
                    break;
                }
                edge_ids.insert(eid);
                if node_ids.insert(next) && node_ids.len() < limits.max_nodes {
                    queue.push_back((next, depth + 1));
                }
            }
        }

        let nodes = node_ids
            .iter()
            .filter_map(|nid| self.node_by_id(*nid))
            .collect();
        let edges = edge_ids
            .iter()
            .filter_map(|eid| self.edge_by_id(*eid))
            .collect();
        Ok(SubgraphResult { nodes, edges })
    }

    fn validate_acyclic_deps(&self) -> Result<Vec<Vec<String>>> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for edge in self.db.iter_edges() {
            if edge.edge_type.as_str() != rel::DEPENDS_ON {
                continue;
            }
            let Some(from) = self.node_to_id.get(&edge.src) else {
                continue;
            };
            let Some(to) = self.node_to_id.get(&edge.dst) else {
                continue;
            };
            adj.entry(from.clone()).or_default().push(to.clone());
        }

        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        let mut path = Vec::new();

        for start in adj.keys().cloned().collect::<Vec<_>>() {
            if visited.contains(&start) {
                continue;
            }
            path.clear();
            stack.clear();
            if let Some(cycle) = dfs_cycle(&start, &adj, &mut visited, &mut stack, &mut path) {
                cycles.push(cycle);
            }
        }
        Ok(cycles)
    }

    fn search_problems(&self, query: &str, k: usize) -> Result<Vec<SearchHit>> {
        let by_id = self.merge_text_scores(query, k)?;
        let mut ranked: Vec<_> = by_id.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked.truncate(k);

        let mut out = Vec::new();
        for (id, score) in ranked {
            let title = self
                .get_node(&id)?
                .and_then(|n| {
                    n.properties
                        .get(prop::TITLE)
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                })
                .unwrap_or_default();
            out.push(SearchHit {
                node_id: id,
                title,
                score,
            });
        }
        Ok(out)
    }

    fn find_similar(&self, draft: &FindSimilarDraft, k: usize) -> Result<Vec<SimilarHit>> {
        let fetch = k.saturating_mul(4).max(10);
        let query_text = format!("{} {}", draft.title, draft.description);
        let query_text = query_text.trim();
        let mut merged: HashMap<String, SimilarHit> = HashMap::new();

        if let Some(ref embedding) = draft.embedding {
            let hybrid = self.hybrid_scores(query_text, Some(embedding), fetch)?;
            for (id, score) in hybrid {
                let entry =
                    merged.entry(id.clone()).or_insert_with(|| empty_similar(&id, self));
                entry.score = entry.score.max(score);
                push_kind(&mut entry.match_kinds, "lexical");
                push_kind(&mut entry.match_kinds, "semantic");
            }
            if !draft.description.is_empty() {
                for (id, score) in self.merge_text_scores(&draft.description, fetch)? {
                    let entry =
                        merged.entry(id.clone()).or_insert_with(|| empty_similar(&id, self));
                    entry.score = entry.score.max(score);
                    push_kind(&mut entry.match_kinds, "lexical");
                }
            }
        } else if !query_text.is_empty() {
            let lexical = self.merge_text_scores(query_text, fetch)?;
            let title_scores: HashMap<String, f64> = self
                .db
                .text_search(label::PROBLEM, prop::TITLE, &draft.title, fetch)?
                .into_iter()
                .filter_map(|(nid, score)| {
                    self.node_to_id
                        .get(&nid)
                        .map(|uuid| (uuid.clone(), score))
                })
                .collect();

            for (id, score) in lexical {
                let entry =
                    merged.entry(id.clone()).or_insert_with(|| empty_similar(&id, self));
                entry.score = entry.score.max(score);
                push_kind(&mut entry.match_kinds, "lexical");
            }

            if !draft.description.is_empty() {
                for (nid, desc_score) in self.db.text_search(
                    label::PROBLEM,
                    prop::DESCRIPTION,
                    &draft.description,
                    fetch,
                )? {
                    if desc_score < MIN_BM25_SCORE {
                        continue;
                    }
                    let Some(id) = self.node_to_id.get(&nid).cloned() else {
                        continue;
                    };
                    let title_score = title_scores.get(&id).copied().unwrap_or(0.0);
                    if desc_score > title_score * 1.5 {
                        let entry = merged
                            .entry(id.clone())
                            .or_insert_with(|| empty_similar(&id, self));
                        entry.score = entry.score.max(desc_score);
                        push_kind(&mut entry.match_kinds, "semantic");
                    }
                }
            }
        }

        if let Some(ref cluster_id) = draft.cluster_id {
            let peers = self.list_nodes(&NodeFilter {
                node_type: Some(label::PROBLEM.into()),
                cluster_id: Some(cluster_id.clone()),
                limit: None,
            })?;
            for node in peers {
                let cand_deps = depends_on_neighbor_set(self, &node.id)?;
                if cand_deps.is_empty() {
                    continue;
                }
                let s = structural_score(self, &node.id, cluster_id)?;
                if s > 0.0 {
                    let scaled = s * STRUCTURAL_SCORE_SCALE;
                    let entry = merged
                        .entry(node.id.clone())
                        .or_insert_with(|| empty_similar(&node.id, self));
                    entry.score = entry.score.max(scaled);
                    push_kind(&mut entry.match_kinds, "structural");
                }
            }
        }

        let mut hits: Vec<SimilarHit> = merged.into_values().collect();
        for hit in &mut hits {
            hit.match_kinds.sort();
            hit.match_kinds.dedup();
            if hit.snippet.is_none() {
                hit.snippet = self.get_node(&hit.node_id)?.and_then(|n| {
                    n.properties
                        .get(prop::DESCRIPTION)
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                });
            }
        }
        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(k);
        Ok(hits)
    }

    fn backup_full(&self, backup_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(backup_dir).map_err(|e| GraphError::msg(e.to_string()))?;
        self.db.backup_full(backup_dir)?;
        Ok(())
    }

    fn rebuild_text_indexes(&self) -> Result<()> {
        self.db.rebuild_text_index(label::PROBLEM, prop::TITLE)?;
        self.db.rebuild_text_index(label::PROBLEM, prop::DESCRIPTION)?;
        Ok(())
    }

    fn rebuild_vector_index(&self) -> Result<()> {
        self.db
            .rebuild_vector_index(label::PROBLEM, prop::EMBEDDING)?;
        Ok(())
    }
}

fn empty_similar(id: &str, store: &GrafeoGraphStore) -> SimilarHit {
    let title = store
        .get_node(id)
        .ok()
        .flatten()
        .and_then(|n| {
            n.properties
                .get(prop::TITLE)
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_default();
    SimilarHit {
        node_id: id.to_string(),
        title,
        score: 0.0,
        match_kinds: Vec::new(),
        snippet: None,
    }
}

fn push_kind(kinds: &mut Vec<String>, kind: &str) {
    if !kinds.iter().any(|k| k == kind) {
        kinds.push(kind.to_string());
    }
}

fn structural_score(
    store: &GrafeoGraphStore,
    candidate_id: &str,
    cluster_id: &str,
) -> Result<f64> {
    let cand_deps = depends_on_neighbor_set(store, candidate_id)?;
    if cand_deps.is_empty() {
        return Ok(0.0);
    }
    let peers = store.list_nodes(&NodeFilter {
        node_type: Some(label::PROBLEM.into()),
        cluster_id: Some(cluster_id.to_string()),
        limit: None,
    })?;
    let mut sum = 0.0;
    let mut n = 0u32;
    for peer in peers {
        if peer.id == candidate_id {
            continue;
        }
        let peer_deps = depends_on_neighbor_set(store, &peer.id)?;
        if peer_deps.is_empty() {
            continue;
        }
        sum += jaccard(&cand_deps, &peer_deps);
        n += 1;
    }
    Ok(if n == 0 { 0.0 } else { sum / f64::from(n) })
}

fn depends_on_neighbor_set(
    store: &GrafeoGraphStore,
    id: &str,
) -> Result<HashSet<String>> {
    let mut set = HashSet::new();
    for n in store.neighbors(id, Some(&[rel::DEPENDS_ON]), true)? {
        set.insert(n);
    }
    for n in store.neighbors(id, Some(&[rel::DEPENDS_ON]), false)? {
        set.insert(n);
    }
    Ok(set)
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let inter = a.intersection(b).count() as f64;
    let union = a.union(b).count() as f64;
    inter / union
}

fn dfs_cycle(
    node: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    visited.insert(node.to_string());
    stack.insert(node.to_string());
    path.push(node.to_string());

    for next in adj.get(node).into_iter().flatten() {
        if stack.contains(next) {
            let pos = path.iter().position(|p| p == next).unwrap_or(0);
            let mut cycle = path[pos..].to_vec();
            cycle.push(next.clone());
            return Some(cycle);
        }
        if !visited.contains(next)
            && let Some(cycle) = dfs_cycle(next, adj, visited, stack, path)
        {
            return Some(cycle);
        }
    }

    stack.remove(node);
    path.pop();
    None
}

fn value_as_str(v: &Value) -> Option<&str> {
    match v {
        Value::String(s) => Some(s.as_str()),
        _ => None,
    }
}

fn json_to_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int64(i)
            } else {
                Value::Float64(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::String(s.clone().into()),
        serde_json::Value::Array(arr) => {
            if arr.iter().all(|v| v.is_number()) {
                let vec: Vec<f32> = arr
                    .iter()
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect();
                if !vec.is_empty() {
                    return Value::Vector(Arc::from(vec.into_boxed_slice()));
                }
            }
            Value::String(json.to_string().into())
        }
        serde_json::Value::Object(_) => Value::String(json.to_string().into()),
    }
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Int64(i) => serde_json::json!(i),
        Value::Float64(f) => serde_json::json!(f),
        Value::String(s) => serde_json::Value::String(s.to_string()),
        Value::Vector(vec) => serde_json::Value::Array(
            vec.iter()
                .map(|f| serde_json::json!(*f))
                .collect::<Vec<_>>(),
        ),
        other => serde_json::Value::String(format!("{other:?}")),
    }
}
