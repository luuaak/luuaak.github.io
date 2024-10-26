pub const STYLE: &str = r#"

.control-box {
    display: flex;
    flex-direction: row; /* ค่าเริ่มต้นให้เป็น row สำหรับขนาดหน้าจอใหญ่ */
}

.control-box > div {
    padding: 10px; /* กำหนดช่องว่างระหว่างคอมโพเนนต์ */
}

@media only screen and (max-width: 640px) {
    .control-box {
        flex-direction: column; /* เปลี่ยนเป็น column สำหรับขนาดหน้าจอมือถือ */
    }

    .control-box > div {
        width: 100%; /* ให้คอมโพเนนต์มีความกว้าง 100% */
    }
}

"#;
