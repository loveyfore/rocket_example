use ::entity::{post, post::Entity as Post};
use sea_orm::*;
use crate::daos::post_dao::PostDao;


pub struct PostService;

impl PostService {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        PostDao::find_post_by_id(db, id).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        PostDao::find_posts_in_page(db, page, posts_per_page).await
    }

    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        PostDao::create_post(db, form_data).await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        PostDao::update_post_by_id(db, id, form_data).await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        PostDao::delete_post(db, id).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        PostDao::delete_all_posts(db).await
    }
}

