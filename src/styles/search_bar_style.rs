pub const STYLE: &str = r#"

/* โครงสร้างหลักที่จัดการการจัดวาง */
#search-pt {
    display: flex;
    justify-content: center; /* จัดให้อยู่กึ่งกลางแนวนอน */
    align-items: center;     /* จัดให้อยู่กึ่งกลางแนวตั้ง */
    margin-top: 25px;      /* ระยะห่างด้านบน */
    margin-bottom: 25px;   /* ระยะห่างด้านล่าง */
}

.search-bar {
    width: 100%;
    max-width: 600px; /* ขนาดสูงสุดของช่องค้นหา */
    padding: 0 20px;  /* เพิ่มระยะห่างด้านข้าง */
}

.input-container {
    position: relative;
    display: flex;
    align-items: center; /* จัดองค์ประกอบภายในให้อยู่กึ่งกลาง */
    width: 100%;
}

input.search-box {
    width: 100%; /* ใช้ความกว้างเต็ม */
    height: 48px; /* กำหนดความสูงตรง ๆ */
    padding: 0 16px; /* ปรับ padding ด้านข้าง */
    border-radius: 20px;
    border: 3px solid #c3c6ce;
    background-color: #f5f5f5;
    color: #2D323B;
    padding-right: 40px; /* เพิ่มพื้นที่ด้านขวาสำหรับ icon */
    box-sizing: border-box; /* ป้องกัน padding ทำให้ขนาดเกิน */
}

input.search-box:hover {
    border-color: #6c00f8;
    box-shadow: 0 0 12px 4px rgba(243, 236, 248, 0.8);
}

.searchIcon {
    position: absolute;
    right: 10px;  /* จัดให้อยู่ขอบขวาของ input */
    width: 20px;  /* ขนาดของ icon */
    height: 20px; /* ปรับขนาดเพื่อให้เหมาะสม */
    fill: rgb(114, 114, 114); /* สีของไอคอน */
}

/* ปรับแต่งให้เหมาะสมกับขนาดหน้าจอต่าง ๆ */
@media only screen and (max-width: 600px) {
    input.search-box {
        padding: 8px 16px;
        border-width: 2px;
        padding-right: 35px; /* ลดขนาด padding ขวาสำหรับ icon */
    }

    .searchIcon {
        right: 8px; /* ลดระยะห่างขวา */
        width: 16px;
        height: 16px;
    }
}

/* สำหรับแท็บเล็ตและเดสก์ท็อป */
@media only screen and (min-width: 601px) {
    input.search-box {
        padding: 16px 24px;
        border-width: 3px;
        padding-right: 40px;
    }

    .searchIcon {
        width: 20px;
        height: 20px;
    }
}

"#;
