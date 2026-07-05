# Community 11: test_decide_valid_choice()

**Members:** 13

## Nodes

- **.prompt()** (`crates_smith_ai_src_agent_rs_agent_m_p_prompt`, Method, degree: 2)
- **make_agent()** (`crates_smith_ai_src_agent_rs_make_agent`, Function, degree: 10)
- **.prompt()** (`crates_smith_ai_src_agent_rs_mockagent_prompt`, Method, degree: 8)
- **.agent_run()** (`crates_smith_ai_src_agent_rs_smithagent_agent_run`, Method, degree: 4)
- **.decide()** (`crates_smith_ai_src_agent_rs_smithagent_decide`, Method, degree: 7)
- **.prompt()** (`crates_smith_ai_src_agent_rs_smithagent_prompt`, Method, degree: 3)
- **test_agent_run_parses_json()** (`crates_smith_ai_src_agent_rs_test_agent_run_parses_json`, Function, degree: 3)
- **test_agent_run_returns_plain_text()** (`crates_smith_ai_src_agent_rs_test_agent_run_returns_plain_text`, Function, degree: 3)
- **test_decide_cancelled()** (`crates_smith_ai_src_agent_rs_test_decide_cancelled`, Function, degree: 3)
- **test_decide_empty_options()** (`crates_smith_ai_src_agent_rs_test_decide_empty_options`, Function, degree: 3)
- **test_decide_invalid_choice()** (`crates_smith_ai_src_agent_rs_test_decide_invalid_choice`, Function, degree: 3)
- **test_decide_trims_quotes()** (`crates_smith_ai_src_agent_rs_test_decide_trims_quotes`, Function, degree: 3)
- **test_decide_valid_choice()** (`crates_smith_ai_src_agent_rs_test_decide_valid_choice`, Function, degree: 3)

## Relationships

- crates_smith_ai_src_agent_rs_agent_m_p_prompt → crates_smith_ai_src_agent_rs_mockagent_prompt (calls)
- crates_smith_ai_src_agent_rs_smithagent_prompt → crates_smith_ai_src_agent_rs_mockagent_prompt (calls)
- crates_smith_ai_src_agent_rs_smithagent_agent_run → crates_smith_ai_src_agent_rs_mockagent_prompt (calls)
- crates_smith_ai_src_agent_rs_smithagent_decide → crates_smith_ai_src_agent_rs_mockagent_prompt (calls)
- crates_smith_ai_src_agent_rs_test_agent_run_returns_plain_text → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_agent_run_returns_plain_text → crates_smith_ai_src_agent_rs_smithagent_agent_run (calls)
- crates_smith_ai_src_agent_rs_test_agent_run_parses_json → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_agent_run_parses_json → crates_smith_ai_src_agent_rs_smithagent_agent_run (calls)
- crates_smith_ai_src_agent_rs_test_decide_cancelled → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_decide_cancelled → crates_smith_ai_src_agent_rs_smithagent_decide (calls)
- crates_smith_ai_src_agent_rs_test_decide_empty_options → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_decide_empty_options → crates_smith_ai_src_agent_rs_smithagent_decide (calls)
- crates_smith_ai_src_agent_rs_test_decide_valid_choice → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_decide_valid_choice → crates_smith_ai_src_agent_rs_smithagent_decide (calls)
- crates_smith_ai_src_agent_rs_test_decide_invalid_choice → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_decide_invalid_choice → crates_smith_ai_src_agent_rs_smithagent_decide (calls)
- crates_smith_ai_src_agent_rs_test_decide_trims_quotes → crates_smith_ai_src_agent_rs_make_agent (calls)
- crates_smith_ai_src_agent_rs_test_decide_trims_quotes → crates_smith_ai_src_agent_rs_smithagent_decide (calls)
- crates_smith_ai_src_agent_rs_agent_m_p_prompt → crates_smith_ai_src_agent_rs_mockagent_prompt (uses)
- crates_smith_ai_src_agent_rs_smithagent_prompt → crates_smith_ai_src_agent_rs_mockagent_prompt (uses)

