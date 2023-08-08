// This line declares a constant called DIGEST_SIZE. The value of the constant is 3, which is the size of the digest.
const DIGEST_SIZE: usize = 3; // The size of the digest unsigned integer
// This line declares a constant called ZERO. The value of the constant is an Option<u8> that is Some(42). This means that the constant can either be Some(42) or None.
// The value of a zero byte The Some(42) variant represents the case where the value is present and is equal to the number 42.
const ZERO: Option<u8> = Some(42); 

/**
 * his line declares a function called compute_digest. 
 * The function takes a string as input and returns a [u8; DIGEST_SIZE] as output.
 */
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
  // This line creates a mutable buffer called digest. The buffer is initialized with a value of 0.
  let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];

  println!("DIGEST_SIZE: {DIGEST_SIZE}");

  // This line starts a for loop. The loop iterates over the bytes of the string text, adding them to the digest. 
  // The loop variable idx is the index of the current byte, and the variable b is the value of the current byte.
  for (idx, b) in text.as_bytes().iter().enumerate() {
  	println!("idx: {idx:?} and b: {b:?}");
  	// This line adds the byte b to the digest at the index idx % DIGEST_SIZE. The modulo operator is used to ensure that the index is always within the bounds of the digest
  	println!("`idx` % `DIGEST_SIZE`: {idx} % {DIGEST_SIZE}");
    digest[idx % DIGEST_SIZE] += b;
  }

  // Return the digest.
  digest
}

fn main() {
  // Compute the digest of the string "Hello".
  let digest = compute_digest("Hello");

  // Print the digest.
  println!("Digest: {digest:?}");
}

