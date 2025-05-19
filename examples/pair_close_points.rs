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
