pub const STYLE: &str = r#"

@keyframes slideIn {
    from {
        transform: translateY(-30px);
        opacity: 0;
        filter: blur(10px);
    }
    to {
        transform: translateY(0);
        opacity: 1;
        filter: blur(0px);
    }
}

.checkbox-container {
    display: flex;
    flex-direction: column;
}

.checkbox-sidebar {
    /*padding: 1rem;*/
    border-radius: 8px;
    margin-left: 2rem;
}


.checkbox-pt {
    padding-left: 2.3rem;
}

.checkbox-pt img {
    animation: slideIn 0.4s ease-in-out forwards;
}

.checkbox-sidebar h3 {
    font-size: 18px;
    margin-bottom: 20px;
    animation: slideIn 0.5s ease-in-out forwards;
}

.checkbox-sidebar ul {
    list-style-type: none;
    display: flex;
    flex-wrap: wrap;
}

.checkbox-sidebar li {
    margin-bottom: 10px;
    display: flex;
    align-items: center;
    width: 48%;
    margin-right: 2%;
    opacity: 0; /* เริ่มต้นจากไม่แสดง */
    animation: slideIn 0.5s ease-in-out forwards;
}

.checkbox-sidebar li:nth-child(1) {
    animation-delay: 0.3s;
}

.checkbox-sidebar li:nth-child(2) {
    animation-delay: 0.6s;
}

.checkbox-sidebar li:nth-child(3) {
    animation-delay: 0.9s;
}

.checkbox-sidebar li:nth-child(4) {
    animation-delay: 1.2s;
}

.checkbox-sidebar li:nth-child(5) {
    animation-delay: 1.5s;
}

.checkbox-sidebar li:nth-child(6) {
    animation-delay: 1.8s;
}

.checkbox-sidebar li:nth-child(7) {
    animation-delay: 2.1s;
}

.checkbox-sidebar li:nth-child(8) {
    animation-delay: 2.4s;
}

.filter-checkbox {
    width: 20px;
    height: 20px;
    cursor: pointer;
    -webkit-appearance: none;
    appearance: none;
    background-color: #151a27;
    border: 2px solid #1DB954;
    border-radius: 4px;
    position: relative;
}

.filter-checkbox:hover {
    box-shadow: 0 0 10px 2px rgba(29, 185, 84, 0.7);
}

.filter-checkbox:checked {
    background-color: #1DB954;
    border: 2px solid #1DB954;
    box-shadow: 0 0 10px 2px rgba(29, 185, 84, 0.7);
}

.filter-checkbox:checked::after {
    content: '';
    position: absolute;
    top: 2px;
    left: 6px;
    width: 6px;
    height: 10px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}

.filter-label {
    margin-left: 8px;
    cursor: pointer;
}

.header {
    font-size: 16px;
    color: #A7AAB5;
    font-weight: normal;
}

.detail {
    font-size: 16px;
    color: #DBDEE8;
    font-weight: 250;
}

/* ปรับแต่งพิเศษสำหรับขนาดหน้าจอ */

/* สำหรับมือถือ: */
@media only screen and (max-width: 640px) {

    .checkbox-container {
        padding: 1rem;
    }

    .icon-container {
        display: flex; /* ใช้ flex เพื่อจัดเรียงไอคอนในแนวนอน */
        justify-content: space-between; /* จัดตำแหน่งให้มีพื้นที่ระหว่างไอคอน */
        align-items: center; /* จัดแนวกลางในแนวตั้ง */
        width: 100%; /* ให้ icon-container กว้างเต็มที่ */
        padding-bottom: 1rem; /* เว้นระยะใต้ icon */
        padding-left: 2.3rem;
    }

    .icon-container img {
        animation: slideIn 0.4s ease-in-out forwards;
        padding-bottom: 4px;
    }

    .checkbox-sidebar {
        padding: 1rem;
        border-radius: 8px;
        margin-left: 0.6rem;
    }

    .checkbox-sidebar ul {
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        margin: 0 auto;
    }

    .checkbox-sidebar li {
        width: 48%;
        margin-bottom: 10px;
    }

}


@media only screen and (min-width: 640px) {

    .checkbox-sidebar ul {
        flex-direction: column;
    }

    .icon-container img {
        animation: slideIn 0.4s ease-in-out forwards;
        padding-bottom: 7px;
        width: 75px; /* ปรับขนาดสำหรับแท็บเล็ตและเดสก์ท็อป */
    }

    .checkbox-sidebar li {
        width: 100%;
    }
}


"#;

