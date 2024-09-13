mod hasher;
mod test2;
mod crypt;

fn main() {
    println!("Hello, world!");

    let password = "my_secure_password";
    let hashed_password = hasher::hash_password(password);
    println!("Hashed password: {}", hashed_password);

    let my_test2 = test2::test2fn();
    println!("my_test2: {}", my_test2);

    let plaintext = "
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    This is some text I want to encrypt!
    ";

    // Encrypt the text
    let (salt, iv, ciphertext) = crypt::encrypt_with_password(plaintext, &hashed_password);
    println!("Ciphertext: {:?}", ciphertext);

    // Decrypt the text
    let decrypted_text = crypt::decrypt_with_password(&ciphertext, &hashed_password, &salt, &iv);
    println!("Decrypted text: {}", decrypted_text);
}

