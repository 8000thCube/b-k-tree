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
