use z_patterns::prelude::*;

fn main() {
    println!("=== Coalition Example ===\n");

    // Create coalition
    let mut coalition = Coalition::new("defi_coalition");

    // Add members
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    let agent3 = AgentId::new();

    coalition.add_member(agent1);
    coalition.add_member(agent2);
    coalition.add_member(agent3);

    // Set strategy
    let strategy =
        Strategy::new(StrategyType::MaximizeUtility).with_parameter("risk_tolerance", 0.6);

    coalition.set_strategy(strategy);
    coalition.set_value(5000.0);

    println!("Coalition: {}", coalition.name());
    println!("Size: {}", coalition.size());
    println!("Value: ${}", coalition.value());
}
