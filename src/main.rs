use md4::{Md4, Digest};
use crypto::digest::Digest as DigestMd5;
use sha1::Sha1;
use sha2::Sha224;

fn hash_vector_to_string(hash_vector: Vec<u8>) -> String{
    hash_vector.into_iter()
        .map(|value| format!("{:x}", value))
        .reduce(|acc, x| {
            format!("{acc}{x}")
        }).unwrap()
}

fn string_to_hash_string_md4(s: &str) -> String {
    let mut hasher = Md4::new();
    hasher.update(s);
    let result = hasher.finalize();
    hash_vector_to_string(result.to_vec())
}

fn string_to_hash_string_md5(s: &str) -> String {
    let mut hasher = crypto::md5::Md5::new();
    hasher.input_str(s);
    let result = hasher.result_str();
    result
}

fn string_to_sha1(s: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.update(s);

    let result = hasher.finalize();
    println!("{:?}", result);
    hash_vector_to_string(result.to_vec())
}

fn string_to_sha224(s: &str) -> String {
    let mut hasher = Sha224::new();

    hasher.update(s);

    let result = hasher.finalize();
    hash_vector_to_string(result.to_vec())
}

fn string_to_sha256(s: &str) -> String {
    let res = sha256::digest(s);
    res

}


fn string_to_sha384(s: &str) -> String {
    let mut hasher = crypto::sha2::Sha384::new();
    hasher.input_str(s);
    let result = hasher.result_str();
    result
}

fn string_to_sha512(s: &str) -> String {
    let mut hasher = crypto::sha2::Sha512::new();
    hasher.input_str(s);
    let result = hasher.result_str();
    result
}

fn main() {
    let phrase0 = "The quick brown fox jumps over the lazy dog";
    let phrase1 = "The quick brown fox jumps over the lazy cog";
    let phrase2 = "";

    println!("MD4");
    let digest0 = string_to_hash_string_md4(phrase0);
    println!("digest is: {digest0}");

    let digest1 = string_to_hash_string_md4(phrase1);
    println!("digest is: {digest1}");

    let digest2 = string_to_hash_string_md4(phrase2);
    println!("digest is: {digest2}");

    println!();
    println!("MD5");
    let digest0 = string_to_hash_string_md5(phrase0);
    println!("digest is: {digest0}");

    let digest1 = string_to_hash_string_md5(phrase1);
    println!("digest is: {digest1}");

    let digest2 = string_to_hash_string_md5(phrase2);
    println!("digest is: {digest2}");


    println!();
    println!("SHA1");
    let digest0 = string_to_sha1(phrase0);
    println!("digest is: {digest0}");

    let digest1 = string_to_sha1(phrase1);
    println!("digest is: {digest1}");

    let digest2 = string_to_sha1(phrase2);
    println!("digest is: {digest2}");


    println!();
    println!("SHA224");
    let digest0 = string_to_sha224(phrase0);
    println!("digest is: {digest0}");

    let digest1 = string_to_sha224(phrase1);
    println!("digest is: {digest1}");

    let digest2 = string_to_sha224(phrase2);
    println!("digest is: {digest2}");



    println!("lancuch pusty:");
    println!("SHA256");
    let digest2 = string_to_sha256(phrase2);
    println!("digest is: {digest2}");

    let digest2 = string_to_sha384(phrase2);
    println!("digest is: {digest2}");


    let digest2 = string_to_sha512(phrase2);
    println!("digest is: {digest2}");

    println!("bye");
}
