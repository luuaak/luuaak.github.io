pub const STYLE: &str = r#"

/* สไตล์หลักของ markdown field */
.markdown-field-body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    font-size: 16px;
    line-height: 1.6;
    color: #c9d1d9;
    background-color: #333947;
    padding: 20px;
    min-height: 100px;
    border-radius: 8px;
}

/* สไตล์สำหรับ headers */
.markdown-field-body h1 {
    font-size: 2em;
    font-weight: bold;
    color: #f0f6fc;
    border-bottom: 1px solid #30363d;
    padding-bottom: 0.3em;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}
.markdown-field-body h2 {
    font-size: 1.75em;
    font-weight: bold;
    color: #f0f6fc;
    border-bottom: 1px solid #30363d;
    padding-bottom: 0.3em;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}
.markdown-field-body h3 {
    font-size: 1.5em;
    font-weight: bold;
    color: #f0f6fc;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}
.markdown-field-body h4, .markdown-field-body h5, .markdown-field-body h6 {
    color: #f0f6fc;
    font-weight: bold;
    margin-top: 1em;
    margin-bottom: 0.5em;
}

/* สไตล์สำหรับลิงก์ */
.markdown-field-body a {
  color: #3bcf5f;
  text-decoration: none;
}

.markdown-field-body a:hover {
    color: #58ff84;
    text-decoration: underline;
}

/* สไตล์สำหรับโค้ด */
.markdown-field-body code {
    font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
    background-color: #232933;
    color: #d1d5da;
    padding: 0.2em 0.4em;
    border-radius: 6px;
    font-size: 85%;
}


/* สไตล์สำหรับบล็อกโค้ด */
.markdown-field-body pre {
      background-color: #20242b;
      color: #d1d5da;
      padding: 1em;
      border-radius: 6px;
      overflow-x: auto;
      font-size: 0.85em;
      line-height: 1.45;
}
.markdown-field-body pre code {
    background: none;
    color: inherit;
    padding: 0;
    font-size: inherit;
}

/* สไตล์สำหรับตาราง */
.markdown-field-body table {
    width: 100%;
    border-collapse: collapse;
    margin: 20px 0;
    background-color: #21262d;
    border: 1px solid #30363d;
}
.markdown-field-body th, .markdown-field-body td {
    border: 1px solid #30363d;
    padding: 8px;
    text-align: left;
    color: #c9d1d9;
}
.markdown-field-body th {
    background-color: #21262d;
    color: #adbac7;
    font-weight: bold;
}

/* สไตล์สำหรับ blockquote */
.markdown-field-body blockquote {
    color: #8b949e;
    border-left: 4px solid #30363d;
    padding-left: 1em;
    margin: 0;
    margin-left: 0;
    margin-top: 1em;
    margin-bottom: 1em;
    background-color: #161b22;
    border-radius: 6px;
    padding: 0.5em 1em;
}

/* Blockquote style for IMPORTANT */
.markdown-field-body blockquote.important {
    border-left: 4px solid #8957e5;
}

/* Blockquote style for WARNING */
.markdown-field-body blockquote.warning {
    border-left: 4px solid #9e6a03;
}

/* สไตล์สำหรับรายการ */
.markdown-field-body ul {
    list-style-type: disc;
    padding-left: 2em;
}
.markdown-field-body ol {
    list-style-type: decimal;
    padding-left: 2em;
}
.markdown-field-body li {
    margin: 0.5em 0;
}

/* สไตล์สำหรับภาพ */
.markdown-field-body img {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
    margin-top: 1em;
    margin-bottom: 1em;
}

/* สไตล์สำหรับการแบ่งเส้น */
.markdown-field-body hr {
    border: none;
    border-top: 1px solid #30363d;
    margin: 1.5em 0;
}
"#;
