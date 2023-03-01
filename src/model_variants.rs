
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ModelId {
#[serde(rename = "ada")]
Ada,
#[serde(rename = "ada-code-search-code")]
AdaCodeSearchCode,
#[serde(rename = "ada-code-search-text")]
AdaCodeSearchText,
#[serde(rename = "ada-search-document")]
AdaSearchDocument,
#[serde(rename = "ada-search-query")]
AdaSearchQuery,
#[serde(rename = "ada-similarity")]
AdaSimilarity,
#[serde(rename = "ada:2020-05-03")]
AdaColon20200503,
#[serde(rename = "audio-transcribe-deprecated")]
AudioTranscribeDeprecated,
#[serde(rename = "babbage")]
Babbage,
#[serde(rename = "babbage-code-search-code")]
BabbageCodeSearchCode,
#[serde(rename = "babbage-code-search-text")]
BabbageCodeSearchText,
#[serde(rename = "babbage-search-document")]
BabbageSearchDocument,
#[serde(rename = "babbage-search-query")]
BabbageSearchQuery,
#[serde(rename = "babbage-similarity")]
BabbageSimilarity,
#[serde(rename = "babbage:2020-05-03")]
BabbageColon20200503,
#[serde(rename = "code-cushman-001")]
CodeCushman001,
#[serde(rename = "code-davinci-002")]
CodeDavinci002,
#[serde(rename = "code-davinci-edit-001")]
CodeDavinciEdit001,
#[serde(rename = "code-search-ada-code-001")]
CodeSearchAdaCode001,
#[serde(rename = "code-search-ada-text-001")]
CodeSearchAdaText001,
#[serde(rename = "code-search-babbage-code-001")]
CodeSearchBabbageCode001,
#[serde(rename = "code-search-babbage-text-001")]
CodeSearchBabbageText001,
#[serde(rename = "curie")]
Curie,
#[serde(rename = "curie-instruct-beta")]
CurieInstructBeta,
#[serde(rename = "curie-search-document")]
CurieSearchDocument,
#[serde(rename = "curie-search-query")]
CurieSearchQuery,
#[serde(rename = "curie-similarity")]
CurieSimilarity,
#[serde(rename = "curie:2020-05-03")]
CurieColon20200503,
#[serde(rename = "cushman:2020-05-03")]
CushmanColon20200503,
#[serde(rename = "davinci")]
Davinci,
#[serde(rename = "davinci-if:3.0.0")]
DavinciIfColon3Period0Period0,
#[serde(rename = "davinci-instruct-beta")]
DavinciInstructBeta,
#[serde(rename = "davinci-instruct-beta:2.0.0")]
DavinciInstructBetaColon2Period0Period0,
#[serde(rename = "davinci-search-document")]
DavinciSearchDocument,
#[serde(rename = "davinci-search-query")]
DavinciSearchQuery,
#[serde(rename = "davinci-similarity")]
DavinciSimilarity,
#[serde(rename = "davinci:2020-05-03")]
DavinciColon20200503,
#[serde(rename = "gpt-3.5-turbo")]
Gpt3Period5Turbo,
#[serde(rename = "gpt-3.5-turbo-0301")]
Gpt3Period5Turbo0301,
#[serde(rename = "if-curie-v2")]
IfCurieV2,
#[serde(rename = "if-davinci-v2")]
IfDavinciV2,
#[serde(rename = "if-davinci:3.0.0")]
IfDavinciColon3Period0Period0,
#[serde(rename = "text-ada-001")]
TextAda001,
#[serde(rename = "text-ada:001")]
TextAdaColon001,
#[serde(rename = "text-babbage-001")]
TextBabbage001,
#[serde(rename = "text-babbage:001")]
TextBabbageColon001,
#[serde(rename = "text-curie-001")]
TextCurie001,
#[serde(rename = "text-curie:001")]
TextCurieColon001,
#[serde(rename = "text-davinci-001")]
TextDavinci001,
#[serde(rename = "text-davinci-002")]
TextDavinci002,
#[serde(rename = "text-davinci-003")]
TextDavinci003,
#[serde(rename = "text-davinci-edit-001")]
TextDavinciEdit001,
#[serde(rename = "text-davinci-insert-001")]
TextDavinciInsert001,
#[serde(rename = "text-davinci-insert-002")]
TextDavinciInsert002,
#[serde(rename = "text-davinci:001")]
TextDavinciColon001,
#[serde(rename = "text-embedding-ada-002")]
TextEmbeddingAda002,
#[serde(rename = "text-search-ada-doc-001")]
TextSearchAdaDoc001,
#[serde(rename = "text-search-ada-query-001")]
TextSearchAdaQuery001,
#[serde(rename = "text-search-babbage-doc-001")]
TextSearchBabbageDoc001,
#[serde(rename = "text-search-babbage-query-001")]
TextSearchBabbageQuery001,
#[serde(rename = "text-search-curie-doc-001")]
TextSearchCurieDoc001,
#[serde(rename = "text-search-curie-query-001")]
TextSearchCurieQuery001,
#[serde(rename = "text-search-davinci-doc-001")]
TextSearchDavinciDoc001,
#[serde(rename = "text-search-davinci-query-001")]
TextSearchDavinciQuery001,
#[serde(rename = "text-similarity-ada-001")]
TextSimilarityAda001,
#[serde(rename = "text-similarity-babbage-001")]
TextSimilarityBabbage001,
#[serde(rename = "text-similarity-curie-001")]
TextSimilarityCurie001,
#[serde(rename = "text-similarity-davinci-001")]
TextSimilarityDavinci001,
#[serde(rename = "whisper-1")]
Whisper1,

}

