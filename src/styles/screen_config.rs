pub const STYLE: &str = r#"

[class*="col-"] {
    box-sizing: border-box;
}

.container {
    margin-left: auto;
    margin-right: auto;
}

.overflow-x-hidden {
    overflow-x: hidden;
}


/************ ขนาดจอต่างๆ ************/

/*
 * `xs` สำหรับจอขนาดเล็กมาก (max-width: 450px)
 *    เหมาะสำหรับ: มือถือขนาดเล็ก เช่น iPhone SE
 *    ขนาดหน้าจอ: 375 x 667 px
 *
 * `sm` สำหรับจอขนาดเล็ก (min-width: 641px)
 *    เหมาะสำหรับ: มือถือขนาดใหญ่ เช่น iPhone 12, Android ขนาดใหญ่
 *    ขนาดหน้าจอ: iPhone 12 (390 x 844 px), Samsung Galaxy S21 (412 x 915 px)
 *
 * `md` สำหรับจอขนาดกลาง (min-width: 740)
 *    เหมาะสำหรับ: แท็บเล็ตขนาดเล็ก เช่น iPad Mini, Samsung Tab S6 Lite
 *    ขนาดหน้าจอ: iPad Mini (744 x 1133 px), Samsung Tab S6 Lite (800 x 1200 px)
 *
 * `lg` สำหรับจอขนาดใหญ่ (min-width: 1024px)
 *    เหมาะสำหรับ: แท็บเล็ตขนาดใหญ่ เช่น iPad Pro 11", Surface Go
 *    ขนาดหน้าจอ: iPad Pro 11" (834 x 1194 px), Surface Go (1800 x 1200 px)
 *
 * `xl` สำหรับจอขนาดใหญ่มาก (min-width: 1280px)
 *    เหมาะสำหรับ: แล็ปท็อป, หน้าจอเดสก์ท็อปขนาดมาตรฐาน
 *    ขนาดหน้าจอ: MacBook Air 13" (1440 x 900 px), หน้าจอเดสก์ท็อปทั่วไป (1920 x 1080 px)
 */


@media only screen and (max-width: 450px) {

    .col-xs-hidden {
        display: none;
    }


    .col-xs-1 {
        width: 8.33%;
    }

    .col-xs-2 {
        width: 16.66%;
    }

    .col-xs-3 {
        width: 25%;
    }

    .col-xs-4 {
        width: 33.33%;
    }

    .col-xs-5 {
        width: 41.66%;
    }

    .col-xs-6 {
        width: 50%;
    }

    .col-xs-7 {
        width: 58.33%;
    }

    .col-xs-8 {
        width: 66.66%;
    }

    .col-xs-9 {
        width: 75%;
    }

    .col-xs-10 {
        width: 83.33%;
    }

    .col-xs-11 {
        width: 91.66%;
    }

    .col-xs-12 {
        width: 100%;
    }

}


@media only screen and (min-width: 641px) {

    .col-sm-hidden {
        display: none;
    }


    .col-sm-1 {
        width: 8.33%;
    }

    .col-sm-2 {
        width: 16.66%;
    }

    .col-sm-3 {
        width: 25%;
    }

    .col-sm-4 {
        width: 33.33%;
    }

    .col-sm-5 {
        width: 41.66%;
    }

    .col-sm-6 {
        width: 50%;
    }

    .col-sm-7 {
        width: 58.33%;
    }

    .col-sm-8 {
        width: 66.66%;
    }

    .col-sm-9 {
        width: 75%;
    }

    .col-sm-10 {
        width: 83.33%;
    }

    .col-sm-11 {
        width: 91.66%;
    }

    .col-sm-12 {
        width: 100%;
    }

}


@media only screen and (min-width: 740) {

    .col-md-hidden {
        display: none;
    }


    .col-md-1 {
        width: 8.33%;
    }

    .col-md-2 {
        width: 16.66%;
    }

    .col-md-3 {
        width: 25%;
    }

    .col-md-4 {
        width: 33.33%;
    }

    .col-md-5 {
        width: 41.66%;
    }

    .col-md-6 {
        width: 50%;
    }

    .col-md-7 {
        width: 58.33%;
    }

    .col-md-8 {
        width: 66.66%;
    }

    .col-md-9 {
        width: 75%;
    }

    .col-md-10 {
        width: 83.33%;
    }

    .col-md-11 {
        width: 91.66%;
    }

    .col-md-12 {
        width: 100%;
    }

}

@media only screen and (min-width: 1024px) {

    .col-lg-hidden {
        display: none;
    }


    .col-lg-1 {
        width: 8.33%;
    }

    .col-lg-2 {
        width: 16.66%;
    }

    .col-lg-3 {
        width: 25%;
    }

    .col-lg-4 {
        width: 33.33%;
    }

    .col-lg-5 {
        width: 41.66%;
    }

    .col-lg-6 {
        width: 50%;
    }

    .col-lg-7 {
        width: 58.33%;
    }

    .col-lg-8 {
        width: 66.66%;
    }

    .col-lg-9 {
        width: 75%;
    }

    .col-lg-10 {
        width: 83.33%;
    }

    .col-lg-11 {
        width: 91.66%;
    }

    .col-lg-12 {
        width: 100%;
    }

}


@media only screen and (min-width: 1280px) {

    .col-xl-hidden {
        display: none;
    }


    .col-xl-1 {
        width: 8.33%;
    }

    .col-xl-2 {
        width: 16.66%;
    }

    .col-xl-3 {
        width: 25%;
    }

    .col-xl-4 {
        width: 33.33%;
    }

    .col-xl-5 {
        width: 41.66%;
    }

    .col-xl-6 {
        width: 50%;
    }

    .col-xl-7 {
        width: 58.33%;
    }

    .col-xl-8 {
        width: 66.66%;
    }

    .col-xl-9 {
        width: 75%;
    }

    .col-xl-10 {
        width: 83.33%;
    }

    .col-xl-11 {
        width: 91.66%;
    }

    .col-xl-12 {
        width: 100%;
    }

}


"#;