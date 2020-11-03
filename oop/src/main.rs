use oop::AveragedCollection;

fn main() {
    let mut ac = AveragedCollection::new();
    println!("average = {}", ac.average());
    ac.add(1);
    ac.add(3);
    ac.add(10);
    println!("average = {}", ac.average());
    ac.remove();
    println!("average = {}", ac.average());
}
