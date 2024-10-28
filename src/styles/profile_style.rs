pub const STYLE: &str = r#"

.profile-box {
    max-width: 96%;
    margin: 10px auto;
    border-radius: 10px;
    overflow: hidden;
    background-color: #404657;
    padding: 15px;
}

/* Banner Image */
.banner-box img {
    margin-bottom: 10px;
    width: 100%;
    height: 270px;
    border-radius: 10px;
    object-fit: cover;
}

/* Profile Info Section */
.profile-info {
    /*padding: 5px 5px;*/
    display: flex;
    align-items: center;
    justify-content: space-between;
}

/* Profile Bar */
.profile-bar {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: space-between;
}

/* Profile Image and Name */
.profile-field-image {
    display: flex;
    align-items: center;
    gap: 10px;
}

.profile-field-image img {
    width: 72px;
    height: 73px;
    border-radius: 17px;
    object-fit: cover;
}

.profile-name {
    font-size: 17px;
    font-weight: bold;
    color: #fff;
}

/* Menu Buttons */
.profile-field-menu {
    display: flex;
    gap: 15px;
}

/***********************************************************/

.menu-btn {
    font-size: 14px;
    color: #fff;
    background: none;
    border: none;
    cursor: pointer;
    transition: color 0.3s;
}

.menu-btn:hover {
    color: #d9534f;
}


/***********************************************************/


/* Edit Profile Button */
.profile-field-options-btn {
    display: flex;
    align-items: center;
    gap: 10px;
}

/***********************************************************/

.edit-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    font-size: 14px;
    color: #fff;
    border: 2px solid #37C25E;
    border-radius: 10px;
    cursor: pointer;
    transition: background-color 0.3s, color 0.3s;
}

.edit-btn img {
    width: 15px;
    height: 15px;
}

.edit-btn:hover {
    background-color: #37C25E;
    color: #fff;
}

/***********************************************************/


.more-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    font-size: 14px;
    height: 35px;
    color: #fff;
    background-color: #37C25E;
    border: 1px solid #37C25E;
    border-radius: 10px;
    cursor: pointer;
    transition: background-color 0.3s, color 0.3s;
}

.more-btn img {
    width: 15px;
    height: 15px;
}

.more-btn:hover {
    background-color: #37C25E;
    color: #fff;
}

/**************************/

.content-box {
    margin-left: auto;
    margin-right: auto;
}


"#;