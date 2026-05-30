use mdux::{
    compile_text_package, ApprovedString, CompiledGlyph, CompiledTextRun, FontAsset,
    GlyphDrawCommand, MduxResult, RasterizedGlyph, TextCompilationInput, TextDirection,
    TextPackage, TextRuntime, ValidationError,
};

pub const HELLO_WORLD_TEXT: &str = "Hello World !";
pub const HELLO_WORLD_STRING_ID: &str = "STR-HELLO-WORLD";
pub const HELLO_WORLD_RUN_ID: &str = "RUN-HELLO-WORLD";
pub const HELLO_WORLD_DRAW_COMMAND_COUNT: usize = 11;
const HELLO_WORLD_ATLAS_WIDTH: u16 = 64;
const HELLO_WORLD_ATLAS_PADDING: u16 = 1;
const HELLO_WORLD_FONT_SHA256: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const HELLO_WORLD_BUILD_RECIPE: &str =
    "embedded-16px-glyphs+deterministic-atlas+compiled-run:Hello World !";
const HELLO_WORLD_GLYPHS: [(char, i32); 13] = [
    ('H', 10),
    ('e', 10),
    ('l', 6),
    ('l', 6),
    ('o', 10),
    (' ', 4),
    ('W', 12),
    ('o', 10),
    ('r', 8),
    ('l', 6),
    ('d', 10),
    (' ', 4),
    ('!', 4),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HelloWorldTextLayout {
    pub package: TextPackage,
    pub commands: [GlyphDrawCommand; HELLO_WORLD_DRAW_COMMAND_COUNT],
}

pub fn hello_world_text_package() -> MduxResult<TextPackage> {
    compile_text_package(hello_world_text_compilation_input())
}

#[allow(dead_code)]
pub fn hello_world_glyph_draw_commands(
    origin_x: i32,
    origin_y: i32,
) -> MduxResult<[GlyphDrawCommand; HELLO_WORLD_DRAW_COMMAND_COUNT]> {
    Ok(hello_world_text_layout(origin_x, origin_y)?.commands)
}

pub fn hello_world_text_layout(origin_x: i32, origin_y: i32) -> MduxResult<HelloWorldTextLayout> {
    let package = hello_world_text_package()?;
    let commands = {
        let runtime = TextRuntime::<HELLO_WORLD_DRAW_COMMAND_COUNT>::new(&package)?;
        runtime
            .render_run(HELLO_WORLD_RUN_ID, origin_x, origin_y)?
            .into_inner()
            .map_err(|_| ValidationError::new("hello world command count changed unexpectedly"))?
    };

    Ok(HelloWorldTextLayout { package, commands })
}

fn hello_world_text_compilation_input() -> TextCompilationInput {
    TextCompilationInput {
        fonts: vec![FontAsset {
            family: "Embedded Hello World Mono".to_string(),
            source_path: "embedded://hello_world/16px".to_string(),
            sha256: HELLO_WORLD_FONT_SHA256.to_string(),
            face_index: 0,
            pixel_height: 16,
            locales: vec!["en-US".to_string()],
        }],
        approved_strings: vec![ApprovedString {
            id: HELLO_WORLD_STRING_ID.to_string(),
            locale: "en-US".to_string(),
            value: HELLO_WORLD_TEXT.to_string(),
            direction: TextDirection::LeftToRight,
        }],
        rasterized_glyphs: hello_world_rasterized_glyphs(),
        runs: vec![hello_world_run()],
        numeric_glyph_sets: Vec::new(),
        numeric_templates: Vec::new(),
        toolchain_id: "rust-embedded-hello-text-v1".to_string(),
        unicode_version: "15.1.0".to_string(),
        build_recipe: HELLO_WORLD_BUILD_RECIPE.to_string(),
        atlas_width: HELLO_WORLD_ATLAS_WIDTH,
        atlas_padding: HELLO_WORLD_ATLAS_PADDING,
    }
}

fn hello_world_run() -> CompiledTextRun {
    let mut cursor_x = 0;
    let mut glyphs = Vec::with_capacity(HELLO_WORLD_GLYPHS.len());

    for &(character, advance_x) in &HELLO_WORLD_GLYPHS {
        glyphs.push(CompiledGlyph {
            atlas_index: 0,
            glyph_id: u32::from(character),
            x: cursor_x,
            y: 0,
            advance_x,
        });
        cursor_x += advance_x;
    }

    CompiledTextRun {
        id: HELLO_WORLD_RUN_ID.to_string(),
        source_string_id: HELLO_WORLD_STRING_ID.to_string(),
        locale: "en-US".to_string(),
        bidi_level: 0,
        glyphs,
    }
}

fn hello_world_rasterized_glyphs() -> Vec<RasterizedGlyph> {
    vec![
        spacer_glyph(),
        bitmap_glyph(
            '!',
            4,
            &[
                "##", "##", "##", "##", "##", "##", "##", "##", "##", "##", "  ", "  ", "  ",
                "##", "##", "  ",
            ],
        ),
        bitmap_glyph(
            'H',
            10,
            &[
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "########",
                "########",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
            ],
        ),
        bitmap_glyph(
            'W',
            12,
            &[
                "##      ##",
                "##      ##",
                "##      ##",
                "##      ##",
                "##      ##",
                "##  ##  ##",
                "##  ##  ##",
                "## #### ##",
                "####  ####",
                "####  ####",
                "###    ###",
                "###    ###",
                "##      ##",
                "##      ##",
                "##      ##",
                "##      ##",
            ],
        ),
        bitmap_glyph(
            'd',
            10,
            &[
                "      ##",
                "      ##",
                "      ##",
                "      ##",
                "  ######",
                " #######",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                " #######",
                "  ######",
                "      ##",
                "      ##",
            ],
        ),
        bitmap_glyph(
            'e',
            10,
            &[
                "        ",
                "        ",
                "  ####  ",
                " ###### ",
                "##    ##",
                "##      ",
                "########",
                "########",
                "##      ",
                "##      ",
                "##      ",
                "##    ##",
                " ###### ",
                "  ####  ",
                "        ",
                "        ",
            ],
        ),
        bitmap_glyph(
            'l',
            6,
            &[
                " ## ", " ## ", " ## ", " ## ", " ## ", " ## ", " ## ", " ## ", " ## ", " ## ",
                " ## ", " ## ", " ## ", " ## ", "####", "####",
            ],
        ),
        bitmap_glyph(
            'o',
            10,
            &[
                "        ",
                "        ",
                "  ####  ",
                " ###### ",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                "##    ##",
                " ###### ",
                "  ####  ",
                "        ",
                "        ",
            ],
        ),
        bitmap_glyph(
            'r',
            8,
            &[
                "      ",
                "      ",
                "## ## ",
                "######",
                "###   ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "##    ",
                "      ",
                "      ",
            ],
        ),
    ]
}

fn spacer_glyph() -> RasterizedGlyph {
    RasterizedGlyph {
        glyph_id: u32::from(' '),
        width: 0,
        height: 0,
        bearing_x: 0,
        bearing_y: 0,
        advance_x: 4,
        pixels: Vec::new(),
    }
}

fn bitmap_glyph(character: char, advance_x: i32, rows: &[&str]) -> RasterizedGlyph {
    let width = rows.first().map(|row| row.len()).unwrap_or_default();
    let mut pixels = Vec::with_capacity(width * rows.len());

    for row in rows {
        assert_eq!(row.len(), width, "bitmap rows must have a stable width");
        for pixel in row.bytes() {
            pixels.push(if pixel == b'#' { u8::MAX } else { 0 });
        }
    }

    RasterizedGlyph {
        glyph_id: u32::from(character),
        width: width as u16,
        height: rows.len() as u16,
        bearing_x: 0,
        bearing_y: 0,
        advance_x,
        pixels,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_deterministic_hello_world_text_package() {
        let first = hello_world_text_package().expect("first package should compile");
        let second = hello_world_text_package().expect("second package should compile");

        assert_eq!(first.approved_strings.len(), 1);
        assert_eq!(first.approved_strings[0].value, HELLO_WORLD_TEXT);
        assert_eq!(first.runs[0].id, HELLO_WORLD_RUN_ID);
        assert_eq!(first.runs[0].advance_width(), 100);
        assert_eq!(first.evidence.package_sha256, second.evidence.package_sha256);
        assert_eq!(first.atlases[0].pixels, second.atlases[0].pixels);
    }

    #[test]
    fn emits_fixed_draw_commands_for_hello_world() {
        let layout = hello_world_text_layout(10, 20).expect("layout should compile and render");

        assert_eq!(layout.commands.len(), HELLO_WORLD_DRAW_COMMAND_COUNT);

        let glyph_ids: Vec<u32> = layout.commands.iter().map(|command| command.glyph_id).collect();
        let x_positions: Vec<i32> = layout.commands.iter().map(|command| command.x).collect();
        let widths: Vec<u16> = layout.commands.iter().map(|command| command.width).collect();

        assert_eq!(
            glyph_ids,
            vec![
                u32::from('H'),
                u32::from('e'),
                u32::from('l'),
                u32::from('l'),
                u32::from('o'),
                u32::from('W'),
                u32::from('o'),
                u32::from('r'),
                u32::from('l'),
                u32::from('d'),
                u32::from('!'),
            ]
        );
        assert_eq!(x_positions, vec![10, 20, 30, 36, 42, 56, 68, 78, 86, 92, 106]);
        assert_eq!(widths, vec![8, 8, 4, 4, 8, 10, 8, 6, 4, 8, 2]);
        assert!(layout.commands.iter().all(|command| command.y == 20));
    }

    #[test]
    fn convenience_command_helper_matches_layout_commands() {
        let layout = hello_world_text_layout(0, 0).expect("layout should render");
        let commands = hello_world_glyph_draw_commands(0, 0).expect("command helper should render");

        assert_eq!(commands, layout.commands);
        assert_eq!(layout.package.runs[0].glyphs.len(), HELLO_WORLD_GLYPHS.len());
    }
}
