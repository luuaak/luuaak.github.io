pub const STYLE: &str = r#"

.item-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

#nav {
  background-color: #151a27;
  padding: 1rem;
  display: flex;
  align-items: center;
}

.nav-profile-round img {
    background-color: #F4F4F4;
    border: 3px solid #3C3D37;
	border-radius: 50%;
	width: 40px; /* ขนาดที่เหมาะสม */
	height: 40px; /* ขนาดที่เหมาะสม */
	object-fit: cover; /* ทำให้รูปภาพเติมเต็ม container โดยไม่บิดเบือน */
}

.login-position {
    margin-right: 40px;
}


.nav-login {
    background-color: transparent;
    border: 2px solid #37C25E;
    color: #37C25E;
    font-size: 17px;
    padding: 0;
    width: 100%;
    height: 34px;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.3s ease;
}

.nav-login:hover {
    background-color: rgba(3, 191, 203, 0.1);
    box-shadow: 0 0 10px #37C25E;
}

/* ปรับแต่งพิเศษสำหรับขนาดหน้าจอ */
@media only screen and (max-width: 599px) {

    img {
        max-width: 200px;
        height: auto;
    }

}


"#;