impl core::fmt::Display for ModelId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
ModelId::Ada => "ada",
ModelId::AdaCodeSearchCode => "ada-code-search-code",
ModelId::AdaCodeSearchText => "ada-code-search-text",
ModelId::AdaSearchDocument => "ada-search-document",
ModelId::AdaSearchQuery => "ada-search-query",
ModelId::AdaSimilarity => "ada-similarity",
ModelId::AdaColon20200503 => "ada:2020-05-03",
ModelId::AudioTranscribeDeprecated => "audio-transcribe-deprecated",
ModelId::Babbage => "babbage",
ModelId::BabbageCodeSearchCode => "babbage-code-search-code",
ModelId::BabbageCodeSearchText => "babbage-code-search-text",
ModelId::BabbageSearchDocument => "babbage-search-document",
ModelId::BabbageSearchQuery => "babbage-search-query",
ModelId::BabbageSimilarity => "babbage-similarity",
ModelId::BabbageColon20200503 => "babbage:2020-05-03",
ModelId::CodeCushman001 => "code-cushman-001",
ModelId::CodeDavinci002 => "code-davinci-002",
ModelId::CodeDavinciEdit001 => "code-davinci-edit-001",
ModelId::CodeSearchAdaCode001 => "code-search-ada-code-001",
ModelId::CodeSearchAdaText001 => "code-search-ada-text-001",
ModelId::CodeSearchBabbageCode001 => "code-search-babbage-code-001",
ModelId::CodeSearchBabbageText001 => "code-search-babbage-text-001",
ModelId::Curie => "curie",
ModelId::CurieInstructBeta => "curie-instruct-beta",
ModelId::CurieSearchDocument => "curie-search-document",
ModelId::CurieSearchQuery => "curie-search-query",
ModelId::CurieSimilarity => "curie-similarity",
ModelId::CurieColon20200503 => "curie:2020-05-03",
ModelId::CushmanColon20200503 => "cushman:2020-05-03",
ModelId::Davinci => "davinci",
ModelId::DavinciIfColon3Period0Period0 => "davinci-if:3.0.0",
ModelId::DavinciInstructBeta => "davinci-instruct-beta",
ModelId::DavinciInstructBetaColon2Period0Period0 => "davinci-instruct-beta:2.0.0",
ModelId::DavinciSearchDocument => "davinci-search-document",
ModelId::DavinciSearchQuery => "davinci-search-query",
ModelId::DavinciSimilarity => "davinci-similarity",
ModelId::DavinciColon20200503 => "davinci:2020-05-03",
ModelId::Gpt3Period5Turbo => "gpt-3.5-turbo",
ModelId::Gpt3Period5Turbo0301 => "gpt-3.5-turbo-0301",
ModelId::IfCurieV2 => "if-curie-v2",
ModelId::IfDavinciV2 => "if-davinci-v2",
ModelId::IfDavinciColon3Period0Period0 => "if-davinci:3.0.0",
ModelId::TextAda001 => "text-ada-001",
ModelId::TextAdaColon001 => "text-ada:001",
ModelId::TextBabbage001 => "text-babbage-001",
ModelId::TextBabbageColon001 => "text-babbage:001",
ModelId::TextCurie001 => "text-curie-001",
ModelId::TextCurieColon001 => "text-curie:001",
ModelId::TextDavinci001 => "text-davinci-001",
ModelId::TextDavinci002 => "text-davinci-002",
ModelId::TextDavinci003 => "text-davinci-003",
ModelId::TextDavinciEdit001 => "text-davinci-edit-001",
ModelId::TextDavinciInsert001 => "text-davinci-insert-001",
ModelId::TextDavinciInsert002 => "text-davinci-insert-002",
ModelId::TextDavinciColon001 => "text-davinci:001",
ModelId::TextEmbeddingAda002 => "text-embedding-ada-002",
ModelId::TextSearchAdaDoc001 => "text-search-ada-doc-001",
ModelId::TextSearchAdaQuery001 => "text-search-ada-query-001",
ModelId::TextSearchBabbageDoc001 => "text-search-babbage-doc-001",
ModelId::TextSearchBabbageQuery001 => "text-search-babbage-query-001",
ModelId::TextSearchCurieDoc001 => "text-search-curie-doc-001",
ModelId::TextSearchCurieQuery001 => "text-search-curie-query-001",
ModelId::TextSearchDavinciDoc001 => "text-search-davinci-doc-001",
ModelId::TextSearchDavinciQuery001 => "text-search-davinci-query-001",
ModelId::TextSimilarityAda001 => "text-similarity-ada-001",
ModelId::TextSimilarityBabbage001 => "text-similarity-babbage-001",
ModelId::TextSimilarityCurie001 => "text-similarity-curie-001",
ModelId::TextSimilarityDavinci001 => "text-similarity-davinci-001",
ModelId::Whisper1 => "whisper-1",

        };
        write!(f, "{}", s)
    }
}