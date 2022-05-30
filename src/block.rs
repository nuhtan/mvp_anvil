use core::{fmt, panic};

use nbt::Value;

/// A Minecraft block. This struct does not store any data about the location because
/// to get a block one must use x, y, and z coordinates on a Chunk and thus would
/// already have the location data.
#[derive(Debug, Eq, PartialEq)]
pub struct Block {
    namespace: String,
    /// The general name of a block, ie. 'stone'
    pub id: String,
}

impl Block {
    /// Returns a new block with a given namespace and id.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace for the found block, for vanilla this will always be 'minecraft'. For modded
    /// versions of Minecraft this would represent the namespace of the mod.
    /// * `block_id` - The id of the block, this is typically the name of the block without spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_anvil::block::Block;
    /// let block = Block::new("minecraft".into(), Some("stone".into()));
    /// println!("{}", block.id);
    /// ```
    pub fn new(namespace: String, block_id: Option<String>) -> Block {
        match block_id {
            Some(id) => return Block { namespace, id },
            None => {
                return Block {
                    namespace: namespace.clone(),
                    id: namespace,
                };
            }
        }
    }

    /// Returns the full name of the block in question, this looks like 'namespace:block_id' or 'minecraft:stone'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_anvil::block::Block;
    /// let block = Block::new("minecraft".into(), Some("stone".into()));
    /// println!("{}", block.name());
    /// ```
    pub fn name(self) -> String {
        let mut name = self.namespace;
        name += ":";
        name += self.id.as_str();
        return name;
    }

    /// Returns a Block from a name
    ///
    /// # Arguments
    ///
    /// * `name` - The fullname of the block, this includes the namespace and the colon.
    ///  
    /// # Examples
    ///
    /// ```rust
    /// use simple_anvil::block::Block;
    /// let block = Block::from_name("minecraft:stone".into());
    /// println!("{}", block.id);
    /// ```
    pub fn from_name(name: String) -> Block {
        let temp: Vec<&str> = name.split(":").collect();
        return Block {
            namespace: temp[0].to_owned(),
            id: temp[1].to_owned(),
        };
    }

    /// Returns a block from a Chunk palette value
    ///
    /// # Arguments
    ///
    /// * `tag` - The value for the block from a chunk. This should be a HashMap containing all of the contents of the block.
    pub fn from_palette(tag: &Value) -> Block {
        let tag = if let Value::Compound(t) = tag {
            t
        } else {
            panic!("Tag passed from palette is not compound")
        };
        let name = if let Value::String(n) = tag.get("Name").unwrap() {
            n
        } else {
            panic!("Palette tag missing name?")
        };
        return Block::from_name(name.to_string());
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.id)
    }
}
