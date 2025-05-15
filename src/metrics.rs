ham_int!(i128,i16,i32,i64,i8,isize,u128,u16,u32,u64,u8,usize);
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
impl HamIter for String{
	fn ham_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item<'a>=char;
	type Iter<'a>=Chars<'a> where Self:'a;
}
impl HamIter for str{
	fn ham_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item<'a>=char;
	type Iter<'a>=Chars<'a> where Self:'a;
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
impl<D:Add<Output=D>+Copy+Into<f64>+Mul<Output=D>,E,const N:usize> CoordIter for [E;N] where for<'a>&'a E:Sub<Output=D>{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter()}
	type D=D;
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
}
impl<D:Add<Output=D>+Copy+Into<f64>+Mul<Output=D>,E> CoordIter for [E] where for<'a>&'a E:Sub<Output=D>{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter()}
	type D=D;
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
}
impl<D:Add<Output=D>+Copy+Into<f64>+Mul<Output=D>,E> CoordIter for Vec<E> where for<'a>&'a E:Sub<Output=D>{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter()}
	type D=D;
	type Item<'a>=&'a E where E:'a;
	type Iter<'a>=SliceIter<'a,E> where Self:'a;
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
impl<T:?Sized+CoordIter> CoordIter for &T{
	fn coord_iter(&self)->Self::Iter<'_>{(*self).coord_iter()}
	type D=T::D;
	type Item<'a>=T::Item<'a> where Self:'a;
	type Iter<'a>=T::Iter<'a> where Self:'a;
}
impl<T:?Sized+CoordIter> CeilL2Distance for T{
	fn ceil_l2_distance(&self,factor:f64,other:&Self)->usize{
		let (mut x,mut y)=(self.coord_iter(),other.coord_iter());
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
impl<T:?Sized+HamIter> HamIter for &T{
	fn ham_iter(&self)->Self::Iter<'_>{(*self).ham_iter()}
	type Item<'a>=T::Item<'a> where Self:'a;
	type Iter<'a>=T::Iter<'a> where Self:'a;
}
impl<T:?Sized+HamIter> HammingDistance for T{
	fn hamming_distance(&self,other:&Self)->usize{
		let (mut x,mut y)=(self.ham_iter(),other.ham_iter());
		let mut count=0;

		loop {
			match (x.next(),y.next()){
				(Some(a),Some(b))=>if a!=b{count+=1},
				(Some(_), None) | (None, Some(_))=>count+=1,
				(None, None)=>break,
			}
		}
		count
	}
}
impl<T:?Sized+LevIter> LevIter for &T{
	fn lev_iter(&self)->Self::Iter<'_>{(*self).lev_iter()}
	type Item<'a>=T::Item<'a> where Self:'a;
	type Iter<'a>=T::Iter<'a> where Self:'a;
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
impl<V:CeilL2Distance+?Sized> DiscreteMetric<V> for CeilL2{
	fn distance(&self,x:&V,y:&V)->usize{x.ceil_l2_distance(self.factor,y)}
}
impl<V:HammingDistance+?Sized> DiscreteMetric<V> for Hamming{
	fn distance(&self,x:&V,y:&V)->usize{x.hamming_distance(y)}
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
/// implements hamming distance for integers
macro_rules! ham_int{
	($($int:ty),*)=>($(
		impl HammingDistance for &$int{
			fn hamming_distance(&self,other:&Self)->usize{(*self^*other).count_ones() as usize}
		}
		impl HammingDistance for $int{
			fn hamming_distance(&self,other:&Self)->usize{(self^other).count_ones() as usize}
		}
	)*)
}
#[cfg(test)]
mod tests{
	#[test]
	fn ceil_l2_2d_int(){
		let m=CeilL2::new();
		assert_eq!(m.distance(&[1,2],&[2,1]),2);
		assert_eq!(m.distance(&[3,3],&[3,4]),1);
		assert_eq!(m.distance(&[0,0],&[3,4]),5);
	}
	#[test]
	fn ham_int(){
		let m=Hamming::new();
		assert_eq!(m.distance(&1,&0),1);
		assert_eq!(m.distance(&0b1011101,&0b1001001),2);
		assert_eq!(m.distance(&9999,&9999),0);
		assert_eq!(m.distance(&-1_i32,&0_i32),32);
	}
	#[test]
	fn ham_string() {
		let m=Hamming::new();
		assert_eq!(m.distance("rust","rust"),0);
		assert_eq!(m.distance("karolin","kathrin"),3);
		assert_eq!(m.distance("1011101","1001001"),2);
		assert_eq!(m.distance("2173896","2233796"),3);
	}
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
/// trait for computing coordinate based distances by iteration over primitives
pub trait CoordIter{
	/// returns an iterator over coordinates
	fn coord_iter(&self)->Self::Iter<'_>;
	/// the coordinate displacement type
	type D:Add<Output=Self::D>+Copy+Into<f64>+Mul<Output=Self::D>;
	/// the item type
	type Item<'a>:Sub<Output=Self::D> where Self:'a;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item<'a>> where Self:'a;
}
/// trait for computing the hamming distance by iteration
pub trait HamIter{
	/// returns an iterator for comparing characters in a hamming distance
	fn ham_iter(&self)->Self::Iter<'_>;
	/// the item type
	type Item<'a>:Eq where Self:'a;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item<'a>> where Self:'a;
}
/// trait for ceil l2 distance compatibility
pub trait CeilL2Distance{
	/// computes the ceiling l2 distance
	fn ceil_l2_distance(&self,factor:f64,other:&Self)->usize;
}
/// trait for hamming distance compatibility
pub trait HammingDistance{
	/// computes the distance given and other string
	fn hamming_distance(&self,other:&Self)->usize;
}
/// trait for computing levenshtein distance by iteration
pub trait LevIter{
	/// returns an iterator for comparing characters in a levenshtein distance. Since this distance requires multiple iterations, it shouldn't change length or sequence without self being mutated
	fn lev_iter(&self)->Self::Iter<'_>;
	/// the item type
	type Item<'a>:Eq where Self:'a;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item<'a>> where Self:'a;
}
/// trait for levenshtein distance compatibility
pub trait LevenshteinDistance{
	/// computes the distance between self and other string given its initial distances to substrings of self
	fn levenshtein_distance(&self,distances:&mut Vec<usize>,other:&Self)->usize;
	/// returns the string length to be used for levenshtein distance purposes
	fn levenshtein_len(&self)->usize;
}
use {
	crate::DiscreteMetric,
	ham_int,
	std::{
		cmp::Eq,mem::{swap,take},ops::{Add,Mul,Sub},slice::Iter as SliceIter,str::Chars,sync::{Arc,Mutex}
	}
};
