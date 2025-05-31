impl<M:?Sized+DiscreteMetric<U,V>,U:?Sized,V:?Sized> DiscreteMetric<U,V> for &M{
	fn distance(&self,u:&U,v:&V)->usize{(**self).distance(u,v)}
}
impl<M:?Sized+DiscreteMetric<U,V>,U:?Sized,V:?Sized> DiscreteMetric<U,V> for &mut M{
	fn distance(&self,u:&U,v:&V)->usize{(**self).distance(u,v)}
}
#[cfg(test)]
mod tests{
	#[test]
	fn ceil_l2_set_radius_2() {
		// Points in the plane: L2 distances are ceil(sqrt(dx²+dy²))
		let mut tree = BKTreeSet::new(CeilL2::new());
		tree.insert([0i32, 0]);
		tree.insert([1, 1]);
		tree.insert([2, 2]);

		// From (0,0) with radius = 2, should get:
		//  (0,0) at d=0
		//  (1,1) at d=ceil(√2)=2
		//  but NOT (2,2) at d=ceil(√8)=3
		let mut found: Vec<([i32;2], usize)>=tree.close_iter([0, 0], 2).map(|(pt, d)| (*pt, d)).collect();
		found.sort_unstable();
		assert_eq!(found, vec![([0, 0], 0),  ([1, 1], 2)]);
	}
	#[test]
	fn hamming_set_radius_3() {
		// Byte‐string Hamming distance: count of differing positions
		let mut tree = BKTreeSet::new(Hamming::new_for(StrRef));
		tree.insert("karolin".to_string());
		tree.insert("kathrin".to_string());
		tree.insert("kerstin".to_string());

		// Hamming("karolin","kathrin") = 3,
		// Hamming("karolin","kerstin") = 3
		// radius = 2 should only return self
		let self_only: Vec<(String,usize)>=tree.close_iter("karolin".to_string(), 2).map(|(s, d)| (s.clone(), d)).collect();
		assert_eq!(self_only, vec![("karolin".to_string(), 0)]);

		// radius = 3 should also pick up both others
		let mut radius3: Vec<(String, usize)>=tree.close_iter("karolin".to_string(),3).map(|(s, d)| (s.clone(), d)).collect();
		radius3.sort_unstable();
		assert_eq!(radius3, vec![("karolin".to_string(),0),("kathrin".to_string(),3),("kerstin".to_string(),3)]);
	}
	#[test]
	fn levenshtein_set_various() {
		// Levenshtein distance: insertions/deletions/substitutions
		let mut tree = BKTreeSet::new(Levenshtein::new());
		tree.insert("kitten".to_string());
		tree.insert("sitting".to_string());
		tree.insert("bitten".to_string());
		tree.insert("mitten".to_string());

		// Known distances:
		//  kitten→bitten = 1  (k→b)
		//  kitten→mitten = 1  (k→m)
		//  kitten→sitting = 3 (kitten→sitten→sittin→sitting)
		let mut results: Vec<(String, usize)>=tree.close_iter("kitten".to_string(), 1).map(|(s, d)| (s.clone(), d)).collect();
		results.sort_unstable();
		assert_eq!(results,vec![("bitten".to_string(), 1), ("kitten".to_string(), 0),("mitten".to_string(),1)]);

		// radius = 3 should also include "sitting"
		let mut radius3: Vec<(String, usize)>=tree.close_iter("kitten".to_string(), 3).map(|(s, d)| (s.clone(), d)).collect();
		radius3.sort_unstable();
		assert_eq!(radius3, vec![("bitten".to_string(), 1),("kitten".to_string(), 0), ("mitten".to_string(), 1),("sitting".to_string(), 3)]);
	}
	use crate::{
		metrics::{CeilL2,Hamming,Levenshtein,StrRef},set::BKTreeSet
	};
}
/// provides a map data structure implemented using a bk tree
pub mod map;
/// builtin discrete metrics for use with bk tree structures
pub mod metrics;
/// provides a set data structure implemented using a bk tree
pub mod set;
/// a discrete distance metric. It should obey the usual axioms of a metric space. An invalid metric will probably cause unexpected behaviors
pub trait DiscreteMetric<U:?Sized,V:?Sized>{
	/// computes the distance between two elements of the metric space
	fn distance(&self,u:&U,v:&V)->usize;
}
pub use {map::BKTreeMap,set::BKTreeSet};
