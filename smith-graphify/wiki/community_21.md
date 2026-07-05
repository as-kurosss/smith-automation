# Community 21: test_with_model_override()

**Members:** 11

## Nodes

- **provider** (`crates_smith_ai_src_provider_rs`, File, degree: 6)
- **super::*** (`crates_smith_ai_src_provider_rs_import_super`, Module, degree: 1)
- **ProviderConfig** (`crates_smith_ai_src_provider_rs_providerconfig`, Enum, degree: 5)
- **.anthropic()** (`crates_smith_ai_src_provider_rs_providerconfig_anthropic`, Method, degree: 2)
- **.openai()** (`crates_smith_ai_src_provider_rs_providerconfig_openai`, Method, degree: 4)
- **.with_base_url()** (`crates_smith_ai_src_provider_rs_providerconfig_with_base_url`, Method, degree: 2)
- **.with_model()** (`crates_smith_ai_src_provider_rs_providerconfig_with_model`, Method, degree: 2)
- **test_anthropic_default_model()** (`crates_smith_ai_src_provider_rs_test_anthropic_default_model`, Function, degree: 2)
- **test_openai_default_model()** (`crates_smith_ai_src_provider_rs_test_openai_default_model`, Function, degree: 2)
- **test_with_base_url()** (`crates_smith_ai_src_provider_rs_test_with_base_url`, Function, degree: 3)
- **test_with_model_override()** (`crates_smith_ai_src_provider_rs_test_with_model_override`, Function, degree: 3)

## Relationships

- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_providerconfig (defines)
- crates_smith_ai_src_provider_rs_providerconfig → crates_smith_ai_src_provider_rs_providerconfig_openai (defines)
- crates_smith_ai_src_provider_rs_providerconfig → crates_smith_ai_src_provider_rs_providerconfig_anthropic (defines)
- crates_smith_ai_src_provider_rs_providerconfig → crates_smith_ai_src_provider_rs_providerconfig_with_model (defines)
- crates_smith_ai_src_provider_rs_providerconfig → crates_smith_ai_src_provider_rs_providerconfig_with_base_url (defines)
- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_import_super (imports)
- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_test_openai_default_model (defines)
- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_test_anthropic_default_model (defines)
- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_test_with_model_override (defines)
- crates_smith_ai_src_provider_rs → crates_smith_ai_src_provider_rs_test_with_base_url (defines)
- crates_smith_ai_src_provider_rs_test_openai_default_model → crates_smith_ai_src_provider_rs_providerconfig_openai (calls)
- crates_smith_ai_src_provider_rs_test_anthropic_default_model → crates_smith_ai_src_provider_rs_providerconfig_anthropic (calls)
- crates_smith_ai_src_provider_rs_test_with_model_override → crates_smith_ai_src_provider_rs_providerconfig_with_model (calls)
- crates_smith_ai_src_provider_rs_test_with_model_override → crates_smith_ai_src_provider_rs_providerconfig_openai (calls)
- crates_smith_ai_src_provider_rs_test_with_base_url → crates_smith_ai_src_provider_rs_providerconfig_with_base_url (calls)
- crates_smith_ai_src_provider_rs_test_with_base_url → crates_smith_ai_src_provider_rs_providerconfig_openai (calls)

