use crate::node::{BtNode, BtNodeStatus};
use bevy::prelude::{Commands, Entity};
use log::trace;

/// Behavior Tree
pub trait BehaviorTree<Data> {
    fn get_name(&self) -> &str;

    fn get_root_mut(&mut self) -> &mut BtNode<Data>;

    fn build() -> Self;

    /// When ticking, a behavior tree will process and eventually mutate a set of given data through actions.
    fn tick(
        &mut self,
        mut commands: &mut Commands,
        entity: Entity,
        data: &mut Data,
        delta: f32,
    ) -> BtNodeStatus {
        trace!("Behavior tree {:}", self.get_name());

        self.get_root_mut().tick(&mut commands, entity, data, delta)
    }
}

/// Behavior
pub enum BtBehavior<Data> {
    /// Action
    Action(String, fn(&mut Data) -> BtNodeStatus),
    /// SuccessfulAction
    SuccessfulAction(String, fn(&mut Data)),
    /// Condition
    Condition(String, fn(&Data) -> bool),
    /// Gate
    Gate(
        String,
        fn(&Data) -> bool,
        Box<BtNode<Data>>,
        Option<Box<BtNode<Data>>>,
    ),
    /// Offers access to bevy's Commands and entity directly inside the BehaviorTree.
    /// Allows inserting or removing bevy Components to the entity.
    /// Always successful (use with an `Invert` decorator to make it fail).
    Commands(String, fn(&mut Commands, Entity, &Data)),
    /// Useful to represent multiple different courses of action,
    /// in order of priority from most favorable to least favorable.
    Selector(String, Vec<BtNode<Data>>, usize),
    /// Sequence
    Sequence(String, Vec<BtNode<Data>>, usize),
    /// Wait
    Wait(String, f32, f32),
}

impl<D> BtBehavior<D> {
    pub fn tick(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        data: &mut D,
        delta: f32,
    ) -> BtNodeStatus {
        match self {
            Self::Action(name, action) => {
                trace!("Action {:}", name);

                action(data)
            }
            Self::SuccessfulAction(name, action) => {
                trace!("SuccessfulAction {:}", name);
                action(data);

                BtNodeStatus::Success
            }
            Self::Condition(name, condition) => {
                trace!("Condition {:}", name);

                if condition(data) {
                    BtNodeStatus::Success
                } else {
                    BtNodeStatus::Failure
                }
            }
            Self::Gate(name, condition, positive_node, negative_node) => {
                trace!("Gate {:}", name);

                if condition(data) {
                    positive_node.tick(commands, entity, data, delta)
                } else {
                    match negative_node {
                        Some(node) => node.tick(commands, entity, data, delta),
                        None => BtNodeStatus::Failure,
                    }
                }
            }
            Self::Commands(name, handle) => {
                trace!("Commands {:}", name);
                handle(commands, entity, data);

                BtNodeStatus::Success
            }
            Self::Selector(name, children, index) => {
                trace!("Selector {:}", name);

                tick_composite(
                    commands,
                    entity,
                    data,
                    delta,
                    children,
                    index,
                    (BtNodeStatus::Failure, BtNodeStatus::Success),
                )
            }
            Self::Sequence(name, children, index) => {
                trace!("Sequence {:}", name);

                tick_composite(
                    commands,
                    entity,
                    data,
                    delta,
                    children,
                    index,
                    (BtNodeStatus::Success, BtNodeStatus::Failure),
                )
            }
            Self::Wait(name, duration, elapsed) => {
                trace!("Wait {:}", name);

                *elapsed += delta;

                if *elapsed >= *duration {
                    BtNodeStatus::Success
                } else {
                    BtNodeStatus::Running
                }
            }
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::Gate(_, _, positive_node, negative_node) => {
                positive_node.reset();
                if let Some(node) = negative_node {
                    node.reset();
                }
            }
            Self::Sequence(name, children, index) => {
                reset_composite(format!("Sequence {}", name), children, index);
            }
            Self::Selector(name, children, index) => {
                reset_composite(format!("Selector {}", name), children, index);
            }
            Self::Wait(_, _, elapsed) => {
                *elapsed = 0.;
            }
            _ => {}
        }
    }
}

/// Composites are nodes that tick a series of children nodes in a specific order.
///
/// Example: Sequences and Selectors
fn tick_composite<D>(
    mut commands: &mut Commands,
    entity: Entity,
    data: &mut D,
    delta: f32,
    children: &mut Vec<BtNode<D>>,
    index: &mut usize,
    (resolve, stop): (BtNodeStatus, BtNodeStatus),
) -> BtNodeStatus {
    let mut status = BtNodeStatus::Running;
    let child_status = children[*index].tick(&mut commands, entity, data, delta);
    if child_status == stop {
        // The current child node just got the "stop" status: stop the composite node.
        status = stop;
    } else if child_status == resolve {
        if *index == children.len() - 1 {
            // End of the composite node: resolve.
            status = resolve;
        } else {
            // Else go to the next child node.
            *index += 1;
        }
    }

    status
}

fn reset_composite<D>(name: String, children: &mut Vec<BtNode<D>>, index: &mut usize) {
    trace!("Reset {:}", name);

    *index = 0;
    for node in children {
        node.reset();
    }
}
