mod graph;

use graph::print_line;
use graph::print_vec;
use graph::add_one;
use graph::Graph;
use graph::IGraph;
use graph::TopoSortVisitor;
use graph::IGraphVisitor;

fn main() {
  let mut v = 12;
  let vec = vec![v, 4];
  print_line(v);
  add_one(&mut v);
  print_line(v);
  print_vec(&vec);
  println!("vec in main: {:?} at {:p}", vec, &vec);
  println!("vec[0]: {}", vec[0]);
  graph_test();
}

fn graph_test() {
  let mut g = Graph::<i32>::new();
  let v1 = g.add(1);
  let v2 = g.add(2);
  let v3 = g.add(3);
  g.link(v1, v2);
  g.link(v1, v3);
  g.link(v2, v3);
  let mut topoVisitor = TopoSortVisitor::new();
  let result = topoVisitor.visit(&mut g);
  println!("------");
  for r in &result {
    println!("{}", *r);
  }
  println!("------");
  let result1 = g.accept(&mut topoVisitor);
  for r in &result1 {
    println!("{}", *r);
  }
  println!("------");
  let vtx = vec![1,2,3];
  for r in g.bulk_get(vtx) {
    match r {
      Some(x) => println!("{}", *x),
      None => println!("none!"),
    }
  }
  println!("------");
  g.clear();
}
