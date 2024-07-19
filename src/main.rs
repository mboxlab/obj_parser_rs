// for tests
fn main() {
    let input = include_bytes!("../cube.obj");
    let model: wavefront::Obj = wavefront::Obj::from_reader(&input[..]).unwrap();
    for (name, _obj) in model.objects() {
        print!("{}\n", name);
    }
}
