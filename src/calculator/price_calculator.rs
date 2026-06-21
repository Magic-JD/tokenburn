use crate::data::claude_log::Usage;
use std::collections::HashMap;

pub struct PriceCalculator {
    pricing_map: HashMap<String, Pricing>,
}
impl PriceCalculator {
    pub fn new() -> Self {
        //TODO Still want to be able to lead this from a resource
        let pricing_map = HashMap::from([
            // Testing
            (String::from("qwen2.5-coder:0.5b"), Pricing::new(50.0, 500.0)),

            // Anthropic
            (String::from("claude-opus-4.6"), Pricing::new(5.0, 25.0)),
            (String::from("claude-opus-4.5"), Pricing::new(5.0, 25.0)),
            (String::from("claude-opus-4.1"), Pricing::new(15.0, 75.0)),

            (String::from("claude-sonnet-4.6"), Pricing::new(3.0, 15.0)),
            (String::from("claude-sonnet-4.5"), Pricing::new(3.0, 15.0)),
            (String::from("claude-sonnet-4"), Pricing::new(3.0, 15.0)),

            (String::from("claude-haiku-4.5"), Pricing::new(1.0, 5.0)),
            (String::from("claude-haiku-3.5"), Pricing::new(0.8, 4.0)),
            (String::from("claude-haiku-3"), Pricing::new(0.25, 1.25)),

            // OpenAI GPT-5
            (String::from("gpt-5"), Pricing::new(1.25, 10.0)),
            (String::from("gpt-5-mini"), Pricing::new(0.25, 2.0)),
            (String::from("gpt-5-nano"), Pricing::new(0.05, 0.4)),
            (String::from("gpt-5-pro"), Pricing::new(15.0, 120.0)),
            (String::from("gpt-5.2"), Pricing::new(1.75, 14.0)),
            (String::from("gpt-5.2-pro"), Pricing::new(21.0, 168.0)),

            // OpenAI GPT-4.x
            (String::from("gpt-4.1"), Pricing::new(2.0, 8.0)),
            (String::from("gpt-4.1-mini"), Pricing::new(0.4, 1.6)),
            (String::from("gpt-4.1-nano"), Pricing::new(0.1, 0.4)),
            (String::from("gpt-4o"), Pricing::new(2.5, 10.0)),
            (String::from("gpt-4o-mini"), Pricing::new(0.15, 0.6)),

            // OpenAI reasoning
            (String::from("o3"), Pricing::new(2.0, 8.0)),
            (String::from("o3-pro"), Pricing::new(20.0, 80.0)),
            (String::from("o4-mini"), Pricing::new(1.1, 4.4)),
            (String::from("o3-mini"), Pricing::new(1.1, 4.4)),
            (String::from("o1"), Pricing::new(15.0, 60.0)),
        ]);
        Self { pricing_map }
    }

    pub fn calculate_cost(&self, model: &String, usage: Usage) -> f32 {
        let pricing = self
            .pricing_map
            .get(model)
            .expect(format!("No cost for {:?}", model).as_str());
        let output_cost: f32 = usage.output_tokens as f32 * pricing.output_per_token;
        let input_cost: f32 = usage.input_tokens as f32 * pricing.input_per_token;
        output_cost + input_cost
    }
}

// Get token data from online somewhere?

pub struct Pricing {
    input_per_token: f32,
    output_per_token: f32,
}

impl Pricing {
    pub(crate) fn new(input_per_million: f32, output_per_million: f32) -> Self {
        Self {
            input_per_token: input_per_million / 1_000_000f32,
            output_per_token: output_per_million / 1_000_000f32,
        }
    }
}
