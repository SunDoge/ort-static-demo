use ort::{Environment, Session, SessionBuilder};

fn main() {
    let env = Environment::builder().build().unwrap().into_arc();
    let sess = SessionBuilder::new(&env)
        .unwrap()
        .with_model_from_file("add.onnx")
        .unwrap();
}
