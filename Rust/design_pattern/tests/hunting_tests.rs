use design_pattern::strategy::{ambush::Ambush, chase::Chase, feline::Feline};

#[test]
fn test_strategy_can_be_swapped_at_runtime() {
    let mut tiger = Feline::new("tiger", Box::new(Ambush));
    let result = tiger.hunt("deer");
    assert_eq!(result, "The tiger is ambushing the deer");

    tiger.set_strategy(Box::new(Chase));
    let result = tiger.hunt("deer");
    assert_eq!(result, "The tiger is chasing the deer");
}
