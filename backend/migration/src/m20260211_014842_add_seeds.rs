use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // We use a raw string literal (r#" ... "#) to keep the SQL exactly as provided
        let sql = r#"
            -- USERS
            INSERT INTO users (email, username, first_name, last_name, password_hash, avatar_url, email_verified)
            VALUES
                ('user@example.com', 'user1', 'User', 'One', '$argon2$ver=2,mem=12,norm=nfkc,pmin=8,pmax=128,lanes=4,len-calc=chars,passes=3,len=128,xhmac=none$+xbxduPocxYtD71CNvNrsQ$qt6nvaN65PMqtVYAatAwEZYrEpkabKkHwISf9Yd91dsn8FKmTNlQEvzwKl314F64ikSlqZzbqBjxYLFDXbFItX5SKgddl7S3vIlDQv6gBQQVCl1tC1SWN4yzqDMKMq1o27Y8Pl5ZGR9Ak2BAH45VWeqF+UAmV8ytoPeara/ATGw','/assets/users/default.png', true),
                ('admin@example.com', 'admin1', 'Admin', 'One', '$argon2$ver=2,mem=12,norm=nfkc,pmin=8,pmax=128,lanes=4,len-calc=chars,passes=3,len=128,xhmac=none$+xbxduPocxYtD71CNvNrsQ$qt6nvaN65PMqtVYAatAwEZYrEpkabKkHwISf9Yd91dsn8FKmTNlQEvzwKl314F64ikSlqZzbqBjxYLFDXbFItX5SKgddl7S3vIlDQv6gBQQVCl1tC1SWN4yzqDMKMq1o27Y8Pl5ZGR9Ak2BAH45VWeqF+UAmV8ytoPeara/ATGw','/assets/users/default.png', true),
                ('mod@example.com', 'mod1', 'Mod', 'One', '$argon2$ver=2,mem=12,norm=nfkc,pmin=8,pmax=128,lanes=4,len-calc=chars,passes=3,len=128,xhmac=none$+xbxduPocxYtD71CNvNrsQ$qt6nvaN65PMqtVYAatAwEZYrEpkabKkHwISf9Yd91dsn8FKmTNlQEvzwKl314F64ikSlqZzbqBjxYLFDXbFItX5SKgddl7S3vIlDQv6gBQQVCl1tC1SWN4yzqDMKMq1o27Y8Pl5ZGR9Ak2BAH45VWeqF+UAmV8ytoPeara/ATGw','/assets/users/default.png', true);

            -- USER ROLES
            INSERT INTO user_roles (user_id, role_id)
            SELECT u.id, r.id
            FROM users u
            JOIN roles r ON (u.username='user1' AND r.name='USER')
                        OR (u.username='admin1' AND r.name='ADMIN')
                        OR (u.username='mod1' AND r.name='MODERATOR');

            -- TAGS
            INSERT INTO tags (name) VALUES ('Breakfast'),('Lunch'),('Dinner'),('Dessert'),('Vegetarian');

            -- RECIPES
            INSERT INTO recipes (title, description, image_url, servings, prep_time_minutes, cook_time_minutes, author, author_id, is_private)
            VALUES
                ('Pancakes','Fluffy breakfast pancakes','','4',10,15,'user1',(SELECT id FROM users WHERE username='user1'),false),
                ('Omelette','Quick egg omelette','','2',5,10,'user1',(SELECT id FROM users WHERE username='user1'),false),
                ('Spaghetti','Classic spaghetti','','4',15,20,'admin1',(SELECT id FROM users WHERE username='admin1'),false),
                ('Caesar Salad','Fresh salad with dressing','','2',10,0,'mod1',(SELECT id FROM users WHERE username='mod1'),false),
                ('Chocolate Cake','Rich chocolate cake','','8',20,35,'user1',(SELECT id FROM users WHERE username='user1'),false),
                ('Grilled Cheese','Cheesy sandwich','','1',5,5,'user1',(SELECT id FROM users WHERE username='user1'),false),
                ('Tomato Soup','Creamy tomato soup','','3',10,15,'admin1',(SELECT id FROM users WHERE username='admin1'),false),
                ('Chicken Curry','Spicy chicken curry','','4',20,40,'admin1',(SELECT id FROM users WHERE username='admin1'),false),
                ('Veggie Stir Fry','Quick veggie stir fry','','2',10,10,'mod1',(SELECT id FROM users WHERE username='mod1'),false),
                ('Fruit Smoothie','Refreshing fruit smoothie','','1',5,0,'user1',(SELECT id FROM users WHERE username='user1'),false);

            -- INGREDIENTS
            INSERT INTO ingredients (name) VALUES ('Flour'),('Eggs'),('Milk'),('Cheese'),('Tomato'),('Lettuce'),('Chicken'),('Chocolate'),('Pasta'),('Vegetables');

            -- INGREDIENT GROUPS
            INSERT INTO ingredient_groups (recipe_id, title, position)
            SELECT r.id, 'Main Ingredients', 1 FROM recipes r;

            -- RECIPE INGREDIENTS
            INSERT INTO recipe_ingredients (ingredient_group_id, ingredient_id, quantity, unit, position)
            SELECT ig.id, i.id, 1, 'gram', 1
            FROM ingredient_groups ig
            JOIN ingredients i ON i.name='Flour'
            WHERE ig.title='Main Ingredients';

            -- STEP GROUPS
            INSERT INTO step_groups (recipe_id, title, position)
            SELECT r.id, 'Preparation', 1 FROM recipes r;

            -- STEPS
            INSERT INTO steps (step_group_id, position, instruction, duration_minutes)
            SELECT sg.id, 1, 'Mix ingredients together', 5
            FROM step_groups sg;

            -- RECIPE TAGS
            INSERT INTO recipe_tags (recipe_id, tag_id)
            SELECT r.id, t.id
            FROM recipes r
            JOIN tags t ON t.name='Breakfast'
            WHERE r.title IN ('Pancakes','Omelette');

            -- FAVORITES
            INSERT INTO favorites (user_id, recipe_id)
            SELECT u.id, r.id
            FROM users u
            JOIN recipes r ON r.title='Pancakes'
            WHERE u.username='user1';

            -- RATINGS
            INSERT INTO recipe_ratings (user_id, recipe_id, rating)
            SELECT u.id, r.id, 5
            FROM users u
            JOIN recipes r ON r.title='Pancakes'
            WHERE u.username='user1';

            -- COMMENTS
            INSERT INTO recipe_comments (recipe_id, user_id, content)
            SELECT r.id, u.id, 'Great recipe!'
            FROM users u
            JOIN recipes r ON r.title='Pancakes'
            WHERE u.username='user1';
        "#;

        manager.get_connection().execute_unprepared(sql).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Optional: Clean up seeded data.
        // Warning: This deletes data based on emails/usernames!
        manager.get_connection().execute_unprepared(
            "DELETE FROM users WHERE username IN ('user1', 'admin1', 'mod1');"
        ).await?;
        Ok(())
    }
}
