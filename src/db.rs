use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

use crate::models::*;
use crate::schema::*;
use crate::StdErr;

#[database("kanban")]
pub struct KanbanDb(diesel::PgConnection);

impl KanbanDb {
    // token methods
    pub async fn validate_token(&self, token_id: String) -> Result<Token, StdErr> {
        let token = self
            .run(move |conn| {
                tokens::table
                    .filter(tokens::id.eq(token_id))
                    .filter(tokens::expired_at.ge(diesel::dsl::now))
                    .first(conn)
                    .unwrap()
            })
            .await;
        Ok(token)
    }

    pub async fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let all_boards = self.run(|conn| boards::table.load(conn).unwrap()).await;
        Ok(all_boards)
    }

    pub async fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let counts = self
            .run(move |conn| {
                diesel::sql_query(format!(
                    "select count(*), status from cards where cards.board_id = {} group by status",
                    board_id
                ))
                .load(conn)
                .unwrap()
            })
            .await;
        Ok(counts.into())
    }

    pub async fn create_board(&self, create_board: CreateBoardDTO) -> Result<Board, StdErr> {
        let board = self
            .run(move |conn| {
                diesel::insert_into(boards::table)
                    .values(&create_board)
                    .get_result(conn)
                    .unwrap()
            })
            .await;
        Ok(board)
    }

    pub async fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        self.run(move |conn| {
            diesel::delete(boards::table.filter(boards::id.eq(board_id)))
                .execute(conn)
                .unwrap()
        })
        .await;
        Ok(())
    }

    pub async fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let cards = self
            .run(move |conn| {
                cards::table
                    .filter(cards::board_id.eq(board_id))
                    .load(conn)
                    .unwrap()
            })
            .await;
        Ok(cards)
    }

    pub async fn create_card(&self, create_card: CreateCardDTO) -> Result<Card, StdErr> {
        let card = self
            .run(move |conn| {
                diesel::insert_into(cards::table)
                    .values(create_card)
                    .get_result(conn)
                    .unwrap()
            })
            .await;
        Ok(card)
    }

    pub async fn update_card(
        &self,
        card_id: i64,
        update_card: UpdateCardDTO,
    ) -> Result<Card, StdErr> {
        let card = self
            .run(move |conn| {
                diesel::update(cards::table.filter(cards::id.eq(card_id)))
                    .set(update_card)
                    .get_result(conn)
                    .unwrap()
            })
            .await;
        Ok(card)
    }

    pub async fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        self.run(move |conn| {
            diesel::delete(cards::table.filter(cards::id.eq(card_id)))
                .execute(conn)
                .unwrap()
        })
        .await;
        Ok(())
    }
}
