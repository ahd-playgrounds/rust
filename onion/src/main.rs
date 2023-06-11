use anyhow::Result;
use tokio;

use crate::{auth::Auth, repos::Repos, services::Services};

#[tokio::main]
async fn main() -> Result<()> {
    let repos = Repos::default();
    let services = Services::new(repos);

    // fake example request
    let session = Auth::new();

    let recipes = services.food.get_recipes(session).await?;

    println!("{recipes}");

    Ok(())
}

mod services {

    use std::ops::Not;

    use crate::auth::Auth;
    use crate::domain::Recipes;
    use crate::repos::{IngredientRepo, RecipeRepo, Repos, UserRepo};
    use anyhow::{anyhow, Result};

    pub struct Services {
        pub food: FoodService,
    }

    impl Services {
        pub fn new(repos: Repos) -> Self {
            Self {
                food: FoodService {
                    users: repos.users,
                    recipe: repos.recipe,
                    ingredient: repos.ingredient,
                },
            }
        }
    }

    pub struct FoodService {
        users: UserRepo,
        recipe: RecipeRepo,
        ingredient: IngredientRepo,
    }

    impl FoodService {
        pub async fn get_recipes(&self, auth: Auth) -> Result<Recipes> {
            if auth.is_valid().not() {
                return Err(anyhow!("session is not valid"));
            }

            let recipe_list = self.users.get(auth.id()).await?.recipe;
            let recipes = self.recipe.list(recipe_list).await?;

            Ok(Recipes::new(recipes))
        }
    }
}

mod repos {
    use anyhow::{anyhow, Result};
    use fake::{Fake, Faker};

    use crate::domain::{Recipe, User};

    #[derive(Default)]
    pub struct Repos {
        pub users: UserRepo,
        pub recipe: RecipeRepo,
        pub ingredient: IngredientRepo,
    }

    #[derive(Default)]
    pub struct UserRepo {}

    impl UserRepo {
        pub async fn get(&self, id: impl Into<String>) -> Result<User> {
            Faker
                .fake::<Result<User, ()>>()
                .map_err(|_| anyhow!("oh no"))
        }
    }

    #[derive(Default)]
    pub struct RecipeRepo {}
    impl RecipeRepo {
        pub async fn list(&self, ids: Vec<i32>) -> Result<Vec<Recipe>> {
            Faker
                .fake::<Result<Vec<Recipe>, ()>>()
                .map_err(|_| anyhow!("oh no"))
        }
    }

    #[derive(Default)]
    pub struct IngredientRepo {}
}

mod auth {
    use fake::faker::boolean::en::Boolean;
    use fake::{Dummy, Fake, Faker};

    #[derive(Dummy)]
    pub struct Auth {
        session: Session,
        user_id: String,
    }

    #[derive(Dummy)]
    struct Session {
        jwt: String,
        #[dummy(faker = "Boolean(80)")]
        valid: bool,
    }

    impl Auth {
        pub fn new() -> Self {
            Faker.fake()
        }
        pub fn is_valid(&self) -> bool {
            println!("jwt: {}", self.session.jwt);
            self.session.valid
        }

        pub fn id(&self) -> &str {
            &self.user_id
        }
    }
}

mod domain {
    use std::fmt::Display;

    use fake::faker::lorem::en::*;
    use fake::faker::name::en::*;
    use fake::Dummy;
    use fake::Faker;

    #[derive(Debug, Clone, Dummy)]
    pub struct User {
        pub id: String,
        #[dummy(faker = "FirstName()")]
        pub name: String,
        #[dummy(faker = "(Faker, 0..5)")]
        pub recipe: Vec<i32>,
    }

    pub struct Recipes(Vec<Recipe>);
    impl Recipes {
        pub fn new(r: Vec<Recipe>) -> Self {
            Self(r)
        }
    }

    #[derive(Debug, Clone, Dummy)]
    pub struct Recipe {
        #[dummy(faker = "Word()")]
        pub name: String,
        pub prep_time: Time,
        pub cook_time: Time,
        #[dummy(faker = "(Faker, 1..3)")]
        pub ingredients: Vec<RecipeIngredient>,
        #[dummy(faker = "Paragraph(1..3)")]
        pub method: String,
    }

    #[derive(Debug, Clone, Dummy)]
    pub struct RecipeIngredient {
        pub ingredient: Ingredient,
        pub quantity: Quantity,
    }

    impl Display for RecipeIngredient {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} x {}", self.quantity, self.ingredient.name)
        }
    }

    #[derive(Debug, Clone, Dummy)]
    pub enum Quantity {
        Weight(i32),
        Portion(f64),
        Amount(u8),
    }

    impl Display for Quantity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Quantity::Weight(n) => write!(f, "{n}g"),
                Quantity::Portion(n) => write!(f, "{n}"),
                Quantity::Amount(n) => write!(f, "{n}"),
            }
        }
    }

    impl Recipes {
        pub fn fakes() -> Self {
            Self(fake::vec![Recipe; 2..4])
        }
    }

    impl Display for Recipes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0
                .iter()
                .try_for_each(|recipe| write!(f, "{recipe}\n\n"))
        }
    }

    impl Display for Recipe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Self {
                name,
                prep_time,
                cook_time,
                ingredients,
                method,
            } = &self;
            let ingredients_list = ingredients
                .iter()
                .map(|i| format!("- {}\n", i.to_string()))
                .collect::<String>();
            write!(
                f,
                "Recipe: {name}

    prep time - {prep_time}
    cook time - {cook_time}

ingredients:
{ingredients_list}

method:
    {method}",
            )
        }
    }

    #[derive(Debug, Clone, Dummy)]
    pub struct Time {
        #[dummy(faker = "0..4")]
        pub hrs: i32,
        #[dummy(faker = "0..60")]
        pub mins: i32,
    }

    impl Display for Time {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.hrs, self.mins)
        }
    }

    #[derive(Debug, Clone, Dummy)]
    pub struct Ingredient {
        id: i32,
        #[dummy(faker = "Word()")]
        pub name: String,
    }
}
