#ifndef NUX_NLP_TOKENIZER_H
#define NUX_NLP_TOKENIZER_H

#include <string>
#include <vector>
#include <map>
#include <memory>

namespace NuxNLP {

// Token representation
struct Token {
    std::string text;
    int start;
    int end;
    std::string pos;  // Part of speech
    std::string lemma;
};

// Tokenizer
class Tokenizer {
public:
    virtual ~Tokenizer() = default;
    virtual std::vector<Token> Tokenize(const std::string& text) = 0;
};

class WhitespaceTokenizer : public Tokenizer {
public:
    std::vector<Token> Tokenize(const std::string& text) override;
};

class WordTokenizer : public Tokenizer {
public:
    std::vector<Token> Tokenize(const std::string& text) override;
};

// Named Entity Recognition
struct Entity {
    std::string text;
    std::string label;  // PERSON, ORG, LOC, etc.
    int start;
    int end;
};

class NER {
public:
    virtual ~NER() = default;
    virtual std::vector<Entity> Extract(const std::string& text) = 0;
};

// Sentiment Analysis
enum class Sentiment {
    POSITIVE,
    NEGATIVE,
    NEUTRAL
};

class SentimentAnalyzer {
public:
    SentimentAnalyzer();
    ~SentimentAnalyzer();
    
    Sentiment Analyze(const std::string& text);
    double Score(const std::string& text);  // -1.0 to 1.0
    
private:
    std::map<std::string, double> m_Lexicon;
    void LoadLexicon();
};

// Text Classification
class TextClassifier {
public:
    TextClassifier();
    ~TextClassifier();
    
    void Train(const std::vector<std::string>& texts, 
               const std::vector<int>& labels);
    int Predict(const std::string& text);
    std::vector<double> PredictProba(const std::string& text);
    
private:
    std::map<std::string, std::map<int, int>> m_WordCounts;
    std::map<int, int> m_ClassCounts;
    int m_NumClasses;
};

// Word Embeddings
class WordEmbeddings {
public:
    WordEmbeddings(int dimensions = 100);
    ~WordEmbeddings();
    
    void Train(const std::vector<std::string>& corpus, int windowSize = 5);
    std::vector<double> GetVector(const std::string& word) const;
    double Similarity(const std::string& word1, const std::string& word2) const;
    std::vector<std::string> MostSimilar(const std::string& word, int topK = 10) const;
    
private:
    int m_Dimensions;
    std::map<std::string, std::vector<double>> m_Embeddings;
};

// Language Model
class LanguageModel {
public:
    LanguageModel(int ngramSize = 2);
    ~LanguageModel();
    
    void Train(const std::vector<std::string>& corpus);
    double Probability(const std::string& text);
    std::string Generate(int maxLength = 50);
    
private:
    int m_NgramSize;
    std::map<std::vector<std::string>, std::map<std::string, int>> m_Ngrams;
};

// Text preprocessing
class TextPreprocessor {
public:
    static std::string ToLower(const std::string& text);
    static std::string RemovePunctuation(const std::string& text);
    static std::vector<std::string> RemoveStopwords(const std::vector<std::string>& tokens);
    static std::string Stem(const std::string& word);
    static std::string Lemmatize(const std::string& word);
};

} // namespace NuxNLP

#endif // NUX_NLP_TOKENIZER_H
