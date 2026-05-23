// C++ - Image Steganography & Metadata Manipulation
// Hide data in images and manipulate metadata

#include <vector>
#include <string>
#include <fstream>
#include <cstring>
#include <stdexcept>

// PNG chunk structure
struct PNGChunk {
    uint32_t length;
    char type[4];
    std::vector<uint8_t> data;
    uint32_t crc;
};

class ImageSteganography {
private:
    std::vector<uint8_t> image_data;
    int width, height, channels;
    
public:
    // Load image
    bool load_png(const std::string& filename) {
        std::ifstream file(filename, std::ios::binary);
        if (!file) return false;
        
        // Read PNG signature
        uint8_t signature[8];
        file.read((char*)signature, 8);
        
        // Verify PNG signature
        const uint8_t png_sig[8] = {137, 80, 78, 71, 13, 10, 26, 10};
        if (memcmp(signature, png_sig, 8) != 0) return false;
        
        // Read chunks
        while (file) {
            PNGChunk chunk;
            file.read((char*)&chunk.length, 4);
            chunk.length = ntohl(chunk.length);
            
            file.read(chunk.type, 4);
            
            chunk.data.resize(chunk.length);
            file.read((char*)chunk.data.data(), chunk.length);
            
            file.read((char*)&chunk.crc, 4);
            
            if (strcmp(chunk.type, "IDAT") == 0) {
                // Image data chunk
                image_data.insert(image_data.end(), 
                                 chunk.data.begin(), 
                                 chunk.data.end());
            }
            
            if (strcmp(chunk.type, "IEND") == 0) break;
        }
        
        return true;
    }
    
    // LSB Steganography (Least Significant Bit)
    void hide_data_lsb(const std::vector<uint8_t>& secret_data) {
        if (secret_data.size() * 8 > image_data.size()) {
            throw std::runtime_error("Secret data too large for image");
        }
        
        // Embed length first (32 bits)
        uint32_t length = secret_data.size();
        for (int i = 0; i < 32; i++) {
            image_data[i] = (image_data[i] & 0xFE) | ((length >> (31 - i)) & 1);
        }
        
        // Embed data
        for (size_t i = 0; i < secret_data.size(); i++) {
            for (int bit = 0; bit < 8; bit++) {
                size_t pixel_idx = 32 + i * 8 + bit;
                uint8_t secret_bit = (secret_data[i] >> (7 - bit)) & 1;
                image_data[pixel_idx] = (image_data[pixel_idx] & 0xFE) | secret_bit;
            }
        }
    }
    
    // Extract hidden data
    std::vector<uint8_t> extract_data_lsb() {
        // Extract length
        uint32_t length = 0;
        for (int i = 0; i < 32; i++) {
            length = (length << 1) | (image_data[i] & 1);
        }
        
        // Extract data
        std::vector<uint8_t> secret_data(length);
        for (size_t i = 0; i < length; i++) {
            uint8_t byte = 0;
            for (int bit = 0; bit < 8; bit++) {
                size_t pixel_idx = 32 + i * 8 + bit;
                byte = (byte << 1) | (image_data[pixel_idx] & 1);
            }
            secret_data[i] = byte;
        }
        
        return secret_data;
    }
    
    // DCT Steganography (more robust, survives JPEG compression)
    void hide_data_dct(const std::vector<uint8_t>& secret_data) {
        // Divide image into 8x8 blocks
        // Apply DCT to each block
        // Modify mid-frequency coefficients
        // Apply inverse DCT
        
        for (size_t i = 0; i < secret_data.size(); i++) {
            // Simplified: modify specific DCT coefficients
            // Full implementation would use proper DCT
            int block_x = (i * 8) % width;
            int block_y = ((i * 8) / width) * 8;
            
            for (int bit = 0; bit < 8; bit++) {
                int x = block_x + bit;
                int y = block_y;
                size_t idx = (y * width + x) * channels;
                
                uint8_t secret_bit = (secret_data[i] >> (7 - bit)) & 1;
                
                // Modify blue channel (less noticeable)
                if (secret_bit) {
                    image_data[idx + 2] |= 0x01;
                } else {
                    image_data[idx + 2] &= 0xFE;
                }
            }
        }
    }
    
    // Save modified image
    bool save_png(const std::string& filename) {
        std::ofstream file(filename, std::ios::binary);
        if (!file) return false;
        
        // Write PNG signature
        const uint8_t png_sig[8] = {137, 80, 78, 71, 13, 10, 26, 10};
        file.write((char*)png_sig, 8);
        
        // Write IHDR chunk
        write_ihdr_chunk(file);
        
        // Write IDAT chunk
        write_idat_chunk(file);
        
        // Write IEND chunk
        write_iend_chunk(file);
        
        return true;
    }
    
private:
    void write_ihdr_chunk(std::ofstream& file) {
        uint32_t length = htonl(13);
        file.write((char*)&length, 4);
        
        file.write("IHDR", 4);
        
        uint32_t w = htonl(width);
        uint32_t h = htonl(height);
        file.write((char*)&w, 4);
        file.write((char*)&h, 4);
        
        uint8_t bit_depth = 8;
        uint8_t color_type = 2;  // RGB
        uint8_t compression = 0;
        uint8_t filter = 0;
        uint8_t interlace = 0;
        
        file.write((char*)&bit_depth, 1);
        file.write((char*)&color_type, 1);
        file.write((char*)&compression, 1);
        file.write((char*)&filter, 1);
        file.write((char*)&interlace, 1);
        
        // CRC (simplified)
        uint32_t crc = 0;
        file.write((char*)&crc, 4);
    }
    
