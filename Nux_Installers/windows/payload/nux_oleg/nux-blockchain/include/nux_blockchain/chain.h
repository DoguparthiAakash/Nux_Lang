#ifndef NUX_BLOCKCHAIN_CHAIN_H
#define NUX_BLOCKCHAIN_CHAIN_H

#include <string>
#include <vector>
#include <ctime>
#include <memory>

namespace NuxBlockchain {

// Transaction
struct Transaction {
    std::string sender;
    std::string recipient;
    double amount;
    time_t timestamp;
    std::string signature;
    
    std::string Hash() const;
};

// Block
class Block {
public:
    Block(int index, const std::vector<Transaction>& transactions,
          const std::string& previousHash);
    
    int Index() const { return m_Index; }
    time_t Timestamp() const { return m_Timestamp; }
    const std::vector<Transaction>& Transactions() const { return m_Transactions; }
    const std::string& PreviousHash() const { return m_PreviousHash; }
    const std::string& Hash() const { return m_Hash; }
    int Nonce() const { return m_Nonce; }
    
    // Mining
    void Mine(int difficulty);
    bool IsValid() const;
    
private:
    int m_Index;
    time_t m_Timestamp;
    std::vector<Transaction> m_Transactions;
    std::string m_PreviousHash;
    std::string m_Hash;
    int m_Nonce;
    
    std::string CalculateHash() const;
};

// Blockchain
class Blockchain {
public:
    Blockchain(int difficulty = 4);
    
    // Add transaction
    void AddTransaction(const Transaction& transaction);
    
    // Mine pending transactions
    void MinePendingTransactions(const std::string& minerAddress);
    
    // Get balance
    double GetBalance(const std::string& address) const;
    
    // Validation
    bool IsValid() const;
    
    // Chain info
    int Length() const { return m_Chain.size(); }
    const std::vector<Block>& Chain() const { return m_Chain; }
    
    void Print() const;
    
private:
    std::vector<Block> m_Chain;
    std::vector<Transaction> m_PendingTransactions;
    int m_Difficulty;
    double m_MiningReward;
    
    Block CreateGenesisBlock();
};

// Smart contract
class SmartContract {
public:
    SmartContract(const std::string& code);
    
    void Execute(const std::map<std::string, std::string>& params);
    std::string GetState(const std::string& key) const;
    
private:
    std::string m_Code;
    std::map<std::string, std::string> m_State;
};

// Wallet
class Wallet {
public:
    Wallet();
    
    std::string PublicKey() const { return m_PublicKey; }
    std::string PrivateKey() const { return m_PrivateKey; }
    
    std::string Sign(const std::string& message) const;
    static bool Verify(const std::string& message, const std::string& signature,
                      const std::string& publicKey);
    
private:
    std::string m_PublicKey;
    std::string m_PrivateKey;
    
    void GenerateKeys();
};

} // namespace NuxBlockchain

#endif // NUX_BLOCKCHAIN_CHAIN_H
