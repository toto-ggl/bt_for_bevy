use crate::node::BtNodeStatus;

/// Decorator
#[derive(PartialEq)]
pub enum BtDecorator {
    Invert,
    Repeat,
    Succeed,
}

impl BtDecorator {
    /// Returns a computed status and eventually a command to reset the node.
    pub fn exec(&self, status: BtNodeStatus) -> (BtNodeStatus, bool) {
        match self {
            BtDecorator::Invert => {
                let computed_status = if status == BtNodeStatus::Success {
                    BtNodeStatus::Failure
                } else if status == BtNodeStatus::Failure {
                    BtNodeStatus::Success
                } else {
                    BtNodeStatus::Running
                };

                (computed_status, false)
            }
            BtDecorator::Repeat => (BtNodeStatus::Running, status.is_done()),
            BtDecorator::Succeed => (BtNodeStatus::Success, false),
        }
    }
}
