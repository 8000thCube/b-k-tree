impl<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q,V> Iterator for CloseKeyIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(k,_v,d)|(k,d))}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a K,usize);
}
impl<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q,V> Iterator for CloseMapIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{
		let (maxdistance,metric)=(self.maxdistance,self.metric);
		let key=&self.key;
		let nodes=&mut self.nodes;

		while let Some(n)=nodes.pop(){
			let (k,v,next)=(&n.key,&n.value,&n.connections);
			let distance=metric.distance(k.borrow(),key);
			self.remaining-=1;

			nodes.extend(next.range(maxdistance.saturating_sub(distance)..=maxdistance.saturating_add(distance)).map(|(_r,n)|n));
			if distance<=maxdistance{return Some((k,v,distance))}
		}
		None
	}
	fn size_hint(&self)->(usize,Option<usize>){(0,Some(self.remaining))}
	type Item=(&'a K,&'a V,usize);
}
impl<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q,V> Iterator for CloseValIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v,d)|(v,d))}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a V,usize);
}
impl<'a,K,M,Q:Clone,V> Clone for CloseKeyIter<'a,K,M,Q,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,K,M,Q:Clone,V> Clone for CloseMapIter<'a,K,M,Q,V>{
	fn clone(&self)->Self{
		Self{key:self.key.clone(),maxdistance:self.maxdistance,metric:self.metric,nodes:self.nodes.clone(),remaining:self.remaining}
	}
	fn clone_from(&mut self,other:&Self){
		(self.key.clone_from(&other.key),self.nodes.clone_from(&other.nodes));
		(self.maxdistance,self.metric,self.remaining)=(other.maxdistance,other.metric,other.remaining);
	}
}
impl<'a,K,M,Q:Clone,V> Clone for CloseValIter<'a,K,M,Q,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,K,M,V> IntoIterator for &'a BKTreeMap<K,M,V>{
	fn into_iter(self)->Self::IntoIter{self.iter()}
	type IntoIter=MapIter<'a,K,V>;
	type Item=(&'a K,&'a V);
}
impl<'a,K,M,V> IntoIterator for &'a mut BKTreeMap<K,M,V>{
	fn into_iter(self)->Self::IntoIter{self.iter_mut()}
	type IntoIter=MapIterMut<'a,K,V>;
	type Item=(&'a K,&'a mut V);
}
impl<'a,K,V> Clone for KeyIter<'a,K,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,K,V> Clone for MapIter<'a,K,V>{
	fn clone(&self)->Self{
		Self{nodes:self.nodes.clone(),remaining:self.remaining}
	}
	fn clone_from(&mut self,other:&Self){
		self.nodes.clone_from(&other.nodes);
		self.remaining=other.remaining;
	}
}
impl<'a,K,V> Clone for ValIter<'a,K,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,K,V> ExactSizeIterator for KeyIter<'a,K,V>{
	fn len(&self)->usize{self.inner.len()}
}
impl<'a,K,V> ExactSizeIterator for MapIter<'a,K,V>{
	fn len(&self)->usize{self.remaining}
}
impl<'a,K,V> ExactSizeIterator for MapIterMut<'a,K,V>{
	fn len(&self)->usize{self.remaining}
}
impl<'a,K,V> ExactSizeIterator for ValIter<'a,K,V>{
	fn len(&self)->usize{self.inner.len()}
}
impl<'a,K,V> Iterator for KeyIter<'a,K,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(k,_v)|k)}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=&'a K;
}
impl<'a,K,V> Iterator for MapIter<'a,K,V>{
	fn next(&mut self)->Option<Self::Item>{
		let nodes=&mut self.nodes;
		let node=nodes.pop()?;
		let (k,v,next)=(&node.key,&node.value,&node.connections);
		self.remaining-=1;

		nodes.extend(next.values());
		Some((k,v))
	}
	fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
	type Item=(&'a K,&'a V);
}
impl<'a,K,V> Iterator for MapIterMut<'a,K,V>{
	fn next(&mut self)->Option<Self::Item>{
		let nodes=&mut self.nodes;
		let node=nodes.pop()?;
		let (k,v,next)=(&node.key,&mut node.value,&mut node.connections);
		self.remaining-=1;

		nodes.extend(next.values_mut());
		Some((k,v))
	}
	fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
	type Item=(&'a K,&'a mut V);
}
impl<'a,K,V> Iterator for ValIter<'a,K,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v)|v)}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=&'a V;
}
impl<'a,K,V> Iterator for ValIterMut<'a,K,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v)|v)}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=&'a mut V;
}
impl<'a,M:DiscreteMetric<Q>,Q,T:Borrow<Q>> Iterator for CloseSetIter<'a,M,Q,T>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a T,usize);
}
impl<'a,M,Q:Clone,T> Clone for CloseSetIter<'a,M,Q,T>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,T,M> IntoIterator for &'a BKTreeSet<T,M>{
	fn into_iter(self)->Self::IntoIter{
		SetIter{inner:self.inner.keys()}
	}
	type IntoIter=SetIter<'a,T>;
	type Item=&'a T;
}
impl<'a,T> Clone for SetIter<'a,T>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<'a,T> ExactSizeIterator for SetIter<'a,T>{
	fn len(&self)->usize{self.inner.len()}
}
impl<'a,T> Iterator for SetIter<'a,T>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=&'a T;
}
impl<K:Clone,V:Clone> Clone for IntoKeysIter<K,V>{
	fn clone(&self)->Self{//TODO this theoretically could avoid cloning the values
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<K:Clone,V:Clone> Clone for IntoValsIter<K,V>{
	fn clone(&self)->Self{//TODO this theoretically could avoid cloning the keys
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<K:Clone,V:Clone> Clone for MapIntoIter<K,V>{
	fn clone(&self)->Self{
		Self{nodes:self.nodes.clone(),remaining:self.remaining}
	}
	fn clone_from(&mut self,other:&Self){
		self.nodes.clone_from(&other.nodes);
		self.remaining=other.remaining;
	}
}
impl<K,M:Default+DiscreteMetric<K>,V> FromIterator<(K,V)> for BKTreeMap<K,M,V>{
	fn from_iter<I:IntoIterator<Item=(K,V)>>(iter:I)->Self{
		let mut map=Self::default();
		iter.into_iter().map(|(k,v)|map.insert(k,v)).for_each(|_|());
		map
	}
}
impl<K,M:Default,V> Default for BKTreeMap<K,M,V>{
	fn default()->Self{Self::new(M::default())}
}
impl<K,M,V> BKTreeMap<K,M,V>{
	/// clears the map, removing all elements
	pub fn clear(&mut self){
		self.length=0;
		self.root=None;
	}
	/// gets the key value pairs whose distance is at most max distance from key
	pub fn close_iter<Q>(&self,key:Q,maxdistance:usize)->CloseMapIter<'_,K,M,Q,V> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		CloseMapIter{key,maxdistance,metric:&self.metric,nodes:self.root.as_ref().into_iter().collect(),remaining:self.length}
	}
	/// gets the keys whose distance is at most max distance from key
	pub fn close_keys<Q>(&self,key:Q,maxdistance:usize)->CloseKeyIter<'_,K,M,Q,V> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		CloseKeyIter{inner:self.close_iter(key,maxdistance)}
	}
	/// gets the values whose keys are at most max distance from key
	pub fn close_values<Q>(&self,key:Q,maxdistance:usize)->CloseValIter<'_,K,M,Q,V> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		CloseValIter{inner:self.close_iter(key,maxdistance)}
	}
	/// returns the closest key value pairs at most maxdistance from the key, sorted by distance
	pub fn closest<'a,Q:?Sized>(&self,key:&Q,maxdistance:usize)->Vec<(&K,&V,usize)> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		fn explore<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a Node<K,V>,results:&mut Vec<(&'a K,&'a V,usize)>){
			let distance=metric.distance(key,node.key.borrow());

			if distance<=maxdistance{results.push((&node.key,&node.value,distance))}
			node.connections.range(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).for_each(|(_d,n)|explore(key,maxdistance,metric,n,results));
		}
		let metric=&self.metric;
		let root=if let Some(r)=&self.root{r}else{return Vec::new()};
		let mut results=Vec::with_capacity(10);

