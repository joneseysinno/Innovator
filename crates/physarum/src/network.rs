//! Core network storage.

use std::collections::HashMap;
use std::hash::Hash;

/// Undirected conductivity network over node ids `N`.
#[derive(Debug, Clone)]
pub struct PhysarumNetwork<N: Eq + Hash + Copy> {
    /// Symmetric edge table: canonical (min,max) key → (length, conductivity).
    edges: HashMap<(N, N), EdgeState>,
    /// Nodal pressure from the last `step` / `inject`.
    pressures: HashMap<N, f64>,
    /// Reinforcement: flux → conductivity growth rate. Default `|Q|`.
    reinforce: fn(f64) -> f64,
    /// Conductivity decay per unit time.
    pub decay: f64,
    /// Floor conductivity (never drop below).
    pub d_min: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EdgeState {
    pub(crate) length: f64,
    pub(crate) conductivity: f64,
}

impl<N: Eq + Hash + Copy> Default for PhysarumNetwork<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Eq + Hash + Copy> PhysarumNetwork<N> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            pressures: HashMap::new(),
            reinforce: |q| q.abs(),
            decay: 0.15,
            d_min: 1e-4,
        }
    }

    /// Override the reinforcement function (default `|Q|^1`).
    pub fn with_reinforce(mut self, f: fn(f64) -> f64) -> Self {
        self.reinforce = f;
        self
    }

    fn key(a: N, b: N) -> (N, N)
    where
        N: Ord,
    {
        if a <= b { (a, b) } else { (b, a) }
    }

    /// Insert or update an undirected edge with the given length.
    /// Initial conductivity is 1.0 when newly created.
    pub fn add_edge(&mut self, a: N, b: N, length: f64)
    where
        N: Ord,
    {
        if a == b {
            return;
        }
        let length = length.max(1e-9);
        let k = Self::key(a, b);
        self.edges
            .entry(k)
            .and_modify(|e| e.length = length)
            .or_insert(EdgeState {
                length,
                conductivity: 1.0,
            });
        self.pressures.entry(a).or_insert(0.0);
        self.pressures.entry(b).or_insert(0.0);
    }

    pub fn conductivity(&self, a: N, b: N) -> f64
    where
        N: Ord,
    {
        if a == b {
            return 0.0;
        }
        self.edges
            .get(&Self::key(a, b))
            .map(|e| e.conductivity)
            .unwrap_or(0.0)
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = N> + '_ {
        self.pressures.keys().copied()
    }

    pub(crate) fn edges_mut(&mut self) -> &mut HashMap<(N, N), EdgeState> {
        &mut self.edges
    }

    pub(crate) fn edges(&self) -> &HashMap<(N, N), EdgeState> {
        &self.edges
    }

    pub(crate) fn pressures_mut(&mut self) -> &mut HashMap<N, f64> {
        &mut self.pressures
    }

    pub(crate) fn pressures(&self) -> &HashMap<N, f64> {
        &self.pressures
    }

    pub(crate) fn reinforce_fn(&self) -> fn(f64) -> f64 {
        self.reinforce
    }
}
