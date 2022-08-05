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

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }
}


fn update_quality_conjured(item: &mut Item) {
    item.sell_in -= 1;
    if item.sell_in < 0 {
        item.quality -= 4;
    } else {
        item.quality -= 2;
    }
}

#[cfg(test)]
mod tests {
    use crate::gildedrose::update_quality_conjured;
    use super::{GildedRose, Item};

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
    fn test_conjured_item() {
        // if before sell_in, quality degrades
        // create single item
        let mut item = Item::new("Conjured pants", 5, 40);
        let mut qualities = Vec::new();
        let mut sell_in_days = Vec::new();
        for _i in 0..10 {
            update_quality_conjured(&mut item);
            qualities.push(item.quality);
            sell_in_days.push(item.sell_in);
        }
        // TODO: test that quality doesn't go below 0
        // TODO: split into 3 tests
        assert_eq!(qualities, vec![38, 36, 34, 32, 30, 26, 22, 18, 14, 10]);
        assert_eq!(sell_in_days, vec![4, 3, 2, 1, 0, -1, -2, -3, -4, -5]);
    }
}