		explore(key,maxdistance,metric,root,&mut results);
		results.sort_unstable_by_key(|(_k,_v,d)|*d);
		results
	}
	/// returns the closest key value pairs at most maxdistance from the key, sorted by distance
	pub fn closest_mut<'a,Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Vec<(&K,&mut V,usize)> where K:Borrow<Q>,M:DiscreteMetric<Q>{
		fn explore<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a mut Node<K,V>,results:&mut Vec<(&'a K,&'a mut V,usize)>){
			let distance=metric.distance(key,node.key.borrow());

			if distance<=maxdistance{results.push((&node.key,&mut node.value,distance))}
			node.connections.range_mut(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).for_each(|(_d,n)|explore(key,maxdistance,metric,n,results));
		}
		let metric=&self.metric;
		let root=if let Some(r)=&mut self.root{r}else{return Vec::new()};
		let mut results=Vec::with_capacity(10);

		explore(key,maxdistance,metric,root,&mut results);
		results.sort_unstable_by_key(|(_k,_v,d)|*d);
		results
	}
	/// tests if a key at most maxdistance from the given key is in the map
	pub fn contains_key<Q:?Sized>(&self,key:&Q,maxdistance:usize)->bool where K:Borrow<Q>,M:DiscreteMetric<Q>{self.get_key_value(key,maxdistance).is_some()}
	/// gets the value whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&V,usize)> where K:Borrow<Q>,M:DiscreteMetric<Q>{self.get_key_value(key,maxdistance).map(|(_k,v,d)|(v,d))}
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
	/// inserts a key-value pair into the map, returning the previous value at that key, or None if there was no previous value. Keys are considered equal if the the distance between them is 0. If the old value is returned the key is not updated.
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
	/// makes an iterator over the keys
	pub fn into_keys(self)->IntoKeysIter<K,V>{
		IntoKeysIter{inner:self.into_iter()}
	}
	/// makes an iterator over the values
	pub fn into_values(self)->IntoValsIter<K,V>{
		IntoValsIter{inner:self.into_iter()}
	}
	/// returns true if the map contains no entries
	pub fn is_empty(&self)->bool{self.length==0}
	/// makes an iterator over the mappings
	pub fn iter(&self)->MapIter<'_,K,V>{
		MapIter{nodes:self.root.as_ref().into_iter().collect(),remaining:self.length}
	}
	/// makes an iterator over the mappings
	pub fn iter_mut(&mut self)->MapIterMut<'_,K,V>{
		MapIterMut{nodes:self.root.as_mut().into_iter().collect(),remaining:self.length}
	}
	/// makes an iterator over the keys
	pub fn keys(&self)->KeyIter<'_,K,V>{
		KeyIter{inner:self.iter()}
	}
	/// returns the number of entries in the map
	pub fn len(&self)->usize{self.length}
	/// creates a new tree
	pub fn new(metric:M)->Self{
		Self{length:0,metric,root:None}
	}
	/// removes the closest mapping whose key at most maxdistance from the given key. If there are multiple closest keys, exactly which is removed is unspecified
	pub fn remove<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(V,usize)> where K:Borrow<Q>,M:DiscreteMetric<K>+DiscreteMetric<Q>{self.remove_entry(key,maxdistance).map(|(_k,v,d)|(v,d))}
	/// removes the closest mapping whose key at most maxdistance from the given key. If there are multiple closest keys, exactly which is removed is unspecified
	pub fn remove_entry<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(K,V,usize)> where K:Borrow<Q>,M:DiscreteMetric<K>+DiscreteMetric<Q>{
		fn explore<'a,K:Borrow<Q>,M:DiscreteMetric<Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a mut Node<K,V>)->Option<(Result<(&'a mut BTreeMap<usize,Node<K,V>>,usize),&'a mut Node<K,V>>,usize)>{
			let distance=metric.distance(key,node.key.borrow());

			let includecurrent=distance<=maxdistance;
			let (nextnode,d,i)=if let Some(n)=node.connections.range_mut(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).filter_map(|(index,n)|{
				let (node,distance)=explore(key,maxdistance,metric,n)?;

				Some((node,distance,*index))
			}).min_by_key(|(_n,d,_i)|*d){
				n
			}else{
				return includecurrent.then_some((Err(node),distance))
			};
			Some(if distance<d{(Err(node),distance)}else if let Ok((_subtree,subindex))=nextnode{(Ok((&mut node.connections.get_mut(&i).unwrap().connections,subindex)),d)}else{(Ok((&mut node.connections,i)),d)})
		}
		fn restore_nodes<K,M:DiscreteMetric<K>,V>(nodes:BTreeMap<usize,Node<K,V>>,tree:&mut BKTreeMap<K,M,V>){
			nodes.into_values().for_each(|n|{
				let (key,value,next)=(n.key,n.value,n.connections);

				if tree.insert(key,value).is_none(){tree.length-=1}
				restore_nodes(next,tree);
			});
		}
		let metric=&self.metric;
		let root=self.root.as_mut()?;
		let (node,distance)=explore(key,maxdistance,metric,root)?;
		let node=match node{Err(_)=>self.root.take(),Ok((subtree,index))=>subtree.remove(&index)}.unwrap();
		let (key,value,torestore)=(node.key,node.value,node.connections);

		self.length-=1;
		restore_nodes(torestore,self);
		Some((key,value,distance))
	}
	/// makes an iterator over the values
	pub fn values(&self)->ValIter<'_,K,V>{
		ValIter{inner:self.iter()}
	}
	/// makes an iterator over the values
	pub fn values_mut(&mut self)->ValIterMut<'_,K,V>{
		ValIterMut{inner:self.iter_mut()}
	}
}
impl<K,M,V> IntoIterator for BKTreeMap<K,M,V>{
	fn into_iter(self)->Self::IntoIter{
		MapIntoIter{nodes:self.root.into_iter().collect(),remaining:self.length}
	}
	type IntoIter=MapIntoIter<K,V>;
	type Item=(K,V);
}
impl<K,V> ExactSizeIterator for IntoKeysIter<K,V>{
	fn len(&self)->usize{self.inner.len()}
}
impl<K,V> ExactSizeIterator for IntoValsIter<K,V>{
	fn len(&self)->usize{self.inner.len()}
}
impl<K,V> ExactSizeIterator for MapIntoIter<K,V>{
	fn len(&self)->usize{self.remaining}
}
impl<K,V> Iterator for IntoKeysIter<K,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(k,_v)|k)}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=K;
}
impl<K,V> Iterator for IntoValsIter<K,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v)|v)}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=V;
}
impl<K,V> Iterator for MapIntoIter<K,V>{
	fn next(&mut self)->Option<Self::Item>{
		let nodes=&mut self.nodes;
		let node=nodes.pop()?;
		let (k,v,next)=(node.key,node.value,node.connections);
		self.remaining-=1;

		nodes.extend(next.into_values());
		Some((k,v))
	}
	fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
	type Item=(K,V);
}
impl<K,V> Node<K,V>{
	/// creates a new node
	fn new(key:K,value:V)->Self{
		Self{connections:BTreeMap::new(),key,value}
	}
}
impl<T:Clone> Clone for SetIntoIter<T>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<T,M:Default> Default for BKTreeSet<T,M>{
	fn default()->Self{Self::new(M::default())}
}
impl<T,M:Default+DiscreteMetric<T>> FromIterator<T> for BKTreeSet<T,M>{
	fn from_iter<I:IntoIterator<Item=T>>(iter:I)->Self{
		Self{inner:iter.into_iter().map(|t|(t,())).collect()}
	}
}
impl<T,M> BKTreeSet<T,M>{
	/// gets the items whose distance is at most max distance from key
	pub fn close_iter<Q>(&self,key:Q,maxdistance:usize)->CloseSetIter<'_,M,Q,T> where T:Borrow<Q>,M:DiscreteMetric<Q>{
		CloseSetIter{inner:self.inner.close_keys(key,maxdistance)}
	}
	/// tests if the set contains an element within max distance of the key
	pub fn contains<Q:?Sized>(&self,key:&Q,maxdistance:usize)->bool where M:DiscreteMetric<Q>,T:Borrow<Q>{self.inner.contains_key(key,maxdistance)}
	/// returns a reference to the element in the set that is closest to the key within max distance, or None if the set contains no element at most max distance from the given element. If there are multiple closest elements, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&T,usize)> where M:DiscreteMetric<Q>,T:Borrow<Q>{self.inner.get_key_value(key,maxdistance).map(|(k,_v,d)|(k,d))}
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
	/// makes an iterator over the items
	pub fn iter(&self)->SetIter<'_,T>{
		SetIter{inner:self.inner.keys()}
	}
	/// removes an item from the tree
	pub fn remove<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(T,usize)> where T:Borrow<Q>,M:DiscreteMetric<Q>+DiscreteMetric<T>{self.inner.remove_entry(key,maxdistance).map(|(k,_v,d)|(k,d))}
}
impl<T,M> IntoIterator for BKTreeSet<T,M>{
	fn into_iter(self)->Self::IntoIter{
		SetIntoIter{inner:self.inner.into_keys()}
	}
	type IntoIter=SetIntoIter<T>;
	type Item=T;
}
impl<T> ExactSizeIterator for SetIntoIter<T>{
	fn len(&self)->usize{self.inner.len()}
}
impl<T> Iterator for SetIntoIter<T>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next()}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=T;
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
	#[test]
	fn insert_remove_rectangle(){
		let mut map=BKTreeMap::new(Cheb2D);

		assert_eq!(map.insert((-1,-1),'A'),None);
		assert_eq!(map.insert((-1,2),'B'),None);
		assert_eq!(map.insert((1,-1),'C'),None);
		assert_eq!(map.insert((1,2),'D'),None);

		assert_eq!(map.remove(&(-1,2),0),Some(('B',0)));
		assert_eq!(map.len(),3);
		assert_eq!(map.remove(&(1,2),0),Some(('D',0)));
		assert_eq!(map.len(),2);
		assert_eq!(map.remove(&(1,-1),0),Some(('C',0)));
		assert_eq!(map.len(),1);
		assert_eq!(map.remove(&(-1,-1),0),Some(('A',0)));
		assert_eq!(map.len(),0);

		for n in 0..10{
			assert_eq!(map.insert((-1,-1),'a'),None);
			assert_eq!(map.insert((-1,2),'b'),None);
			assert_eq!(map.insert((1,-1),'c'),None);
			assert_eq!(map.insert((1,2),'d'),None);
			assert_eq!(map.len(),4);

			assert_eq!(map.remove(&(-1,-1),n),Some(('a',0)));
			assert_eq!(map.get_key_value(&(-1,3),1),Some((&(-1,2),&'b',1)));
			assert_eq!(map.get_key_value(&(2,-1),1),Some((&(1,-1),&'c',1)));
			assert_eq!(map.remove(&(-1,2),n),Some(('b',0)));
			assert_eq!(map.len(),2);
			assert_eq!(map.remove(&(1,-1),n),Some(('c',0)));
			assert_eq!(map.len(),1);
			assert_eq!(map.remove(&(1,2),n),Some(('d',0)));
		}

		assert_eq!(map.insert((-1,-1),'a'),None);
		assert_eq!(map.insert((-1,2),'b'),None);
		assert_eq!(map.insert((1,-1),'c'),None);
		assert_eq!(map.insert((1,2),'d'),None);
		assert_eq!(map.len(),4);

		assert_eq!(map.remove(&(-1,-2),0),None);
		assert_eq!(map.remove(&(-1,3),0),None);
		assert_eq!(map.remove(&(2,-1),0),None);
		assert_eq!(map.remove(&(2,1),0),None);
		assert_eq!(map.len(),4);

		assert_eq!(map.get_key_value(&(-1,-2),1),Some((&(-1,-1),&'a',1)));
		assert_eq!(map.get_key_value(&(-1,3),1),Some((&(-1,2),&'b',1)));
		assert_eq!(map.get_key_value(&(2,-1),1),Some((&(1,-1),&'c',1)));
		assert_eq!(map.get_key_value(&(2,2),1),Some((&(1,2),&'d',1)));
	}
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

	#[test]
	fn test_insert_and_close_values() {
		let mut map = BKTreeMap::<i32, AbsDiff, &'static str>::default();
		map.insert(10, "ten");
		map.insert(20, "twenty");
		map.insert(15, "fifteen");

		// Close to 12 within distance 3 -> only 10 (dist=2) and 15 (dist=3)
		let mut results: Vec<(&&str, usize)> = map.close_values(12, 3).collect();
		results.sort_by_key(|&(_v, d)| d);
		assert_eq!(results, vec![(&"ten", 2), (&"fifteen", 3)]);
	}

	#[test]
	fn test_closest_sorted_output() {
		let mut map = BKTreeMap::<i32, AbsDiff, &'static str>::default();
		for &(k, v) in &[(5, "five"), (2, "two"), (8, "eight"), (6, "six")] {
			map.insert(k, v);
		}
		// closest returns sorted by distance
		let closest = map.closest(&6, 3);
		let distances: Vec<usize> = closest.iter().map(|&(_k, _v, d)| d).collect();
		assert_eq!(distances, vec![0, 1, 2]);
		let keys: Vec<i32> = closest.iter().map(|&(k, _v, _d)| *k).collect();
		assert_eq!(keys, vec![6, 5, 8]);
	}

	#[test]
	fn test_iterators_key_map_val() {
		let mut map = BKTreeMap::<i32, AbsDiff, i32>::default();
		for i in 1..=5 {
			map.insert(i, i * 10);
		}

		// Test key iterator
		let mut keys: Vec<i32> = map.keys().cloned().collect();
		keys.sort_unstable();
		assert_eq!(keys, vec![1, 2, 3, 4, 5]);

		// Test value iterator
		let mut vals: Vec<i32> = map.values().cloned().collect();
		vals.sort_unstable();
		assert_eq!(vals, vec![10, 20, 30, 40, 50]);

		// Test map iterator (key, value)
		let mut pairs: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
		pairs.sort_by_key(|&(k, _v)| k);
		assert_eq!(pairs, vec![(1,10), (2,20), (3,30), (4,40), (5,50)]);
	}

	#[test]
	fn test_clear_and_length() {
		let mut map = BKTreeMap::<i32, AbsDiff, ()>::new(AbsDiff);
		map.insert(1, ());
		map.insert(2, ());
		assert_eq!(map.len(), 2);
		map.clear();
		assert_eq!(map.len(), 0);
		// After clear, no close results
		assert_eq!(map.close_keys(1, 1).count(), 0);
	}
	#[test]
	fn test_from_iterator_and_default() {
		let data = vec![(0, "zero"), (3, "three"), (7, "seven")];
		let map: BKTreeMap<_, AbsDiff, _> = data.clone().into_iter().collect();
		// Using Default
		let default_map: BKTreeMap<i32, AbsDiff, &str> = BKTreeMap::default();
		assert_eq!(default_map.len(), 0);

		let mut collected: Vec<(i32, &str)> = map.iter().map(|(&k, &v)| (k, v)).collect();
		let mut expected = data;
		collected.sort_unstable_by_key(|&(k, _)| k);
		expected.sort_unstable_by_key(|&(k, _)| k);
		assert_eq!(collected, expected);
	}
	impl DiscreteMetric<(isize,isize)> for Cheb2D{
		fn distance(&self,x:&(isize,isize),y:&(isize,isize))->usize{
			let ((xx,xy),(yx,yy))=(x,y);
			((xx-yx).abs() as usize).max((xy-yy).abs() as usize)
		}
	}
	impl DiscreteMetric<i32> for AbsDiff{
		fn distance(&self,x:&i32,y:&i32)->usize{(*x-*y).abs() as usize}
	}
	#[derive(Clone,Copy,Debug,Default)]
	/// A simple absolute difference metric for integers
	struct AbsDiff;
	#[derive(Clone,Copy,Debug,Default)]
	/// 2d integer chebyshev distance
	struct Cheb2D;
	use super::*;
}
#[derive(Clone,Debug)]
/// a tree for quickly finding items separated by a small discrete distance
pub struct BKTreeMap<K,M,V>{length:usize,metric:M,root:Option<Node<K,V>>}
#[derive(Clone,Debug)]
/// a set for quickly finding items separated by a small discrete distance
pub struct BKTreeSet<T,M>{inner:BKTreeMap<T,M,()>}//TODO switch order of T and M because the non alphabeticalness is confusing
#[derive(Debug)]
/// iterator over keys close to some key
pub struct CloseKeyIter<'a,K,M,Q,V>{inner:CloseMapIter<'a,K,M,Q,V>}
#[derive(Debug)]
/// iterator over mappings close to some key
pub struct CloseMapIter<'a,K,M,Q,V>{key:Q,maxdistance:usize,metric:&'a M,nodes:Vec<&'a Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over the items close to some key
pub struct CloseSetIter<'a,M,Q,T>{inner:CloseKeyIter<'a,T,M,Q,()>}
#[derive(Debug)]
/// iterator over values with keys close to some key
pub struct CloseValIter<'a,K,M,Q,V>{inner:CloseMapIter<'a,K,M,Q,V>}
#[derive(Debug)]
/// iterator over the keys in the tree
pub struct IntoKeysIter<K,V>{inner:MapIntoIter<K,V>}
#[derive(Debug)]
/// iterator over the keys in the tree
pub struct IntoValsIter<K,V>{inner:MapIntoIter<K,V>}
#[derive(Debug)]
/// iterator over the keys in the tree
pub struct KeyIter<'a,K,V>{inner:MapIter<'a,K,V>}
#[derive(Debug)]
/// iterator over the mappings in the tree
pub struct MapIter<'a,K,V>{nodes:Vec<&'a Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over the mappings in the tree
pub struct MapIntoIter<K,V>{nodes:Vec<Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over the mappings in the tree
pub struct MapIterMut<'a,K,V>{nodes:Vec<&'a mut Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over the items in the tree
pub struct SetIntoIter<T>{inner:IntoKeysIter<T,()>}
#[derive(Debug)]
/// iterator over the items in the tree
pub struct SetIter<'a,T>{inner:KeyIter<'a,T,()>}
#[derive(Debug)]
/// iterator over the values in the tree
pub struct ValIter<'a,K,V>{inner:MapIter<'a,K,V>}
#[derive(Debug)]
/// iterator over the values in the tree
pub struct ValIterMut<'a,K,V>{inner:MapIterMut<'a,K,V>}
/// a discrete distance metric. It should obey the usual axioms of a metric space. An invalid metric will probably cause unexpected behaviors
pub trait DiscreteMetric<T:?Sized>{
	/// computes the distance between two elements of the metric space
	fn distance(&self,x:&T,y:&T)->usize;
}
#[derive(Clone,Debug,Default)]
/// tree node
struct Node<K,V>{connections:BTreeMap<usize,Node<K,V>>,key:K,value:V}
use std::{
	borrow::Borrow,collections::BTreeMap,iter::FromIterator,mem::replace
};
