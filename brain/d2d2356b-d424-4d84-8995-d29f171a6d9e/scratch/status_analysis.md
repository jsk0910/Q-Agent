# Q-Agent Current Status Analysis (2026-04-17)

## 1. Phase 2.5 (Intelligent Harness) Progress
### ✅ Completed
- [x] **Project Isolation**: SurrealDB queries now filter by `project_id`.
- [x] **Guideline Routing**: `KeywordGuidelineRouter` selects rules based on prompt.
- [x] **Project Switcher**: UI for creating and switching projects in the Sidebar.
- [x] **Evaluator-Critic Loop**: Settings toggle for 2nd pass verification.
- [x] **Dynamic Context Injection**: Web search and RAG results injected into system prompt.

### 🛠️ Partially Completed / Needs Refinement
- [/] **Context Builder**: `AdaptiveContextBuilder` exists but is NOT integrated into the main `chat_with_ollama` flow. History management is still manual.
- [/] **Token Counting**: Currently uses naive `len() / 4` approximation. Needs `tiktoken` or similar.
- [/] **Progress UI**: Eval loop indicator is just text pinned to the last message. Could be more elegant.

## 2. Phase 1 & 2 "Debt" (Bugs & Missing Features)
- [ ] **RAG Citation UI**: Clicking `[Source 1]` in chat should scroll/open the Knowledge Panel to that source.
- [ ] **Knowledge Panel UX**: Add ability to delete or manage documents within a project.
- [ ] **Export Options**: Currently Markdown only. Need PDF/HTML export.
- [ ] **Responsive Refinement**: Sidebar behavior on small screens.

## 3. Recommended Next Phase: Phase 3 (Remote Mastery)
- **Goal**: Connect to remote GPU servers via SSH for monitoring and job management.
- **Key Features**:
  - SSH Connection Manager.
  - VRAM/GPU Load monitoring (nvidia-smi).
  - Webview-based Xterm.js for remote shell.

## 4. Architectural Refinement
- **Execution Separation**: Move RAG/WebSearch logic from `agent.rs` (tauri commands) into the `harness` module for better testability and cleaner `lib.rs`.
