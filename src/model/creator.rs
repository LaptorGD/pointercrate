use crate::{
    model::player::Player,
    schema::{creators, demons, players},
};
use diesel::{expression::bound::Bound, sql_types, ExpressionMethods, QueryDsl, Queryable};
use std::fmt::{Display, Formatter};

mod delete;
mod get;
mod post;

pub use self::post::PostCreator;

pub(crate) struct Creators(pub(crate) Vec<Player>);

#[derive(Debug, Queryable, Hash)]
pub struct Creator {
    demon: String,
    creator: i32,
}

impl Display for Creator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "creator with id {} on demon {}",
            self.creator, self.demon
        )
    }
}

type ByDemon<'a> = diesel::dsl::Eq<creators::demon, Bound<sql_types::Text, &'a str>>;
type WithDemon<'a> = diesel::dsl::Filter<
    diesel::dsl::Select<
        diesel::dsl::InnerJoin<creators::table, players::table>,
        (players::id, players::name, players::banned),
    >,
    ByDemon<'a>,
>;

type ByPlayer = diesel::dsl::Eq<creators::creator, Bound<sql_types::Int4, i32>>;
type WithPlayer = diesel::dsl::Filter<
    diesel::dsl::Select<
        diesel::dsl::InnerJoin<creators::table, demons::table>,
        (demons::position, demons::name),
    >,
    ByPlayer,
>;

pub fn created_by(player: i32) -> WithPlayer {
    creators::table
        .inner_join(demons::table)
        .select((demons::position, demons::name))
        .filter(creators::creator.eq(player))
}

pub fn creators_of(demon: &str) -> WithDemon {
    creators::table
        .inner_join(players::table)
        .select((players::id, players::name, players::banned))
        .filter(creators::demon.eq(demon))
}
