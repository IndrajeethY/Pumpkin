use pumpkin_data::item_stack::ItemStack;
use pumpkin_macros::{Event, cancellable};

/// An event that occurs when a villager acquires a new trade.
#[cancellable]
#[derive(Event, Clone)]
pub struct VillagerAcquireTradeEvent {
    /// The ID of the villager entity.
    pub entity_id: i32,

    /// The trade recipe index.
    pub recipe_index: i32,

    /// The first cost item.
    pub cost_a: ItemStack,

    /// The optional second cost item.
    pub cost_b: Option<ItemStack>,

    /// The output item the villager sells.
    pub output: ItemStack,

    /// Maximum number of times this trade can be used before restocking.
    pub max_uses: i32,

    /// Experience points the villager gains per trade.
    pub xp: i32,

    /// Price multiplier for demand-based pricing.
    pub price_multiplier: f32,
}
