use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct HtmlAssets;

mod tests {
    use crate::http::assets::HtmlAssets;
    use rust_embed::EmbeddedFile;

    #[test]
    fn directory_mapped() {
        let asset: Option<EmbeddedFile> = HtmlAssets::get("html/error_page.html");
        assert!(asset.is_some());
    }
    #[test]
    fn directory_mapped_css() {
        let asset: Option<EmbeddedFile> = HtmlAssets::get("html/css/index.css");
        assert!(asset.is_some());
    }
    #[test]
    fn directory_mapped_img() {
        let asset: Option<EmbeddedFile> = HtmlAssets::get("html/img/img.png");
        assert!(asset.is_some());
    }
}
