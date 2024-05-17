use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::generals::ai_task_request;
use crate::models::agent_basic::basic_agent::AgentState;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;
use crate::models::{
    agent_basic::basic_agent::BasicAgent,
    agents::agent_traits::{FactSheet, SpecialFunctions},
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project Manager".to_string();
        let attributes = BasicAgent {
            objective: "Mange agents who are building an excellent website for the user"
                .to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            user_request,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes,
            agents,
            factsheet,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_result: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}
