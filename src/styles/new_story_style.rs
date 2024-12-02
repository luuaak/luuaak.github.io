pub const STYLE: &str = r#"

.story-write-box {
    margin-top: 10px;
    display: flex;
    flex-direction: column;
    max-width: 1000px;
    gap: 1rem;
    padding: 1rem;
    border-radius: 8px;
    margin: 0 auto; /* ทำให้กล่องจัดอยู่กึ่งกลางในแนวนอน */
}


.story-header-box {
    display: flex;
    flex-direction: column; /* เปลี่ยนการจัดเรียงเป็นแนวตั้ง */
    gap: 0.4rem; /* เพิ่มช่องว่างระหว่างองค์ประกอบ */
    padding: 5px;
    border-radius: 10px;
}


.story-header-box span {
    color: #fff;
}

.input-text-title, .input-text-summary, .input-text-image-url {
    width: 100%;
    padding: 5px;
    border-radius: 5px;
    background-color: #404657; /* Same background as write-box */
}

.input-field {
    width: 100%;
    padding: 10px;
    border: none;
    border-radius: 5px;
    font-size: 1.1rem;
    color: #fff;
    background-color: transparent; /* Matches input-text background */
    outline: none;
}

.input-field::placeholder {
    color: #aaa; /* Light gray for placeholder */
    font-size: 1rem;
}



.title-box {
    display: flex;
    justify-content: space-between;
}


.input-title,
.input-tags {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.input-title span,
.input-tags span {
    font-size: 1rem;
    font-weight: 500;
    color: #fff;
}

.input-tags img {
    width: 20px;
    height: 20px;
    cursor: pointer;
}

.write-box {
    padding: 17px;
    margin-bottom: 10px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border-radius: 7px;
    background-color: #404657;
}

.write-box span {
    font-size: 0.9rem;
    color: #fff;
}

.option-btn {
    display: flex;
    gap: 0.5rem;
}

.option-btn-item {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    background: #3d424f;
    border: none;
    border-radius: 5px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;
    font-size: 0.9rem;
    color: #fff;
    transition: background 0.3s ease;
}

.option-btn-item:hover {
    background: #3d424f;
    color: #e0e0e0;
}

.option-btn-item img {
    width: 16px;
    height: 16px;
}

.input-long-text {
    min-height: 200px;
    padding: 0.5rem;
    border-radius: 5px;
    background: #333947;
    font-size: 1.1rem;
    color: #fff;
    resize: none;
}

/* จัดให้ submit-btn และ cancel-btn เรียงต่อกัน */
.action-btn {
    display: flex;
    gap: 0.5rem;
}

.submit-btn {
    background-color: #04b85d; /* สีเขียวพาสเทล */
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.3s ease;
}

.submit-btn:hover {
    background-color: #00FF7F;
}

.cancel-btn {
    background-color: #ef5350; /* สีแดงพาสเทล */
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.3s ease;
}

.cancel-btn:hover {
    background-color: #e53935;
}


"#;