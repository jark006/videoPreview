use winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico")
    .set("InternalName", "videoPreview.exe")
    .set("OriginalFilename", "videoPreview.exe")
    .set("FileDescription", "视频批量生成预览图")
    .set("LegalCopyright", "Copyright © 2025 JARK006")
    .set("ProductName", "视频批量生成预览图")
    .set("CompanyName", "JARK006");
    res.compile().unwrap();
}