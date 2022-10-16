use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

use crate::models::*;
use crate::schema::*;
use crate::StdErr;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct KanbanDb {
    pool: PgPool,
}

impl KanbanDb {
    pub fn connect() -> Result<Self, StdErr> {
        let db_url = env::var("DATABASE_URL")?;
        let manager = ConnectionManager::new(db_url);
        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }
    // token methods
    pub fn validate_token(&self, token_id: String) -> Result<Token, StdErr> {
        let ref mut conn = self.pool.get()?;
        let token = tokens::table
            .filter(tokens::id.eq(token_id))
            .filter(tokens::expired_at.ge(diesel::dsl::now))
            .first(conn)?;
        Ok(token)
    }

    pub fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let ref mut conn = self.pool.get()?;
        let all_boards = boards::table.load(conn)?;
        Ok(all_boards)
    }

    pub fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let ref mut conn = self.pool.get()?;
        let counts = diesel::sql_query(format!(
                    "select count(*), status from cards where cards.board_id = {} group by status",
                    board_id
                ))
                .load(conn)?;
        Ok(counts.into())
    }

    pub fn create_board(&self, create_board: CreateBoardDTO) -> Result<Board, StdErr> {
        let ref mut conn = self.pool.get()?;
        let board = diesel::insert_into(boards::table)
                    .values(&create_board)
                    .get_result(conn)?;
        Ok(board)
    }

    pub fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        let ref mut conn = self.pool.get()?;
        diesel::delete(boards::table.filter(boards::id.eq(board_id)))
                .execute(conn)?;
        Ok(())
    }

    pub fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let ref mut conn = self.pool.get()?;
        let cards = cards::table
                    .filter(cards::board_id.eq(board_id))
                    .load(conn)?;
        Ok(cards)
    }

    pub fn create_card(&self, create_card: CreateCardDTO) -> Result<Card, StdErr> {
        let ref mut conn = self.pool.get()?;
        let card = diesel::insert_into(cards::table)
                    .values(create_card)
                    .get_result(conn)?;
        Ok(card)
    }

    pub fn update_card(
        &self,
        card_id: i64,
        update_card: UpdateCardDTO,
    ) -> Result<Card, StdErr> {
        let ref mut conn = self.pool.get()?;
        let card = diesel::update(cards::table.filter(cards::id.eq(card_id)))
                    .set(update_card)
                    .get_result(conn)?;
        Ok(card)
    }

    pub fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        let ref mut conn = self.pool.get()?;
        diesel::delete(cards::table.filter(cards::id.eq(card_id)))
                .execute(conn)?;
        Ok(())
    }
}
