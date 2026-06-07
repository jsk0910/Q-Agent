import os
import uuid
import json
import re
import httpx
from datetime import datetime
from pydantic import BaseModel, Field
from pydantic_ai import Agent, RunContext
from pydantic_ai.models.openai import OpenAIChatModel
from pydantic_ai.providers.openai import OpenAIProvider

# Import db checkpointing helper (we will create this in db.py)
from db import save_agent_state

model = OpenAIChatModel(
    model_name='qwen2.5-coder-7b',
    provider=OpenAIProvider(
        base_url=os.getenv('LLAMA_API_URL', 'http://llama-server:8080/v1'),
        api_key='not-needed'
    )
)

class AgentState(BaseModel):
    task_id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    original_query: str = ""
    harness_type: str = "general"
    permission_mode: str = "balanced"  # strict | balanced | agentic (E)
    search_results: list[dict] = Field(default_factory=list)
    plan: str = ""
    draft_answer: str = ""
    revisions: int = 0
    max_revisions: int = 3
    token_budget: int = 8192
    tokens_used: int = 0
    critic_score: float = 0.0
    status: str = "Idle"
    final_answer: str = ""

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

# --- Agents ---

planner_agent = Agent(
    model,
    system_prompt=(
        "당신은 Planner 에이전트입니다. 사용자의 질문을 분석하여 어떤 정보를 찾아야 할지, "
        "어떤 논리적 순서로 답변을 구성해야 할지 계획(Plan)을 세워주세요. 아주 간결한 개요 형태로 작성하세요."
    )
)

executor_agent = Agent(
    model,
    deps_type=AgentState,
    tool_retries=2
)

@executor_agent.system_prompt
def build_executor_prompt(ctx: RunContext[AgentState]) -> str:
    base_prompt = HARNESS_PROMPTS.get(ctx.deps.harness_type, HARNESS_PROMPTS["general"])
    sys_prompt = f"{base_prompt}\n\n[Planner의 계획]\n{ctx.deps.plan}\n\n"
    tool_instructions = (
        "[도구 사용 지침]\n"
        "인터넷 검색이 필요하다면 반드시 다음과 같은 정확한 JSON 형식으로만 응답하세요. 다른 설명은 덧붙이지 마세요.\n"
        "```json\n"
        '{"name": "search_web", "arguments": {"query": "검색어"}}\n'
        "```\n"
        "만약 사용자가 이미 검색 결과를 제공했다면 해당 결과를 바탕으로 최종 답변을 한글로 작성하세요. "
        "응답을 작성할 때는 반드시 제공된 검색 결과의 인용 번호(예: [1], [2])를 텍스트에 포함하여 출처를 명시하세요."
    )
    return sys_prompt + tool_instructions

critic_agent = Agent(
    model,
    system_prompt=(
        "당신은 Critic 에이전트입니다. 작성된 답변(Draft)이 사용자의 질문을 완벽히 해결하는지 비판적으로 평가하세요. "
        "응답의 마지막 줄에 반드시 'SCORE: X.XX' 형식으로 0.00에서 1.00 사이의 점수를 기입하세요. "
        "만약 정보가 부족하거나 논리적 오류가 있다면 0.85 미만의 점수를 주고 보완할 점을 제시하세요. 0.85 이상이면 승인합니다."
    )
)

# --- Tools ---

async def do_search(state: AgentState, query: str) -> str:
    # E. Strict 모드에서 외부 검색 비활성화
    if state.permission_mode == "strict":
        return "보안 모드(Strict)에서는 외부 검색이 비활성화됩니다."

    searxng_url = os.getenv('SEARXNG_URL', 'http://searxng:8080')
    async with httpx.AsyncClient(timeout=10.0) as client:
        try:
            response = await client.get(f"{searxng_url}/search", params={"q": query, "format": "json"})
            response.raise_for_status()
            results = response.json().get('results', [])
            if not results:
                return "관련된 검색 결과를 찾을 수 없습니다."

            output = []
            for r in results[:3]:
                idx = len(state.search_results) + 1
                state.search_results.append({
                    "index": idx,
                    "title": r.get("title", ""),
                    "url": r.get("url", ""),
                    "content": r.get("content", ""),
                })
                output.append(f"[Source {idx}] {r.get('title', '')}: {r.get('content', '')}")

            return "\n\n".join(output)
        except Exception as e:
            return f"검색 도구 실행 중 오류 발생: {str(e)}"

