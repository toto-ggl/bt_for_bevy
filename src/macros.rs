/// Returns a [`BtNode`] containing an [`BtBehavior::Action`]
#[macro_export]
macro_rules! bt_action {
    ($name:expr, $action:expr, $decorators:expr) => {
        BtNode::new(BtBehavior::Action($name.to_string(), $action), $decorators)
    };
    ($name:expr, $action:expr) => {
        BtNode::new(BtBehavior::Action($name.to_string(), $action), vec![])
    };
}

/// Returns a BtNode containing a SuccessfulAction BtBehavior
#[macro_export]
macro_rules! bt_successful_action {
    ($name:expr, $action:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::SuccessfulAction($name.to_string(), $action),
            $decorators,
        )
    };
    ($name:expr, $action:expr) => {
        BtNode::new(
            BtBehavior::SuccessfulAction($name.to_string(), $action),
            vec![],
        )
    };
}

/// Returns a BtNode containing a Condition BtBehavior
#[macro_export]
macro_rules! bt_condition {
    ($name:expr, $condition:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Condition($name.to_string(), $condition),
            $decorators,
        )
    };
    ($name:expr, $condition:expr) => {
        BtNode::new(BtBehavior::Condition($name.to_string(), $condition), vec![])
    };
}

/// Returns a BtNode containing a Gate BtBehavior
#[macro_export]
macro_rules! bt_gate {
    ($name:expr, $condition:expr, $positive:expr, $negative: expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Gate(
                $name.to_string(),
                $condition,
                Box::new($positive),
                Some(Box::new($negative)),
            ),
            $decorators,
        )
    };
    ($name:expr, $condition:expr, $positive:expr, $negative: expr) => {
        BtNode::new(
            BtBehavior::Gate(
                $name.to_string(),
                $condition,
                Box::new($positive),
                Some(Box::new($negative)),
            ),
            vec![],
        )
    };
    ($name:expr, $condition:expr, $positive:expr) => {
        BtNode::new(
            BtBehavior::Gate($name.to_string(), $condition, Box::new($positive), None),
            vec![],
        )
    };
}

/// Returns a BtNode containing a Sequence BtBehavior
#[macro_export]
macro_rules! bt_sequence {
    ($name:expr, $children:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Sequence($name.to_string(), $children, 0),
            $decorators,
        )
    };
    ($name:expr, $children:expr) => {
        BtNode::new(
            BtBehavior::Sequence($name.to_string(), $children, 0),
            vec![],
        )
    };
}

/// Returns a BtNode containing a Selector BtBehavior
#[macro_export]
macro_rules! bt_selector {
    ($name:expr, $children:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Selector($name.to_string(), $children, 0),
            $decorators,
        )
    };
    ($name:expr, $children:expr) => {
        BtNode::new(
            BtBehavior::Selector($name.to_string(), $children, 0),
            vec![],
        )
    };
}

/// Returns a BtNode containing a Wait BtBehavior
#[macro_export]
macro_rules! bt_wait {
    ($name:expr, $duration:expr, $elapsed:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Wait($name.to_string(), $duration, $elapsed),
            $decorators,
        )
    };
    ($name:expr, $duration:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Wait($name.to_string(), $duration, 0.),
            $decorators,
        )
    };
    ($name:expr, $duration:expr) => {
        BtNode::new(BtBehavior::Wait($name.to_string(), $duration, 0.), vec![])
    };
}

/// Returns a [`BtNode`] containing a [`BtBehavior::Commands`]
#[macro_export]
macro_rules! bt_commands {
    ($name:expr, $handle:expr, $decorators:expr) => {
        BtNode::new(
            BtBehavior::Commands($name.to_string(), $handle),
            $decorators,
        )
    };
    ($name:expr, $handle:expr) => {
        BtNode::new(BtBehavior::Commands($name.to_string(), $handle), vec![])
    };
}

/// Returns a [`BtDecorator::Invert`] in a vec.
#[macro_export]
macro_rules! bt_invert {
    () => {
        vec![BtDecorator::Invert]
    };
}

/// Returns a [`BtDecorator::Repeat`] in a vec.
#[macro_export]
macro_rules! bt_repeat {
    () => {
        vec![BtDecorator::Repeat]
    };
}

/// Utility for tests. Ticks a specified BehaviorTree and returns the result.
#[macro_export]
macro_rules! bt_tick_test {
    ($bt:expr, $commands:expr, $entity:expr) => {
        $bt.tick($commands, $entity, &mut (), 0.)
    };
}
