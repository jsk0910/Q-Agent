import os
from surrealdb import Surreal
import logging

logger = logging.getLogger(__name__)

# SurrealDB 접속 정보 설정 (환경 변수 또는 기본값)
SURREAL_URL = os.getenv("SURREAL_URL", "ws://surrealdb:8000/rpc")
SURREAL_USER = os.getenv("SURREAL_USER", "root")
SURREAL_PASS = os.getenv("SURREAL_PASS", "root")
SURREAL_NS = os.getenv("SURREAL_NS", "qagent")
SURREAL_DB = os.getenv("SURREAL_DB", "qagent")

# SurrealDB 클라이언트 인스턴스
db = Surreal(SURREAL_URL)

async def init_db():
    """
    SurrealDB에 연결하고 네임스페이스, 데이터베이스를 선택한 뒤
    schema.surql 파일을 읽어 스키마를 초기화합니다.
    """
    logger.info(f"Connecting to SurrealDB at {SURREAL_URL}...")
    try:
        await db.connect()
        await db.signin({"user": SURREAL_USER, "pass": SURREAL_PASS})
        await db.use(SURREAL_NS, SURREAL_DB)
        logger.info(f"Successfully connected to SurrealDB (Namespace: {SURREAL_NS}, Database: {SURREAL_DB})")

        # 스키마 초기화
        schema_path = os.path.join(os.path.dirname(__file__), "schema.surql")
        if os.path.exists(schema_path):
            with open(schema_path, "r", encoding="utf-8") as f:
                schema_query = f.read()
            logger.info("Applying database schema...")
            # SurrealDB에 쿼리 실행
            await db.query(schema_query)
            logger.info("Database schema applied successfully.")
        else:
            logger.warning(f"Schema file not found at {schema_path}")

    except Exception as e:
        logger.error(f"Failed to initialize database: {e}")
        raise

async def close_db():
    """
    SurrealDB 연결을 닫습니다.
    """
    logger.info("Closing SurrealDB connection...")
    try:
        pass # python surrealdb client does not have a explicit close method right now. Alternatively, just leave it to GC or close transport if available.
    except Exception as e:
        logger.error(f"Error while closing db: {e}")

def get_db():
    """
    FastAPI 의존성 주입용 함수
    """
    return db