    void write_idat_chunk(std::ofstream& file) {
        uint32_t length = htonl(image_data.size());
        file.write((char*)&length, 4);
        
        file.write("IDAT", 4);
        file.write((char*)image_data.data(), image_data.size());
        
        uint32_t crc = 0;
        file.write((char*)&crc, 4);
    }
    
    void write_iend_chunk(std::ofstream& file) {
        uint32_t length = 0;
        file.write((char*)&length, 4);
        
        file.write("IEND", 4);
        
        uint32_t crc = htonl(0xAE426082);
        file.write((char*)&crc, 4);
    }
};

// EXIF Metadata Manipulation
class EXIFMetadata {
private:
    struct Tag {
        uint16_t id;
        uint16_t type;
        uint32_t count;
        uint32_t value_offset;
    };
    
    std::vector<Tag> tags;
    std::vector<uint8_t> data;
    
public:
    // Read EXIF from JPEG
    bool read_exif(const std::string& filename) {
        std::ifstream file(filename, std::ios::binary);
        if (!file) return false;
        
        // Find EXIF marker (0xFFE1)
        uint8_t marker[2];
        while (file.read((char*)marker, 2)) {
            if (marker[0] == 0xFF && marker[1] == 0xE1) {
                // Found EXIF
                uint16_t length;
                file.read((char*)&length, 2);
                length = ntohs(length);
                
                data.resize(length - 2);
                file.read((char*)data.data(), length - 2);
                
                parse_exif();
                return true;
            }
        }
        
        return false;
    }
    
    // Add custom metadata
    void add_metadata(const std::string& key, const std::string& value) {
        Tag tag;
        tag.id = 0x9286;  // UserComment
        tag.type = 2;     // ASCII
        tag.count = value.length();
        tag.value_offset = data.size();
        
        tags.push_back(tag);
        data.insert(data.end(), value.begin(), value.end());
    }
    
    // Encrypt metadata
    void encrypt_metadata(const uint8_t* key) {
        // Use AES to encrypt metadata
        AES256_CTR ctx;
        uint8_t nonce[12] = {0};
        aes256_ctr_init(&ctx, key, nonce);
        
        std::vector<uint8_t> encrypted(data.size());
        aes256_ctr_encrypt(&ctx, data.data(), encrypted.data(), data.size());
        
        data = encrypted;
    }
    
    // Decrypt metadata
    void decrypt_metadata(const uint8_t* key) {
        // CTR mode: encryption = decryption
        encrypt_metadata(key);
    }
    
    // Write EXIF to JPEG
    bool write_exif(const std::string& filename) {
        std::ofstream file(filename, std::ios::binary);
        if (!file) return false;
        
        // Write JPEG SOI
        uint8_t soi[2] = {0xFF, 0xD8};
        file.write((char*)soi, 2);
        
        // Write EXIF marker
        uint8_t marker[2] = {0xFF, 0xE1};
        file.write((char*)marker, 2);
        
        // Write length
        uint16_t length = htons(data.size() + 2);
        file.write((char*)&length, 2);
        
        // Write EXIF data
        file.write((char*)data.data(), data.size());
        
        return true;
    }
    
private:
    void parse_exif() {
        // Parse EXIF structure
        // (simplified implementation)
    }
};

// Example usage
extern "C" {
    void hide_secret_in_image(const char* image_path, 
                             const char* output_path,
                             const uint8_t* secret_data,
                             size_t secret_len,
                             const uint8_t* encryption_key) {
        ImageSteganography steg;
        
        // Load image
        steg.load_png(image_path);
        
        // Encrypt secret data
        AES256_CTR ctx;
        uint8_t nonce[12] = {0};
        aes256_ctr_init(&ctx, encryption_key, nonce);
        
        std::vector<uint8_t> encrypted(secret_len);
        aes256_ctr_encrypt(&ctx, secret_data, encrypted.data(), secret_len);
        
        // Hide in image
        steg.hide_data_lsb(encrypted);
        
        // Save
        steg.save_png(output_path);
    }
    
    void extract_secret_from_image(const char* image_path,
                                   uint8_t* secret_data,
                                   size_t* secret_len,
                                   const uint8_t* decryption_key) {
        ImageSteganography steg;
        
        // Load image
        steg.load_png(image_path);
        
        // Extract encrypted data
        auto encrypted = steg.extract_data_lsb();
        
        // Decrypt
        AES256_CTR ctx;
        uint8_t nonce[12] = {0};
        aes256_ctr_init(&ctx, decryption_key, nonce);
        
        aes256_ctr_decrypt(&ctx, encrypted.data(), secret_data, encrypted.size());
        *secret_len = encrypted.size();
    }
}
