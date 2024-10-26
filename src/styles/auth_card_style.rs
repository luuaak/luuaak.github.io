pub const STYLE: &str = r#"

#overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.5); /* เพิ่มความมืดจาง ๆ */
    backdrop-filter: blur(8px); /* เบลอพื้นหลัง */
    z-index: 999; /* ซ้อนอยู่ล่างสุดของ AuthCard */
}


#form-ui {
    position: fixed; /* เปลี่ยนเป็น fixed เพื่อให้ลอยอยู่ */
    display: flex;
    z-index: 1000;

    /* จัดตำแหน่งให้กลางจอ */
    top: 50%; /* ตรงกลางในแนวตั้ง */
    left: 50%; /* ตรงกลางในแนวนอน */
    transform: translate(-50%, -50%); /* ปรับให้แน่ใจว่าตรงกลาง */
    padding: 10px; /* เพิ่ม padding เพื่อความสวยงาม */
    border-radius: 10px; /* เพิ่มความโค้งมน */
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.2); /* เงาเพื่อให้ดูลอยขึ้น */
}

#form-card {
    display: grid;
    place-items: center;
    width: 300px;
    height: 472px;
    padding: 25px;
    background-color: #161616; /* พื้นหลังของ form card */
    box-shadow: 0px 15px 60px #00FF7F; /* เงาของ card */
    outline: 1px solid #2b9962; /* เส้นขอบ */
    border-radius: 10px; /* โค้งมน */
}

#form-body {
    width: 100%; /* ให้เต็มความกว้าง */
    margin: 0; /* ไม่มี margin */
}

/* ส่วนอื่นๆ ยังใช้ได้ตามที่คุณให้มา */
#welcome-lines {
    text-align: center;
    line-height: 1;
}

#logo-top-card {
    display: flex;
    justify-content: center;
}

#logo-top-card img {
    width: 100%; /* ปรับความกว้างให้เต็มที่ */
    height: auto; /* ความสูงปรับอัตโนมัติตามสัดส่วน */
    max-width: 100px; /* กำหนดขนาดสูงสุดเพื่อป้องกันการขยายเกิน */
}

#close-button {
    position: absolute;
    top: 30px; /* จัดตำแหน่งให้ใกล้มุมขวาบน */
    right: 30px; /* จัดตำแหน่งให้ใกล้มุมขวาบน */
    cursor: pointer;
}

#close-button img {
    width: 20px; /* กำหนดขนาดของไอคอน cross */
    height: 20px;
}


#welcome-line-2 {
    color: #ffffff;
    font-size: 18px;
    margin-top: 10px;
    margin-bottom: 40px;
}

#input-area {
    margin-top: 40px;
}

.form-inp {
    padding: 11px 25px;
    background: transparent;
    border: 1px solid #e3e3e3;
    line-height: 1;
    border-radius: 8px;
}

.form-inp:focus {
    border: 1px solid #00FF7F;
}

.form-inp:first-child {
    margin-bottom: 15px;
}

.form-inp input {
    width: 100%;
    background: none;
    font-size: 13.4px;
    color: #00FF7F;
    border: none;
    padding: 0;
    margin: 0;
}

.form-inp input:focus {
    outline: none;
}

#submit-button-cvr {
    margin-top: 20px;
}

#submit-button {
    display: block;
    width: 100%;
    background-color: transparent;
    background-color: #28292a; /* พื้นหลังปุ่มเป็นสีเดียวกับ card */
    color: #ffffff;
    font-weight: 600;
    font-size: 14px;
    margin: 0;
    padding: 14px 13px 12px 13px;
    border: 0;
    border-radius: 8px;
    line-height: 1;
    cursor: pointer;
    transition: background-color 0.3s ease, color 0.3s ease;
}

#submit-button:hover {
    border: 1px solid #5f6368; /* กำหนดเส้นขอบ */
    background-color: #444950; /* เปลี่ยนสีปุ่มเมื่อ hover */
    color: #ffffff; /* สีตัวอักษรขาว */
    cursor: pointer;
}

#new-nsec-desc {
    text-align: center;
    margin-top: 30px;
}

#new-nsec-desc a {
    color: #868686;
    font-size: 15px;
    text-decoration: none;
}


"#;
