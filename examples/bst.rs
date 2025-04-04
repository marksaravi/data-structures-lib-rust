use data_structures_lib_rust::bst::BST;

fn main() {
    let tree = BST {
        depth_left: 1,
        depth_right: 3,
    };
    print!("BST example\n");
    print!("depth left: {0}\n", tree.depth_left);
    print!("depth right: {0}\n", tree.depth_right)
}