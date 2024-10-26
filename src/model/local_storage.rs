use web_sys::window;

pub struct LocalStorage;

impl LocalStorage {

    // ฟังก์ชันเพื่อดึงคีย์ทั้งหมดใน Local Storage
    pub fn get_all_keys() -> Option<Vec<String>> {
        let storage = window().and_then(|win| win.local_storage().ok().flatten())?;
        let mut keys = Vec::new();

        for i in 0..storage.length().unwrap_or(0) {

            // เก็บเฉพาะ key ที่ขึ้นต้นด้วย 'story-teller_' ลงใน `story_teller_keys`
            if let Some(key) = storage.key(i).ok().flatten() {
                // เพิ่มเงื่อนไขในการตรวจสอบคีย์ที่ขึ้นต้นด้วย "story-teller_"
                if key.starts_with("story-teller_") {
                    keys.push(key);
                }
            }
        }

        Some(keys)
    }

    // สร้างเมธอดสำหรับดึงข้อมูลจาก Local Storage
    pub fn get(key: &str) -> Option<String> {
        if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
            storage.get_item(key).ok().flatten()
        } else {
            None
        }
    }

    // สร้างเมธอดสำหรับบันทึกข้อมูลลงใน Local Storage
    pub fn set(key: &str, value: &str) -> Result<(), String> {
        if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
            storage.set_item(key, value).map_err(|_| "Failed to set item in localStorage".to_string())
        } else {
            Err("Failed to access localStorage".to_string())
        }
    }

    // สร้างเมธอดสำหรับลบข้อมูลจาก Local Storage
    pub fn remove(key: &str) -> Result<(), String> {
        if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
            storage.remove_item(key).map_err(|_| "Failed to remove item from localStorage".to_string())
        } else {
            Err("Failed to access localStorage".to_string())
        }
    }
}
