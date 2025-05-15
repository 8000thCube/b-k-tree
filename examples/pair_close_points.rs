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
