use crate::AdjList;

use log::{debug, error, info, warn};
use num_traits::Num;
use num_traits::Zero;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

enum ShortestPathAlg {
    Dijksra,
    AStar,
    BellmanFord,
    Thorup,
}

// Structure for Dijkstra priority queue
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PrioQDist<T>
where
    T: Num,
{
    v: i32,
    dist: T,
}

impl<VertexProp, EdgeProp> AdjList<VertexProp, EdgeProp>
where
    VertexProp: Default + Clone,
    EdgeProp: Default + Clone,
{
    fn dijstra<W>(&mut self, start: i32, goal: Option<i32>, w: fn(EdgeProp) -> W) -> HashMap<i32, W>
    where
        W: Num + Ord + Copy,
    {
        // Set up
        let mut dist: HashMap<i32, W> = HashMap::new();
        let mut prev: HashMap<i32, i32> = HashMap::new();
        let mut prio_q: BinaryHeap<Reverse<PrioQDist<W>>> = BinaryHeap::new();
        let mut in_prio_q: HashSet<i32> = HashSet::new(); // Just to keep track of what is in the priority queue. There should be a way to do this without this

        dist.insert(start, W::zero());

        prio_q.push(Reverse(PrioQDist {
            v: start,
            dist: W::zero(),
        }));
        in_prio_q.insert(start);

        debug!("Starting dijkstra from {}", start);
        if !self.v.contains(&start) {
            warn!("Calling Dijkstra on a graph that does not contain the given start node");
            return dist;
        }

        'outer: while let Some(rp) = prio_q.pop() {
            let d: W = rp.0.dist;
            let v: i32 = rp.0.v;
            in_prio_q.remove(&v);
            println!("Popping value {} from queue", v);

            // Iterate over the nieghbors. Need to find a better way to do this.
            // This is just to use the scope to "give back" the reference to self. Im sure there is
            // a way to do this programatically
            let mut temp_vec: Vec<i32> = Vec::new();
            {
                let neighbors: &Vec<i32> = self.get_out_egdes(v).unwrap();
                for n_ref in neighbors {
                    temp_vec.push(*n_ref)
                }
            }
            for n in temp_vec {
                println!("Looking at neighbor {}->{}", v, n);
                // Get the distance to this neighbor

                let ep: EdgeProp = self.e_map.get(&(v, n)).unwrap().clone();
                let d_n = d + w(ep);

                // If we have hit our goal, break early
                if let Some(g) = goal {
                    if g == n {
                        dist.insert(n, d_n);
                        break 'outer;
                    }
                }

                // Check if we have found a shorter distance to the
                if !dist.contains_key(&n) || d_n < *dist.get(&n).unwrap() {
                    if in_prio_q.contains(&n) {
                        continue;
                    }
                    prio_q.push(Reverse(PrioQDist { v: n, dist: d_n }));
                    in_prio_q.insert(n);
                    prev.insert(n, v);
                    dist.insert(n, d_n);
                }
            }
        }
        return dist;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::DefaultEdgeProp;
    use crate::DefaultVertexProp;

    #[test]
    fn dijkstra_test() {
        let mut adjlist: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();
        adjlist.directed = true;
        //   1 ---(w:2)--- 2 ---(w:10)--- 3
        //   -             -              -
        // (w:3)         (w:2)          (w:3)
        //   -             -              -
        //   4 ---(w:7)--- 5 ---(w:20)--- 6

        adjlist.add_edge((1, 2), DefaultEdgeProp { weight: 2 });
        adjlist.add_edge((1, 4), DefaultEdgeProp { weight: 3 });
        adjlist.add_edge((4, 5), DefaultEdgeProp { weight: 7 });
        adjlist.add_edge((5, 6), DefaultEdgeProp { weight: 20 });
        adjlist.add_edge((2, 3), DefaultEdgeProp { weight: 10 });
        adjlist.add_edge((3, 6), DefaultEdgeProp { weight: 3 });

        let m: HashMap<i32, i32> = adjlist.dijstra(1, Some(6), |e: DefaultEdgeProp| e.weight);

        let mut expected_values: HashMap<i32, i32> = HashMap::new();
        expected_values.insert(1, 0);
        expected_values.insert(4, 3);
        expected_values.insert(2, 2);
        expected_values.insert(5, 4);
        expected_values.insert(3, 12);
        expected_values.insert(6, 15);

        for a in expected_values.iter() {
            assert_eq!(a.1, m.get(a.0).unwrap());
        }

        assert!(true);
    }
}
