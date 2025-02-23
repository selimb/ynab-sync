use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BudgetSummaryResponse {
    pub budgets: Vec<BudgetSummary>,
    pub default_budget: Option<BudgetSummary>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BudgetSummary {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub accounts: Option<Vec<Account>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
}
