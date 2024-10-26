pub const STYLE: &str = r#"


.article-author-box {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 12px;       /* เพิ่มมุมโค้ง */
    max-width: 800px;
    margin: 0 auto;
}


.article-button-item {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 9px 20px; /* เพิ่ม padding เพื่อให้ปุ่มดูสมดุลกับเนื้อหา */
    gap: 8px;
    height: 40px;
    border: none;
    background: #505566;
    border-radius: 10px;
    cursor: pointer;
    white-space: nowrap; /* ป้องกันไม่ให้ข้อความตัดบรรทัด */
}


.article-button-follow {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 9px 20px; /* เพิ่ม padding เพื่อให้ปุ่มดูสมดุลกับเนื้อหา */
    gap: 8px;
    height: 45px;
    border: none;
    background: #FF2849;
    border-radius: 10px;
    cursor: pointer;
    white-space: nowrap; /* ป้องกันไม่ให้ข้อความตัดบรรทัด */
}



.article-lable-item {
    line-height: 22px;
    font-size: 17px;
    color: #fff;
    font-family: sans-serif;
    letter-spacing: 1px;
}

.article-button-item:hover {
    background: #e52441;
}

.field-button-pt {
    max-width: 600px;
    /*width: 100%;*/
    margin: 0 auto;
    padding: 15px;
}

.field-button-util {
    display: flex;
    justify-content: space-between;
    gap: 14px; /* เพิ่มช่องว่างระหว่างปุ่ม */
}

/*************************/

.author-info {
    display: flex;
    align-items: center;
    margin-top: 10px;
    width: 87%;
}

#article-author {
    display: flex;
    align-items: center;
}

.article-author-bar {
    background-color: rgba(80, 85, 102, 0.8);
    border-radius: 12px;
    width: 100%; /* กำหนดความกว้างเป็น 100% */
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px; /* เพิ่ม padding ให้ดูสวยงามขึ้น */
    margin: 0 auto; /* จัดให้อยู่ตรงกลาง */
}


#author-info {
    background-color: rgba(80, 85, 102, 0.8);
    border-radius: 10px;
    width: 100%;
    height: 60px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
}

.article-profile-image {
    width: 83px;
    height: 84px;
    border-radius: 20%;
    object-fit: cover; /* ให้ภาพไม่บิดเบี้ยว */
    margin: 5px;
}

.article-author-name {
    color: #fff;
    margin-bottom: 8px;
}

.article-author-details {
    margin-left: 5px;
}

.article-count {
    color: #fff;
    font-size: 14px;
    margin-bottom: 52px;
    width: 20%;
}


@media only screen and (max-width: 1298px) {

    .article-author-box {
        max-width: 870px;
    }

    .field-button-util {
        gap: 16px;
    }

    .article-lable-item {
        line-height: 22px;
        font-size: 16px;
        letter-spacing: 1px;
    }

    .article-author-bar {
       width: 600px;
    }

    .article-button-item {
        padding: 5px 15px;
        gap: 12px;
        height: 50px;
    }

}


@media only screen and (max-width: 641px) {

    .article-lable-item {
        line-height: 22px;
        font-size: 14px;
        letter-spacing: 1px;
    }

    .article-author-bar {
       width: 500px;
    }

    .article-button-item {
        padding: 5px 15px;
        gap: 8px;
        height: 40px;
    }

    .author-info {
        margin-top: 10px;
        width: 90%;
    }

}


@media only screen and (max-width: 450px) {

    .field-button-pt {
        max-width: 340px;
        margin: 0 auto;
        padding: 15px;
    }

    .article-lable-item {
        line-height: 22px;
        font-size: 10px;
    }

    .article-button-item {
        padding: 5px 14px;
        gap: 8px;
        height: 35px;
    }

    .author-info {
        margin-top: 10px;
        width: 90%;
    }

}

"#;