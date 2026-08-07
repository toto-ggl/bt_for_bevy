use crate::behavior::BtBehavior;
use crate::decorator::BtDecorator;
use bevy_ecs::prelude::{Commands, Component, Entity};

/// Node
#[derive(Component)]
pub struct BtNode<Data> {
    pub status: BtNodeStatus,
    pub behavior: BtBehavior<Data>,
    pub decorators: Vec<BtDecorator>,
}

impl<D> BtNode<D> {
    pub fn new(behavior: BtBehavior<D>, decorators: Vec<BtDecorator>) -> Self {
        Self {
            behavior,
            decorators,
            status: BtNodeStatus::Running,
        }
    }

    pub fn tick(
        &mut self,
        mut commands: &mut Commands,
        entity: Entity,
        data: &mut D,
        delta: f32,
    ) -> BtNodeStatus {
        let mut reset = false;

        // Tick the node's behavior and get the result status.
        let mut status = self.behavior.tick(&mut commands, entity, data, delta);

        // Apply a serie of decorators to the status (decorators can also ask for a reset).
        for decorator in &self.decorators {
            (status, reset) = decorator.exec(status);
        }

        // Node status = its behavior status
        self.status = status;

        // Reset if wanted.
        if reset {
            self.reset();
        }

        self.status
    }

    pub fn reset(&mut self) {
        self.status = BtNodeStatus::Running;
        self.behavior.reset();
    }
}

/// Node status
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BtNodeStatus {
    Running,
    Success,
    Failure,
}

impl BtNodeStatus {
    pub fn is_done(self) -> bool {
        self == Self::Success || self == Self::Failure
    }
}
