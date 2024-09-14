use e2ee::server::E2ee;

fn main() {
    const FILES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/");

    let public_key_pem =
        std::fs::read_to_string(format!("{}public.pem", FILES_PATH))
            .expect("Failed to read public key file");
    let private_key_pem =
        std::fs::read_to_string(format!("{}private.pem", FILES_PATH))
            .expect("Failed to read private key file");

    // Hi mom!
    let mess = "X6+fo0QNdbS7RTMSHlPapxBUPGgeeWHhlOryODynfkxWrngMMQkzm7R+D+htTQbKAWN20BxV3VpAbcfk0idwPTSjEfGjQAx0Qxx5BaG4de0d0RZ5dmFm9Se27bLXzPYgWR2JCd2RZcHULLwJQctIHgE2NLs26Xz+fs1pMELTXrlroDe8PVQflde4zueJWQTMgXQoC2pF+qrYPF+ZLBiOp5eqR9OBMTIQiAQrkD+QHDGEr7eBt2JDwHd4mWGPlT/08B44E7uzatSzTLe4nH9yHqNCXj/FVEJEirh/7SRl8hBR5NAmnnQkUL4qywlLuRRfPD20HA7BhQAd03cF/4m68g";

    // Hello World from Kha!
    // let mess = "JEaVbT4weaZLUrqlBck9GpnWm/4AwV6xd45usrQzs+tBWiTtbJSEiD/Z5t5gVGeU+tQfY7NLXbUy86rDtBYdqmEfqeNLu2hf1FoUpskL5rx/kj5wXx/mLXF0FmfQGxgp4omuQOll1pr+NbPw5BafdMXwvhcA7SjJvghuqDS2v7rwT9YlBoJ5I/kmyn1ZjHAaPOq+LSJNslxgc2IP4ZQZueaJ2bepypfUVtypVqaIK/qXW9IYAC4lfdbP84LV0hZViaE2n9ED04yV58dUygBZdWBOrSu/E7P866zdLbCvrrUh9PjJXLtKCChMAOe0gRLmJ4NNjbjhYIHwuc982nfyOA";

    // Create E2EE instance with key from PEM files
    let e2ee =
        E2ee::new_from_pem(private_key_pem.to_string(), public_key_pem.to_string())
            .expect("Failed to create E2EE instance");

    // Decrypt the message
    let decrypted = e2ee.decrypt(mess).expect("Failed to decrypt message");

    // Output the encrypted message
    println!("The original message is: {}", decrypted); // The original message is: Hi mom!
}
