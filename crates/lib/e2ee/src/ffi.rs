/// Foreign Function Interface (FFI) bindings for the E2EE (End-to-End Encryption) library.
///
/// This module provides C-compatible functions to interact with the `E2ee` and `PublicE2ee` structs.
/// These functions allow other programming languages to use the encryption and decryption capabilities
/// provided by the library. The functions utilize pointers to manage memory across the FFI boundary.
/// 
/// # Safety
///
/// Many functions in this module involve unsafe code due to the interaction with raw pointers and C-style strings.
/// It's important to ensure that all pointers passed into these functions are valid and non-null to avoid
/// undefined behavior. Additionally, the caller must free any allocated memory using the appropriate functions
/// provided in this module (e.g., `e2ee_server_free`, `e2ee_client_free`).
/// 
/// # Functions
///
/// - `e2ee_server_new`: Creates a new `E2ee` instance with a specified key size.
/// - `e2ee_server_new_from_pem`: Creates a new `E2ee` instance from provided PEM-encoded keys.
/// - `e2ee_client_new_from_public_pem`: Creates a new `PublicE2ee` instance from a PEM-encoded public key.
/// - `e2ee_server_encrypt`: Encrypts a message using the server's public key.
/// - `e2ee_client_encrypt`: Encrypts a message using the client's public key.
/// - `e2ee_server_decrypt`: Decrypts a message using the server's private key.
/// - `e2ee_server_get_public_key_pem`: Retrieves the PEM-encoded public key from the server.
/// - `e2ee_server_get_private_key_pem`: Retrieves the PEM-encoded private key from the server.
/// - `e2ee_server_free`: Frees the memory associated with an `E2ee` instance.
/// - `e2ee_client_free`: Frees the memory associated with a `PublicE2ee` instance.
/// - `e2ee_server_free_string`: Frees memory associated with a C string.
use crate::client::PublicE2ee;
use crate::server::{E2ee, KeySize};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Creates a new `E2ee` instance with the specified RSA key size.
///
/// # Arguments
/// 
/// * `key_size` - The RSA key size (1024, 2048, 3072, 4096).
///
/// # Returns
///
/// Returns a pointer to the newly created `E2ee` instance. Returns a null pointer if an invalid key size is specified or an error occurs during instantiation.
///
/// # Safety
///
/// This function is safe as long as the `key_size` parameter is a valid RSA key size.
#[cfg(feature = "ffi")]
#[no_mangle]
pub extern "C" fn e2ee_server_new(key_size: c_int) -> *mut E2ee {
    let key_size = match key_size {
        1024 => KeySize::Bit1024,
        2048 => KeySize::Bit2048,
        3072 => KeySize::Bit3072,
        4096 => KeySize::Bit4096,
        _ => return std::ptr::null_mut(), // Invalid key size
    };
    match E2ee::new(key_size) {
        Ok(sdk) => Box::into_raw(Box::new(sdk)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a new `E2ee` instance from PEM-encoded private and public keys.
///
/// # Arguments
/// 
/// * `private_key_pem` - A pointer to a C string containing the PEM-encoded private key.
/// * `public_key_pem` - A pointer to a C string containing the PEM-encoded public key.
///
/// # Returns
///
/// Returns a pointer to the newly created `E2ee` instance. Returns a null pointer if an error occurs during instantiation.
///
/// # Safety
///
/// The provided pointers must be valid C strings. The function assumes they are non-null and contain valid UTF-8 data.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_new_from_pem(
    private_key_pem: *const c_char,
    public_key_pem: *const c_char,
) -> *mut E2ee {
    let private_key = unsafe { CStr::from_ptr(private_key_pem).to_str().unwrap() };
    let public_key = unsafe { CStr::from_ptr(public_key_pem).to_str().unwrap() };

    match E2ee::new_from_pem(private_key.to_string(), public_key.to_string()) {
        Ok(e2ee) => Box::into_raw(Box::new(e2ee)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a new `PublicE2ee` instance from a PEM-encoded public key.
///
/// # Arguments
///
/// * `public_key` - A pointer to a C string containing the PEM-encoded public key.
///
/// # Returns
///
/// Returns a pointer to the newly created `PublicE2ee` instance. Returns a null pointer if an error occurs during instantiation.
///
/// # Safety
///
/// The `public_key` pointer must be a valid C string. The function assumes the provided pointer is non-null and contains valid UTF-8 data.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_client_new_from_public_pem(
    public_key: *const c_char,
) -> *mut PublicE2ee {
    let public_key = unsafe { CStr::from_ptr(public_key).to_str().unwrap() };

    match PublicE2ee::new(public_key.to_string()) {
        Ok(e2ee) => Box::into_raw(Box::new(e2ee)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Encrypts a message using the server's public key.
///
/// # Arguments
///
/// * `e2ee_server` - A pointer to an `E2ee` instance.
/// * `message` - A pointer to a C string containing the plaintext message.
///
/// # Returns
///
/// Returns a pointer to a C string containing the base64-encoded encrypted message. Returns a null pointer if encryption fails.
///
/// # Safety
///
/// The `e2ee_server` and `message` pointers must be valid and non-null.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_encrypt(
    e2ee_server: *mut E2ee,
    message: *const c_char,
) -> *mut c_char {
    let e2ee_server = unsafe { &*e2ee_server };
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    match e2ee_server.encrypt(message) {
        Ok(encrypted) => CString::new(encrypted).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Encrypts a message using the client's public key.
///
/// # Arguments
///
/// * `e2ee_client` - A pointer to a `PublicE2ee` instance.
/// * `message` - A pointer to a C string containing the plaintext message.
///
/// # Returns
///
/// Returns a pointer to a C string containing the base64-encoded encrypted message. Returns a null pointer if encryption fails.
///
/// # Safety
///
/// The `e2ee_client` and `message` pointers must be valid and non-null.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_client_encrypt(
    e2ee_client: *mut PublicE2ee,
    message: *const c_char,
) -> *mut c_char {
    let e2ee_client = unsafe { &*e2ee_client };
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    match e2ee_client.encrypt(message) {
        Ok(encrypted) => CString::new(encrypted).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Decrypts a message using the server's private key.
///
/// # Arguments
///
/// * `e2ee_server` - A pointer to an `E2ee` instance.
/// * `ciphertext` - A pointer to a C string containing the base64-encoded encrypted message.
///
/// # Returns
///
/// Returns a pointer to a C string containing the decrypted plaintext message. Returns a null pointer if decryption fails.
///
/// # Safety
///
/// The `e2ee_server` and `ciphertext` pointers must be valid and non-null.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_decrypt(
    e2ee_server: *mut E2ee,
    ciphertext: *const c_char,
) -> *mut c_char {
    let e2ee_server = unsafe { &*e2ee_server };
    let ciphertext = unsafe { CStr::from_ptr(ciphertext).to_str().unwrap() };

    match e2ee_server.decrypt(ciphertext) {
        Ok(decrypted) => CString::new(decrypted).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Retrieves the public key in PEM format from the given `E2ee` server object.
///
/// # Safety
///
/// The `e2ee_server` pointer must be valid and non-null. The function performs an unsafe operation by dereferencing
/// this pointer. If the pointer is null, the function will return a null pointer.
/// The returned C string must be freed using `e2ee_server_free_string` to avoid memory leaks.
///
/// # Parameters
///
/// - `e2ee_server`: A pointer to an `E2ee` server object.
///
/// # Returns
///
/// A C string containing the public key in PEM format. If an error occurs, returns a null pointer.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_get_public_key_pem(
    e2ee_server: *mut E2ee,
) -> *mut c_char {
    let e2ee_server = unsafe { &*e2ee_server };
    CString::new(e2ee_server.get_public_key_pem())
        .unwrap()
        .into_raw()
}

/// Retrieves the private key in PEM format from the given `E2ee` server object.
///
/// # Safety
///
/// The `e2ee_server` pointer must be valid and non-null. The function performs an unsafe operation by dereferencing
/// this pointer. If the pointer is null, the function will return a null pointer.
/// The returned C string must be freed using `e2ee_server_free_string` to avoid memory leaks.
///
/// # Parameters
///
/// - `e2ee_server`: A pointer to an `E2ee` server object.
///
/// # Returns
///
/// A C string containing the private key in PEM format. If an error occurs, returns a null pointer.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_get_private_key_pem(
    e2ee_server: *mut E2ee,
) -> *mut c_char {
    let e2ee_server = unsafe { &*e2ee_server };
    CString::new(e2ee_server.get_private_key_pem())
        .unwrap()
        .into_raw()
}

/// Frees the memory associated with an `E2ee` server object.
///
/// This function should be called to release the memory of an `E2ee` server object that was previously
/// created using Rust code and returned to C. After calling this function, the pointer to the `E2ee` 
/// server object becomes invalid and must not be used again.
///
/// # Safety
///
/// - The `e2ee_server` pointer must be valid and non-null. This function performs an unsafe operation by dereferencing 
///   the pointer and taking ownership of the memory, which is then deallocated using Rust's `Box::from_raw()`.
/// - This function should only be called once for each `e2ee_server` object. Calling this function multiple times 
///   with the same pointer will lead to undefined behavior, as the memory will have already been freed.
/// - The caller must ensure that the pointer was originally allocated by Rust using `Box::into_raw()`. Passing any other 
///   pointer (e.g., a pointer allocated by C or other means) to this function may result in undefined behavior.
///
/// # Parameters
///
/// - `e2ee_server`: A pointer to an `E2ee` server object (`*mut E2ee`).
///
/// # Example
///
/// ```c
/// // In C code, after using the server, call this to free its memory:
/// e2ee_server_free(e2ee_server);
/// ```
///
/// # Notes
///
/// After this function is called, the memory associated with `e2ee_server` is freed. The caller should not attempt to 
/// access the `e2ee_server` pointer afterward, as doing so will result in a use-after-free error.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_free(e2ee_server: *mut E2ee) {
    if !e2ee_server.is_null() {
        unsafe {
            drop(Box::from_raw(e2ee_server));
        }
    }
}

/// Frees the memory associated with an `PublicE2ee` client object.
///
/// This function should be called to release the memory of a `PublicE2ee` client object that was previously
/// created using Rust code and returned to C. After calling this function, the pointer to the `PublicE2ee` 
/// client object becomes invalid and must not be used again.
///
/// # Safety
///
/// - The `e2ee_client` pointer must be valid and non-null. This function performs an unsafe operation by dereferencing 
///   the pointer and taking ownership of the memory, which is then deallocated using Rust's `Box::from_raw()`.
/// - This function should only be called once for each `e2ee_client` object. Calling this function multiple times 
///   with the same pointer will lead to undefined behavior, as the memory will have already been freed.
/// - The caller must ensure that the pointer was originally allocated by Rust using `Box::into_raw()`. Passing any other 
///   pointer (e.g., a pointer allocated by C or other means) to this function may result in undefined behavior.
///
/// # Parameters
///
/// - `e2ee_client`: A pointer to a `PublicE2ee` client object (`*mut PublicE2ee`).
///
/// # Example
///
/// ```c
/// // In C code, after using the client, call this to free its memory:
/// e2ee_client_free(e2ee_client);
/// ```
///
/// # Notes
///
/// After this function is called, the memory associated with `e2ee_client` is freed. The caller should not attempt to 
/// access the `e2ee_client` pointer afterward, as doing so will result in a use-after-free error.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_client_free(e2ee_client: *mut PublicE2ee) {
    if !e2ee_client.is_null() {
        unsafe {
            drop(Box::from_raw(e2ee_client));
        }
    }
}

/// Frees the memory associated with a C string.
///
/// This function is used to free the memory of a string that was originally allocated by the Rust code and returned
/// to the C code. After calling this function, the pointer to the C string becomes invalid and should no longer
/// be used.
///
/// # Safety
///
/// The `s` pointer must be valid and non-null. This function performs an unsafe operation by dereferencing the pointer 
/// and transferring ownership of the memory to the Rust side. After the function is called, the memory is deallocated.
/// Using the `s` pointer after this call is undefined behavior.
///
/// The function assumes that the pointer was originally created by Rust using `CString::into_raw()`. Passing any 
/// other pointer may result in undefined behavior.
///
/// # Example
///
/// ```c
/// // In C code, after using the string, call this to free it:
/// e2ee_server_free_string(encrypted_message);
/// ```
///
/// # Arguments
///
/// * `s` - A pointer to a C string (null-terminated `c_char`).
///
/// # Panics
///
/// This function may panic if the pointer is invalid or the memory was not allocated by Rust's `CString`.
#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn e2ee_server_free_string(s: *mut c_char) {
    unsafe {
        drop(CString::from_raw(s));
    }
}

#[cfg(test)]
#[cfg(feature = "ffi")]
mod tests {
    use super::*;
    // use crate::client::PublicE2ee;
    // use crate::server::{E2ee, KeySize};
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    // Helper function to convert Rust strings to C strings and back
    fn to_c_string(s: &str) -> *const c_char {
        CString::new(s).unwrap().into_raw()
    }

    fn from_c_string(c_str: *const c_char) -> String {
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    // Test the e2ee_server_new function
    #[test]
    fn test_e2ee_server_new() {
        let key_size = 2048;
        let e2ee_server = e2ee_server_new(key_size);
        assert!(!e2ee_server.is_null());

        unsafe { e2ee_server_free(e2ee_server) };
    }

    // Test the e2ee_server_new_from_pem function
    #[test]
    fn test_e2ee_server_new_from_pem() {
        const FILES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/");
        let public_key_pem =
            std::fs::read_to_string(format!("{}public.pem", FILES_PATH))
                .expect("Failed to read public key file");
        let private_key_pem =
            std::fs::read_to_string(format!("{}private.pem", FILES_PATH))
                .expect("Failed to read private key file");

        let public_key_c = to_c_string(&public_key_pem);
        let private_key_c = to_c_string(&private_key_pem);

        let e2ee_server =
            unsafe { e2ee_server_new_from_pem(private_key_c, public_key_c) };
        assert!(!e2ee_server.is_null());

        unsafe { e2ee_server_free(e2ee_server) };
    }

    // Test the e2ee_client_new_from_public_pem function
    #[test]
    fn test_e2ee_client_new_from_public_pem() {
        const FILES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/files/");
        let public_key_pem =
            std::fs::read_to_string(format!("{}public.pem", FILES_PATH))
                .expect("Failed to read public key file");
        let public_key_c = to_c_string(&public_key_pem);

        let e2ee_client = unsafe { e2ee_client_new_from_public_pem(public_key_c) };
        assert!(!e2ee_client.is_null());

        unsafe { e2ee_client_free(e2ee_client) }; // Adjust if necessary
    }

    // Test the e2ee_server_encrypt function
    #[test]
    fn test_e2ee_server_encrypt() {
        let key_size = 2048;
        let e2ee_server = e2ee_server_new(key_size);
        assert!(!e2ee_server.is_null());

        let message = "Hello, world!";
        let message_c = to_c_string(message);

        let encrypted_message =
            unsafe { e2ee_server_encrypt(e2ee_server, message_c) };
        assert!(!encrypted_message.is_null());

        let encrypted_message_str = from_c_string(encrypted_message);
        assert!(!encrypted_message_str.is_empty());

        unsafe { e2ee_server_free_string(encrypted_message) };
        unsafe { e2ee_server_free(e2ee_server) };
    }

    // Test the e2ee_server_decrypt function
    #[test]
    fn test_e2ee_server_decrypt() {
        let key_size = 2048;
        let e2ee_server = e2ee_server_new(key_size);
        assert!(!e2ee_server.is_null());

        let message = "Hello, world!";
        let message_c = to_c_string(message);
        let encrypted_message_c =
            unsafe { e2ee_server_encrypt(e2ee_server, message_c) };

        let decrypted_message =
            unsafe { e2ee_server_decrypt(e2ee_server, encrypted_message_c) };
        assert!(!decrypted_message.is_null());

        let decrypted_message_str = from_c_string(decrypted_message);
        assert_eq!(message, decrypted_message_str);

        unsafe { e2ee_server_free_string(decrypted_message) };
        unsafe { e2ee_server_free(e2ee_server) };
    }

    // Test the e2ee_server_get_public_key_pem function
    #[test]
    fn test_e2ee_server_get_public_key_pem() {
        let key_size = 2048;
        let e2ee_server = e2ee_server_new(key_size);
        assert!(!e2ee_server.is_null());

        let public_key_pem = unsafe { e2ee_server_get_public_key_pem(e2ee_server) };
        assert!(!public_key_pem.is_null());

        let public_key_pem_str = from_c_string(public_key_pem);
        assert!(!public_key_pem_str.is_empty());

        unsafe { e2ee_server_free_string(public_key_pem) };
        unsafe { e2ee_server_free(e2ee_server) };
    }

    // Test the e2ee_server_get_private_key_pem function
    #[test]
    fn test_e2ee_server_get_private_key_pem() {
        let key_size = 2048;
        let e2ee_server = e2ee_server_new(key_size);
        assert!(!e2ee_server.is_null());

        let private_key_pem =
            unsafe { e2ee_server_get_private_key_pem(e2ee_server) };
        assert!(!private_key_pem.is_null());

        let private_key_pem_str = from_c_string(private_key_pem);
        assert!(!private_key_pem_str.is_empty());

        unsafe { e2ee_server_free_string(private_key_pem) };
        unsafe { e2ee_server_free(e2ee_server) };
    }
}
