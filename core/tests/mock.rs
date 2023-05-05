mod prepare;

use entity::post;
use prepare::prepare_mock_db;
use rocket_example_core::{PostDao};

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let post = PostDao::find_post_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(post.id, 1);
    }

    {
        let post = PostDao::find_post_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(post.id, 5);
    }

    {
        let post = PostDao::create_post(
            db,
            post::Model {
                id: 0,
                title: "Title D".to_owned(),
                text: "Text D".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            post,
            post::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(6),
                title: sea_orm::ActiveValue::Unchanged("Title D".to_owned()),
                text: sea_orm::ActiveValue::Unchanged("Text D".to_owned())
            }
        );
    }

    {
        let post = PostDao::update_post_by_id(
            db,
            1,
            post::Model {
                id: 1,
                title: "New Title A".to_owned(),
                text: "New Text A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            post,
            post::Model {
                id: 1,
                title: "New Title A".to_owned(),
                text: "New Text A".to_owned(),
            }
        );
    }

    {
        let result = PostDao::delete_post(db, 5).await.unwrap();

        assert_eq!(result.rows_affected, 1);
    }

    {
        let result = PostDao::delete_all_posts(db).await.unwrap();

        assert_eq!(result.rows_affected, 5);
    }
}
