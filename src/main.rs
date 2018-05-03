fn main() {
  println!(
    "My API ID is {} and KEY is {}.",
    env!("AWS_API_ID"),
    env!("AWS_API_KEY")
  );
}
