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
