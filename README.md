# b-k-tree
Burkhard-Keller tree data structure for finding items separated by a small discrete distance

# examples

making edges between spatially close together (distance <= 1) vertices in a graph:
```rust
pub fn main(){
	let vertices=[[-1.5, 2.0],[-1.6, 1.6],[-1.6, 1.1],[ 1.5, 1.3],[ 1.0,-0.1],
				  [ 0.5, 0.3],[-2.0, 1.0],[ 0.5,-1.0],[ 0.9, 0.8],[ 2.0, 2.0]];
	let tree:BKTreeSet<[f32;2],CeilL2>=vertices.into_iter().collect();
	let mut edges:Vec<([f32;2],[f32;2])>=vertices.into_iter().flat_map(|u|{
		let close=tree.close_iter(u,1).map(|(v,_d)|*v).filter(move|&v|u!=v);
		close.map(move|v|if u<v{(u,v)}else{(v,u)})
	}).collect();
	edges.sort_unstable_by_key(|([ux,uy],[vx,vy])|[ux.to_bits(),uy.to_bits(),vx.to_bits(),vy.to_bits()]);
	edges.dedup();

	edges.iter().for_each(|(p,q)|{
		println!("([{}, {}], [{}, {}])",p[0],p[1],q[0],q[1]);
	});
}
use {
	b_k_tree::{BKTreeSet,metrics::CeilL2},
};
```
output:
```
([0.5, 0.3], [0.9, 0.8])
([0.5, 0.3], [1, -0.1])
([0.9, 0.8], [1, -0.1])
([0.9, 0.8], [1.5, 1.3])
([1.5, 1.3], [2, 2])
([-1.6, 1.1], [-1.5, 2])
([-1.6, 1.1], [-1.6, 1.6])
([-1.6, 1.6], [-1.5, 2])
([-2, 1], [-1.6, 1.1])
([-2, 1], [-1.6, 1.6])
```

classifying by the category associated with closest string in a dictionary:
```rust
pub fn main(){
	let mut tree=BKTreeMap::new(Levenshtein::new());
	tree.insert("calculate","mathematics");
	tree.insert("cat","pet");
	tree.insert("kat","name");
	tree.insert("hello","greeting");
	tree.insert("hi","greeting");
	tree.insert("linear","mathematics");

	println!("{}",tree.get("calculator",2).map(|(s,_d)|*s).unwrap_or("not found"));
	println!("{}",tree.get("hey",2).map(|(s,_d)|*s).unwrap_or("not found"));
	println!("{}",tree.get("kate",2).map(|(s,_d)|*s).unwrap_or("not found"));
	println!("{}",tree.get("line",2).map(|(s,_d)|*s).unwrap_or("not found"));
	println!("{}",tree.get("serotonin",2).map(|(s,_d)|*s).unwrap_or("not found"));
}
use b_k_tree::{BKTreeMap,metrics::Levenshtein};
```
output:
```
mathematics
greeting
name
mathematics
not found
```
