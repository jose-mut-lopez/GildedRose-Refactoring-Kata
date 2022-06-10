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
    pub items: Vec<Item>,
}

const SULFURAS: &'static str = "Sulfuras, Hand of Ragnaros";
const BACKSTAGE_PASSES: &'static str = "Backstage passes to a TAFKAL80ETC concert";
const BRIE: &'static str = "Aged Brie";

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            Self::update_quality_single_item(item)
        }
    }

    fn update_quality_single_item(item: &mut Item) {
        if item.name == SULFURAS {
            Self::update_quality_sulfuras(item);
        } else if item.name == BACKSTAGE_PASSES {
            Self::update_quality_backstage_passes(item);
        } else if item.name == BRIE {
            Self::update_quality_brie(item)
        } else {
            Self::update_quality_normal_item(item)
        }
    }

    /// "Sulfuras", being a legendary item, never has to be sold or decreases in Quality
    fn update_quality_sulfuras(_item: &mut Item) {

    }

    fn update_quality_brie(item: &mut Item) {
        item.sell_in -= 1;
        Self::increase_quality(item);
        if item.sell_in < 0 {
            Self::increase_quality(item);
        }
    }

    fn update_quality_backstage_passes(item: &mut Item) {
        Self::increase_quality(item);
        if item.sell_in <= 10 {
            Self::increase_quality(item);
        }
        if item.sell_in <= 5 {
            Self::increase_quality(item);
        }
        if item.sell_in <= 0 {
            item.quality = 0;
        }
        item.sell_in -= 1;
    }

    fn update_quality_normal_item(item: &mut Item) {
        Self::decrease_quality(item);

        item.sell_in = item.sell_in - 1;

        if item.sell_in < 0 {
            Self::decrease_quality(item);
        }
    }

    fn decrease_quality(item: &mut Item) {
        if item.quality > 0 {
            item.quality -= 1;
        }
    }

    fn increase_quality(item: &mut Item) {
        if item.quality < 50 {
            item.quality += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    fn test_quality_degrades_twice_as_fast() {
        let items = vec![Item::new("Elixir of the Mongoose", 1, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(rose.items[0].quality, 9);
        assert_eq!(rose.items[0].sell_in, 0);

        rose.update_quality();

        assert_eq!(rose.items[0].quality, 7);
        assert_eq!(rose.items[0].sell_in, -1);
    }

    #[test]
    fn test_quality_never_negative() {
        let items = vec![Item::new("Elixir of the Mongoose", 1, 10)];
        let mut rose = GildedRose::new(items);
        for _i in 0..100 {
            rose.update_quality();
        }

        assert!(rose.items[0].quality >= 0);
    }

    #[test]
    fn test_brie_increases_quality() {
        let items = vec![Item::new("Aged Brie", 1, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(rose.items[0].quality, 11);
    }

    #[test]
    fn test_brie_does_not_increase_over_50() {
        let items = vec![Item::new("Aged Brie", 1, 10)];
        let mut rose = GildedRose::new(items);
        for _i in 0..120 {
            rose.update_quality();
        }

        assert_eq!(rose.items[0].quality, 50);
    }

    #[test]
    fn test_pass_does_not_increase_over_50() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 45)];
        let mut rose = GildedRose::new(items);
        for _i in 0..8 {
            rose.update_quality();
        }

        assert_eq!(rose.items[0].quality, 50);
    }

    #[test]
    fn test_immutable_sulfuras() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 5, 10)];
        let mut rose = GildedRose::new(items);
        for _i in 0..120 {
            rose.update_quality();
        }

        assert_eq!(rose.items[0].sell_in, 5);
        assert_eq!(rose.items[0].quality, 10);
    }

    #[test]
    fn test_passes_increase_quality() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            6,
            10,
        )];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(rose.items[0].sell_in, 5);
        assert_eq!(rose.items[0].quality, 12);

        rose.update_quality();
        assert_eq!(rose.items[0].sell_in, 4);
        assert_eq!(rose.items[0].quality, 15);

        rose.items[0].sell_in = 0;
        rose.update_quality();
        assert_eq!(rose.items[0].quality, 0);
    }
}
