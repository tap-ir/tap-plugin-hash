//! Hash plugin take a VFile attribute return from a node and  add the result of an hash function to the attribute of this node

use std::io;

use tap::config_schema;
use tap::plugin;
use tap::plugin::{PluginInfo, PluginInstance, PluginConfig, PluginArgument, PluginResult, PluginEnvironment};
use tap::vfile::{VFile};
use tap::tree::{TreeNodeId, VecTreeNodeIdSchema};
use tap::error::RustructError;

use serde::{Serialize, Deserialize};
use schemars::{JsonSchema};
use sha1::{Sha1, Digest};

plugin!("hash", "Metadata", "Hash file attribute", Hash, Arguments);


#[derive(Default)]
pub struct Hash 
{
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Arguments
{
  #[schemars(with = "VecTreeNodeIdSchema")] 
  files : Vec<TreeNodeId>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Results
{
}

impl Hash 
{
  pub fn hash(&self, mut file : Box<dyn VFile>) -> String
  {
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher).unwrap(); //XXX check read result
    let hash = hasher.result();
    format!("{:x}", hash)
  }

  fn run(&mut self, args : Arguments, env : PluginEnvironment) -> anyhow::Result<Results>
  {
    for file in args.files
    {
      let file_node = env.tree.get_node_from_id(file).ok_or(RustructError::ArgumentNotFound("file"))?;
      let data = file_node.value().get_value("data").ok_or(RustructError::ValueNotFound("data"))?;
      let data_builder = data.try_as_vfile_builder().ok_or(RustructError::ValueTypeMismatch)?;

      match data_builder.open()
      {
        Ok(file) => file_node.value().add_attribute(self.name(), self.hash(file), None),
        Err(_err) => file_node.value().add_attribute(self.name(), None, None),
      };
    }

    Ok(Results{})
  }
}
