import os
from surrealdb import AsyncSurreal
import logging

logger = logging.getLogger(__name__)

# SurrealDB 접속 정보 설정 (환경 변수 또는 기본값)
SURREAL_URL = os.getenv("SURREAL_URL", "ws://surrealdb:8000/rpc")
SURREAL_USER = os.getenv("SURREAL_USER", "root")
SURREAL_PASS = os.getenv("SURREAL_PASS", "root")
SURREAL_NS = os.getenv("SURREAL_NS", "qagent")
SURREAL_DB = os.getenv("SURREAL_DB", "qagent")

# AsyncSurreal 클라이언트 인스턴스
db = AsyncSurreal(SURREAL_URL)

async def init_db():
    """
    SurrealDB에 비동기로 연결하고 네임스페이스, 데이터베이스를 선택한 뒤
    schema.surql 파일을 읽어 스키마를 초기화합니다.
    """
    logger.info(f"Connecting to SurrealDB at {SURREAL_URL}...")
    try:
        await db.signin({"user": SURREAL_USER, "pass": SURREAL_PASS})
        await db.use(SURREAL_NS, SURREAL_DB)
        logger.info(f"Successfully connected to SurrealDB (Namespace: {SURREAL_NS}, Database: {SURREAL_DB})")

        # 스키마 초기화
        schema_path = os.path.join(os.path.dirname(__file__), "schema.surql")
        if os.path.exists(schema_path):
            with open(schema_path, "r", encoding="utf-8") as f:
                schema_query = f.read()
            logger.info("Applying database schema...")
            try:
                await db.query(schema_query)
                logger.info("Database schema applied successfully.")
            except Exception as e:
                if "already exists" in str(e).lower():
                    logger.info("Database schema already exists, skipping initialization.")
                else:
                    logger.warning(f"Schema apply warning: {e}")
        else:
            logger.warning(f"Schema file not found at {schema_path}")

    except Exception as e:
        logger.error(f"Failed to initialize database: {e}")

async def close_db():
    """
    SurrealDB 연결을 닫습니다.
    """
    logger.info("Closing SurrealDB connection...")
    try:
        await db.close()
    except Exception as e:
        logger.error(f"Error while closing db: {e}")

def get_db() -> AsyncSurreal:
    """
    FastAPI 의존성 주입용 함수
    """
    return db

async def save_message(conversation_id: str, role: str, content: str, citations: list = None, thought_trace: list = None) -> str:
    """
    메시지를 SurrealDB에 저장하고 생성된 ID를 반환합니다.
    """
    try:
        data = {
            "conversation_id": conversation_id,
            "role": role,
            "content": content,
            "citations": citations or [],
            "thought_trace": thought_trace or [],
            "artifact_ids": [],
        }
        result = await db.create("message", data)
        if result and isinstance(result, list) and len(result) > 0:
            return str(result[0].get("id", ""))
        elif result and isinstance(result, dict):
            return str(result.get("id", ""))
        return ""
    except Exception as e:
        logger.error(f"Failed to save message: {e}")
        return ""

async def get_conversation_messages(conversation_id: str) -> list:
    """
    특정 대화 ID에 속한 메시지들을 시간순으로 조회합니다.
    """
    try:
        result = await db.query(
            "SELECT * FROM message WHERE conversation_id = $conv_id ORDER BY created_at ASC",
            {"conv_id": conversation_id}
        )
        if result and isinstance(result, list) and len(result) > 0:
            rows = result[0].get("result", []) if isinstance(result[0], dict) else result[0]
            return rows if isinstance(rows, list) else []
        return []
    except Exception as e:
        logger.error(f"Failed to get messages for conversation {conversation_id}: {e}")
        return []

async def save_agent_state(state) -> None:
    """
    Agent 상태를 SurrealDB에 저장합니다. (Checkpointing)
    """
    try:
        data = state.model_dump() if hasattr(state, 'model_dump') else state
        task_id = data.get("task_id", "default")
        await db.query(
            f"UPDATE agent_state:⟨{task_id}⟩ CONTENT $data",
            {"data": data}
        )
    except Exception as e:
        logger.error(f"Failed to save agent state: {e}")
