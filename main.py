import os
import httpx
from typing import Literal
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field
from pydantic_ai import Agent, RunContext
from pydantic_ai.models.openai import OpenAIChatModel
from pydantic_ai.providers.openai import OpenAIProvider
from contextlib import asynccontextmanager
from db import init_db, close_db

@asynccontextmanager
async def lifespan(app: FastAPI):
    # 시작 시 스키마 초기화
    await init_db()
    yield
    # 종료 시 DB 연결 해제
    await close_db()

app = FastAPI(title="Q-Agent API Server", lifespan=lifespan)

# 1. 모델 설정
model = OpenAIChatModel(
    model_name='qwen2.5-coder-7b',
    provider=OpenAIProvider(
        base_url=os.getenv('LLAMA_API_URL', 'http://llama-server:8080/v1'),
        api_key='not-needed'
    )
)

# 2. Harness 상태를 관리할 의존성 모델
class AgentDependencies(BaseModel):
    harness_type: str

HARNESS_PROMPTS = {
    "general": (
        "당신은 개인 비서 Q-Agent입니다. 친절하고 간결하게 답변하세요."
    ),
    "deep_research": (
        "당신은 Computer Vision 및 비디오 압축(VCM/FCM) 연구를 보조하는 전문 Q-Agent입니다. "
        "모르는 최신 표준이나 동향이 있다면 반드시 'search_web' 도구를 사용하세요."
    ),
    "agentic_coding": (
        "당신은 최고 수준의 코드 최적화 및 아키텍처 설계 에이전트입니다. "
        "코드를 작성한 후에는 내부적으로 논리적 오류가 없는지 'Self-Reflection(자기 성찰)' 과정을 거쳐 수정된 최종본을 제시하세요."
    )
}

# 3. 에이전트 정의 (retries -> tool_retries 로 수정하여 경고 해결)
q_agent = Agent(
    model,
    deps_type=AgentDependencies,
    tool_retries=2
)

@q_agent.system_prompt
def build_system_prompt(ctx: RunContext[AgentDependencies]) -> str:
    base_prompt = HARNESS_PROMPTS.get(ctx.deps.harness_type, HARNESS_PROMPTS["general"])
    return f"{base_prompt}\n\n도움이 필요하다면 주저하지 말고 연결된 도구를 호출하세요."

# 4. 검색 도구
@q_agent.tool
async def search_web(ctx: RunContext[AgentDependencies], query: str) -> str:
    """최신 인터넷 검색이 필요할 때 호출합니다."""
    searxng_url = os.getenv('SEARXNG_URL', 'http://searxng:8080')
    async with httpx.AsyncClient(timeout=10.0) as client:
        try:
            response = await client.get(f"{searxng_url}/search", params={"q": query, "format": "json"})
            response.raise_for_status()
            results = response.json().get('results', [])
            if not results:
                return "관련된 검색 결과를 찾을 수 없습니다."
            return "\n".join([f"[{r.get('title', '')}] {r.get('content', '')}" for r in results[:3]])
        except Exception as e:
            return f"검색 도구 실행 중 오류 발생: {str(e)}"

# 5. API 엔드포인트
class ChatRequest(BaseModel):
    message: str = Field(..., description="사용자 질문 또는 지시사항")
    harness_type: Literal["general", "deep_research", "agentic_coding"] = "general"

class ChatResponse(BaseModel):
    answer: str
    harness_used: str

@app.post("/chat", response_model=ChatResponse)
async def chat_endpoint(request: ChatRequest):
    try:
        deps = AgentDependencies(harness_type=request.harness_type)
        result = await q_agent.run(request.message, deps=deps)
        
        # [핵심 수정] PydanticAI 버전별 응답 객체 호환성 패치
        if hasattr(result, 'data'):
            answer_text = result.data
        elif hasattr(result, 'content'):
            answer_text = result.content
        else:
            # 데이터 속성 이름이 변경되었을 경우를 대비한 안전 장치
            answer_text = str(getattr(result, 'message', result))

        # [핵심 수정] PydanticAI 최신 버전 응답 속성(output) 추출
        if hasattr(result, 'data'):
            answer_text = result.data
        elif hasattr(result, 'output'):
            answer_text = result.output
        else:
            answer_text = str(result)

        return ChatResponse(
            answer=answer_text,
            harness_used=request.harness_type
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)