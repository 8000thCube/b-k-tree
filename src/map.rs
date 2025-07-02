impl<'a,K,M:DiscreteMetric<K,Q>,Q,V> Iterator for CloseKeyIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(k,_v,d)|(k,d))}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a K,usize);
}
impl<'a,K,M:DiscreteMetric<K,Q>,Q,V> Iterator for CloseMapIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{
		let (maxdistance,metric)=(self.maxdistance,self.metric);
		let key=&self.key;
		let nodes=&mut self.nodes;

		while let Some(n)=nodes.pop(){
			let (k,v,next)=(&n.key,&n.value,&n.connections);
			let distance=metric.distance(k,key);
			self.remaining-=1;

			nodes.extend(next.range(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).map(|(_r,n)|n));
			if distance<=maxdistance{return Some((k,v,distance))}
		}
		None
	}
	fn size_hint(&self)->(usize,Option<usize>){(0,Some(self.remaining))}
	type Item=(&'a K,&'a V,usize);
}
impl<'a,K,M:DiscreteMetric<K,Q>,Q,V> Iterator for CloseMapIterMut<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{
		let (maxdistance,metric)=(self.maxdistance,self.metric);
		let key=&self.key;
		let nodes=&mut self.nodes;

		while let Some(n)=nodes.pop(){
			let (k,v,next)=(&n.key,&mut n.value,&mut n.connections);
			let distance=metric.distance(k,key);
			self.remaining-=1;

			nodes.extend(next.range_mut(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).map(|(_r,n)|n));//TODO remaining can have a tighter bound by subtracting one for every node not examined here, or the number of total sub nodes if that becomes tracked
			if distance<=maxdistance{return Some((k,v,distance))}
		}
		None
	}
	fn size_hint(&self)->(usize,Option<usize>){(0,Some(self.remaining))}
	type Item=(&'a K,&'a mut V,usize);
}
impl<'a,K,M:DiscreteMetric<K,Q>,Q,V> Iterator for CloseValIter<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v,d)|(v,d))}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a V,usize);
}
impl<'a,K,M:DiscreteMetric<K,Q>,Q,V> Iterator for CloseValIterMut<'a,K,M,Q,V>{
	fn next(&mut self)->Option<Self::Item>{self.inner.next().map(|(_k,v,d)|(v,d))}
	fn size_hint(&self)->(usize,Option<usize>){self.inner.size_hint()}
	type Item=(&'a mut V,usize);
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
impl<K:Clone,V:Clone> Clone for IntoKeysIter<K,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<K:Clone,V:Clone> Clone for IntoValsIter<K,V>{
	fn clone(&self)->Self{
		Self{inner:self.inner.clone()}
	}
	fn clone_from(&mut self,other:&Self){self.inner.clone_from(&other.inner)}
}
impl<K:Clone,V:Clone> Clone for MapIntoIter<K,V>{
	fn clone(&self)->Self{
		let nodes=&self.nodes;
		let remaining=self.remaining;

		let nodes=MapIter::<K,V>{nodes:nodes.iter().collect(),remaining}.map(|(k,v)|Node::new(k.clone(),v.clone())).collect();
		Self{nodes,remaining}
	}
	fn clone_from(&mut self,other:&Self){
		let remaining=other.remaining;
		let nodes=MapIter::<K,V>{nodes:other.nodes.iter().collect(),remaining}.map(|(k,v)|Node::new(k.clone(),v.clone()));

		self.nodes.clear();
		self.nodes.extend(nodes);
		self.remaining=remaining;
	}
}
impl<K,M:Default+DiscreteMetric<K,K>,V> FromIterator<(K,V)> for BKTreeMap<K,M,V>{
	fn from_iter<I:IntoIterator<Item=(K,V)>>(iter:I)->Self{
		let mut map=Self::default();
		iter.into_iter().map(|(k,v)|map.insert(k,v)).for_each(|_|());
		map
	}
}
impl<K,M:Default,V> Default for BKTreeMap<K,M,V>{
	fn default()->Self{Self::new(M::default())}
}
impl<K,M:DiscreteMetric<K,K>,V> Extend<(K,V)> for BKTreeMap<K,M,V>{
	fn extend<I:IntoIterator<Item=(K,V)>>(&mut self,iter:I){iter.into_iter().map(|(k,v)|self.insert(k,v)).for_each(|_|())}
}
impl<K,M:DiscreteMetric<K,Q>,Q,V> Index<&Q> for BKTreeMap<K,M,V>{
	fn index(&self,index:&Q)->&Self::Output{self.get(index,0).map(|(v,_d)|v).expect("mapping must exist to use index")}
	type Output=V;
}
impl<K,M:DiscreteMetric<K,Q>,Q,V> IndexMut<&Q> for BKTreeMap<K,M,V>{
	fn index_mut(&mut self,index:&Q)->&mut Self::Output{self.get_mut(index,0).map(|(v,_d)|v).expect("mapping must exist to use index")}
}
impl<K,M,V> AsMut<Self> for BKTreeMap<K,M,V>{
	fn as_mut(&mut self)->&mut Self{self}
}
impl<K,M,V> AsRef<Self> for BKTreeMap<K,M,V>{
	fn as_ref(&self)->&Self{self}
}
impl<K,M,V> BKTreeMap<K,M,V>{//TODO other sterotypical map operations
	/// moves all elements from other into self, leaving other empty. If a key from other is already present in self, the respective value from self will be overwritten with the respective value from other
	pub fn append<M2>(&mut self,other:&mut BKTreeMap<K,M2,V>) where M:DiscreteMetric<K,K>{
		fn explore<K,M:DiscreteMetric<K,K>,V>(node:Node<K,V>,tree:&mut BKTreeMap<K,M,V>){
			let (k,v,next)=(node.key,node.value,node.connections);
			tree.insert(k,v);
			next.into_iter().for_each(|(_d,n)|explore(n,tree));
		}

		if let Some(n)=other.root.take(){explore(n,self)}
	}
	/// clears the map, removing all elements
	pub fn clear(&mut self){
		self.length=0;
		self.root=None;
	}
	/// gets the key value pairs whose distance is at most max distance from key
	pub fn close_iter<Q>(&self,key:Q,maxdistance:usize)->CloseMapIter<'_,K,M,Q,V> where M:DiscreteMetric<K,Q>{
		CloseMapIter{key,maxdistance,metric:&self.metric,nodes:self.root.as_ref().into_iter().collect(),remaining:self.length}
	}
	/// gets the key value pairs whose distance is at most max distance from key
	pub fn close_iter_mut<Q>(&mut self,key:Q,maxdistance:usize)->CloseMapIterMut<'_,K,M,Q,V> where M:DiscreteMetric<K,Q>{
		CloseMapIterMut{key,maxdistance,metric:&self.metric,nodes:self.root.as_mut().into_iter().collect(),remaining:self.length}
	}
	/// gets the keys whose distance is at most max distance from key
	pub fn close_keys<Q>(&self,key:Q,maxdistance:usize)->CloseKeyIter<'_,K,M,Q,V> where M:DiscreteMetric<K,Q>{
		CloseKeyIter{inner:self.close_iter(key,maxdistance)}
	}
	/// returns the key value pairs at most maxdistance from the key, sorted by distance
	pub fn close_sorted<'a,Q:?Sized>(&self,key:&Q,maxdistance:usize)->Vec<(&K,&V,usize)> where M:DiscreteMetric<K,Q>{
		fn explore<'a,K,M:DiscreteMetric<K,Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a Node<K,V>,results:&mut Vec<(&'a K,&'a V,usize)>){
			let distance=metric.distance(&node.key,key);

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
	/// returns the key value pairs at most maxdistance from the key, sorted by distance
	pub fn close_sorted_mut<'a,Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Vec<(&K,&mut V,usize)> where M:DiscreteMetric<K,Q>{
		fn explore<'a,K,M:DiscreteMetric<K,Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a mut Node<K,V>,results:&mut Vec<(&'a K,&'a mut V,usize)>){
			let distance=metric.distance(&node.key,key);

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
	/// gets the values whose keys are at most max distance from key
	pub fn close_values<Q>(&self,key:Q,maxdistance:usize)->CloseValIter<'_,K,M,Q,V> where M:DiscreteMetric<K,Q>{
		CloseValIter{inner:self.close_iter(key,maxdistance)}
	}
	/// gets the values whose keys are at most max distance from key
	pub fn close_values_mut<Q>(&mut self,key:Q,maxdistance:usize)->CloseValIterMut<'_,K,M,Q,V> where M:DiscreteMetric<K,Q>{
		CloseValIterMut{inner:self.close_iter_mut(key,maxdistance)}
	}
	/// tests if a key at most maxdistance from the given key is in the map
	pub fn contains_key<Q:?Sized>(&self,key:&Q,maxdistance:usize)->bool where M:DiscreteMetric<K,Q>{self.get_key_value(key,maxdistance).is_some()}
	/// drains the values close to the key
	pub fn drain<Q>(&mut self,key:Q,maxdistance:usize)->DrainMapIter<'_,K,M,Q,V> where M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>{
		let (matches,nodes)=if self.root.as_ref().map(|r|self.metric.distance(&r.key,&key)<=maxdistance).unwrap_or(true){
			let mut matches:Vec<(K,V,usize)>=MapIntoIter{nodes:self.root.take().into_iter().collect(),remaining:self.length}.map(|(k,v)|{
				let d=self.metric.distance(&k,&key);
				(k,v,d)
			}).collect();
			self.length=0;
			self.extend(matches.extract_if(..,|(_k,_v,d)|*d>maxdistance).map(|(k,v,_d)|(k,v)));
			let matches=matches.into_iter().map(|(k,v,d)|(Node::new(k,v),d)).collect();
			let nodes=Vec::new();
			(matches,nodes)
		}else{
			let matches=Vec::new();
			let nodes=self.root.as_mut().into_iter().map(|n|{
				let d=self.metric.distance(&n.key,&key);
				(Some(n),d)
			}).collect();
			(matches,nodes)
		};
		let maplen=&mut self.length;
		let metric=&self.metric;
		DrainMapIter{key,maxdistance,maplen,matches,metric,nodes}
	}
	/// gets the value whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&V,usize)> where M:DiscreteMetric<K,Q>{self.get_key_value(key,maxdistance).map(|(_k,v,d)|(v,d))}
	/// gets the key value pair and distance whose key is closest to the given key, or None if the map contains no key at most max distance from the given key. If there are multiple closest keys, exactly which is returned is unspecified
	pub fn get_key_value<Q:?Sized>(&self,key:&Q,maxdistance:usize)->Option<(&K,&V,usize)> where M:DiscreteMetric<K,Q>{
		fn explore<'a,K,M:DiscreteMetric<K,Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a Node<K,V>)->Option<(&'a K,&'a V,usize)>{
			let distance=metric.distance(&node.key,key);

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
	pub fn get_mut<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(&mut V,usize)> where M:DiscreteMetric<K,Q>{
		fn explore<'a,K,M:DiscreteMetric<K,Q>,Q:?Sized,V>(key:&Q,maxdistance:usize,metric:&M,node:&'a mut Node<K,V>)->Option<(&'a mut V,usize)>{
			let distance=metric.distance(&node.key,key);

			if distance==0{return Some((&mut node.value,0))}
			let includecurrent=distance<=maxdistance;
			let nextnodes=node.connections.range_mut(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).filter_map(|(_d,n)|explore(key,maxdistance,metric,n));
			nextnodes.chain(includecurrent.then_some((&mut node.value,distance))).min_by_key(|(_v,d)|*d)
		}
		let metric=&self.metric;
		let root=if let Some(r)=&mut self.root{r}else{return None};

		explore(key,maxdistance,metric,root)
	}
	/// inserts a key-value pair into the map, returning the previous value at that key, or None if there was no previous value. Keys are considered equal if the the distance between them is 0. If the old value is returned the key is not updated.
	pub fn insert(&mut self,key:K,value:V)->Option<V> where M:DiscreteMetric<K,K>{
		let metric=&self.metric;
		let root=if let Some(n)=self.root.as_mut(){
			n
		}else{
			self.length+=1;
			self.root=Some(Node::new(key,value));
			return None;
		};

		if let Some(v)=root.insert(key,metric,value){return Some(v)}
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
	/// references the metric. avoid modifying the metric in a way that changes the distances because that will most likely cause unspecified incorrect behavior
	pub fn metric(&self)->&M{&self.metric}
	/// returns the number of entries in the map
	pub fn len(&self)->usize{self.length}
	/// creates a new tree
	pub fn new(metric:M)->Self{
		Self{length:0,metric,root:None}
	}
	/// removes the closest mapping whose key at most maxdistance from the given key. If there are multiple closest keys, exactly which is removed is unspecified. This particular tree type doesn't allow super efficient removal, so try to avoid using too much.
	pub fn remove<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(V,usize)> where M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>{self.remove_entry(key,maxdistance).map(|(_k,v,d)|(v,d))}
	/// removes the closest mapping whose key at most maxdistance from the given key. If there are multiple closest keys, exactly which is removed is unspecified. This particular tree type doesn't allow super efficient removal, so try to avoid using too much.
	pub fn remove_entry<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Option<(K,V,usize)> where M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>{
		fn restore_nodes<K,M:DiscreteMetric<K,K>,V>(branch:&mut Node<K,V>,metric:&M,nodes:BTreeMap<usize,Node<K,V>>){
			nodes.into_iter().for_each(|(_d,n)|{
				branch.insert(n.key,metric,n.value);
				restore_nodes(branch,metric,n.connections);
			})
		}
		let metric=&self.metric;
		let mut path=Vec::with_capacity(10);
		let mut branch=if let Some(r)=&mut self.root{r}else{return None};
		let nodedistance=branch.get_path(key,maxdistance,metric,&mut path);
		if nodedistance>maxdistance{return None}
		let mut path=path.into_iter();
		let lastindex=path.next_back();
		for i in path{branch=branch.connections.get_mut(&i).unwrap()}
		self.length-=1;
		if let Some(i)=lastindex{
			let node=branch.connections.remove(&i).unwrap();
			restore_nodes(branch,metric,node.connections);
			return Some((node.key,node.value,nodedistance))
		}
		let mut oldroot=self.root.take().unwrap();
		if let Some((_d,mut newroot))=oldroot.connections.pop_first(){
			restore_nodes(&mut newroot,metric,oldroot.connections);
			self.root=Some(newroot);
		}
		Some((oldroot.key,oldroot.value,nodedistance))
	}
	/// removes all the mappings for which f returns false
	pub fn retain<F:FnMut(&K,&mut V)->bool>(&mut self,mut f:F) where M:DiscreteMetric<K,K>{
		fn explore<F:FnMut(&K,&mut V)->bool,K,M:DiscreteMetric<K,K>,V>(f:&mut F,node:Node<K,V>,tree:&mut BKTreeMap<K,M,V>){
			let (connections,key)=(node.connections,node.key);
			let mut value=node.value;

			if f(&key,&mut value){
				tree.insert(key,value);
			}
			for n in connections.into_values(){explore(f,n,tree)}
		}
		let root=if let Some(r)=self.root.take(){r}else{return};
		self.length=0;
		explore(&mut f,root,self);
	}
	/// splits off the items close to the key
	pub fn split_off<Q:?Sized>(&mut self,key:&Q,maxdistance:usize)->Self where M:Clone+DiscreteMetric<K,K>+DiscreteMetric<K,Q>{//TODO when drain make this use it
		let metric=self.metric.clone();
		let mut y=Self::new(metric.clone());
		let x=BKTreeMap{length:self.length,metric,root:self.root.take()};

		self.length=0;
		x.into_iter().map(|(k,v)|if y.metric.distance(&k,key)<=maxdistance{&mut y}else{&mut *self}.insert(k,v)).for_each(|_|());
		y
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
	/*/// recursive depth first search of the items within maxdistance
	fn bkdfs<B,F:FnMut(&Node<K,V>,usize)->ControlFlow<B>,M:DiscreteMetric<K,Q>,Q:?Sized>(&self,behavior:&mut F,key:&Q,maxdistance:usize,metric:&M)->ControlFlow<B>{
		let distance=metric.distance(&self.key,key);
		if distance<=maxdistance{behavior(self,distance)?}
		self.connections.range(distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance)).try_for_each(|(_r,n)|n.bkdfs(&mut *behavior,key,maxdistance,metric))?;
		ControlFlow::Continue(())
	}*/
	/// gets index path to a node, returning the distance. Returns greater than max distance if there is no node within maxdistance
	fn get_path<M:DiscreteMetric<K,Q>,Q:?Sized>(&self,key:&Q,maxdistance:usize,metric:&M,v:&mut Vec<usize>)->usize{
		let mut mindistance=metric.distance(&self.key,key);
		if mindistance==0{return 0}
		let l0=v.len();
		for (i,n) in self.connections.range(mindistance.saturating_sub(maxdistance)..=mindistance.saturating_add(maxdistance)){
			let l1=v.len();
			v.push(*i);
			let candidatedistance=n.get_path(key,maxdistance,metric,v);
			if candidatedistance<=maxdistance&&candidatedistance<mindistance{
				mindistance=candidatedistance;
				v.drain(l0..l1);
			}else{
				v.truncate(l1);
			}
		}
		mindistance
	}
	/// inserts a node in this one using the metric
	fn insert<M:DiscreteMetric<K,K>>(&mut self,key:K,metric:&M,value:V)->Option<V>{self.insert_existing(metric,Node::new(key,value))}
	/// inserts a node in this one using the metric
	fn insert_existing<M:DiscreteMetric<K,K>>(&mut self,metric:&M,node:Node<K,V>)->Option<V>{
		let mut n=Some(node);
		let mut node=self;

		loop{
			let distance=if let Some(n)=&n{metric.distance(&n.key,&node.key)}else{break};
			if distance==0{return Some(replace(&mut node.value,n.unwrap().value))}
			node=node.connections.entry(distance).or_insert_with(||n.take().unwrap());
		}
		None
	}
	/// creates a new node
	fn new(key:K,value:V)->Self{
		Self{connections:BTreeMap::new(),key,value}
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
	fn test_insert_and_close_values() {
		let mut map = BKTreeMap::<i32, AbsDiff, &'static str>::default();
		map.insert(10, "ten");
		map.insert(20, "twenty");
		map.insert(15, "fifteen");

		// Close to 12 within distance 3 -> only 10 (dist=2) and 15 (dist=3)
		let mut results: Vec<(&&str, usize)> = map.close_values(12, 3).collect();
		results.sort_unstable_by_key(|&(_v, d)| d);
		assert_eq!(results, vec![(&"ten", 2), (&"fifteen", 3)]);
	}

	#[test]
	fn test_closest_sorted_output() {
		let mut map = BKTreeMap::<i32, AbsDiff, &'static str>::default();
		for &(k, v) in &[(5, "five"), (2, "two"), (8, "eight"), (6, "six")] {
			map.insert(k, v);
		}
		// close returns sorted by distance
		let close = map.close_sorted(&6, 3);
		let distances: Vec<usize> = close.iter().map(|&(_k, _v, d)| d).collect();
		assert_eq!(distances, vec![0, 1, 2]);
		let keys: Vec<i32> = close.iter().map(|&(k, _v, _d)| *k).collect();
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
		pairs.sort_unstable();
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
		collected.sort_unstable();
		expected.sort_unstable();
		assert_eq!(collected, expected);
	}
	impl DiscreteMetric<(isize,isize),(isize,isize)> for Cheb2D{
		fn distance(&self,x:&(isize,isize),y:&(isize,isize))->usize{
			let ((xx,xy),(yx,yy))=(x,y);
			((xx-yx).abs() as usize).max((xy-yy).abs() as usize)
		}
	}
	impl DiscreteMetric<i32,i32> for AbsDiff{
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
#[derive(Debug)]
/// iterator over keys close to some key
pub struct CloseKeyIter<'a,K,M,Q,V>{inner:CloseMapIter<'a,K,M,Q,V>}
#[derive(Debug)]
/// iterator over mappings close to some key
pub struct CloseMapIter<'a,K,M,Q,V>{key:Q,maxdistance:usize,metric:&'a M,nodes:Vec<&'a Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over mappings close to some key
pub struct CloseMapIterMut<'a,K,M,Q,V>{key:Q,maxdistance:usize,metric:&'a M,nodes:Vec<&'a mut Node<K,V>>,remaining:usize}
#[derive(Debug)]
/// iterator over values with keys close to some key
pub struct CloseValIter<'a,K,M,Q,V>{inner:CloseMapIter<'a,K,M,Q,V>}
#[derive(Debug)]
/// iterator over values with keys close to some key
pub struct CloseValIterMut<'a,K,M,Q,V>{inner:CloseMapIterMut<'a,K,M,Q,V>}
#[derive(Debug)]
/// iterator that removes mappings close to a key
pub struct DrainMapIter<'a,K,M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>,Q,V>{key:Q,maxdistance:usize,maplen:&'a mut usize,matches:Vec<(Node<K,V>,usize)>,metric:&'a M,nodes:Vec<(Option<&'a mut Node<K,V>>,usize)>}

