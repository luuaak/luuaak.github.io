use web_sys::window;

pub struct SessionStorage;

impl SessionStorage {

    // ฟังก์ชันเพื่อดึงคีย์ทั้งหมดใน Session Storage ที่ขึ้นต้นด้วย "story-teller_"
    pub fn get_all_keys() -> Option<Vec<String>> {
        let storage = window().and_then(|win| win.session_storage().ok().flatten())?;
        let mut keys = Vec::new();

        for i in 0..storage.length().unwrap_or(0) {
            if let Some(key) = storage.key(i).ok().flatten() {
                // เพิ่มเงื่อนไขในการตรวจสอบคีย์ที่ขึ้นต้นด้วย "story-teller_"
                if key.starts_with("story-teller_") {
                    keys.push(key);
                }
            }
        }

        Some(keys)
    }

    // สร้างเมธอดสำหรับดึงข้อมูลจาก Session Storage
    pub fn get(key: &str) -> Option<String> {
        if let Some(storage) = window().and_then(|win| win.session_storage().ok().flatten()) {
            storage.get_item(key).ok().flatten()
        } else {
            None
        }
    }

    // สร้างเมธอดสำหรับบันทึกข้อมูลลงใน Session Storage
    pub fn set(key: &str, value: &str) -> Result<(), String> {
        if let Some(storage) = window().and_then(|win| win.session_storage().ok().flatten()) {
            storage.set_item(key, value).map_err(|_| "Failed to set item in sessionStorage".to_string())
        } else {
            Err("Failed to access sessionStorage".to_string())
        }
    }

    // สร้างเมธอดสำหรับลบข้อมูลจาก Session Storage
    pub fn remove(key: &str) -> Result<(), String> {
        if let Some(storage) = window().and_then(|win| win.session_storage().ok().flatten()) {
            storage.remove_item(key).map_err(|_| "Failed to remove item from sessionStorage".to_string())
        } else {
            Err("Failed to access sessionStorage".to_string())
        }
    }


    pub fn has_key_starting_with(prefix: &str) -> bool {
        if let Some(keys) = Self::get_all_keys() {
            keys.iter().any(|key| key.starts_with(prefix))
        } else {
            false
        }
    }

}
