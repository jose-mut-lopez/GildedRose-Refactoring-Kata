use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Box<dyn Perishable>>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        let mut perishables = Vec::new();
        for item in items {
            let perishable = build_perishable(item);
            perishables.push(Box::new(perishable));
        }
        GildedRose { items: perishables }
    }

    pub fn update_quality(&mut self) {
        for perishable in &mut self.items {
            perishable.update();
        }
    }
}

enum ItemType {
    Normal,
    Conjured,
    // etc
}
fn build_perishable(item: Item) -> Box<dyn Perishable> {
    Box::new(NormalItem{item})
}
trait Perishable {
    fn update(&mut self);
    fn get_type(&self) -> ItemType;
}

struct NormalItem {
    item: Item,
}

impl Perishable for NormalItem {
    fn update(&mut self) {
        todo!()
    }

    fn get_type(&self) -> ItemType {
        ItemType::Normal
    }
}

const CONJURED_DEGRADING_RATE: i32 = 2;
const EXPIRED_DEGRADING_RATE: i32 = 2;
const DEGRADING_RATE: i32 = 1;
const MINIMUM_QUALITY: i32 = 0;

fn update_conjured(item: &mut Item) {
    item.sell_in -= 1;
    if is_expired(item) {
        item.quality -= CONJURED_DEGRADING_RATE * EXPIRED_DEGRADING_RATE;
    } else {
        item.quality -= CONJURED_DEGRADING_RATE * DEGRADING_RATE;
    }
    item.quality = i32::max(item.quality, MINIMUM_QUALITY);
}

fn is_expired(item: &mut Item) -> bool {
    item.sell_in < 0
}

#[cfg(test)]
mod tests {
    use crate::gildedrose::update_conjured;
    use super::{GildedRose, Item};
    use super::*;

    #[test]
    fn test_build_perishable() {
        let p = build_perishable(Item::new("normal item", 0, 0));
        assert_eq!(p.get_type(), ItemType::Normal);
    }

    #[test]
    fn test_basic_item_several_days() {
        let items = vec![Item::new("basic item", 5, 10)];
        let mut rose = GildedRose::new(items);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..10 {
            rose.update_quality();
            let item = &rose.items[0];
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        assert_eq!(qualities, vec![9, 8, 7, 6, 5, 3, 1, 0, 0, 0]);
        assert_eq!(sell_in_days, vec![4, 3, 2, 1, 0, -1, -2, -3, -4, -5]);
    }

    #[test]
    fn test_aged_brie_several_days() {
        let items = vec![Item::new("Aged Brie", 5, 10)];
        let mut rose = GildedRose::new(items);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..10 {
            rose.update_quality();
            let item = &rose.items[0];
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        assert_eq!(qualities, vec![11, 12, 13, 14, 15, 17, 19, 21, 23, 25]);
        assert_eq!(sell_in_days, vec![4, 3, 2, 1, 0, -1, -2, -3, -4, -5]);
    }

    #[test]
    fn test_backstage_passes_several_days() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 11, 10)];
        let mut rose = GildedRose::new(items);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..14 {
            rose.update_quality();
            let item = &rose.items[0];
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        assert_eq!(qualities, vec![11, 13, 15, 17, 19, 21, 24, 27, 30, 33, 36, 0, 0, 0]);
        assert_eq!(sell_in_days, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3]);
    }

    #[test]
    fn test_sulfuras_several_days() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 5, 10)];
        let mut rose = GildedRose::new(items);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..10 {
            rose.update_quality();
            let item = &rose.items[0];
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        assert_eq!(qualities, vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
        assert_eq!(sell_in_days, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_conjured_decrease_by_2_then_4_when_expired() {
        let mut item = Item::new("Conjured pants", 5, 40);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..10 {
            update_conjured(&mut item);
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        // TODO: split in 2 loops and 2 sets of assertions
        assert_eq!(qualities, vec![38, 36, 34, 32, 30, 26, 22, 18, 14, 10]);
        assert_eq!(sell_in_days, vec![4, 3, 2, 1, 0, -1, -2, -3, -4, -5]);
    }

    #[test]
    fn test_conjured_value_is_never_negative() {
        let mut item = Item::new("Conjured pants", 3, 3);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..4 {
            update_conjured(&mut item);
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        assert_eq!(qualities, vec![1, 0, 0, 0]);
        assert_eq!(sell_in_days, vec![2, 1, 0, -1]);
    }
}
