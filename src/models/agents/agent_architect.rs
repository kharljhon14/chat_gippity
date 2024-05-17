use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::{
        command_line::PrintCommand,
        generals::{ai_task_request_decoded, check_status_code},
    },
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTrait,
    },
};

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

#[derive(Debug)]
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

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // ! WARNING - BE CAREFUL OF INFINITE LOOPS
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;

                    // Confirm if external urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_extenal_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }

                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];

                    let client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Find faulty urls

                    let urls = factsheet
                        .external_urls
                        .as_ref()
                        .expect("No URL object on factssheet");

                    for url in urls {
                        let endpoint_str = format!("Testing URL Endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );

                        // Perform URL test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone())
                                }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e),
                        }
                    }

                    // Exclude any faulty urls

                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();

                        factsheet.external_urls = Some(new_urls);
                    }

                    // Confirm done
                    self.attributes.state = AgentState::Finished;
                }

                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent = AgentSolutionArchitect::new();

        let mut factsheet = FactSheet{
            project_description: "Build a full stack website with user login and logout that shows the latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        agent
            .execute(&mut factsheet)
            .await
            .expect("Unable to  execute Solutions Architect Agent");

        assert!(factsheet.project_scope.is_some());
        assert!(factsheet.external_urls.is_some());
        dbg!(agent);
    }
}
