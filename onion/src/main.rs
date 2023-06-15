use anyhow::Result;
use tokio;

use crate::repos::IngredientRepo;
#[cfg_attr(test, mockall_double::double)]
use crate::repos::{RecipeRepo, UserRepo};
use crate::{auth::Auth, services::Services};

#[tokio::main]
async fn main() -> Result<()> {
    let services = Services::new(
        UserRepo::default(),
        RecipeRepo::default(),
        IngredientRepo::default(),
    );

    // fake example request
    let session = Auth::new();

    let recipes = services.food.get_recipes(session).await?;

    println!("{recipes}");

    Ok(())
}

mod services {
    use std::ops::Not;

    use crate::auth::{Auth, Authed};
    use crate::domain::Recipes;

    #[cfg_attr(test, mockall_double::double)]
    use crate::repos::{RecipeRepo, UserRepo};

    use crate::repos::IngredientRepo;
    use anyhow::{anyhow, Result};

    pub struct Services {
        pub food: FoodService,
    }

    impl Services {
        pub fn new(users: UserRepo, recipe: RecipeRepo, ingredient: IngredientRepo) -> Self {
            Self {
                food: FoodService {
                    users,
                    recipe,
                    ingredient,
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
        pub async fn get_recipes(&self, auth: impl Authed) -> Result<Recipes> {
            if auth.is_valid().not() {
                return Err(anyhow!("session is not valid"));
            }

            let id = auth.id();
            let recipe_list = self.users.get(id.to_string()).await?.recipe;
            let recipes = self.recipe.list(recipe_list).await?;

            Ok(Recipes::new(recipes))
        }
    }

    #[cfg(test)]
    mod tests {
        use fake::{Fake, Faker};
        use mockall::predicate::eq;

        use crate::{
            auth::MockAuthed,
            domain::{Recipe, User},
        };

        use super::*;

        #[tokio::test]
        async fn test_food_service_get_recipes() {
            let mut mock_users = UserRepo::default();
            let mut user = Faker.fake::<User>();
            user.recipe = vec![1, 2, 3];

            mock_users
                .expect_get()
                .with(eq("123".to_string()))
                .return_once(move |_| Ok(user.clone()));

            let mut mock_recipes = RecipeRepo::default();
            mock_recipes
                .expect_list()
                .with(eq(vec![1, 2, 3]))
                .return_once(|_| Ok(Faker.fake::<Vec<Recipe>>()));

            let services = Services::new(mock_users, mock_recipes, IngredientRepo::default());

            let mut mock_auth = MockAuthed::default();
            mock_auth.expect_is_valid().return_once(|| true).once();
            mock_auth.expect_id().return_const("123".into());

            let res = services.food.get_recipes(mock_auth).await;
            let Ok(_) = res else {
                panic!("got {res:#?}");
            };
        }
    }
}

mod repos {
    use anyhow::{anyhow, Result};
    use fake::{Fake, Faker};

    use crate::domain::{Recipe, User};

    #[derive(Default)]
    pub struct UserRepo {}

    #[cfg_attr(test, mockall::automock)]
    impl UserRepo {
        pub async fn get(&self, id: String) -> Result<User> {
            Faker
                .fake::<Result<User, ()>>()
                .map_err(|_| anyhow!("oh user"))
        }
    }

    #[derive(Default)]
    pub struct RecipeRepo {}

    #[cfg_attr(test, mockall::automock)]
    impl RecipeRepo {
        pub async fn list(&self, ids: Vec<i32>) -> Result<Vec<Recipe>> {
            Faker
                .fake::<Result<Vec<Recipe>, ()>>()
                .map_err(|_| anyhow!("oh recipe"))
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

    impl Auth {
        pub fn new() -> Self {
            Faker.fake()
        }
    }

    #[cfg_attr(test, mockall::automock)]
    pub trait Authed {
        fn is_valid(&self) -> bool;
        fn id(&self) -> &str;
    }

    impl Authed for Auth {
        fn is_valid(&self) -> bool {
            println!("jwt: {}", self.session.jwt);
            self.session.valid
        }

        fn id(&self) -> &str {
            &self.user_id
        }
    }

    #[derive(Dummy)]
    struct Session {
        jwt: String,
        #[dummy(faker = "Boolean(80)")]
        valid: bool,
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

    #[derive(Debug)]
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
