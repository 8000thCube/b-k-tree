# b-k-tree
Burkhard-Keller tree data structure for finding items separated by a small discrete distance

# examples

making edges between spatially close together (distance < 1) vertices in a graph:
```
pub fn main(){
	let vertices=[[-1.5, 2.0],[-1.6, 1.6],[-1.6, 1.1],[ 1.5, 1.3],[ 1.0,-0.1],
				  [ 0.5, 0.3],[-2.0, 1.0],[ 0.5,-1.0],[ 0.9, 0.8],[ 2.0, 2.0]];
	let tree:BKTreeSet<[f32;2],CeilL2>=vertices.into_iter().collect();
	let edges:HashSet<([u32;2],[u32;2])>=vertices.into_iter().flat_map(|x|{
		let close=tree.close_iter(x,1).map(|(y,_d)|[y[0].to_bits(),y[1].to_bits()]);
		let x=[x[0].to_bits(),x[1].to_bits()];
		close.filter(move|&y|x!=y).map(move|y|if x<y{(x,y)}else{(y,x)})
	}).collect();

	edges.iter().map(|([x0,x1],[y0,y1])|([f32::from_bits(*x0),f32::from_bits(*x1)],[f32::from_bits(*y0),f32::from_bits(*y1)])).for_each(|(p,q)|{
		println!("([{}, {}], [{}, {}])",p[0],p[1],q[0],q[1]);
	});
}
use {
	b_k_tree::{BKTreeSet,metrics::CeilL2},std::{collections::HashSet}
};
```
output:
```
([-1.6, 1.1], [-1.6, 1.6])
([0.5, 0.3], [0.9, 0.8])
([0.9, 0.8], [1.5, 1.3])
([-1.5, 2], [-1.6, 1.1])
([1.5, 1.3], [2, 2])
([0.5, 0.3], [1, -0.1])
([0.9, 0.8], [1, -0.1])
([-1.6, 1.1], [-2, 1])
([-1.6, 1.6], [-2, 1])
([-1.5, 2], [-1.6, 1.6])

classifying by the category associated with closest string in a dictionary:
```
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


