use crate::data::claude_log::Usage;
use std::collections::HashMap;

pub struct PriceCalculator {
    pricing_map: HashMap<String, Pricing>,
}
impl PriceCalculator {
    pub fn new() -> Self {
        //TODO Still want to be able to lead this from a resource
        let pricing_map = HashMap::from([
            (String::from("<synthetic>"), Pricing::new(10, 100)),
            (String::from("qwen3.5:0.8b"), Pricing::new(1, 10)),
            (String::from("qwen2.5-coder:0.5b"), Pricing::new(50, 500)),
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
    pub(crate) fn new(input_per_million: isize, output_per_million: isize) -> Self {
        Self {
            input_per_token: input_per_million as f32 / 1_000_000f32,
            output_per_token: output_per_million as f32 / 1_000_000f32,
        }
    }
}
