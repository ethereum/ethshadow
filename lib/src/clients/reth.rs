use crate::clients::{Client, JSON_RPC_PORT};
use crate::clients::ENGINE_API_PORT;
use crate::config::shadow::Process;
use crate::error::Error;
use crate::node::{Node, SimulationContext};
use crate::validators::Validator;
use crate::{CowStr};
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;

const PORT: &str = "21000";

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Reth {
    pub executable: CowStr,
}

impl Default for Reth {
    fn default() -> Self {
        Self {
            executable: "reth".into(),
        }
    }
}

#[typetag::deserialize(name = "reth")]
impl Client for Reth {
    fn add_to_node(
        &self,
        node: &Node,
        ctx: &mut SimulationContext,
        _validators: &[Validator],
    ) -> Result<Process, Error> {
        let genesis_file = ctx.metadata_path().join("genesis.json");
        let genesis_file = genesis_file.to_str().ok_or(Error::NonUTF8Path)?;

        let dir = node.dir().join("reth");
        let dir = dir.to_str().ok_or(Error::NonUTF8Path)?;

        ctx.add_el_http_endpoint(format!("http://{}:{JSON_RPC_PORT}", node.ip()));

        Ok(Process {
            path: self.executable.clone(),
            args: format!(
                "node \
                --chain {genesis_file} \
                --datadir {dir} \
                --authrpc.port {ENGINE_API_PORT} \
                --authrpc.jwtsecret {} \
                --http \
                --http.addr 0.0.0.0 \
                --http.port {JSON_RPC_PORT} \
                --http.api eth,rpc,web3 \
                --port {PORT} \
                --bootnodes {} \
                --nat extip:{} \
                --ipcdisable \
                --log.file.directory {dir}",
                ctx.jwt_path().to_str().ok_or(Error::NonUTF8Path)?,
                ctx.el_bootnode_enodes().join(","),
                node.ip(),
            ),
            environment: HashMap::new(),
            expected_final_state: "running".into(),
            start_time: format!("{}ms", ctx.rng().gen_range(5000..10000)).into(),
        })
    }
}
