mod errors;
pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use models::{Board, BoardSummary, Card, CreateBoardDTO, CreateCardDTO, Token, UpdateCardDTO};
use schema::{boards, cards, tokens};
use std::env;

pub type DbErr = errors::Error;
pub type DbResult<T> = errors::Result<T>;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type SinglePgConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DB {
    pool: PgPool,
}

impl DB {
    pub fn connect() -> DbResult<Self> {
        let db_url = env::var("DATABASE_URL")?;
        let manager = ConnectionManager::new(db_url);
        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }

    pub fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut SinglePgConnection) -> R,
    {
        let ref mut conn = self.pool.get().unwrap();
        f(conn)
    }
    // token methods
    pub fn validate_token(&self, token_id: &str) -> DbResult<Token> {
        let token = self.run(|conn| {
            tokens::table
                .filter(tokens::id.eq(token_id))
                .filter(tokens::expired_at.ge(diesel::dsl::now))
                .first(conn)
        })?;
        return Ok(token);
    }

    pub fn boards(&self) -> DbResult<Vec<Board>> {
        let all_boards = self.run(|conn| boards::table.load(conn))?;
        Ok(all_boards)
    }

    pub fn board_summary(&self, board_id: i64) -> DbResult<BoardSummary> {
        let counts = self.run(|conn| {
            diesel::sql_query(format!(
                "select count(*), status from cards where cards.board_id = {} group by status",
                board_id
            ))
            .load(conn)
        })?;
        Ok(counts.into())
    }

    pub fn create_board(&self, create_board: CreateBoardDTO) -> DbResult<Board> {
        let board = self.run(|conn| {
            diesel::insert_into(boards::table)
                .values(&create_board)
                .get_result(conn)
        })?;
        Ok(board)
    }

    pub fn delete_board(&self, board_id: i64) -> DbResult<()> {
        self.run(|conn| {
            diesel::delete(boards::table.filter(boards::id.eq(board_id))).execute(conn)
        })?;
        Ok(())
    }

    pub fn cards(&self, board_id: i64) -> DbResult<Vec<Card>> {
        let cards =
            self.run(|conn| cards::table.filter(cards::board_id.eq(board_id)).load(conn))?;
        Ok(cards)
    }

    pub fn create_card(&self, create_card: CreateCardDTO) -> DbResult<Card> {
        let card = self.run(|conn| {
            diesel::insert_into(cards::table)
                .values(create_card)
                .get_result(conn)
        })?;
        Ok(card)
    }

    pub fn update_card(&self, card_id: i64, update_card: UpdateCardDTO) -> DbResult<Card> {
        let card = self.run(|conn| {
            diesel::update(cards::table.filter(cards::id.eq(card_id)))
                .set(update_card)
                .get_result(conn)
        })?;
        Ok(card)
    }

    pub fn delete_card(&self, card_id: i64) -> DbResult<()> {
        self.run(|conn| diesel::delete(cards::table.filter(cards::id.eq(card_id))).execute(conn))?;
        Ok(())
    }
}
