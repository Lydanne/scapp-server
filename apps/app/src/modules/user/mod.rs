use nidrs::macros::module;

pub mod controller;
pub mod dto;
pub mod service;

use crate::models::doo::users::UserEntity;
use controller::UserController;
use service::UserService;

#[module({
  imports: [],
  controllers: [UserController],
  services: [UserService, UserEntity],
  exports: [UserService],
})]
pub struct UserModule;
