pub const STYLE: &str = r#"


.article-box {
    max-width: 1000px; /* ปรับขนาดความกว้างให้การ์ด */
    margin: 20px auto; /* จัดการ์ดให้อยู่ตรงกลาง */
    padding: 20px;
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    background-color: #404657;
}

.article-field {
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.markdown-field-text-title {
    color: #FFFFFF;
    font-size: 26px;
    font-weight: bold;
    margin-bottom: 10px;
    text-align: center;
}

.article-field-image-header {
    position: relative;
    width: 100%;
    height: auto;
}

.field-title-image-box {
    height: auto;
    max-width: 100%;
    max-height: 420px;
    border-radius: 8px;
    object-fit: cover;   /* ให้ภาพอยู่ในกรอบโดยไม่เสียสัดส่วน */
    margin: 0 auto;        /* จัดให้อยู่ตรงกลาง */
}

.field-pt {
    max-width: 600px;
    margin: 0 auto;    /* จัดให้ div อยู่ตรงกลาง */
    padding: 5px;     /* เพิ่ม padding ภายใน */
}

.article-field-icons {
    color: #FFFFFF;
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
}

.field-icon-box {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 14px;
}

.field-icon-box img {
    width: 23px;        /* ปรับขนาดความกว้างของไอคอน */
    height: 23px;       /* ปรับขนาดความสูงของไอคอน */
    object-fit: cover;  /* ให้ขนาดของไอคอนไม่บิดเบี้ยว */
}

/*******************************/


.markdown-field-text {
    white-space: pre-wrap;
    font-size: 19px;
    line-height: 1.5;
    color: #FFFFFF;
    margin-top: 10px;
}

.markdown-field-text pre {
    background-color: #242430;
    padding: 10px;
    border-radius: 5px;
    overflow-x: auto; /* ทำให้สามารถเลื่อนด้านข้างได้เมื่อเนื้อหายาว */
}

/* สไตล์สำหรับลิงก์ */
.markdown-field-text a {
    color: #90EE90;
    text-decoration: underline;
    transition: color 0.3s;
    white-space: nowrap;       /* ทำให้ข้อความไม่ขึ้นบรรทัดใหม่ */
    overflow: hidden;          /* ซ่อนข้อความที่เกินขอบเขต */
    text-overflow: ellipsis;   /* แสดงจุดสามจุดเมื่อข้อความเกินขอบเขต */
    max-width: 100%;           /* กำหนดขอบเขตความกว้างสูงสุด */
    display: inline-block;     /* ปรับให้ทำงานกับ max-width ได้ */
}

.markdown-field-text a:hover {
    color: #32CD32;
}


.markdown-field-text img {
    margin-top: 10px;
    border-radius: 8px;
    max-width: 100%; /* ให้รูปภาพไม่กว้างเกินขอบเขตของ container */
    height: auto;    /* รักษาอัตราส่วนภาพ */
    object-fit: cover;
}



/* เพิ่มสไตล์สำหรับการแสดงผลตาราง */
.markdown-field-text table {
    width: 100%;
    border-collapse: collapse;
    margin: 20px 0;
    overflow-x: auto;
    display: block;
    white-space: nowrap;
}

/* ปรับให้เซลล์ของตารางเรียงอยู่ตรงกลางและเพิ่มพื้นที่ padding */
.markdown-field-text th,
.markdown-field-text td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: center;
}

/* สไตล์สำหรับ header ของตาราง */
.markdown-field-text th {
    background-color: #404657;
    color: white;
    font-weight: bold;
}



@media only screen and (max-width: 768px) {

    .field-pt {
        max-width: 500px;
        padding: 5px;
    }

    .article-field-icons {
        gap: 5px;               /* เพิ่มระยะห่างระหว่าง icon */
    }

    .field-icon-box span {
        font-size: 12px;         /* ปรับขนาดฟอนต์ของ icon ใหญ่ขึ้น */
        /*gap: 5px;*/
    }

    .field-icon-box img {
        width: 17px;             /* ปรับขนาดไอคอน */
        height: 17px;
    }
}



@media only screen and (max-width: 600px) {

    .markdown-field-text {
        font-size: 16px;
        line-height: 1.5;
        margin-top: 10px;
    }

    .markdown-field-text table {
        display: block;
        overflow-x: auto;
        width: 100%;
    }

    .markdown-field-text th, .markdown-field-text td {
        font-size: 14px; /* ลดขนาดฟอนต์ในหน้าจอเล็ก */
        padding: 6px;
    }
}



@media only screen and (max-width: 599px) {

    .markdown-field-text-title {
        font-size: 23px;
    }


    .field-pt {
        max-width: 440px;
        padding: 5px;
    }

    .article-field-icons {
        gap: 5px;               /* เพิ่มระยะห่างระหว่าง icon */
    }

    .field-icon-box span {
        font-size: 10px;         /* ปรับขนาดฟอนต์ของ icon ใหญ่ขึ้น */
    }

    .field-icon-box img {
        width: 14px;             /* ปรับขนาดไอคอน */
        height: 14px;
    }

}



@media only screen and (max-width: 450px) {

    .markdown-field-text-title {
        font-size: 17px;
    }

    .field-pt {
        max-width: 430px;
    }

    .field-icon-box span {
        font-size: 7px;         /* ปรับขนาดฟอนต์ของ icon ใหญ่ขึ้น */
    }

    .field-icon-box img {
        width: 15px;             /* ปรับขนาดไอคอน */
        height: 15px;
    }

}


"#;