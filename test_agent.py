import requests
import json
import time

API_URL = "http://localhost:8000/chat"

def test_chat(test_name, message, harness_type):
    print(f"\n🚀 [테스트 진행 중] {test_name} (Harness: {harness_type})")
    print(f"👉 질문: {message}")
    
    start_time = time.time()
    try:
        response = requests.post(API_URL, json={"message": message, "harness_type": harness_type})
        response.raise_for_status()
        result = response.json()
        
        print(f"⏱️ 응답 시간: {time.time() - start_time:.2f}초")
        print("\n✅ [답변 결과]\n" + "-"*40)
        print(result.get("answer", "답변 없음"))
        print("-" * 40)
        
        # 검색 도구를 사용했다면 출처를 출력합니다.
        if result.get("sources"):
            print("\n🔍 [참고한 검색 출처]\n" + "-"*40)
            print(result["sources"])
            print("-" * 40)
            
    except Exception as e:
        print(f"❌ 에러 발생: {e}")

# ---------------------------------------------------------
# 테스트 1: 인터넷 검색 판단 및 요약 능력 (RAG)
# 에이전트가 최신 정보가 필요하다고 판단하면 스스로 SearXNG를 호출합니다.
test_chat(
    test_name="인터넷 우선 검색 테스트",
    message="가장 최근의 VCM(Video Coding for Machines) 표준화 동향에 대해 요약해줘.",
    harness_type="general"
)

# 테스트 2: 페르소나 전환 및 코딩 자기 성찰 능력
# 에이전트가 'agentic_coding' 페르소나를 부여받고 코드를 작성합니다.
test_chat(
    test_name="에이전틱 코딩 테스트",
    message="파이썬으로 리스트의 중복을 제거하고 정렬하는 효율적인 코드를 작성해줘. 작성 후 스스로 최적화 여부를 검토(Self-reflection)해.",
    harness_type="agentic_coding"
)