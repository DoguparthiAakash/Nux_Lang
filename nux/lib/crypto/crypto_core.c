// C - Cryptography Core Implementation
// High-performance encryption/decryption

#include <stdint.h>
#include <string.h>
#include <stdlib.h>

// External assembly functions
extern void aes_encrypt_block_asm(uint8_t* plaintext, uint8_t* ciphertext, uint8_t* key);
extern void aes_decrypt_block_asm(uint8_t* ciphertext, uint8_t* plaintext, uint8_t* key);
extern void aes_keygen_asm(uint8_t* key, uint8_t* expanded_key);
extern void chacha20_encrypt_asm(uint8_t* state, uint8_t* plaintext, uint8_t* ciphertext);
extern void sha256_hash_asm(uint8_t* message, uint8_t* hash);

// AES-256 CTR Mode (Counter Mode)
typedef struct {
    uint8_t key[32];
    uint8_t expanded_key[240];  // 14 rounds for AES-256
    uint8_t counter[16];
} AES256_CTR;

void aes256_ctr_init(AES256_CTR* ctx, const uint8_t* key, const uint8_t* nonce) {
    memcpy(ctx->key, key, 32);
    aes_keygen_asm(ctx->key, ctx->expanded_key);
    
    // Initialize counter with nonce
    memcpy(ctx->counter, nonce, 12);
    memset(ctx->counter + 12, 0, 4);
}

void aes256_ctr_encrypt(AES256_CTR* ctx, const uint8_t* plaintext, 
                        uint8_t* ciphertext, size_t length) {
    uint8_t keystream[16];
    size_t i;
    
    for (i = 0; i < length; i += 16) {
        // Encrypt counter to generate keystream
        aes_encrypt_block_asm(ctx->counter, keystream, ctx->expanded_key);
        
        // XOR plaintext with keystream
        size_t block_size = (length - i < 16) ? length - i : 16;
        for (size_t j = 0; j < block_size; j++) {
            ciphertext[i + j] = plaintext[i + j] ^ keystream[j];
        }
        
        // Increment counter
        for (int k = 15; k >= 12; k--) {
            if (++ctx->counter[k] != 0) break;
        }
    }
}

// Same function for decryption (CTR mode is symmetric)
#define aes256_ctr_decrypt aes256_ctr_encrypt

// ChaCha20 Stream Cipher
typedef struct {
    uint32_t state[16];
    uint32_t counter;
} ChaCha20;

void chacha20_init(ChaCha20* ctx, const uint8_t* key, const uint8_t* nonce) {
    // Constants
    ctx->state[0] = 0x61707865;
    ctx->state[1] = 0x3320646e;
    ctx->state[2] = 0x79622d32;
    ctx->state[3] = 0x6b206574;
    
    // Key (256-bit)
    memcpy(&ctx->state[4], key, 32);
    
    // Counter
    ctx->counter = 0;
    ctx->state[12] = 0;
    
    // Nonce (96-bit)
    memcpy(&ctx->state[13], nonce, 12);
}

void chacha20_encrypt(ChaCha20* ctx, const uint8_t* plaintext,
                      uint8_t* ciphertext, size_t length) {
    for (size_t i = 0; i < length; i += 64) {
        ctx->state[12] = ctx->counter++;
        
        chacha20_encrypt_asm((uint8_t*)ctx->state, 
                            (uint8_t*)(plaintext + i),
                            ciphertext + i);
    }
}

// RSA (simplified, use OpenSSL for production)
typedef struct {
    uint64_t n;  // Modulus
    uint64_t e;  // Public exponent
    uint64_t d;  // Private exponent
} RSA_Key;

uint64_t mod_exp(uint64_t base, uint64_t exp, uint64_t mod) {
    uint64_t result = 1;
    base %= mod;
    
    while (exp > 0) {
        if (exp & 1) {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp >>= 1;
    }
    
    return result;
}

uint64_t rsa_encrypt(uint64_t plaintext, RSA_Key* key) {
    return mod_exp(plaintext, key->e, key->n);
}

uint64_t rsa_decrypt(uint64_t ciphertext, RSA_Key* key) {
    return mod_exp(ciphertext, key->d, key->n);
}

// HMAC-SHA256
void hmac_sha256(const uint8_t* key, size_t key_len,
                 const uint8_t* message, size_t msg_len,
                 uint8_t* mac) {
    uint8_t k_pad[64];
    uint8_t i_key_pad[64];
    uint8_t o_key_pad[64];
    
    // Prepare key
    memset(k_pad, 0, 64);
    if (key_len <= 64) {
        memcpy(k_pad, key, key_len);
    } else {
        sha256_hash_asm((uint8_t*)key, k_pad);
    }
    
    // Inner padding
    for (int i = 0; i < 64; i++) {
        i_key_pad[i] = k_pad[i] ^ 0x36;
        o_key_pad[i] = k_pad[i] ^ 0x5c;
    }
    
    // Inner hash
    uint8_t inner_hash[32];
    uint8_t* inner_msg = malloc(64 + msg_len);
    memcpy(inner_msg, i_key_pad, 64);
    memcpy(inner_msg + 64, message, msg_len);
    sha256_hash_asm(inner_msg, inner_hash);
    free(inner_msg);
    
    // Outer hash
    uint8_t outer_msg[64 + 32];
    memcpy(outer_msg, o_key_pad, 64);
    memcpy(outer_msg + 64, inner_hash, 32);
    sha256_hash_asm(outer_msg, mac);
}

// Password-Based Key Derivation (PBKDF2)
void pbkdf2_sha256(const uint8_t* password, size_t pass_len,
                   const uint8_t* salt, size_t salt_len,
                   int iterations, uint8_t* key, size_t key_len) {
    uint8_t block[32];
    uint8_t temp[32];
    
    for (size_t i = 0; i < key_len; i += 32) {
        // First iteration
        uint8_t* msg = malloc(salt_len + 4);
        memcpy(msg, salt, salt_len);
        uint32_t block_num = (i / 32) + 1;
        msg[salt_len] = (block_num >> 24) & 0xff;
        msg[salt_len + 1] = (block_num >> 16) & 0xff;
        msg[salt_len + 2] = (block_num >> 8) & 0xff;
        msg[salt_len + 3] = block_num & 0xff;
        
        hmac_sha256(password, pass_len, msg, salt_len + 4, block);
        memcpy(temp, block, 32);
        free(msg);
        
        // Remaining iterations
        for (int j = 1; j < iterations; j++) {
            hmac_sha256(password, pass_len, temp, 32, temp);
            for (int k = 0; k < 32; k++) {
                block[k] ^= temp[k];
            }
        }
        
        size_t copy_len = (key_len - i < 32) ? key_len - i : 32;
        memcpy(key + i, block, copy_len);
    }
}

// Secure random number generation
void secure_random(uint8_t* buffer, size_t length) {
    FILE* urandom = fopen("/dev/urandom", "rb");
    if (urandom) {
        fread(buffer, 1, length, urandom);
        fclose(urandom);
    }
}

// Constant-time comparison (prevents timing attacks)
int constant_time_compare(const uint8_t* a, const uint8_t* b, size_t length) {
    uint8_t result = 0;
    for (size_t i = 0; i < length; i++) {
        result |= a[i] ^ b[i];
    }
    return result == 0;
}
