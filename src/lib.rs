impl<M:?Sized+DiscreteMetric<U,V>,U:?Sized,V:?Sized> DiscreteMetric<U,V> for &M{
	fn distance(&self,u:&U,v:&V)->usize{(**self).distance(u,v)}
}
impl<M:?Sized+DiscreteMetric<U,V>,U:?Sized,V:?Sized> DiscreteMetric<U,V> for &mut M{
	fn distance(&self,u:&U,v:&V)->usize{(**self).distance(u,v)}
}
#[cfg(test)]
mod tests{
	#[test]
	fn ceil_l2_set_close_iter(){
		let mut tree=BKTreeSet::new(CeilL2::new());
		tree.insert([0i32,0]);
		tree.insert([1,1]);
		tree.insert([2,2]);
		let mut found:Vec<([i32;2],usize)>=tree.close_iter([0,0],2).map(|(pt,d)| (*pt,d)).collect();
		found.sort_unstable();
		assert_eq!(found,vec![([0,0],0),([1,1],2)]);
	}
	#[test]
	fn ceil_l2_set_close_sorted(){
		let mut tree=BKTreeSet::new(CeilL2::new());
		tree.insert([0i32,0]);
		tree.insert([1,1]);
		tree.insert([2,2]);
		let found:Vec<(&[i32;2],usize)>=tree.close_sorted(&[0,0],2);
		assert_eq!(found,vec![(&[0,0],0),(&[1,1],2)]);
	}
	#[test]
	fn hamming_set_close_iter(){
		let mut tree = BKTreeSet::new(Hamming::new_for(StrRef));
		tree.insert("karolin".to_string());
		tree.insert("kathrin".to_string());
		tree.insert("kerstin".to_string());
		// hamming("karolin", "kathrin") = 3
		// hamming("karolin", "kerstin") = 3
		let karolin:Vec<(String,usize)>=tree.close_iter("karolin".to_string(),2).map(|(s,d)|(s.clone(),d)).collect();
		assert_eq!(karolin,vec![("karolin".to_string(),0)]);
		let mut radius3:Vec<(String,usize)>=tree.close_iter("karolin".to_string(),3).map(|(s,d)|(s.clone(),d)).collect();
		radius3.sort_unstable();
		assert_eq!(radius3,vec![("karolin".to_string(),0),("kathrin".to_string(),3),("kerstin".to_string(),3)]);
	}
	#[test]
	fn hamming_set_close_sorted(){
		let mut tree = BKTreeSet::new(Hamming::new_for(StrRef));
		tree.insert("karolin".to_string());
		tree.insert("kathrin".to_string());
		tree.insert("kerstin".to_string());
		// hamming("karolin", "kathrin") = 3
		// hamming("karolin", "kerstin") = 3
		// hamming("katolin", "kathrin") = 2
		let karolin:Vec<(&String,usize)>=tree.close_sorted("karolin",2);
		assert_eq!(karolin,vec![(&"karolin".to_string(),0)]);
		let radius3:Vec<(&String,usize)>=tree.close_sorted("katolin",3);
		assert_eq!(radius3,vec![(&"karolin".to_string(),1),(&"kathrin".to_string(),2)]);
	}
	#[test]
	fn levenshtein_extend_retain(){
		let mut tree=BKTreeMap::new(Levenshtein::new());
		tree.extend([("bitten","bitten"),("calculate","mathematics"),("cat","pet"),("hello","greeting"),("hi","greeting"),("kat","name"),("kitten","kitten"),("linear","mathematics"),("mittens","mittens"),("sitting","sitting")]);
		assert_eq!(tree[&"bitten"],"bitten");
		assert_eq!(tree[&"calculate"],"mathematics");
		assert_eq!(tree[&"cat"],"pet");
		assert_eq!(tree[&"hello"],"greeting");
		assert_eq!(tree[&"hi"],"greeting");
		assert_eq!(tree[&"kat"],"name");
		assert_eq!(tree[&"kitten"],"kitten");
		assert_eq!(tree[&"linear"],"mathematics");
		assert_eq!(tree[&"mittens"],"mittens");
		assert_eq!(tree[&"sitting"],"sitting");
		tree.retain(|k,v|if k==v{
			false
		}else{
			*v=*k;
			true
		});
		assert_eq!(tree[&"calculate"],"calculate");
		assert_eq!(tree[&"cat"],"cat");
		assert_eq!(tree[&"hello"],"hello");
		assert_eq!(tree[&"hi"],"hi");
		assert_eq!(tree[&"kat"],"kat");
		assert_eq!(tree[&"linear"],"linear");
		assert_eq!(tree.get(&"cal",1),Some((&"cat",1)));
		assert_eq!(tree.get(&"calc",5),Some((&"cat",2)));
		assert_eq!(tree.get(&"calculat",5),Some((&"calculate",1)));
		assert_eq!(tree.get(&"hllo",2),Some((&"hello",1)));
		assert_eq!(tree.get(&"i",3),Some((&"hi",1)));
		assert_eq!(tree.get(&"line",3),Some((&"linear",2)));
	}
	#[test]
	fn levenshtein_set_close_iter(){
		let mut tree = BKTreeSet::new(Levenshtein::new());
		tree.insert("kitten".to_string());
		tree.insert("sitting".to_string());
		tree.insert("bitten".to_string());
		tree.insert("mitten".to_string());
		// levenshtein("kitten", "bitten") = 1
		// levenshtein("kitten", "mitten") = 1
		// levenshtein("kitten", "sitting") = 3
		let mut results:Vec<(String,usize)>=tree.close_iter("kitten".to_string(),1).map(|(s,d)|(s.clone(),d)).collect();
		results.sort_unstable();
		assert_eq!(results,vec![("bitten".to_string(),1),("kitten".to_string(),0),("mitten".to_string(),1)]);
		let mut radius3:Vec<(String,usize)>=tree.close_iter("kitten".to_string(),3).map(|(s,d)| (s.clone(),d)).collect();
		radius3.sort_unstable();
		assert_eq!(radius3,vec![("bitten".to_string(),1),("kitten".to_string(),0),("mitten".to_string(),1),("sitting".to_string(),3)]);
	}
	#[test]
	fn levenshtein_set_close_sorted(){
		let mut tree = BKTreeSet::new(Levenshtein::new());
		tree.insert("kitten".to_string());
		tree.insert("sitting".to_string());
		tree.insert("bitten".to_string());
		tree.insert("mittens".to_string());
		// levenshtein("kitten", "bitten") = 1
		// levenshtein("kitten", "mittens") = 2
		// levenshtein("kitten", "sitting") = 3
		let results:Vec<(&String,usize)>=tree.close_sorted("kitten",1);
		assert_eq!(results,vec![(&"kitten".to_string(),0),(&"bitten".to_string(),1)]);
		let results:Vec<(&String,usize)>=tree.close_sorted("kitten",3);
		assert_eq!(results,vec![(&"kitten".to_string(),0),(&"bitten".to_string(),1),(&"mittens".to_string(),2),(&"sitting".to_string(),3)]);
	}
	use crate::metrics::{CeilL2,Hamming,Levenshtein,StrRef};
	use super::*;
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
