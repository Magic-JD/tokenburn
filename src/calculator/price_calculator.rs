use crate::data::claude_log::Usage;
use std::collections::HashMap;

pub struct PriceCalculator {
    pricing_map: HashMap<String, Pricing>,
}
impl PriceCalculator {
    pub fn new() -> Self {
        //TODO Still want to be able to lead this from a resource - For now maintaining Anthropic manually
        let pricing_map = HashMap::from([
            // Testing
            (
                String::from("qwen2.5-coder:0.5b"),
                Pricing::new(50.0, 500.0),
            ),
            // Using sonnet 4.6 for synthetic
            (String::from("<synthetic>"), Pricing::new(3.0, 15.0)),
            // Anthropic
            (String::from("claude-opus-4-6"), Pricing::new(5.0, 25.0)),
            (String::from("claude-opus-4-5"), Pricing::new(5.0, 25.0)),
            (String::from("claude-opus-4-1"), Pricing::new(15.0, 75.0)),
            (String::from("claude-sonnet-4-6"), Pricing::new(3.0, 15.0)),
            (String::from("claude-sonnet-4-5"), Pricing::new(3.0, 15.0)),
            (String::from("claude-sonnet-4"), Pricing::new(3.0, 15.0)),
            (String::from("claude-haiku-4-5"), Pricing::new(1.0, 5.0)),
            (String::from("claude-haiku-3-5"), Pricing::new(0.8, 4.0)),
            (String::from("claude-haiku-3"), Pricing::new(0.25, 1.25)),
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
        let cache_read_cost: f32 =
            usage.cache_read_input_tokens as f32 * pricing.cache_read_per_token;
        let cache_write_5m_cost: f32 = usage.cache_creation.ephemeral_5m_input_tokens as f32
            * pricing.cache_5m_write_per_token;
        let cache_write_1h_cost: f32 = usage.cache_creation.ephemeral_1h_input_tokens as f32
            * pricing.cache_1h_write_per_token;
        output_cost + input_cost + cache_read_cost + cache_write_5m_cost + cache_write_1h_cost
    }
}

// Get token data from online somewhere?

pub struct Pricing {
    input_per_token: f32,
    output_per_token: f32,
    cache_5m_write_per_token: f32,
    cache_1h_write_per_token: f32,
    cache_read_per_token: f32,
}

impl Pricing {
    pub(crate) fn new(input_per_million: f32, output_per_million: f32) -> Self {
        Self {
            input_per_token: input_per_million / 1_000_000f32,
            output_per_token: output_per_million / 1_000_000f32,
            cache_5m_write_per_token: (input_per_million * 1.25) / 1_000_000f32,
            cache_1h_write_per_token: (input_per_million * 2.0) / 1_000_000f32,
            cache_read_per_token: (output_per_million / 50.0) / 1_000_000f32,
        }
    }
}
