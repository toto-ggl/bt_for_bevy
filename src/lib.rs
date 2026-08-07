pub mod behavior;
pub mod decorator;
pub mod macros;
pub mod node;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::bt_tick_test;
    use crate::prelude::*;
    use bevy_ecs::prelude::{Commands, Component, Entity, World};

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

    // ***********************************
    // *    GateWithoutNegativeBranch    *
    // ***********************************
    struct GateWithoutNegativeBranch<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for GateWithoutNegativeBranch<()> {
        fn get_name(&self) -> &str {
            "GateWithoutNegativeBranch"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree should play Step2 then Condition1 and Step3, then fail on Gate3
            // because it has no negative branch to fall back on.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        // Decorated gate: the negative branch fails, the Invert turns it into a success.
                        bt_gate!(
                            "Gate1",
                            failing_condition,
                            bt_action!("Step1", successful_action),
                            bt_action!("Step2", failing_action),
                            bt_invert!()
                        ),
                        // Gate with a positive branch spanning several ticks and no negative branch.
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
                        // No negative branch and a failing condition: the gate fails.
                        bt_gate!(
                            "Gate3",
                            failing_condition,
                            bt_action!("Step4", successful_action)
                        ),
                        // Should never tick
                        bt_action!("Step5", successful_action)
                    ]
                ),
            }
        }
    }

    #[test]
    fn gate_without_negative_branch() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = GateWithoutNegativeBranch::build();
        let mut status;

        // Gate1: negative branch fails, inverted into a success.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        // Gate2: Condition1 of the positive branch.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        // Gate2: Step3 of the positive branch, the inner sequence resolves.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Running);

        // Gate3: failing condition and no negative branch.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT: the gates must reset their branches, so Gate2 needs two ticks again.
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

    // **************
    // *    Wait    *
    // **************
    /// Time step used by the Wait test.
    const DELTA: f32 = 0.5;

    struct Wait<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<()> for Wait<()> {
        fn get_name(&self) -> &str {
            "Wait"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<()> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed at the end.
            // Wait1 needs two ticks of DELTA to elapse, Wait2 resolves on its first tick.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_wait!("Wait1", 1.),
                        bt_action!("Step1", successful_action),
                        bt_wait!("Wait2", 0.),
                    ]
                ),
            }
        }
    }

    #[test]
    fn wait() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = Wait::build();
        let mut status;

        // Wait1: half of the duration has elapsed.
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        // Wait1: the duration is now elapsed, the sequence moves on.
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        // Step1
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        // Wait2: a null duration resolves immediately.
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Success);

        // Reset BT: the elapsed time is cleared, so Wait1 needs two ticks again.
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Success);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity, DELTA);
        assert_eq!(status, BtNodeStatus::Success);
    }

    // ******************
    // *    Commands    *
    // ******************
    #[derive(Component, Debug, PartialEq)]
    struct Marker(u32);

    #[derive(Component)]
    struct Removable;

    #[derive(Component)]
    struct Flag;

    struct CommandsData {
        value: u32,
    }

    fn insert_marker(commands: &mut Commands, entity: Entity, data: &CommandsData) {
        commands.entity(entity).insert(Marker(data.value));
    }

    fn remove_removable(commands: &mut Commands, entity: Entity, _: &CommandsData) {
        commands.entity(entity).remove::<Removable>();
    }

    fn insert_flag(commands: &mut Commands, entity: Entity, _: &CommandsData) {
        commands.entity(entity).insert(Flag);
    }

    struct CommandsTree<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<CommandsData> for CommandsTree<CommandsData> {
        fn get_name(&self) -> &str {
            "CommandsTree"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<CommandsData> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will fail and stop on third step.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_commands!("Insert", insert_marker),
                        bt_commands!("Remove", remove_removable),
                        // A Commands node is always successful: the Invert decorator makes it fail.
                        bt_commands!("Flag", insert_flag, bt_invert!()),
                        // Should never tick
                        bt_commands!("Never", insert_marker),
                    ]
                ),
            }
        }
    }

    /// Ticks the behavior tree, then applies the queued commands to the world.
    fn tick_and_flush(
        test_bt: &mut CommandsTree<CommandsData>,
        world: &mut World,
        entity: Entity,
        data: &mut CommandsData,
    ) -> BtNodeStatus {
        let status = {
            let mut commands = world.commands();
            bt_tick_test!(test_bt, &mut commands, entity, data, 0.)
        };
        world.flush();

        status
    }

    #[test]
    fn commands() {
        let mut world = World::new();
        let entity = world.spawn(Removable).id();
        let mut test_bt = CommandsTree::build();
        let mut data = CommandsData { value: 7 };
        let mut status;

        // Insert: the handler builds the component out of the behavior tree data.
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(world.get::<Marker>(entity), Some(&Marker(7)));

        // Remove
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Running);
        assert!(!world.entity(entity).contains::<Removable>());

        // Flag: the command is applied, but the Invert decorator fails the sequence.
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Failure);
        assert!(world.entity(entity).contains::<Flag>());

        // Test status hasn't changed unexpectedly.
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Failure);

        // Reset BT
        test_bt.root.reset();
        data.value = 42;

        // Insert: the handler reads the updated data.
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(world.get::<Marker>(entity), Some(&Marker(42)));

        // Remove: the component is already gone, removing it again is a no-op.
        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Running);

        // Only the last node would pick this value up, and it never ticks.
        data.value = 99;

        status = tick_and_flush(&mut test_bt, &mut world, entity, &mut data);
        assert_eq!(status, BtNodeStatus::Failure);
        assert_eq!(world.get::<Marker>(entity), Some(&Marker(42)));
    }

    // *****************
    // *    Actions    *
    // *****************
    struct ActionData {
        count: u32,
    }

    /// Counts one tick, and keeps running until three ticks have been counted.
    fn count_up(data: &mut ActionData) -> BtNodeStatus {
        data.count += 1;

        if data.count >= 3 {
            BtNodeStatus::Success
        } else {
            BtNodeStatus::Running
        }
    }

    /// Reads the data without mutating it.
    fn fail_if_even(data: &mut ActionData) -> BtNodeStatus {
        if data.count % 2 == 0 {
            BtNodeStatus::Failure
        } else {
            BtNodeStatus::Success
        }
    }

    struct Actions<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<ActionData> for Actions<ActionData> {
        fn get_name(&self) -> &str {
            "Actions"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<ActionData> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed on its third tick.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_action!("Count", count_up),
                        bt_action!("FailIfEven", fail_if_even),
                    ]
                ),
            }
        }
    }

    #[test]
    fn actions() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = Actions::build();
        let mut data = ActionData { count: 0 };
        let mut status;

        // Count: an action can stay Running over several ticks, the sequence waits for it.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 1);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 2);

        // Count: the action succeeds, the sequence moves on.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 3);

        // FailIfEven: the count is odd, the action succeeds and the sequence resolves.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);
        assert_eq!(data.count, 3);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);

        // Reset BT: an action holds no state of its own, so the data is left untouched
        // and Count succeeds on its first tick this time.
        test_bt.root.reset();
        assert_eq!(data.count, 3);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 4);

        // FailIfEven: the count is now even, the action fails and so does the sequence.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Failure);

        // Test status hasn't changed unexpectedly.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Failure);
    }

    // ****************************
    // *    SuccessfulActions     *
    // ****************************
    struct SuccessfulActionData {
        count: u32,
    }

    /// A successful action has no return value: it cannot report a failure.
    fn increment(data: &mut SuccessfulActionData) {
        data.count += 1;
    }

    fn double(data: &mut SuccessfulActionData) {
        data.count *= 2;
    }

    struct SuccessfulActions<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<SuccessfulActionData> for SuccessfulActions<SuccessfulActionData> {
        fn get_name(&self) -> &str {
            "SuccessfulActions"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<SuccessfulActionData> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed on its third tick.
            Self {
                root: bt_sequence!(
                    "RootSeq",
                    vec![
                        bt_successful_action!("Increment", increment),
                        bt_selector!(
                            "Select",
                            vec![
                                // A SuccessfulAction always succeeds, whatever the data:
                                // the Invert decorator is the only way to make it fail.
                                bt_successful_action!("Double", double, bt_invert!()),
                                bt_successful_action!("IncrementAgain", increment),
                            ]
                        ),
                    ]
                ),
            }
        }
    }

    #[test]
    fn successful_actions() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = SuccessfulActions::build();
        let mut data = SuccessfulActionData { count: 0 };
        let mut status;

        // Increment: the action mutates the data and resolves on its first tick,
        // so the sequence moves on right away.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 1);

        // Double: the action runs, and the Invert decorator turns its success into
        // a failure, so the selector falls back on its second branch.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 2);

        // IncrementAgain: the selector resolves, and so does the sequence.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);
        assert_eq!(data.count, 3);

        // Test status hasn't changed unexpectedly. A resolved sequence keeps ticking
        // its last child, so the action runs once more.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);
        assert_eq!(data.count, 4);

        // Reset BT: a successful action holds no state of its own,
        // so the data is left untouched.
        test_bt.root.reset();
        assert_eq!(data.count, 4);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 5);

        // The selector is back on its first branch, so Double runs again.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert_eq!(data.count, 10);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);
        assert_eq!(data.count, 11);
    }

    // ********************
    // *    Conditions    *
    // ********************
    struct ConditionData {
        open: bool,
    }

    fn open_door(data: &mut ConditionData) -> BtNodeStatus {
        data.open = true;

        BtNodeStatus::Success
    }

    fn is_door_open(data: &ConditionData) -> bool {
        data.open
    }

    struct Conditions<D> {
        root: BtNode<D>,
    }

    impl BehaviorTree<ConditionData> for Conditions<ConditionData> {
        fn get_name(&self) -> &str {
            "Conditions"
        }

        fn get_root_mut(&mut self) -> &mut BtNode<ConditionData> {
            &mut self.root
        }

        fn build() -> Self {
            // This behavior tree will succeed, either right away if the door is already open,
            // or through the fallback branch which opens it first.
            Self {
                root: bt_selector!(
                    "RootSelect",
                    vec![
                        bt_condition!("IsOpen", is_door_open),
                        bt_sequence!(
                            "OpenSeq",
                            vec![
                                bt_action!("Open", open_door),
                                bt_condition!("IsOpenNow", is_door_open),
                            ]
                        ),
                    ]
                ),
            }
        }
    }

    #[test]
    fn conditions() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut commands = world.commands();
        let mut test_bt = Conditions::build();
        let mut data = ConditionData { open: false };
        let mut status;

        // IsOpen: the door is closed, the condition fails and the selector falls back.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);

        // Open: the action opens the door.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);
        assert!(data.open);

        // IsOpenNow: the condition reads the data the action just mutated, and succeeds.
        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);

        // Reset BT: a condition holds no state of its own, so the door is still open
        // and the first branch of the selector succeeds right away.
        test_bt.root.reset();

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);

        // Closing the door puts the selector back on its fallback branch,
        // proving the condition is read against the current data on every tick.
        test_bt.root.reset();
        data.open = false;

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Running);

        status = bt_tick_test!(test_bt, &mut commands, entity, &mut data, 0.);
        assert_eq!(status, BtNodeStatus::Success);
    }
}
