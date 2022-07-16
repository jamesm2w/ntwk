use iced::Point;
use std::sync::Arc;
use std::cell::RefCell;

/// This module is basically just the actual data structures and stuff for the network
/// all wrapped up to keep namespaces clear.
/// 
/// TODO: Make storage more generic so we can swap between say Adj List and G=(V, E) forms (e.g.) 

#[derive(Debug, Default)]
pub struct NetworkGraph {
    nodes: Vec<Arc<Node<Point>>>,
    last_id: usize
}

impl NetworkGraph {

    pub fn new () -> Self {
        NetworkGraph::default()
    }

    pub fn node_list(&self) -> &Vec<Arc<Node<Point>>> {
        &self.nodes
    }

    pub fn add_node(&mut self, pos: &Point) {
        self.nodes.push(Node::new(format!("{}", self.last_id), pos.clone()));
        self.last_id += 1;
    }

    pub fn remove_node(&mut self, node_ref: &Arc<Node<Point>>) { // rewrite using filter?
        let mut current_index = 0;
        let mut matching_index = vec![];
        
        for node in self.node_list() {
            if node.eq(node_ref) {
                matching_index.push(current_index);
            }
            node.disconnect(node_ref);
            current_index += 1;
        }

        for i in matching_index {
            self.nodes.remove(i);
        }
    }

    pub fn add_edge(&mut self, node_src: &Arc<Node<Point>>, node_dst: &Arc<Node<Point>>) {
        Node::connect(node_src, node_dst)
    }

    pub fn add_curve(&mut self, node_src: &Arc<Node<Point>>, node_dst: &Arc<Node<Point>>, control: Point) {
        Node::connect_with_curve(node_src, node_dst, control)
    }

    pub fn remove_edge(&mut self, _node_src: &Arc<Node<Point>>, _node_dst: &Arc<Node<Point>>) {
        println!("How the hell am I supposed to do this");
        unimplemented!() // TODO
    }

    pub fn get_exact_point(&self, pos: Point) -> Option<Arc<Node<Point>>> {
        
        for node in self.node_list().iter() {
            
            if node.data.eq(&pos) {
                return Some(node.clone())
            }
        }

        None
    }

    pub fn get_near_point(&self, pos: Point) -> Option<Arc<Node<Point>>> {

        for node in self.node_list().iter() {

            let dx = f32::abs(node.data.x - pos.x);
            let dy = f32::abs(node.data.y - pos.y);

            if dx < 5.0 && dy < 5.0 {
                return Some(node.clone())
            }
        }

        None
    }


}

/// Node<T>
/// Basic node in the network structure has a identifying name and
/// some data associated with it, as well as a list of connected nodes
/// edges is wrapped in a RefCell to allow the inner Vec to be borrowed mutuably even though the Rc
/// would not normally allow mutuable borrows.
#[derive(Debug, Clone)]
pub struct Node<T: PartialEq> {
    name: String,
    data: T,
    edges: RefCell<Vec<Connection<T>>>
}

impl<T: PartialEq> PartialEq for Node<T> {
    
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data    
    }

}

/// Connection<T>
/// Represents connection between two nodes (the implict owner of this connection) 
/// and the referenced node dst. 
/// Optionally this connection can have some weight, and a ctl point for display as a bezier curve
/// dst is wraped in Weak because of circular pointers and stuff. Weak doesnt "own" its value, and doesn't prevent
/// the inner Node<T> from being dropped.
#[derive(Debug, Clone)]
pub struct Connection<T: PartialEq> {
    dst: Arc<Node<T>>,
    weight: Option<f32>,
    ctl: Option<Point>
}

impl<T: PartialEq> Node<T> {

    /// Create a new Rc<Node<T>> based off some name and data. Has empty connection list.
    /// Rc is needed to put this onto the heap and allow multiple pointers to it.
    pub fn new(name: String, data: T) -> Arc<Node<T>> {
        Arc::new(Node {name, data, edges: RefCell::new(vec![])})
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn edges(&self) -> &RefCell<Vec<Connection<T>>> {
        &self.edges
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    /// Connect two Nodes by a connection. Creates mutual Connection structs in both their edge lists
    /// clones the references.
    /// Takes &Rc<Node<T>> to not take ownership of the original Rc<Node<T>>
    pub fn connect(source: &Arc<Node<T>>, dest: &Arc<Node<T>>) {
        source.edges.borrow_mut().push(Connection::new(dest.clone()));
        dest.edges.borrow_mut().push(Connection::new(source.clone()));
    }

    /// Connect two nodes via bezier curve
    pub fn connect_with_curve(source: &Arc<Node<T>>, dest: &Arc<Node<T>>, ctl: Point) {
        source.edges.borrow_mut().push(Connection::new_curve(dest.clone(), ctl.clone()));
        dest.edges.borrow_mut().push(Connection::new_curve(source.clone(), ctl));
    }

    pub fn disconnect(&self, other: &Arc<Node<T>>) { // might be able to do this more idiomatically

        let mut conn_index = vec![];
        let mut index = 0;

        for conn in self.edges.borrow().iter() {

            if conn.dst.eq(other) {
                conn_index.push(index)
            }
            index += 1;
        }

        for i in conn_index {
            self.edges.borrow_mut().swap_remove(i);
        }
    }
}

impl<T: PartialEq> Connection<T> {
    /// Create a new connection from a Rc Node<T>.
    /// TODO: fully expand this impl
    pub fn new(dst: Arc<Node<T>>) -> Self {
        Connection { dst: dst.clone(), weight: None, ctl: None }
    }

    pub fn new_curve(dst: Arc<Node<T>>, ctl: Point) -> Self {
        Connection { dst: dst.clone(), weight: None, ctl: Some(ctl) }
    }

    pub fn destination(&self) -> &Arc<Node<T>> {
        &self.dst
    }

    pub fn weight(&self) -> &Option<f32> {
        &self.weight
    }

    pub fn control(&self) -> &Option<Point> {
        &self.ctl
    }
}

// TODO: Expand impl and maybe add iterators?

#[cfg(test)]
mod test {

    use super::*;

    /// Can we actually create a new Node?
    #[test]
    pub fn test_node_creation () {

        let node: Arc<Node<i32>> = Node::new(String::from("Hello World"), 42);

        assert_eq!(node.name, String::from("Hello World"));
        assert_eq!(node.data, 42);
        assert_eq!(node.edges.borrow().len(), 0);
    }

    /// Can we connect two nodes?
    #[test]
    pub fn test_node_connection () {
        let node: Arc<Node<i32>> = Node::new(String::from("Node 1"), 10);

        let node2: Arc<Node<i32>> = Node::new(String::from("Node 2"), 20);

        Node::connect(&node, &node2);

        println!("{:?}", node.edges.borrow().iter().map(|x| {
            format!("Connection {{ {:?}, {:?}, {:?} }}", x.dst, x.weight, x.ctl)
        }).collect::<Vec<String>>()
        );
        println!("{:?}", node2.edges.borrow());

        assert_eq!(1,1);
    }
}