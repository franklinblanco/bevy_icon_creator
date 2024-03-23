use bevy::{ecs::{query::QueryEntityError, system::Command}, prelude::*};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityTreeNode {
    pub source: Entity,
    pub destination: Entity,
    pub children: Vec<EntityTreeNode>,
}

impl EntityTreeNode {
    pub fn from_entity_recursive(
        commands: &mut Commands,
        from_entity: Entity,
        q_children: &Query<&Children>,
    ) -> EntityTreeNode {
        let children = match q_children.get(from_entity) {
            Ok(children) => children
                .iter()
                .map(|&child| {
                    EntityTreeNode::from_entity_recursive(
                        commands, child, q_children,
                    )
                })
                .collect::<Vec<_>>(),
            Err(QueryEntityError::QueryDoesNotMatch(_)) => vec![],
            Err(e) => panic!("{}", e),
        };
        EntityTreeNode {
            source: from_entity,
            destination: commands.spawn_empty().id(),
            children,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &EntityTreeNode> {
        self.children.iter()
    }
}

pub fn clone_entity_tree(
    world: &mut World,
    EntityTreeNode {
        source,
        destination,
        children,
    }: &EntityTreeNode,
) {
    //clone_entity_components(world, *source, *destination);
    for node in children {
        clone_entity_tree(world, node);
        let mut destination = world.get_entity_mut(*destination).unwrap();
        destination.add_child(node.destination);
    }
}
// uses arc to prevent cloning the whole tree.
pub struct CloneEntityTree(Arc<EntityTreeNode>);
impl Command for CloneEntityTree {
    fn apply(self, world: &mut World) {
        clone_entity_tree(world, &self.0);
    }
}