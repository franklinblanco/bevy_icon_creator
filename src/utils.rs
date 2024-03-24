use bevy::prelude::*;

pub fn mark_all_children_with_component<T: Bundle + Clone>(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
    bundle: T,
    mark_parent: bool,
) {
    if mark_parent {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.try_insert(bundle.clone());
        }
    }
    mark_all_children_with_component_recursive(commands, entity, children_query, bundle, true);
}

fn mark_all_children_with_component_recursive<T: Bundle + Clone>(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
    bundle: T,
    first_time: bool,
) {
    if let Ok(new_children) = children_query.get(entity) {
        for child in new_children.iter() {
            mark_all_children_with_component_recursive(commands, *child, children_query, bundle.clone(), false);
        }
    }
    if !first_time {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.try_insert(bundle);
        }
    }
}