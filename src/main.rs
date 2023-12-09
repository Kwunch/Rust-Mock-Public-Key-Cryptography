/*
    * Mock Public Key Cryptography System
    * Generates a public and private key pair using Euler's Totient Function
    * Encrypts and decrypts a message using the generated keys
    * NOTE: This is a mock system and should not be used for any real world applications

    * This has many security flaws and is not a secure system
    * No user input is sanitized
    * No input is checked for validity
    * No input is checked for primality
    * No input is checked for coprimality
    * No password is used to encrypt the private key

    * Not choosing 2 prime numbers for p and q may result in incorrect output

    * Only use this for educational purposes
*/

use std::io::Write;

use num_bigint::{BigUint, ToBigUint};
use rand::Rng;

fn gcd(a: u32, b: u32) -> u32 {
    /*
        * Euclidean Algorithm
        * gcd(a, b) = gcd(b, a % b)
        * gcd(a, 0) = a

        * Recursively finds the greatest common divisor of a and b

        * Returns the greatest common divisor of a and b or b if a == 0
    */
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn phi(n: u32) -> (u32, Vec<u32>) {
    /*
        * Calculates Phi(n) using the formula:
        * Phi(n) = n * (1 - 1/p1) * (1 - 1/p2) * ... * (1 - 1/pk)
        * Where p1, p2, ..., pk are the prime factors of n
    */
    let mut num_coprimes: u32 = 0;
    let mut coprimes: Vec<u32> = Vec::new();
    for i in 1..n {
        if gcd(i, n) == 1 {
            num_coprimes += 1;
            coprimes.push(i);
        }
    }
    (num_coprimes, coprimes)
}

fn random_coprime(n: u32) -> u32 {
    /*
        * Returns a random coprime of n
        * n is the value of phi(n)
    */
    let mut coprime: u32 = 0;
    loop {
        let mut rng = rand::thread_rng();
        coprime = rng.gen_range(1..n);

        if gcd(coprime, n) == 1 && coprime != 1 {
            break;
        }
    }
    coprime
}

fn compute_inverse(e: u32, phi_n: u32) -> u32 {
    /*
        * Computes the inverse of e (mod phi_n)

        * Returns d such that d * e = 1 (mod phi_n)
    */
    let mut d: u32 = 0;
    loop {
        if (d * e) % phi_n == 1 {
            break;
        }
        d += 1;
    }
    d
}

fn encrypt(message: String, public_key: [u128; 2]) -> Vec<BigUint> {
    /*
        * Encrypts a message using the public key
        * message is the message to encrypt
        * public_key is the public key to encrypt the message with

        * Returns the encrypted message
    */

    let mut encrypted_message: Vec<BigUint> = Vec::new();
    for c in message.chars() {
        encrypted_message.push((c as u8).to_biguint()
            .unwrap()
            .pow(public_key[1] as u32)
            .to_biguint()
            .unwrap()
            % public_key[0]);
    }

    encrypted_message
}

fn decrypt(encrypted_message: Vec<BigUint>, private_key: [u128; 2]) -> String {
    /*
        * Decrypts an encrypted message using the private key
        * encrypted_message is the encrypted message to decrypt
        * private_key is the private key to decrypt the message with

        * Returns the decrypted message
    */

    let mut decrypted_message: String = String::new();
    for m in encrypted_message {
        decrypted_message.push((m.pow(private_key[1] as u32) % private_key[0])
            .to_u32_digits()[0] as u8 as char);
    }

    decrypted_message
}

fn main() {
    /*
        * Take user input for two prime numbers, p and q
        * Calculate n = p * q
        * Calculate phi(n)
        * Pick random prime number e such that e < phi(n) and e is coprime to phi(n)
        * Calculate d such that d * e = 1 (mod phi(n))
        * Public Key: (n, e)
        * Private Key: (n, d)
    */

    loop {
        print!("Enter a prime number: ");
        std::io::stdout().flush().unwrap();

        // Take user input for p
        let mut p: String = String::new();
        std::io::stdin().read_line(&mut p).unwrap();

        if p.trim() == "quit" {
            break;
        }

        p = p.trim().to_string();
        let p: u128 = p.parse().unwrap();

        print!("Enter another prime number: ");
        std::io::stdout().flush().unwrap();

        // Take user input for q
        let mut q: String = String::new();
        std::io::stdin().read_line(&mut q).unwrap();
        let q: u128 = q.trim().parse().unwrap();

        // Calculate n (p*q) and phi(n) (Euler's Totient Function)
        let n = p * q;
        let phi_n = phi(n as u32).0;

        // Pick random prime number e such that e < phi(n) and e is coprime to phi(n)
        let e = random_coprime(phi_n);
        // Calculate d such that d * e = 1 (mod phi(n))
        let d = compute_inverse(e, phi_n);

        // Create key pairs
        let public_key = [n, e as u128];
        let private_key = [n, d as u128];

        // Enter Message to Encrypt and Decrypt
        print!("Enter a message to encrypt: ");
        std::io::stdout().flush().unwrap();
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        message = message.trim().to_string();

        // Get encrypted text
        let encrypted_text = encrypt(message, public_key);

        // Print encrypted text (This is output as a vector of natural numbers)
        println!("Encrypted message: {:?}", encrypted_text);

        // Get decrypted text
        let decrypted_text = decrypt(encrypted_text, private_key);

        // Print decrypted text (Output should be the same as the original input string)
        println!("Decrypted message: {}", decrypted_text);
    }
}
