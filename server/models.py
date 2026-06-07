from pydantic import BaseModel, Field
from typing import Optional, List, Any, Dict
from datetime import datetime

# 공통 필드 모델
class BaseTableModel(BaseModel):
    id: str
    created_at: datetime
    updated_at: datetime
    deleted_at: Optional[datetime] = None

class Project(BaseTableModel):
    name: str
    description: Optional[str] = None
    icon: Optional[str] = None
    color: Optional[str] = None
    persona_template: str = "default"
    harness_template: str = "standard"
    orchestration_template: str = "balanced"
    model_small: Optional[str] = None
    model_heavy: Optional[str] = None
    max_iterations: int = 5
    token_budget: int = 8000
    fvcore_limit: Optional[float] = None
    security_level: int = 2
    allowed_paths: List[str] = Field(default_factory=list)
    is_active: bool = False

class Conversation(BaseTableModel):
    project_id: Optional[str] = None
    title: str = "New Conversation"
    mode: str = "chat"

class MessageCitation(BaseModel):
    index: int
    source_id: str
    excerpt: str
    confidence: float

class MessageThoughtTrace(BaseModel):
    agent: str
    step: str
    timestamp: datetime

class Message(BaseTableModel):
    conversation_id: str
    role: str
    content: str
    citations: List[MessageCitation] = Field(default_factory=list)
    thought_trace: List[MessageThoughtTrace] = Field(default_factory=list)
    artifact_ids: List[str] = Field(default_factory=list)
    model_used: Optional[str] = None
    tokens_used: Optional[int] = None
    duration_ms: Optional[int] = None

class AgentTask(BaseTableModel):
    conversation_id: str
    project_id: Optional[str] = None
    status: str = "pending"
    mode: str
    user_input: str
    iteration: int = 0
    max_iterations: int = 5
    tokens_used: int = 0
    token_budget: int = 8000
    critic_score: Optional[float] = None
    completed_at: Optional[datetime] = None

class Artifact(BaseTableModel):
    project_id: Optional[str] = None
    task_id: Optional[str] = None
    message_id: Optional[str] = None
    title: str
    artifact_type: str
    language: Optional[str] = None
    content: str
    file_path: Optional[str] = None
    is_pinned: bool = False
    version: int = 1
    parent_id: Optional[str] = None
