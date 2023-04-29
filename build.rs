fn main () -> Result<(), Box<dyn std::error::Error>> {
  println!("building!");
  tonic_build::compile_protos("src/protos/hello.proto")?;
  Ok(())
}

// fn main() {
//   println!("hello, building.rs!");
// }