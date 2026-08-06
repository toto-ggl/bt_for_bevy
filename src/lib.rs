pub mod behavior;
pub mod decorator;
pub mod macros;
pub mod node;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::bt_tick_test;
    use crate::prelude::*;
    use bevy::prelude::World;

    fn successful_action(_: &mut ()) -> BtNodeStatus {
        BtNodeStatus::Success
    }
    fn failing_action(_: &mut ()) -> BtNodeStatus {
        BtNodeStatus::Failure
    }
    fn successful_condition(_: &()) -> bool {
        true
    }
    fn failing_condition(_: &()) -> bool {
        false
    }

    // **********************************
    // *    SimpleSuccessfulSequence    *
    // **********************************
    struct SimpleSuccessfulSequence<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for SimpleSuccessfulSequence<()> {
        fn get_name(&self) -> &str {
            "SimpleSuccessfulSequence"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed at the end.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_condition!("Step1", successful_condition),
                        bt_action!("Step2", successful_action),
                        bt_condition!("Step3", successful_condition),
                        bt_action!("Step4", successful_action),
                    ]
                ),
            }
        }
    }

    #[test]
    fn simple_successful_sequence() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = SimpleSuccessfulSequence::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);
    }

    // *******************************
    // *    SimpleFailingSequence    *
    // *******************************
    struct SimpleFailingSequence<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for SimpleFailingSequence<()> {
        fn get_name(&self) -> &str {
            "SimpleFailingSequence"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will fail and stop on third step.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_condition!("Step1", successful_condition),
                        bt_action!("Step2", successful_action),
                        bt_condition!("Step3", failing_condition),
                        // Step4 should never tick because Step3 is failing.
                        bt_action!("Step4", successful_action),
                    ]
                ),
            }
        }
    }

    #[test]
    fn simple_failing_sequence() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = SimpleFailingSequence::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);
    }

    // *************************
    // *    NestedSequences    *
    // *************************
    struct NestedSequences<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for NestedSequences<()> {
        fn get_name(&self) -> &str {
            "NestedSequences"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will fail and stop on third step (Step1 of Seq2).
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_sequence!(
                            "Seq1",
                            vec![
                                bt_condition!("Step1", successful_condition),
                                bt_action!("Step2", successful_action),
                            ]
                        ),
                        bt_sequence!(
                            "Seq2",
                            vec![
                                bt_condition!("Step1", failing_condition),
                                // Step2 should never tick because Step1 is failing.
                                bt_action!("Step2", successful_action),
                            ]
                        )
                    ]
                ),
            }
        }
    }

    #[test]
    fn nested_sequences() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = NestedSequences::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);
    }

    // **************
    // *    Gate    *
    // **************
    struct Gate<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for Gate<()> {
        fn get_name(&self) -> &str {
            "Gate"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree should play Step1 then Step3 then Step5 and fail.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_gate!(
                            "Gate1",
                            successful_condition,
                            bt_action!("Step1", successful_action),
                            bt_action!("Step2", failing_action)
                        ),
                        bt_gate!(
                            "Gate2",
                            successful_condition,
                            bt_sequence!(
                                "Seq1",
                                vec![
                                    bt_condition!("Condition1", successful_condition),
                                    bt_action!("Step3", successful_action)
                                ]
                            )
                        ),
                        bt_gate!(
                            "Gate3",
                            failing_condition,
                            bt_action!("Step4", successful_action),
                            bt_action!("Step5", failing_action)
                        ),
                        // Should never tick
                        bt_action!("Step6", successful_action)
                    ]
                ),
            }
        }
    }

    #[test]
    fn gate() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = Gate::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);
    }

    // **********************************
    // *    SimpleSuccessfulSelector    *
    // **********************************
    struct SimpleSuccessfulSelector<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for SimpleSuccessfulSelector<()> {
        fn get_name(&self) -> &str {
            "SimpleSuccessfulSelector"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed and stop on second step.
            Self {
                root: bt_selector!(
                    "RootSelect",
                    vec![
                        bt_condition!("Step1", failing_condition),
                        bt_action!("Step2", successful_action),
                        // Step3 should never tick because Step2 is successful.
                        bt_condition!("Step3", failing_condition),
                    ]
                ),
            }
        }
    }

    #[test]
    fn simple_successful_selector() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = SimpleSuccessfulSelector::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Success);
    }

    // **********************************
    // *    SimpleSuccessfulSelector    *
    // **********************************
    struct SimpleFailingSelector<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for SimpleFailingSelector<()> {
        fn get_name(&self) -> &str {
            "SimpleFailingSelector"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will fail at the end.
            Self {
                root: bt_selector!(
                    "RootSelect",
                    vec![
                        bt_condition!("Step1", failing_condition),
                        bt_action!("Step2", failing_action),
                        bt_condition!("Step3", failing_condition),
                    ]
                ),
            }
        }
    }

    #[test]
    fn simple_failing_selector() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = SimpleFailingSelector::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);
    }

    // ********************
    // *    Decorators    *
    // ********************
    struct Decorators<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for Decorators<()> {
        fn get_name(&self) -> &str {
            "Decorators"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed and repeat itself.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_sequence!(
                            "Seq1",
                            vec![
                                bt_condition!(
                                    "Step1",
                                    failing_condition,
                                    vec![BtDecorator::Invert]
                                ),
                                bt_action!("Step2", successful_action),
                            ]
                        ),
                        bt_sequence!(
                            "Seq2",
                            vec![bt_action!(
                                "Step1",
                                failing_action,
                                vec![BtDecorator::Succeed]
                            ),]
                        )
                    ],
                    vec![BtDecorator::Repeat]
                ),
            }
        }
    }

    #[test]
    fn decorators() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = Decorators::build();
        let mut status;

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        // New cycle here:
        // The repeat decorator applied on the BT root asks for a reset
        // because both child sequences have succeeded.

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);
    }
}
