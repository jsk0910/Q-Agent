<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let prompt = "";
  let response = "";
  let isLoading = false;

  async function askAgent() {
    if (!prompt.trim()) return;
    
    isLoading = true;
    response = "생각 중입니다...";

    try {
      const result: string = await invoke("chat_with_ollama", { prompt });
      response = result;
    } catch (error) {
      response = `에러 발생: ${error}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="container">
  <h1>Q-Agent Workspace</h1>
  <p>Qwen 3.5 4B 로컬 모델에 연결되었습니다.</p>

  <div class="chat-box">
    <textarea 
      bind:value={prompt} 
      placeholder="지시사항이나 코딩 질문을 입력하세요..." 
      rows="4"
    ></textarea>
    
    <button on:click={askAgent} disabled={isLoading}>
      {isLoading ? "처리 중..." : "전송"}
    </button>
  </div>

  {#if response}
    <div class="response-box">
      <strong>AI의 답변:</strong>
      <pre>{response}</pre> 
    </div>
  {/if}
</main>

<style>
  .container { max-width: 800px; margin: 0 auto; padding: 2rem; font-family: sans-serif; }
  .chat-box { display: flex; flex-direction: column; gap: 10px; margin-bottom: 2rem; }
  textarea { width: 100%; padding: 10px; font-size: 1rem; border-radius: 8px; border: 1px solid #ccc; }
  button { padding: 10px 20px; font-size: 1rem; cursor: pointer; background-color: #2563eb; color: white; border: none; border-radius: 8px; }
  button:disabled { background-color: #94a3b8; }
  .response-box { background-color: #f8fafc; padding: 1rem; border-radius: 8px; white-space: pre-wrap; word-break: break-all; }
  pre { font-family: 'Courier New', Courier, monospace; overflow-x: auto; }
</style>