impl<'a,K,M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>,Q,V> Drop for DrainMapIter<'a,K,M,Q,V>{
	fn drop(&mut self){self.for_each(|_|())}
}
impl<'a,K,M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>,Q,V> Iterator for DrainMapIter<'a,K,M,Q,V>{//TODO finish and test
	fn next(&mut self)->Option<Self::Item>{
		fn explore<'a,K,M:DiscreteMetric<K,K>+DiscreteMetric<K,Q>,Q,V>(insertionindex:usize,insertionnode:&mut Node<K,V>,key:&Q,matches:&mut Vec<(Node<K,V>,usize)>,mut matchesstart:usize,maxdistance:usize,metric:&'a M,nodes:&mut Vec<(Option<&'a mut Node<K,V>>,usize)>){
			while matchesstart<matches.len(){
				take(&mut matches[matchesstart].0.connections).into_values().for_each(|n|{
					let distance=metric.distance(&n.key,key);
					if distance<=maxdistance{
						matches.push((n,distance));
					}else{
						if let Some(node)=insertionnode.connections.get_mut(&insertionindex){
							node.insert_existing(metric,n);
						}else{
							insertionnode.connections.insert(insertionindex,n);
						}
						nodes.push((None,distance));
					}
				});
				matchesstart+=1;
			}
		}
		let (maxdistance,metric)=(self.maxdistance,self.metric);
		let (maplen,matches,nodes)=(&mut self.maplen,&mut self.matches,&mut self.nodes);
		let key=&self.key;

		if let Some((node,distance))=matches.pop(){
			**maplen-=1;
			return Some((node.key,node.value,distance))
		}
		while let Some((Some(node),distance))=nodes.pop(){
			let candidaterange=distance.saturating_sub(maxdistance)..=distance.saturating_add(maxdistance);
			let mut d=0;
			let start=nodes.len();

			take(&mut node.connections).into_iter().for_each(|(r,n)|if candidaterange.contains(&r)&&{
				d=metric.distance(&n.key,key);
				let remove=d<=maxdistance;
				if !remove{nodes.push((None,d))}
				remove
			}{
				matches.push((n,d));
				explore(r,&mut *node,key,matches,matches.len()-1,maxdistance,metric,&mut *nodes);
			}else{
				node.connections.insert(r,n);
			});
			node.connections.range_mut(candidaterange).zip(nodes[start..].iter_mut()).for_each(|((_r,n),(node,_d))|*node=Some(n));
			if let Some((n,d))=matches.pop(){return Some((n.key,n.value,d))}
		}
		None
	}
	fn size_hint(&self)->(usize,Option<usize>){(self.matches.len(),Some(*self.maplen))}
	type Item=(K,V,usize);
}

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
/// iterator over the values in the tree
pub struct ValIter<'a,K,V>{inner:MapIter<'a,K,V>}
#[derive(Debug)]
/// iterator over the values in the tree
pub struct ValIterMut<'a,K,V>{inner:MapIterMut<'a,K,V>}
#[derive(Clone,Debug)]
/// tree node
struct Node<K,V>{connections:BTreeMap<usize,Node<K,V>>,key:K,value:V}
use {
	crate::DiscreteMetric,
	std::{
		collections::BTreeMap,iter::{Extend,FromIterator},mem::{replace,take},ops::{Index,IndexMut}
	}
};
