//! Pressure solve + conductivity reinforcement step.

use super::network::PhysarumNetwork;
use std::collections::HashMap;
use std::hash::Hash;

impl<N: Eq + Hash + Copy + Ord> PhysarumNetwork<N> {
    /// One flow injection: set source/sink fluxes, solve pressures, leave
    /// conductivities unchanged. Call [`step`] afterward to reinforce.
    ///
    /// `sources` / `sinks` are `(node, flux)` pairs. Positive source flux
    /// injects; positive sink flux extracts. Totals need not balance — residual
    /// is absorbed by a weak grounding term so the Laplacian stays invertible.
    pub fn inject(&mut self, sources: &[(N, f64)], sinks: &[(N, f64)]) {
        let mut rhs: HashMap<N, f64> = HashMap::new();
        for (n, f) in sources {
            *rhs.entry(*n).or_default() += *f;
            self.pressures_mut().entry(*n).or_insert(0.0);
        }
        for (n, f) in sinks {
            *rhs.entry(*n).or_default() -= *f;
            self.pressures_mut().entry(*n).or_insert(0.0);
        }
        for n in self.pressures().keys().copied().collect::<Vec<_>>() {
            rhs.entry(n).or_insert(0.0);
        }
        self.solve_pressures(&rhs);
    }

    /// Advance conductivities by `dt` using fluxes from current pressures,
    /// then re-decay. Call after [`inject`] (or when pressures are already set).
    pub fn step(&mut self, dt: f64) {
        let reinforce = self.reinforce_fn();
        let decay = self.decay;
        let d_min = self.d_min;
        let pressures: HashMap<N, f64> = self.pressures().clone();

        let mut updates: Vec<((N, N), f64)> = Vec::new();
        for (&(a, b), edge) in self.edges() {
            let p_a = pressures.get(&a).copied().unwrap_or(0.0);
            let p_b = pressures.get(&b).copied().unwrap_or(0.0);
            let q = edge.conductivity / edge.length * (p_a - p_b);
            let growth = reinforce(q);
            let d_new = (edge.conductivity + dt * (growth - decay * edge.conductivity)).max(d_min);
            updates.push(((a, b), d_new));
        }
        for (k, d) in updates {
            if let Some(e) = self.edges_mut().get_mut(&k) {
                e.conductivity = d;
            }
        }
    }

    /// Jacobi iteration on the weighted Laplacian.
    fn solve_pressures(&mut self, rhs: &HashMap<N, f64>) {
        let nodes: Vec<N> = self.pressures().keys().copied().collect();
        if nodes.is_empty() {
            return;
        }

        // Adjacency: node → [(neighbor, D/L)]
        let mut adj: HashMap<N, Vec<(N, f64)>> = HashMap::new();
        for (&(a, b), edge) in self.edges() {
            let w = edge.conductivity / edge.length;
            adj.entry(a).or_default().push((b, w));
            adj.entry(b).or_default().push((a, w));
        }

        let ground = 1e-3; // weak diagonal grounding
        let mut p = self.pressures().clone();
        for _ in 0..64 {
            let mut next = p.clone();
            for &n in &nodes {
                let neighbors = adj.get(&n).map(|v| v.as_slice()).unwrap_or(&[]);
                let mut diag = ground;
                let mut sum = rhs.get(&n).copied().unwrap_or(0.0);
                for &(m, w) in neighbors {
                    diag += w;
                    sum += w * p.get(&m).copied().unwrap_or(0.0);
                }
                next.insert(n, sum / diag);
            }
            p = next;
        }
        *self.pressures_mut() = p;
    }
}

#[cfg(test)]
mod tests {
    use crate::PhysarumNetwork;

    /// Two food sources on a small mesh converge to a sparse high-conductivity path.
    #[test]
    fn two_food_sources_reinforce_direct_path() {
        let mut net = PhysarumNetwork::new().with_reinforce(|q| q.abs());
        // Diamond: 0—1—3 and 0—2—3, plus cross 1—2.
        net.add_edge(0u32, 1, 1.0);
        net.add_edge(1, 3, 1.0);
        net.add_edge(0, 2, 1.0);
        net.add_edge(2, 3, 1.0);
        net.add_edge(1, 2, 1.0);
        // Longer direct path 0—3 (should win if reinforced enough, or lose if length hurts).
        net.add_edge(0, 3, 2.5);

        for _ in 0..80 {
            net.inject(&[(0, 1.0)], &[(3, 1.0)]);
            net.step(0.2);
        }

        let direct = net.conductivity(0, 3);
        let via1 = net.conductivity(0, 1).min(net.conductivity(1, 3));
        let via2 = net.conductivity(0, 2).min(net.conductivity(2, 3));
        let cross = net.conductivity(1, 2);

        // Short paths reinforced; long direct and cross fade.
        assert!(
            via1 > direct || via2 > direct,
            "short path should beat long direct: via1={via1} via2={via2} direct={direct}"
        );
        assert!(
            cross < via1.max(via2),
            "cross edge should be weaker than primary path: cross={cross}"
        );
    }
}
