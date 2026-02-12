<!-- memory-id: mem_1770834220774_li99fkg -->
# 2026-02-11
TITLE: 文件存在检查使用is_file而非exists
TAGS: #code_style #rust
CONTENT:
- 检查文件是否存在时使用 is_file() 而非 exists()，避免目录被误判为存在