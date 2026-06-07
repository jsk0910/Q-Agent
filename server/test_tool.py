import asyncio
from pydantic_ai import Agent, RunContext
from pydantic_ai.models.openai import OpenAIChatModel
from pydantic_ai.providers.openai import OpenAIProvider

model = OpenAIChatModel(
    'qwen2.5-coder-7b-instruct-q4_k_m',
    provider=OpenAIProvider(base_url='http://llama-server:8080/v1', api_key='none')
)
agent = Agent(model)

@agent.tool
def search_web(ctx: RunContext, query: str) -> str:
    """Searches the web"""
    print(f"Tool called with {query}")
    return "Result of web search: VCM is cool."

async def main():
    print("Starting agent...")
    result = await agent.run("Search the web for VCM")
    if hasattr(result, 'data'):
        print("FINAL RESULT:", result.data)
    elif hasattr(result, 'output'):
        print("FINAL RESULT:", result.output)
    else:
        print("FINAL RESULT DIR:", dir(result))

if __name__ == "__main__":
    asyncio.run(main())
