#![allow(dead_code)]

use super::{
    model::{NewPet, Pet},
    new_conn_pool,
    schema::pets,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DB {
    // r2d2 线程池。 通过同时管理多个链接，解决了并发访问数据库的问题。
    // 但是 r2d2 是个同步的线程池 —— 当新建链接，且正好网络拥塞响应慢时，
    // 建立链接的时间会使客户端线程阻塞住。
    // 如果 r2d2 可以管理异步链接的话就可以解决这个问题， 但遗憾的是作者并
    // 不打算加入异步支持。
    // 或许需要用 bb8 来添加异步支持。
    conns: Pool<ConnectionManager<PgConnection>>,
}
impl DB {
    pub fn new() -> Self {
        Self {
            conns: new_conn_pool(),
        }
    }

    pub fn insert_pet(&self, p: NewPet) -> Result<Pet> {
        let pet = diesel::insert_into(pets::table)
            .values(vec![p])
            .get_result(&mut self.conns.get()?)?;
        Ok(pet)
    }

    pub fn insert_pets(&self, p: Vec<NewPet>) -> Result<Vec<Pet>> {
        let pets = diesel::insert_into(pets::table)
            .values(p)
            .get_results(&mut self.conns.get()?)?;
        Ok(pets)
    }

    pub fn take_pet(&self, id: i64) -> Result<Pet> {
        let pet = pets::table.find(id).first(&mut self.conns.get()?)?;
        Ok(pet)
    }

    pub fn list_pets(&self) -> Result<Vec<Pet>> {
        let pets = pets::table.load(&mut self.conns.get()?)?;
        Ok(pets)
    }

    pub fn delete_pet(&self, id: i64) -> Result<()> {
        diesel::delete(pets::dsl::pets.find(id)).execute(&mut self.conns.get()?)?;
        Ok(())
    }
}
