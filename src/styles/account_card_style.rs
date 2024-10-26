pub const STYLE: &str = r#"

.account-card {
    background-color: transparent;
    background-color: #303134; /* สีพื้นหลังเทาเข้มคล้ายในภาพ */
    border-radius: 12px; /* ทำให้มุมโค้งมน */
    color: #ffffff;
    padding-top: 30px;
    position: fixed;
    top: 60px;
    right: 20px;
    width: 350px;
    max-width: 100%;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    z-index: 1000;
}

.nip05-info {
    font-size: 12px;
    color: #bbbbbb; /* สีข้อความ nip05 เป็นสีเทาอ่อน */
    margin-bottom: 10px;
}

.profile-image {
    border: 4px solid #3C3D37;
    border-radius: 50%;
    width: 90px;
    height: 90px;
    object-fit: cover;
    margin-bottom: 20px;
}

.user-info {
    margin-bottom: 20px;
    width: 90%;
}

.user-name {
    margin: 10px 0;
    font-size: 22px;
    font-weight: bold;
}

#submit-on-card-pt {
    margin-top: 10px;
}

#submit-on-card {
    width: 100%;
    max-width: 300px;
    background-color: transparent;
    background-color: #28292a; /* พื้นหลังปุ่มเป็นสีเดียวกับ card */
    color: #ffffff;
    padding: 12px;
    font-size: 15px;
    font-weight: bold;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.3s ease, color 0.3s ease;
}

#submit-on-card:hover {
    border: 1px solid #5f6368; /* กำหนดเส้นขอบ */
    background-color: #444950; /* เปลี่ยนสีปุ่มเมื่อ hover */
    color: #ffffff; /* สีตัวอักษรขาว */
}



"#;