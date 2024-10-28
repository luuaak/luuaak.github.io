pub const STYLE: &str = r#"

.displayed-box {
    display: flex; /* ใช้ flexbox สำหรับการจัดเรียง */
    flex-direction: column; /* จัดเรียงแนวตั้ง */
    gap: 10px; /* ระยะห่างระหว่างชั้น */
}

.note-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}

.note-box {
    background-color: #404657; /* กำหนดสีพื้นหลัง */
    background-image: radial-gradient(
        ellipse farthest-corner at 45px 45px,
        rgba(64, 70, 87, 0.8) 0%,
        rgba(64, 70, 87, 0.2)
    );
    border-radius: 15px;
    transition: 0.3s;
    box-sizing: border-box;
    overflow: hidden;
    width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
    max-width: 280px; /* กำหนดขนาดสูงสุด */
    margin: 20px;
}

.note-box:hover {
    border-color: #6c00f8;
    box-shadow: 0 0 12px 4px rgba(243, 236, 248, 0.8);
}

.note-out:hover {
    transform: scale(1.07);
    border: 3px solid #6c00f8;
}


.textbox-note {
   height: 70px;
}


.note-image {
    width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
    height: 256px;
    object-fit: cover; /* ให้ภาพไม่บิดเบี้ยว */
    border-radius: 15px 15px 0px 0px;
}

.note-text {
    padding-top: 10px;
    padding-bottom: 10px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.note-text h2 {
    margin-bottom: 10px;
}

.line-clamping {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
}

.note-desc {
    padding-top: 12px;
    padding-left: 10px;
    padding-right: 10px;
    font-size: 16px;
    color: #a7aab5;
    line-height: 16px;
}

.note-desc h2 {
    color: #f7f7f7;
    font-size: 18px;
}

.note-icon {
    padding-left: 5px;
    padding-right: 5px;
    padding-top: 10px;
    padding-bottom: 10px;
    display: flex;
    align-items: center;
}

#note-author-bar {
    /*background-color: #505566;*/
    background-color: rgba(80, 85, 102, 0.8);
    border-radius: 10px;
    width: 100%;
    height: 60px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
}

#note-author {
    display: flex;
    justify-content: column;
    align-items: center;
    width: 80%;
}


.note-profile-image {
    width: 40px;
    height: 40px;
    border-radius: 27%;
    object-fit: cover; /* ให้ภาพไม่บิดเบี้ยว */
    margin: 5px;
}


/******************************************/
.mark-icon {
    fill: #FFFFFF;
    width: 24px;
    height: 24px;
    transition: fill 0.3s;
}

.mark-icon:hover,
.mark-icon:active {
    fill: ##ff004c;
}

/******************************************/


.author-info {
    display: flex;
    flex-direction: column;
    margin-left: 10px;
}

.author-info h3 {
    margin: 0;
    font-size: 12px;
    color: #f7f7f7;
}

.author-info p {
    margin: 0;
    font-size: 10px;
    color: #a7aab5;
}

/******************************************/
/* pagination */

/*
.foot-pt .pagination-box {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 10px;
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
}
*/

.foot-pt .btn-pagination ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
}

.foot-pt .btn-pagination .page-item {
    margin-right: 8px;
    cursor: pointer;
}

.foot-pt .btn-pagination .page-item label {
    padding: 6px 12px;
    background-color: #404657; /* สีพื้นหลังเริ่มต้น */
    color: white;
    border-radius: 4px;
    text-align: center;
    transition: background-color 0.3s, color 0.3s; /* เพิ่มการเปลี่ยนสี */
}

/* เปลี่ยนสีเมื่อ hover */
.foot-pt .btn-pagination .page-item label:hover {
    background-color: #00FF7F;
    color: #404657;
}

/* เปลี่ยนสีเมื่อ active (กด) */
.foot-pt .btn-pagination .page-item label:active {
    background-color: #00FF7F; /* สีเดียวกับ label:hover */
    color: #404657; /* สีข้อความ */
}

/* เปลี่ยนสีเมื่อมีการโฟกัส */
.foot-pt .btn-pagination .page-item label:focus {
    background-color: #00FF7F; /* สีเดียวกับ label:hover */
    color: #404657; /* สีข้อความ */
}

/* สำหรับ ellipsis */
.foot-pt .btn-pagination .page-item.ellipsis {
    pointer-events: none;
    color: #888;
}

/* สำหรับ next */
.foot-pt .btn-pagination .page-item.next label {
    background-color: #404657;
}

/* สำหรับ active page */
.foot-pt .btn-pagination .page-item.active label {
    background-color: #00FF7F; /* สีของปุ่มที่ active */
    color: #404657; /* สีข้อความ */
}




.foot-pt {
    display: flex;
    justify-content: center;
    /*border: 1px solid #ddd;*/
    border-radius: 5px;
    padding: 10px; /* เพิ่ม padding ภายใน */
}


/******************************************/

@media only screen and (max-width: 991px) {

    .note-box {
        width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
        max-width: 240px; /* กำหนดขนาดสูงสุด */
        margin: 15px;
        height: 350px;
    }

    .note-image {
        width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
        height: 189px;
    }

    .note-desc {
        font-size: 14px;
    }

    .note-desc h2 {
        font-size: 16px;
    }

    .author-info h3 {
        font-size: 10px;
    }

    .author-info p {
        font-size: 8px;
    }

}




"#;
