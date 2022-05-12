use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub entropy: String

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    HandleEx {},
}


#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    HandleExAns {
    },
    ViewingKey {
        key: String,
    },
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryEx {

    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    QueryExAns {

    }
}
