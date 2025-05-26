impl<'a,E,M:DiscreteMetric<E,Q>,Q> Iterator for CloseSetIter<'a,E,M,Q>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a E,usize);
}
impl<'a,E,M,Q:Clone> Clone for CloseSetIter<'a,E,M,Q>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,E,M> IntoIterator for &'a BKTreeSet<E,M>{
	fn into_iter(self)->Self::IntoIter{
		SetIter{inner:self.inner.keys()}
	}
	type IntoIter=SetIter<'a,E>;
	type Item=&'a E;
}
impl<'a,E> Clone for SetIter<'a,E>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,E> ExactSizeIterator for SetIter<'a,E>{
	fn len(&self)->usize{self.inner.len()}
}
impl<'a,E> Iterator for SetIter<'a,E>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=&'a E;
}
impl<E:Clone> Clone for SetIntoIter<E>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<E,M:Default> Default for BKTreeSet<E,M>{
	fn default()->Self{Self::new(M::default())}
}
impl<E,M:Default+DiscreteMetric<E,E>> FromIterator<E> for BKTreeSet<E,M>{
	fn from_iter<I:IntoIterator<Item=E>>(iter:I)->Self{
		Self{inner:iter.into_iter().map(|e|(e,())).collect()}
	}
}
impl<E,M:DiscreteMetric<E,E>> Extend<E> for BKTreeSet<E,M>{
	fn extend<I:IntoIterator<Item=E>>(&mut self,iter:I){iter.into_iter().map(|e|self.insert(e)).for_each(|_|())}
}
impl<E,M> BKTreeSet<E,M>{//TODO other sterotypical set operations
	/// moves all elements from other into self, leaving other empty
	pub fn append<M2>(&mut self,other:&mut BKTreeSet<E,M2>) where M:DiscreteMetric<E,E>{self.inner.append(&mut other.inner)}
	/// gets the items whose distance is at most max distance from key
	pub fn close_iter<Q>(&self,key:Q,maxdistance:usize)->CloseSetIter<'_,E,M,Q> where M:DiscreteMetric<E,Q>{
		CloseSetIter{inner:self.inner.close_keys(key,maxdistance)}
	}
	/// returns the elements at most maxdistance from the key, sorted by distance
	pub fn close_sorted<'a,Q:?Sized>(&self,key:&Q,maxdistance:usize)->Vec<(&E,usize)> where M:DiscreteMetric<E,Q>{self.inner.close_sorted(key,maxdistance).into_iter().map(|(k,_v,d)|(k,d)).collect()}
	/// tests if the set contains an element within max distance of the key
	pub fn contains<Q:?Sized>(&self,key:&Q,maxdistance:usize)->bool where M:DiscreteMetric<E,Q>{self.inner.contains_key(key,maxdistance)}
	/// returns a reference to the element in the set that is closest to the key within max distance, or None if the set contains no element at most max distance from the given element. If there are multiple closest elements, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&E,usize)> where M:DiscreteMetric<E,Q>{self.inner.get_key_value(key,maxdistance).map(|(k,_v,d)|(k,d))}
	/// inserts
	pub fn insert(&mut self,value:E)->bool where M:DiscreteMetric<E,E>{self.inner.insert(value,()).is_none()}
	/// returns true if the set contains no elements
	pub fn is_empty(&self)->bool{self.inner.is_empty()}
	/// makes an iterator over the items
	pub fn iter(&self)->SetIter<'_,E>{
		SetIter{inner:self.inner.keys()}
	}
	/// returns the number of elements in the set
	pub fn len(&self)->usize{self.inner.len()}
	/// creates a new tree
	pub fn new(metric:M)->Self{
		Self{inner:BKTreeMap::new(metric)}
	}
	/// references the metric. avoid modifying the metric in a way that changes the distances because that will most likely cause unspecified incorrect behavior
	pub fn metric(&self)->&M{self.inner.metric()}
	/// removes an item from the tree. This particular tree type doesn't allow super efficient removal, so try to avoid using too much.
	pub fn remove<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->bool where M:DiscreteMetric<E,Q>+DiscreteMetric<E,E>{self.inner.remove_entry(key,maxdistance).is_some()}
	/// removes all the elements for which f returns false
	pub fn retain<F:FnMut(&E)->bool>(&mut self,mut f:F) where M:DiscreteMetric<E,E>{self.inner.retain(|k,_v|f(k))}
	/// removes an item from the tree. This particular tree type doesn't allow super efficient removal, so try to avoid using too much.
	pub fn take<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(E,usize)> where M:DiscreteMetric<E,Q>+DiscreteMetric<E,E>{self.inner.remove_entry(key,maxdistance).map(|(k,_v,d)|(k,d))}
}
impl<E,M> IntoIterator for BKTreeSet<E,M>{
	fn into_iter(self)->Self::IntoIter{
		SetIntoIter{inner:self.inner.into_keys()}
	}
	type IntoIter=SetIntoIter<E>;
	type Item=E;
}
impl<E> ExactSizeIterator for SetIntoIter<E>{
	fn len(&self)->usize{self.inner.len()}
}
impl<E> Iterator for SetIntoIter<E>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=E;
}
#[cfg(test)]
mod tests{
	#[test]
	fn two_clusters(){
		let mut tree=BKTreeSet::new(Cheb2D);

		tree.insert((0,0));
		tree.insert((1,0));
		tree.insert((0,1));
		tree.insert((1,1));

		tree.insert((10,10));
		tree.insert((11,10));
		tree.insert((10,11));
		tree.insert((11,11));

		let mut a:Vec<(isize,isize,usize)>=tree.close_iter((-1,-1),2).map(|((x,y),d)|(*x,*y,d)).collect();
		let mut b:Vec<(isize,isize,usize)>=tree.close_iter((12,12),2).map(|((x,y),d)|(*x,*y,d)).collect();
		a.sort_unstable();
		b.sort_unstable();
		assert_eq!(a,[(0,0,1),(0,1,2),(1,0,2),(1,1,2)]);
		assert_eq!(b,[(10,10,2),(10,11,2),(11,10,2),(11,11,1)]);
	}
	impl DiscreteMetric<(isize,isize),(isize,isize)> for Cheb2D{
		fn distance(&self,x:&(isize,isize),y:&(isize,isize))->usize{
			let ((xx,xy),(yx,yy))=(x,y);
			((xx-yx).abs() as usize).max((xy-yy).abs() as usize)
		}
	}
	#[derive(Clone,Copy,Debug,Default)]
	/// 2d integer chebyshev distance
	struct Cheb2D;
	use super::*;
}
#[derive(Clone,Debug)]
/// a set for quickly finding items separated by a small discrete distance, implemented as a thin wrapper over BKTreeMap
pub struct BKTreeSet<E,M>{inner:BKTreeMap<E,M,()>}
#[derive(Debug)]
/// iterator over the items close to some key
pub struct CloseSetIter<'a,E,M,Q>{inner:CloseKeyIter<'a,E,M,Q,()>}
#[derive(Debug)]
/// iterator over the items in the tree
pub struct SetIntoIter<E>{inner:IntoKeysIter<E,()>}
#[derive(Debug)]
/// iterator over the items in the tree
pub struct SetIter<'a,E>{inner:KeyIter<'a,E,()>}
use {
	crate::{
		DiscreteMetric,map::{BKTreeMap,CloseKeyIter,IntoKeysIter,KeyIter}
	},
	std::iter::{Extend,FromIterator}
};
