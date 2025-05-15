impl CeilL2{
	/// creates a new metric that is the ceiling function of the euclidean metric
	pub fn new()->Self{Self::with_factor(1.0)}
	/// creates a new ceiling euclidean metric that rounds up to the given factor
	pub fn with_factor(factor:f64)->Self{
		Self{factor}
	}
}
impl Default for CeilL2{
	fn default()->Self{Self::with_factor(1.0)}
}
impl Hamming{
	/// creates a new hamming distance metric
	pub fn new()->Self{
		Self{_seal:()}
	}
}
impl LevIter for String{
	fn lev_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item<'a>=char;
	type Iter<'a>=Chars<'a> where Self:'a;
}
impl LevIter for str{
	fn lev_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item<'a>=char;
	type Iter<'a>=Chars<'a> where Self:'a;
}
impl Levenshtein{
	/// creates a new levenshtein distance metric
	pub fn new()->Self{
		Self{cache:Arc::new(Mutex::new(Vec::new()))}
	}
}
impl<E:Eq,const N:usize> LevIter for [E;N]{
	fn lev_iter(&self)->Self::Iter<'_>{self.iter()}
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
}
impl<E:Eq> LevIter for [E]{
	fn lev_iter(&self)->Self::Iter<'_>{self.iter()}
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
}
impl<E:Eq> LevIter for Vec<E>{
	fn lev_iter(&self)->Self::Iter<'_>{self.iter()}
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
}
impl<F:Add<Output=F>+Copy+Into<f64>+Mul<Output=F>,V:?Sized> DiscreteMetric<V> for CeilL2 where for<'a><&'a V as IntoIterator>::Item:Sub<Output=F>,for<'a>&'a V:IntoIterator{
	fn distance(&self,x:&V,y:&V)->usize{
		let (mut x,mut y)=(x.into_iter(),y.into_iter());
		let factor=self.factor;
		let mut r2=None;
		loop{
			let d=if let (Some(x),Some(y))=(x.next(),y.next()){x-y}else{break};
			let d2=d*d;
			r2=Some(if let Some(r2)=r2{d2+r2}else{d2})
		}
		let r2=r2.map(Into::into).unwrap_or(0.0);
		((r2.sqrt()/factor).ceil()*factor) as usize
	}
}
impl<T:?Sized+LevIter> LevenshteinDistance for T{
	fn levenshtein_distance(&self,distances:&mut Vec<usize>,other:&Self)->usize{
		let len=self.levenshtein_len();

		if len==0{return other.levenshtein_len()}
		for n in distances.len()..len{
			distances.push(if n>0{distances[n-1]+1}else{1});
		}
		for (k,y) in other.lev_iter().enumerate(){
			let mut diagonal=k;
			let mut left=k+1;
			let mut up;
			for (n,x) in self.lev_iter().enumerate(){
				up=distances[n];
				let d=(diagonal+if x==y{0}else{1}).min(left+1).min(up+1);
				(diagonal,left)=(up,d);
				distances[n]=d;
			}
		}
		*distances.last().unwrap()
	}
	fn levenshtein_len(&self)->usize{self.lev_iter().count()}
}
impl<V:LevenshteinDistance+?Sized> DiscreteMetric<V> for Levenshtein{
	fn distance(&self,x:&V,y:&V)->usize{
		let (mut x,mut y)=(x,y);
		let (mut xl,mut yl)=(x.levenshtein_len(),y.levenshtein_len());
		if x.levenshtein_len()>y.levenshtein_len(){
			swap(&mut x,&mut y);
			swap(&mut xl,&mut yl);
		}

		let mut cache=self.cache.try_lock();
		let mut distances:Vec<usize>=if let Ok(c)=&mut cache{take(&mut *c)}else{Vec::new()};

		distances.clear();
		distances.reserve(xl);
		let distance=x.levenshtein_distance(&mut distances,y);
		if let Ok(c)=&mut cache{**c=distances}
		distance
	}
}
#[cfg(test)]
mod tests{
	#[test]
	fn lev(){
		let metric=Levenshtein::new();
		assert_eq!(metric.distance("here","there"),1);
		assert_eq!(metric.distance("hi","hello"),4);
		assert_eq!(metric.distance("kitten","sitting"),3);
		assert_eq!(metric.distance("saturday","sunday"),3);
		assert_eq!(metric.distance("there","there"),0);
		assert_eq!(metric.distance("there","there's"),2);
	}

	use super::*;
}
#[derive(Clone,Debug)]
/// distance metric that is the usual euclidean metric, rounded up to a factor. behavior on length mismatch is currently unspecified
pub struct CeilL2{factor:f64}
#[derive(Clone,Debug,Default)]
/// hamming distance metric that is bitwise on integers and charwise on strings. behavior on length mismatch is currently unspecified
pub struct Hamming{_seal:()}
#[derive(Clone,Debug,Default)]
/// levenshtein distance metric for strings
pub struct Levenshtein{cache:Arc<Mutex<Vec<usize>>>}
/// trait for levenshtein distance compatibility
pub trait LevenshteinDistance{
	/// computes the distance given and other string its initial distances to substrings of self
	fn levenshtein_distance(&self,distances:&mut Vec<usize>,other:&Self)->usize;
	/// returns the string length to be used for lev distance purposes
	fn levenshtein_len(&self)->usize;
}
/// trait for computing levenshtein distance by iteration
pub trait LevIter{
	/// returns an iterator for comparing characters in a levenshtein distance. It shouldn't change length or sequence without self being mutated
	fn lev_iter(&self)->Self::Iter<'_>;
	/// the item type
	type Item<'a>:Eq where Self:'a;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item<'a>> where Self:'a;
}
use {
	crate::DiscreteMetric,
	std::{
		cmp::Eq,mem::{swap,take},ops::{Add,Mul,Sub},slice::Iter as SliceIter,str::Chars,sync::{Arc,Mutex}
	}
};
