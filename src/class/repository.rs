use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, Table};

use crate::{db::DB_MANAGER, errors::ServiceError, schema::classes_students};

use super::model::{Class, NewClass, UpdateClass};
use crate::diesel::OptionalExtension;
use crate::diesel::QueryDsl;
use crate::schema::classes::dsl::*;

pub fn create_class(new_class: NewClass) -> Result<Class, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class: Class = diesel::insert_into(classes)
        .values(&new_class)
        .returning(classes::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn get_class_by_id(class_id: i32) -> Result<Option<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = classes
        .filter(id.eq(class_id))
        .select(Class::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn delete_class_by_id(class_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    diesel::delete(classes.filter(id.eq(class_id)))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn update_class(class_id: i32, update_class: UpdateClass) -> Result<Class, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = diesel::update(classes.filter(id.eq(class_id)))
        .set(&update_class)
        .returning(classes::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn get_class_by_code(class_code: &str) -> Result<Option<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = classes
        .filter(code.eq(class_code))
        .select(Class::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn enroll_student(cid: i32, sid: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    diesel::insert_into(classes_students::table)
        .values((
            classes_students::class_id.eq(cid),
            classes_students::student_id.eq(sid),
        ))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn is_student_enrolled(cid: i32, sid: i32) -> Result<bool, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let is_enrolled = classes_students::table
        .filter(classes_students::class_id.eq(cid))
        .filter(classes_students::student_id.eq(sid))
        .select(classes_students::columns::student_id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(is_enrolled.is_some())
}

pub fn list_classes_that_student_is_enrolled(sid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let ccs: Vec<Class> = classes
        .inner_join(classes_students::table)
        .filter(classes_students::student_id.eq(sid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(ccs)
}

pub fn list_classes_that_student_is_not_enrolled(sid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let ccs: Vec<Class> = classes
        .left_join(classes_students::table)
        .filter(classes_students::student_id.is_null())
        .or_filter(classes_students::student_id.ne(sid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(ccs)
}

pub fn list_classes_by_teacher(tid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let ccs: Vec<Class> = classes
        .filter(user_id.eq(tid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(ccs)
}
