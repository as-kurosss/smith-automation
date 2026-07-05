# Community 49: action_stop()

**Members:** 5

## Nodes

- **action_sleep()** (`crates_smith_windows_src_tools_process_rs_action_sleep`, Function, degree: 2)
- **action_start()** (`crates_smith_windows_src_tools_process_rs_action_start`, Function, degree: 4)
- **action_stop()** (`crates_smith_windows_src_tools_process_rs_action_stop`, Function, degree: 3)
- **.execute()** (`crates_smith_windows_src_tools_process_rs_processtool_execute`, Method, degree: 5)
- **.new()** (`crates_smith_windows_src_tools_process_rs_processtool_new`, Method, degree: 5)

## Relationships

- crates_smith_windows_src_tools_process_rs_processtool_execute → crates_smith_windows_src_tools_process_rs_action_start (calls)
- crates_smith_windows_src_tools_process_rs_processtool_execute → crates_smith_windows_src_tools_process_rs_action_stop (calls)
- crates_smith_windows_src_tools_process_rs_processtool_execute → crates_smith_windows_src_tools_process_rs_processtool_new (calls)
- crates_smith_windows_src_tools_process_rs_processtool_execute → crates_smith_windows_src_tools_process_rs_action_sleep (calls)
- crates_smith_windows_src_tools_process_rs_action_start → crates_smith_windows_src_tools_process_rs_processtool_new (calls)
- crates_smith_windows_src_tools_process_rs_action_stop → crates_smith_windows_src_tools_process_rs_processtool_new (calls)

