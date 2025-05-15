impl<K,M,V> BKTreeMap<K,M,V>{
	/// clears the map, removing all elements
	pub fn clear(&mut self){
		self.length=0;
		self.root=None;
	}
	/// gets the value whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<&V> where K:Borrow<Q>,M:DiscreteMetric<Q>{self.get_key_value(key,maxdistance).map(|(_k,v,_d)|v)}
	/// gets the key value pair and distance whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get_key_value<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&K,&V,usize)> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		fn explore<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a Node<K,V>)->Option<(&'a K,&'a V,usize)>{
			let distance=metric.distance(key,node.key.borrow());

			if distance==0{return Some((&node.key,&node.value,0))}
			let includecurrent=distance<=maxdistance;
			let nextnodes=node.connections.range(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).filter_map(|(_d,n)|explore(key,maxdistance,metric,n));
			nextnodes.chain(includecurrent.then_some((&node.key,&node.value,distance))).min_by_key(|(_k,_v,d)|*d)
		}
		let metric=&self.metric;
		let root=if let Some(r)=&self.root{r}else{return None};

		explore(key,maxdistance,metric,root)
	}
	/// gets the value whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get_mut<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<&mut V> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		fn explore<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a mut Node<K,V>)->Option<(&'a mut V,usize)>{
			let distance=metric.distance(key,node.key.borrow());

			if distance==0{return Some((&mut node.value,0))}
			let includecurrent=distance<=maxdistance;
			let nextnodes=node.connections.range_mut(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).filter_map(|(_d,n)|explore(key,maxdistance,metric,n));
			nextnodes.chain(includecurrent.then_some((&mut node.value,distance))).min_by_key(|(_v,d)|*d)
		}
		let metric=&self.metric;
		let root=if let Some(r)=&mut self.root{r}else{return None};

		explore(key,maxdistance,metric,root).map(|(n,_d)|n)
	}
	/// inserts a key-value pair into the map, returning the previous value at that key, or None if there was no previous value. If the old value is returned the key is not updated.
	pub fn insert(&mut self,key:K,value:V)->Option<V> where M:DiscreteMetric<K>{
		let metric=&self.metric;
		let mut node=if let Some(n)=self.root.as_mut(){
			n
		}else{
			self.length+=1;
			self.root=Some(Node::new(key,value));
			return None;
		};
		let (mut k,mut v)=(Some(key),Some(value));

		loop{
			let distance=if let Some(k)=&k{metric.distance(k,&node.key)}else{break};
			if distance==0{return Some(replace(&mut node.value,v.unwrap()))}
			node=node.connections.entry(distance).or_insert_with(||Node::new(k.take().unwrap(),v.take().unwrap()));
		}
		self.length+=1;
		None
	}
	/// returns true if the map contains no entries
	pub fn is_empty(&self)->bool{self.length==0}
	/// returns the number of entries in the map
	pub fn len(&self)->usize{self.length}
	/// creates a new tree
	pub fn new(metric:M)->Self{
		Self{length:0,metric,root:None}
	}
}
impl<K,V> Node<K,V>{
	/// creates a new node
	fn new(key:K,value:V)->Self{
		Self{connections:BTreeMap::new(),key,value}
	}
}
impl<T,M> BKTreeSet<T,M>{
	/// tests if the set contains an element within max distance of the key
	pub fn contains<Q:?Sized>(&self,key:&Q,maxdistance:usize)->bool where M:DiscreteMetric<Q>,T:Borrow<Q>{self.inner.get_key_value(key,maxdistance).is_some()}
	/// returns a reference to the element in the set that is closest to the key within max distance, or None if the set contains no element at most max distance from the given element. If there are multiple closest elements, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<&T> where M:DiscreteMetric<Q>,T:Borrow<Q>{self.inner.get_key_value(key,maxdistance).map(|(k,_v,_d)|k)}
	/// inserts
	pub fn insert(&mut self,value:T)->bool where M:DiscreteMetric<T>{self.inner.insert(value,()).is_none()}
	/// returns true if the set contains no entries
	pub fn is_empty(&self)->bool{self.inner.is_empty()}
	/// returns the number of entries in the set
	pub fn len(&self)->usize{self.inner.len()}
	/// creates a new tree
	pub fn new(metric:M)->Self{
		Self{inner:BKTreeMap::new(metric)}
	}
}
#[cfg(test)]
mod tests{
	#[test]
	fn insert_get_rectangle(){
		let mut map=BKTreeMap::new(Cheb2D);

		assert_eq!(map.insert((-1,-1),'A'),None);
		assert_eq!(map.insert((-1,2),'B'),None);
		assert_eq!(map.insert((1,-1),'C'),None);
		assert_eq!(map.insert((1,2),'D'),None);
		assert_eq!(map.insert((-1,2),'b'),Some('B'));
		assert_eq!(map.insert((1,2),'d'),Some('D'));
		assert_eq!(map.insert((1,-1),'c'),Some('C'));
		assert_eq!(map.insert((-1,-1),'a'),Some('A'));
		assert_eq!(map.len(),4);

		for n in 0..10{
			assert_eq!(map.get_key_value(&(-1,-1),n),Some((&(-1,-1),&'a',0)));
			assert_eq!(map.get_key_value(&(-1,2),n),Some((&(-1,2),&'b',0)));
			assert_eq!(map.get_key_value(&(1,-1),n),Some((&(1,-1),&'c',0)));
			assert_eq!(map.get_key_value(&(1,2),n),Some((&(1,2),&'d',0)));
		}

		assert_eq!(map.get_key_value(&(-1,-2),0),None);
		assert_eq!(map.get_key_value(&(-1,3),0),None);
		assert_eq!(map.get_key_value(&(2,-1),0),None);
		assert_eq!(map.get_key_value(&(2,1),0),None);

		assert_eq!(map.get_key_value(&(-1,-2),1),Some((&(-1,-1),&'a',1)));
		assert_eq!(map.get_key_value(&(-1,3),1),Some((&(-1,2),&'b',1)));
		assert_eq!(map.get_key_value(&(2,-1),1),Some((&(1,-1),&'c',1)));
		assert_eq!(map.get_key_value(&(2,2),1),Some((&(1,2),&'d',1)));
	}
	impl DiscreteMetric<(isize,isize)> for Cheb2D{
		fn distance(&self,x:&(isize,isize),y:&(isize,isize))->usize{
			let ((xx,xy),(yx,yy))=(x,y);
			((xx-yx).abs() as usize).max((xy-yy).abs() as usize)
		}
	}
	#[derive(Clone,Copy,Debug,Default)]
	/// 2d integer chebyshev distance for testing purposes
	struct Cheb2D;
	use super::*;
}
#[derive(Clone,Debug,Default)]
/// a tree for quickly finding items separated by a small discrete distance
pub struct BKTreeMap<K,M,V>{length:usize,metric:M,root:Option<Node<K,V>>}
#[derive(Clone,Debug,Default)]
/// a set for quickly finding items separated by a small discrete distance
pub struct BKTreeSet<T,M>{inner:BKTreeMap<T,M,()>}
/// a discrete distance metric. It should obey the usual axioms of a metric space
pub trait DiscreteMetric<T:?Sized>{
	/// computes the distance between two elements of the metric space
	fn distance(&self,x:&T,y:&T)->usize;
}
#[derive(Clone,Debug,Default)]
/// tree node
struct Node<K,V>{connections:BTreeMap<usize,Node<K,V>>,key:K,value:V}
use std::{
	borrow::Borrow,collections::BTreeMap,mem::replace
};