# --- Graph Engine ---

async def run_workflow(
    query: str,
    harness_type: str,
    permission_mode: str = "balanced",  # E
    token_budget: int = 8192,
    max_revisions: int = 3,
) -> AgentState:
    state = AgentState(
        original_query=query,
        harness_type=harness_type,
        permission_mode=permission_mode,
        token_budget=token_budget,
        max_revisions=max_revisions,
        status="Planning",
    )
    
    await save_agent_state(state)
    
    while state.status not in ["Finished", "Error"]:
        # Budget Guard Check
        if state.revisions >= state.max_revisions:
            state.status = "Finished"
            state.final_answer = state.draft_answer
            await save_agent_state(state)
            break
        
        if state.tokens_used >= state.token_budget:
            state.status = "Error"
            state.final_answer = "Error: Token budget exceeded."
            await save_agent_state(state)
            break

        if state.status == "Planning":
            plan_res = await planner_agent.run(f"User Query: {state.original_query}")
            state.plan = _extract_text(plan_res)
            _add_usage(state, plan_res)
            state.status = "Executing"
            await save_agent_state(state)
            
        elif state.status == "Executing":
            prompt = f"사용자 질문: {state.original_query}"
            if state.revisions > 0:
                prompt += f"\n\n[이전 답변]\n{state.draft_answer}\n\n[Critic의 피드백]\n이 부분을 개선하세요."
            
            exec_res = await executor_agent.run(prompt, deps=state)
            answer_text = _extract_text(exec_res)
            _add_usage(state, exec_res)
            
            # Manual tool call loop (similar to previous main.py)
            loops = 0
            while "search_web" in answer_text and loops < 3:
                loops += 1
                json_match = re.search(r'\{.*"name":\s*"search_web".*\}', answer_text, re.DOTALL)
                if json_match:
                    try:
                        tool_call = json.loads(json_match.group(0))
                        if tool_call.get("name") == "search_web":
                            q = tool_call.get("arguments", {}).get("query", "")
                            if q:
                                search_res = await do_search(state, q)
                                follow_up = f"Tool 'search_web' returned:\n{search_res}\n\nPlease provide the final answer."
                                exec_res = await executor_agent.run(follow_up, deps=state, message_history=exec_res.new_messages())
                                answer_text = _extract_text(exec_res)
                                _add_usage(state, exec_res)
                                continue
                    except Exception:
                        pass
                break
            
            state.draft_answer = answer_text
            state.status = "Reviewing"
            await save_agent_state(state)
            
        elif state.status == "Reviewing":
            state.revisions += 1
            critic_prompt = f"질문: {state.original_query}\n답변: {state.draft_answer}"
            critic_res = await critic_agent.run(critic_prompt)
            critic_text = _extract_text(critic_res)
            _add_usage(state, critic_res)
            
            score_match = re.search(r'SCORE:\s*([0-9.]+)', critic_text.upper())
            score = 0.0
            if score_match:
                try:
                    score = float(score_match.group(1))
                except ValueError:
                    pass
                    
            state.critic_score = score
            if score >= 0.85:
                state.status = "Finished"
                state.final_answer = state.draft_answer
            else:
                state.status = "Executing"
                
            await save_agent_state(state)
            
    return state

def _extract_text(result) -> str:
    if hasattr(result, 'data'):
        return result.data
    elif hasattr(result, 'output'):
        return result.output
    elif hasattr(result, 'content'):
        return result.content
    return str(getattr(result, 'message', result))

def _add_usage(state: AgentState, result):
    # Dummy token calculation since pydantic_ai doesn't reliably expose usage directly in a unified way
    # Or try to extract if usage is available
    usage = getattr(result, 'usage', None)
    if usage:
        total = getattr(usage, 'total_tokens', 0)
        if total:
            state.tokens_used += total
    else:
        # Fallback estimation
        state.tokens_used += len(_extract_text(result)) // 4
