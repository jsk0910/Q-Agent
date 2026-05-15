pub fn get_planner_prompt(task: &str) -> String {
    format!("You are the Planner. Analyze the user request and create a step-by-step plan.\nTask: {}", task)
}

pub fn get_coder_prompt(plan: &str) -> String {
    format!("You are the Coder. Execute the following plan and generate the required code or files.\nPlan: {}", plan)
}

pub fn get_critic_prompt(result: &str) -> String {
    format!("You are the Critic. Evaluate the execution result. Reply with a score between 0.0 and 1.0.\nResult: {}", result)
}

pub fn get_mlops_prompt(task: &str) -> String {
    format!("You are the MLOps Engineer. Design an experiment or pipeline for this request.\nTask: {}", task)
}
