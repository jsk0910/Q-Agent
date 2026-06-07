import os
import uuid
import asyncio
import json
from typing import Literal, AsyncGenerator
from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from contextlib import asynccontextmanager
from urllib.parse import urlparse
from db import init_db, close_db, save_message
from models import MessageCitation
from graph import run_workflow, AgentState

# ─────────────────────────────────────────────
# 1. 앱 라이프사이클
# ─────────────────────────────────────────────
@asynccontextmanager
async def lifespan(app: FastAPI):
    await init_db()
    yield
    await close_db()

app = FastAPI(title="Q-Agent API Server", version="1.0.0", lifespan=lifespan)

# CORS 설정 (로컬 Tauri 및 웹 환경 허용)
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "tauri://localhost", "http://localhost", "http://127.0.0.1",
        "http://localhost:1420", "http://localhost:5173",
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# ─────────────────────────────────────────────
# 2. Citation 빌드 유틸
# ─────────────────────────────────────────────
def _domain_authority(url: str) -> float:
    if not url:
        return 3.0
    try:
        domain = urlparse(url).netloc.lower()
        if domain.endswith((".gov", ".edu")):
            return 4.8
        if any(x in domain for x in ("wikipedia.org", "github.com", "arxiv.org")):
            return 4.5
        if domain.endswith(".org"):
            return 4.0
        if domain.endswith((".com", ".net", ".io")):
            return 3.5
        return 3.0
    except Exception:
        return 3.0

def _relevance_score(query: str, snippet: str) -> float:
    if not query or not snippet:
        return 3.0
    q = set(query.lower().split())
    s = set(snippet.lower().split())
    if not q:
        return 3.0
    ratio = len(q & s) / len(q)
    if ratio > 0.8: return 5.0
    if ratio > 0.5: return 4.5
    if ratio > 0.2: return 4.0
    if ratio > 0.0: return 3.5
    return 3.0

def build_citations_from_state(state: AgentState, query: str) -> list[MessageCitation]:
    """AgentState.search_results → MessageCitation 리스트"""
    result = []
    for res in state.search_results:
        url = res.get("url", "")
        content = res.get("content", "")
        title = res.get("title", "")
        confidence = round(
            _domain_authority(url) * 0.6 + _relevance_score(query, content + " " + title) * 0.4, 1
        )
        result.append(MessageCitation(
            index=res["index"],
            source_id=url or "Unknown",
            excerpt=content,
            confidence=confidence,
        ))
    return result

# ─────────────────────────────────────────────
# 3. 에이전트 실행 코어 (A, D — graph.run_workflow 위임)
# ─────────────────────────────────────────────
async def run_agent(
    message: str,
    harness_type: str,
    permission_mode: str,
    token_budget: int,
    max_iterations: int,
    conversation_id: str | None = None,
) -> tuple[str, list[MessageCitation], AgentState]:
    """
    Planner → Executor → Critic 루프 실행 (graph.py 위임).
    Returns (answer_text, citations, state)
    """
    state = await run_workflow(
        query=message,
        harness_type=harness_type,
        permission_mode=permission_mode,
        token_budget=token_budget,
        max_revisions=max_iterations,
    )
    answer = state.final_answer or state.draft_answer or "응답을 생성하지 못했습니다."
    citations = build_citations_from_state(state, message)
    return answer, citations, state

# ─────────────────────────────────────────────
# 4. API 스키마
# ─────────────────────────────────────────────
class ChatRequest(BaseModel):
    message: str = Field(..., description="사용자 질문 또는 지시사항")
    harness_type: Literal["general", "deep_research", "agentic_coding"] = "general"
    permission_mode: Literal["strict", "balanced", "agentic"] = "balanced"
    conversation_id: str | None = None
    token_budget: int = 8000
    max_iterations: int = 5

class ChatResponse(BaseModel):
    answer: str
    harness_used: str
    citations: list[MessageCitation] = Field(default_factory=list)
    conversation_id: str | None = None
    tokens_used: int = 0
    iteration: int = 0
    critic_score: float = 0.0

class StreamRequest(BaseModel):
    message: str
    harness_type: Literal["general", "deep_research", "agentic_coding"] = "general"
    permission_mode: Literal["strict", "balanced", "agentic"] = "balanced"
    conversation_id: str | None = None
    token_budget: int = 8000
    max_iterations: int = 5

# ─────────────────────────────────────────────
# 5. REST 엔드포인트 — /chat (B 수정: DB 저장)
# ─────────────────────────────────────────────
@app.post("/chat", response_model=ChatResponse)
async def chat_endpoint(request: ChatRequest):
    """멀티에이전트 JSON 응답. 응답 완료 후 SurrealDB에 대화 저장."""
    try:
        conv_id = request.conversation_id or str(uuid.uuid4())

        answer_text, citations, state = await run_agent(
            message=request.message,
            harness_type=request.harness_type,
            permission_mode=request.permission_mode,
            token_budget=request.token_budget,
            max_iterations=request.max_iterations,
            conversation_id=conv_id,
        )

        # B. Checkpointing — SurrealDB에 저장
        citation_dicts = [c.model_dump() for c in citations]
        await save_message(conv_id, "user", request.message)
        await save_message(conv_id, "assistant", answer_text, citations=citation_dicts)

        return ChatResponse(
            answer=answer_text,
            harness_used=request.harness_type,
            citations=citations,
            conversation_id=conv_id,
            tokens_used=state.tokens_used,
            iteration=state.revisions,
            critic_score=state.critic_score,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# ─────────────────────────────────────────────
# 6. SSE 스트리밍 엔드포인트 — /chat/stream (H 수정)
# ─────────────────────────────────────────────
async def _stream_gen(request: StreamRequest) -> AsyncGenerator[str, None]:
    conv_id = request.conversation_id or str(uuid.uuid4())
    yield f"data: {json.dumps({'event': 'start', 'conversation_id': conv_id})}\n\n"

    try:
        answer_text, citations, state = await run_agent(
            message=request.message,
            harness_type=request.harness_type,
            permission_mode=request.permission_mode,
            token_budget=request.token_budget,
            max_iterations=request.max_iterations,
            conversation_id=conv_id,
        )

        # 단어 청크 단위 스트리밍
        words = answer_text.split(' ')
        chunk_size = 4
        for i in range(0, len(words), chunk_size):
            chunk = ' '.join(words[i:i + chunk_size])
            if i + chunk_size < len(words):
                chunk += ' '
            yield f"data: {json.dumps({'event': 'token', 'token': chunk})}\n\n"
            await asyncio.sleep(0.025)

        # DB 저장
        citation_dicts = [c.model_dump() for c in citations]
        await save_message(conv_id, "user", request.message)
        await save_message(conv_id, "assistant", answer_text, citations=citation_dicts)

        yield f"data: {json.dumps({'event': 'done', 'citations': citation_dicts, 'tokens_used': state.tokens_used, 'critic_score': state.critic_score})}\n\n"

    except Exception as e:
        yield f"data: {json.dumps({'event': 'error', 'detail': str(e)})}\n\n"

@app.post("/chat/stream")
async def chat_stream_endpoint(request: StreamRequest):
    """SSE 기반 실시간 스트리밍 응답"""
    return StreamingResponse(
        _stream_gen(request),
        media_type="text/event-stream",
        headers={"Cache-Control": "no-cache", "X-Accel-Buffering": "no"},
    )

# ─────────────────────────────────────────────
# 7. 헬스 체크
# ─────────────────────────────────────────────
@app.get("/health")
async def health_check():
    return {"status": "ok", "version": "1.0.0"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)