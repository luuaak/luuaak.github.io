#![allow(non_snake_case)]
use dioxus::prelude::*;

/// `SharedAuthVisibility` คือ struct ที่ใช้สำหรับจัดการสถานะการแสดง AuthCard
/// มี field เดียวคือ `show_auth_card` ซึ่งเป็น `Signal<bool>`
/// ใช้เก็บค่าที่บอกว่า AuthCard ควรจะแสดงหรือไม่
#[derive(Clone, PartialEq)]
pub struct SharedAuthVisibility {
    pub show_auth_card: Signal<bool>,
}

/// # ลำดับการทำงานของคอมโพเนนต์
/// **HomePage**: `home_page.rs`
///
/// - สร้าง instance ของ `SharedAuthVisibility`
/// - ส่ง `state_auth` ไปยังคอมโพเนนต์ **Banner**
///
/// **Banner**: `navigation_bar`
///
/// - รับ `state_auth` ที่ส่งมาจาก **HomePage**
/// - เมื่อคลิกที่ปุ่ม **Login** จะเปลี่ยนสถานะ `show_auth_card` โดยการเขียนค่าตรงข้ามจากสถานะปัจจุบันใน `state_auth`
///
/// **AuthCard**: `auth_card.rs`
///
/// - รับ `state_auth` ที่ส่งมาจาก **Banner**
/// - มีฟังก์ชันจัดการ การคลิกที่ overlay และปุ่ม **cross** เพื่อปิด **AuthCard** โดยการตั้งค่า `show_auth_card` เป็น false
/// - มีฟังก์ชันสำหรับจัดการการคลิกที่ปุ่ม **Sign in with extension** ซึ่งจะใช้ nip07 เพื่อรับ public key และปิด **AuthCard** เมื่อสำเร็จ

impl SharedAuthVisibility {

    /// เริ่มต้นค่า `show_auth_card` เป็น false
    pub fn new() -> Self {
        let show_auth_card = use_signal(|| false);
        Self { show_auth_card }
    }

}
