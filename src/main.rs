#[derive(Copy, Clone)]
struct Item {
    name: &'static str,
    weight: f32,
    value: f32,
    value_ratio: f32,
}

fn main() {
    println!("knapsacking problem");

    const KNAPSACK_MAX_WEIGHT: u32 = 400;

    let items = vec![
        Item {
            name: "map",
            weight: 9f32,
            value: 150f32,
            value_ratio: 0.0,
        },
        Item {
            name: "compass",
            weight: 13f32,
            value: 35f32,
            value_ratio: 0.0,
        },
        Item {
            name: "water",
            weight: 153f32,
            value: 200f32,
            value_ratio: 0.0,
        },
        Item {
            name: "sandwich",
            weight: 50f32,
            value: 160f32,
            value_ratio: 0.0,
        },
        Item {
            name: "glucose",
            weight: 15f32,
            value: 60f32,
            value_ratio: 0.0,
        },
        Item {
            name: "tin",
            weight: 68f32,
            value: 45f32,
            value_ratio: 0.0,
        },
        Item {
            name: "banana",
            weight: 27f32,
            value: 60f32,
            value_ratio: 0.0,
        },
        Item {
            name: "apple",
            weight: 39f32,
            value: 40f32,
            value_ratio: 0.0,
        },
        Item {
            name: "cheese",
            weight: 23f32,
            value: 30f32,
            value_ratio: 0.0,
        },
        Item {
            name: "beer",
            weight: 52f32,
            value: 10f32,
            value_ratio: 0.0,
        },
        Item {
            name: "suntancream",
            weight: 11f32,
            value: 70f32,
            value_ratio: 0.0,
        },
        Item {
            name: "camera",
            weight: 32f32,
            value: 30f32,
            value_ratio: 0.0,
        },
        Item {
            name: "T-shirt",
            weight: 24f32,
            value: 15f32,
            value_ratio: 0.0,
        },
        Item {
            name: "trousers",
            weight: 48f32,
            value: 10f32,
            value_ratio: 0.0,
        },
        Item {
            name: "umbrella",
            weight: 73f32,
            value: 40f32,
            value_ratio: 0.0,
        },
        Item {
            name: "waterproof trousers",
            weight: 42f32,
            value: 70f32,
            value_ratio: 0.0,
        },
        Item {
            name: "waterproof overclothes",
            weight: 43f32,
            value: 75f32,
            value_ratio: 0.0,
        },
        Item {
            name: "note-case",
            weight: 22f32,
            value: 80f32,
            value_ratio: 0.0,
        },
        Item {
            name: "sunglasses",
            weight: 7f32,
            value: 20f32,
            value_ratio: 0.0,
        },
        Item {
            name: "towel",
            weight: 18f32,
            value: 12f32,
            value_ratio: 0.0,
        },
        Item {
            name: "socks",
            weight: 4f32,
            value: 50f32,
            value_ratio: 0.0,
        },
        Item {
            name: "book",
            weight: 30f32,
            value: 10f32,
            value_ratio: 0.0,
        },
    ];

    println!("----knapsack items-----");
    print_items(&items);
    greedy_solution(items.clone(), KNAPSACK_MAX_WEIGHT)
}

fn greedy_solution(items: Vec<Item>, knapsack_max_weight: u32) {
    println!("----Greedy Solution knapsack-----");
    let mut greedy_items = items.to_vec();

    for i in 0..greedy_items.len() {
        for k in 0..greedy_items.len() {
            if greedy_items[i].value_ratio > greedy_items[k].value_ratio {
                greedy_items.swap(i, k);
            }
        }
    }

    println!("----sorted items-----");
    sort_items(&mut greedy_items);

    let mut current_weight = 0.0;
    let mut total_value = 0.0;

    println!("----Greedy Backpack-----");
    for item in greedy_items {
        if (current_weight + item.weight) < knapsack_max_weight as f32 {
            current_weight += item.weight;
            total_value += item.value;
            println!("Item: {}", item.name);
        }
    }

    println!("------------------------");
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", current_weight);
}

fn print_items(items: &[Item]) {
    for item in items {
        println!("Item: {}", item.name);
    }
}

fn sort_items(items: &mut [Item]) {
    for i in items {
        i.value_ratio = i.value / i.weight;
    }
}
