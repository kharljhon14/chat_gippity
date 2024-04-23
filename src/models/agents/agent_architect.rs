use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::generals::ai_task_request_decoded,
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTrait,
    },
};

use super::agent_traits::{FactSheet, ProjectScope};

pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solution for webserver development"
                .to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrive projects scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context = format!("{}", factsheet.project_description);

        let ai_response = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);

        ai_response
    }

    async fn call_determine_extenal_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
        let ai_response = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting
    }
}
