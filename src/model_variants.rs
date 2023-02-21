#[derive(
    Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Ord, PartialOrd, Eq, PartialEq, Hash,
)]
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
}
