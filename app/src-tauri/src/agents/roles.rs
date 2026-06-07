pub fn get_planner_prompt(task: &str, context: &str) -> String {
    format!(
        "You are an expert AI software planner. 
Your task is to analyze the user's request and the provided context, then formulate a step-by-step implementation plan.

Context:
{}

User Task:
{}

Output your plan as a numbered list.",
        context, task
    )
}

pub fn get_coder_prompt(plan: &str, context: &str) -> String {
    format!(
        "You are an expert AI software engineer. 
Your task is to execute the following implementation plan based on the provided context.

Context:
{}

Implementation Plan:
{}

Output the exact code changes or actions needed. Use clear explanations and code blocks.",
        context, plan
    )
}

pub fn get_critic_prompt(result: &str) -> String {
    format!("You are the Critic. Evaluate the execution result. Reply with a score between 0.0 and 1.0.\nResult: {}", result)
}

pub fn get_mlops_prompt(task: &str) -> String {
    format!("You are the MLOps Engineer. Design an experiment or pipeline for this request.\nTask: {}", task)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planner_prompt_contains_task() {
        let prompt = get_planner_prompt("Write a hello world app");
        assert!(prompt.contains("Planner"));
        assert!(prompt.contains("Write a hello world app"));
    }

    #[test]
    fn test_coder_prompt_contains_plan() {
        let prompt = get_coder_prompt("Step 1: setup project");
        assert!(prompt.contains("Coder"));
        assert!(prompt.contains("Step 1: setup project"));
    }

    #[test]
    fn test_critic_prompt_contains_result() {
        let prompt = get_critic_prompt("fn main() { println!(\"ok\"); }");
        assert!(prompt.contains("Critic"));
        assert!(prompt.contains("0.0 and 1.0"));
    }

    #[test]
    fn test_mlops_prompt_contains_task() {
        let prompt = get_mlops_prompt("Train a classification model");
        assert!(prompt.contains("MLOps"));
        assert!(prompt.contains("Train a classification model"));
    }

    #[test]
    fn test_prompts_are_non_empty() {
        // 빈 문자열 입력에서도 프롬프트 구조는 유지되어야 함
        assert!(!get_planner_prompt("").is_empty());
        assert!(!get_coder_prompt("").is_empty());
        assert!(!get_critic_prompt("").is_empty());
        assert!(!get_mlops_prompt("").is_empty());
    }
}
