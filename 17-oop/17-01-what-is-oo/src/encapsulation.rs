use what_is_oo::AveragedCollection;

pub fn run() {
    let mut averaged_collection = AveragedCollection::new();
    averaged_collection.add(10);
    averaged_collection.add(50);
    println!("average: {}", averaged_collection.average());
}
