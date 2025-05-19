ham_int!(i128,i16,i32,i64,i8,isize,u128,u16,u32,u64,u8,usize);
impl CeilL2<Coords>{
	/// creates a new metric that is the ceiling function of the euclidean metric
	pub fn new()->Self{Self::with_factor(1.0)}
	/// creates a new ceiling euclidean metric that rounds up to the given factor
	pub fn with_factor(factor:f64)->Self{Self::with_factor_for(factor,Coords)}
}
impl Hamming<Bits>{
	/// creates a new hamming distance metric
	pub fn new()->Self{Self::new_for(Bits)}
}
impl Levenshtein<Symbols>{
	/// creates a new levenshtein distance metric
	pub fn new()->Self{Self::new_for(Symbols)}
}
impl SymbolIter for String{
	fn symbol_count(&self)->usize{self.chars().count()}
	fn symbol_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item=char;
	type Iter<'a>=Chars<'a> where Self:'a;
}
impl SymbolIter for str{
	fn symbol_count(&self)->usize{self.chars().count()}
	fn symbol_iter(&self)->Self::Iter<'_>{self.chars()}
	type Item=char;
	type Iter<'a>=Chars<'a> where Self:'a;
}
impl<E:Add<Output=E>+Copy+Into<f64>+Mul<Output=E>+Sub<Output=E>,const N:usize> CoordIter for [E;N]{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<E:Add<Output=E>+Copy+Into<f64>+Mul<Output=E>+Sub<Output=E>> CoordIter for [E]{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<E:Add<Output=E>+Copy+Into<f64>+Mul<Output=E>+Sub<Output=E>> CoordIter for Vec<E>{
	fn coord_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<E:Copy+Eq,const N:usize> SymbolIter for [E;N]{
	fn symbol_count(&self)->usize{self.len()}
	fn symbol_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<E:Copy+Eq> SymbolIter for [E]{
	fn symbol_count(&self)->usize{self.len()}
	fn symbol_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<E:Copy+Eq> SymbolIter for Vec<E>{
	fn symbol_count(&self)->usize{self.len()}
	fn symbol_iter(&self)->Self::Iter<'_>{self.iter().copied()}
	type Item=E;
	type Iter<'a>=Copied<SliceIter<'a,E>> where Self:'a;
}
impl<T:?Sized+CoordIter> CoordIter for &T{
	fn coord_iter(&self)->Self::Iter<'_>{(*self).coord_iter()}
	type Item=T::Item;
	type Iter<'a>=T::Iter<'a> where Self:'a;
}
impl<T:Clone> Clone for Levenshtein<T>{
	fn clone(&self)->Self{Self::new_for(self._marker.clone())}
}
impl<T:Default> Default for CeilL2<T>{
	fn default()->Self{Self::with_factor_for(1.0,Default::default())}
}

impl<T:?Sized+SymbolIter> SymbolIter for &T{
	fn symbol_count(&self)->usize{(*self).symbol_count()}
	fn symbol_iter(&self)->Self::Iter<'_>{(*self).symbol_iter()}
	type Item=T::Item;
	type Iter<'a>=T::Iter<'a> where Self:'a;
}
impl<T> AsMut<Self> for CeilL2<T>{
	fn as_mut(&mut self)->&mut Self{self}
}
impl<T> AsMut<Self> for Hamming<T>{
	fn as_mut(&mut self)->&mut Self{self}
}
impl<T> AsMut<Self> for Levenshtein<T>{
	fn as_mut(&mut self)->&mut Self{self}
}
impl<T> AsRef<Self> for CeilL2<T>{
	fn as_ref(&self)->&Self{self}
}
impl<T> AsRef<Self> for Hamming<T>{
	fn as_ref(&self)->&Self{self}
}
impl<T> AsRef<Self> for Levenshtein<T>{
	fn as_ref(&self)->&Self{self}
}
impl<T> CeilL2<T>{
	/// creates a new metric that is the ceiling function of the euclidean metric
	pub fn new_for(t:T)->Self{Self::with_factor_for(1.0,t)}
	/// creates a new ceiling euclidean metric that rounds up to the given factor
	pub fn with_factor_for(factor:f64,t:T)->Self{
		Self{factor,_marker:t}
	}
}
impl<T> Hamming<T>{
	/// internal symbol based calculation
	fn _symbolic_distance<U:?Sized+SymbolIter,V:?Sized+SymbolIter<Item=U::Item>>(&self,x:&U,y:&V)->usize{
		let (mut x,mut y)=(x.symbol_iter(),y.symbol_iter());
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
	/// creates a new hamming distance metric
	pub fn new_for(t:T)->Self{
		Self{_marker:t}
	}
}
impl<T> Levenshtein<T>{
	/// internal symbol based calculation
	fn _symbolic_distance<U:?Sized+SymbolIter,V:?Sized+SymbolIter<Item=U::Item>>(&self,x:&U,y:&V)->usize{
		let (xl,yl)=(x.symbol_count(),y.symbol_count());
		if xl==0{return yl}
		if yl==0{return xl}

		let mut cache=self.cache.try_lock();
		let mut distances:Vec<usize>=if let Ok(c)=&mut cache{take(&mut *c)}else{Vec::new()};

		distances.clear();
		distances.reserve(xl);
		for n in 0..xl{distances.push(n+1)}
		for (k,y) in y.symbol_iter().enumerate(){
			let mut diagonal=k;
			let mut left=k+1;
			let mut up;
			for (n,x) in x.symbol_iter().enumerate(){
				up=distances[n];
				let d=(diagonal+if x==y{0}else{1}).min(left+1).min(up+1);
				(diagonal,left)=(up,d);
				distances[n]=d;
			}
		}
		let distance=*distances.last().unwrap();

		if let Ok(c)=&mut cache{**c=distances}
		distance
	}
	/// creates a new levenshtein distance metric
	pub fn new_for(t:T)->Self{
		Self{cache:Mutex::new(Vec::new()),_marker:t}
	}
}
impl<U:?Sized+AsRef<str>,V:?Sized+AsRef<str>> DiscreteMetric<U,V> for Hamming<StrRef>{
	fn distance(&self,u:&U,v:&V)->usize{self._symbolic_distance(u.as_ref(),v.as_ref())}
}
impl<U:?Sized+AsRef<str>,V:?Sized+AsRef<str>> DiscreteMetric<U,V> for Levenshtein<StrRef>{
	fn distance(&self,u:&U,v:&V)->usize{
		let (mut u,mut v)=(u.as_ref(),v.as_ref());

		if u.len()>v.len(){swap(&mut u,&mut v)}
		self._symbolic_distance(u,v)
	}
}
impl<U:?Sized+CoordIter,V:?Sized+CoordIter<Item=U::Item>> DiscreteMetric<U,V> for CeilL2{
	fn distance(&self,u:&U,v:&V)->usize{
		let (mut u,mut v)=(u.coord_iter(),v.coord_iter());
		let factor=self.factor;
		let mut r2=None;

		loop{
			let d=if let (Some(u),Some(v))=(u.next(),v.next()){u-v}else{break};
			let d2=d*d;
			r2=Some(if let Some(r2)=r2{d2+r2}else{d2})
		}
		let r2=r2.map(Into::into).unwrap_or(0.0);
		(r2.sqrt()/factor).ceil() as usize
	}
}
impl<U:?Sized+InherentDiscreteMetric<V>,V:?Sized> DiscreteMetric<U,V> for Inherent{
	fn distance(&self,u:&U,v:&V)->usize{u.distance_to(v)}
}
impl<U:?Sized+SymbolIter,V:?Sized+SymbolIter<Item=U::Item>> DiscreteMetric<U,V> for Hamming<Symbols>{
	fn distance(&self,u:&U,v:&V)->usize{self._symbolic_distance(u,v)}
}
impl<U:?Sized+SymbolIter,V:?Sized+SymbolIter<Item=U::Item>> DiscreteMetric<U,V> for Levenshtein<Symbols>{
	fn distance(&self,u:&U,v:&V)->usize{self._symbolic_distance(u,v)}
}
/// implements hamming distance for integers
macro_rules! ham_int{
	($($int:ty),*)=>($(
		impl DiscreteMetric<&$int,&$int> for Hamming<Bits>{
			fn distance(&self,x:&&$int,y:&&$int)->usize{(*x^*y).count_ones() as usize}
		}
		impl DiscreteMetric<&$int,$int> for Hamming<Bits>{
			fn distance(&self,x:&&$int,y:&$int)->usize{(*x^*y).count_ones() as usize}
		}
		impl DiscreteMetric<$int,&$int> for Hamming<Bits>{
			fn distance(&self,x:&$int,y:&&$int)->usize{(*x^*y).count_ones() as usize}
		}
		impl DiscreteMetric<$int,$int> for Hamming<Bits>{
			fn distance(&self,x:&$int,y:&$int)->usize{(*x^*y).count_ones() as usize}
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
		let m=Hamming::new_for(StrRef);
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
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
/// marker for bitwise distance computation
pub struct Bits;
#[derive(Clone,Debug)]
/// distance metric that is the usual euclidean metric, divided by a scale factor and rounded up. behavior on length mismatch is currently unspecified
pub struct CeilL2<T=Coords>{factor:f64,_marker:T}
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
/// marker for coordinate based distance computation
pub struct Coords;
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
/// metric for types that have an inherent discrete distance metric
pub struct Inherent;
#[derive(Clone,Debug,Default)]
/// hamming distance metric that is bitwise on integers and charwise on strings. behavior on length mismatch is currently unspecified
pub struct Hamming<T=Bits>{_marker:T}
#[derive(Debug,Default)]
/// levenshtein distance metric for strings
pub struct Levenshtein<T=Symbols>{cache:Mutex<Vec<usize>>,_marker:T}
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
/// marker for usage of string references for edit distance computation
pub struct StrRef;
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
/// marker for usage of the symbol iter trait for edit distance computation
pub struct Symbols;
/// helper trait for computing edit distances by iteration by iteration over primitives
pub trait CoordIter{
	/// returns an iterator over coordinates
	fn coord_iter(&self)->Self::Iter<'_>;
	/// the item type
	type Item:Add<Output=Self::Item>+Copy+Into<f64>+Mul<Output=Self::Item>+Sub<Output=Self::Item>;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item> where Self:'a;
}
/// allows metrics inherent to type
pub trait InherentDiscreteMetric<V:?Sized>{
	/// finds the distance between self and v
	fn distance_to(&self,v:&V)->usize;
}
/// helper trait for computing edit distances by iteration
pub trait SymbolIter{
	/// returns the length of the sequence
	fn symbol_count(&self)->usize;
	/// returns an iterator for comparing characters for an edit distance. Since the computation may require multiple iterations, the returned iterator shouldn't change length or sequence without self being mutated
	fn symbol_iter(&self)->Self::Iter<'_>;
	/// the item type
	type Item:Eq;
	/// the iterator type
	type Iter<'a>:Iterator<Item=Self::Item> where Self:'a;
}
use {
	crate::DiscreteMetric,
	ham_int,
	std::{
		cmp::Eq,iter::Copied,mem::{swap,take},ops::{Add,Mul,Sub},slice::Iter as SliceIter,str::Chars,sync::Mutex
	}
};
