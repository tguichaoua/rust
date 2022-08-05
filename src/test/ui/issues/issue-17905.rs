// run-pass

#[derive(Debug)]
#[allow(unused_tuple_struct_fields)]
struct Pair<T, V> (T, V);

impl Pair<
    &str,
    isize
> {
    fn say(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let result = &Pair("shane", 1);
    result.say();
}
