#ifndef NUX_CRYPTO_ENCRYPTION_H
#define NUX_CRYPTO_ENCRYPTION_H

#include <string>
#include <vector>

namespace NuxCrypto {

// Hash functions
class Hash {
public:
    static std::string SHA256(const std::string& input);
    static std::string SHA512(const std::string& input);
    static std::string MD5(const std::string& input);
    static std::string BLAKE2(const std::string& input);
};

// Symmetric encryption
class AES {
public:
    AES(const std::string& key);
    
    std::string Encrypt(const std::string& plaintext);
    std::string Decrypt(const std::string& ciphertext);
    
private:
    std::string m_Key;
};

// Asymmetric encryption
class RSA {
public:
    struct KeyPair {
        std::string publicKey;
        std::string privateKey;
    };
    
    static KeyPair GenerateKeyPair(int keySize = 2048);
    
    static std::string Encrypt(const std::string& plaintext, const std::string& publicKey);
    static std::string Decrypt(const std::string& ciphertext, const std::string& privateKey);
    
    static std::string Sign(const std::string& message, const std::string& privateKey);
    static bool Verify(const std::string& message, const std::string& signature,
                      const std::string& publicKey);
};

// Elliptic curve cryptography
class ECC {
public:
    struct KeyPair {
        std::string publicKey;
        std::string privateKey;
    };
    
    static KeyPair GenerateKeyPair();
    
    static std::string Sign(const std::string& message, const std::string& privateKey);
    static bool Verify(const std::string& message, const std::string& signature,
                      const std::string& publicKey);
    
    static std::string SharedSecret(const std::string& privateKey,
                                    const std::string& publicKey);
};

// Password hashing
class PasswordHash {
public:
    static std::string PBKDF2(const std::string& password, const std::string& salt,
                             int iterations = 100000);
    static std::string Bcrypt(const std::string& password, int cost = 12);
    static std::string Argon2(const std::string& password, const std::string& salt);
    
    static bool Verify(const std::string& password, const std::string& hash);
};

// Random number generation
class SecureRandom {
public:
    static std::vector<uint8_t> Bytes(int count);
    static std::string String(int length);
    static int Int(int min, int max);
};

} // namespace NuxCrypto

#endif // NUX_CRYPTO_ENCRYPTION_H
