use anyhow::Result;
use tokio;

use crate::services::Services;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let services = Services::new();

    let recipes = services.food.get_recipes().await?;

    println!("{recipes:#?}");

    Ok(())
}

mod services {
    use anyhow::Result;

    use crate::domain::{Recipe, Recipes};

    pub struct Services {
        pub food: FoodService,
    }

    impl Services {
        pub fn new() -> Self {
            Self {
                food: FoodService {},
            }
        }
    }

    pub struct FoodService {}
    impl FoodService {
        pub async fn get_recipes(&self) -> Result<Recipes> {
            Ok(fake::vec![Recipe; 2..4])
        }
    }
}

mod domain {
    use fake::faker::lorem::en::*;
    use fake::Dummy;
    use fake::Faker;

    pub type Recipes = Vec<Recipe>;

    #[derive(Debug, Clone, Dummy)]
    pub struct Recipe {
        pub prep_time: Time,
        pub cook_time: Time,
        #[dummy(faker = "(Faker, 1..3)")]
        pub ingredients: Vec<Ingredient>,
        #[dummy(faker = "Paragraph(1..3)")]
        pub method: String,
    }

    #[derive(Debug, Clone, Dummy)]
    pub struct Time {
        #[dummy(faker = "0..4")]
        pub hrs: i32,
        #[dummy(faker = "0..60")]
        pub mins: i32,
    }

    #[derive(Debug, Clone, Dummy)]
    pub enum Ingredient {
        /// some whole food like apples, chicken, fruit
        Whole(
            #[dummy(faker = "Word()")] String,
            #[dummy(faker = "1..3")] i32,
        ),
        /// some proprtioned food like flour or milk
        Weight(String, i32),
        // #[dummy(faker = "Word")]
        Shop(String),
    }
}
