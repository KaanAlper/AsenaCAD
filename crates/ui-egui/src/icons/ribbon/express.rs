//! Express Tools tab icons: block, text, modify, layout, draw, dimension, tools and web tools.
//! Shared motifs: the block box for the block tools, a large letter for the text tools, two
//! frames for the layout tools, a globe for the hyperlink tools.

use egui::Color32;

use super::GREEN;
use super::panels::dim;
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// A leader from its arrow at (x0, y0) to (x1, y1).
    fn lead(&self, x0: f32, y0: f32, x1: f32, y1: f32, c: Color32) {
        self.poly(&[(x0, y0), (x1, y1)], false, c);
        self.arrow_tip(x0, y0, x0 - x1, y0 - y1, c);
    }
    /// A small text box (an mtext's frame with two lines), top-left at (x, y).
    fn note(&self, x: f32, y: f32, w: f32) {
        self.rect(x, y, w, 7.0, self.base);
        self.poly(&[(x + 1.5, y + 2.5), (x + w - 1.5, y + 2.5)], false, self.base);
        self.poly(&[(x + 1.5, y + 4.8), (x + w - 3.5, y + 4.8)], false, self.base);
    }
    /// A circular arrow around (x, y) from `a0` to `a1` degrees (counter-clockwise when `a1` is
    /// larger), its tip at `a1`.
    fn turn(&self, x: f32, y: f32, r: f32, a0: f32, a1: f32, c: Color32) {
        self.arc(x, y, r, a0, a1, c);
        let t = a1.to_radians();
        let k = if a1 >= a0 { 1.0 } else { -1.0 };
        self.arrow_tip(x + r * t.cos(), y - r * t.sin(), -k * t.sin(), -k * t.cos(), c);
    }
}

/// Express Tools tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    blocks_text(pen, icon) || modify_layout(pen, icon) || dims_tools(pen, icon)
}

fn blocks_text(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        ExplodeAttributes => {
            pen.block(5.0, 6.0, 0.8, b);
            for (x, y) in [(2.0, 3.0), (22.0, 3.0), (2.0, 22.0), (22.0, 22.0)] {
                pen.arrow(12.0 + (x - 12.0) * 0.45, 12.0 + (y - 12.0) * 0.45, x, y, a);
            }
        }
        ListProperties => {
            pen.block(1.0, 4.0, 0.7, b);
            for y in [6.0, 11.0, 16.0] {
                pen.fill_rect(14.0, y - 1.0, 2.0, 2.0, a);
                pen.la(17.5, y, 23.0, y);
            }
        }
        ImportAttributes | ExportAttributes => {
            pen.grid(9.0, 4.0, 13.0, 16.0, 2, 4, b);
            if icon == ImportAttributes {
                pen.arrow(1.0, 12.0, 8.0, 12.0, a);
            } else {
                pen.knock(13.0, 9.0, 11.0, 7.0);
                pen.arrow(12.0, 12.0, 23.0, 12.0, a);
            }
        }
        BlockToXref => {
            pen.block(1.0, 2.0, 0.7, b);
            pen.arrow(6.0, 15.0, 12.0, 15.0, a);
            pen.page(13.0, 9.0, 10.0, 13.0, b);
            pen.chain(18.0, 16.0, a);
        }
        ExtendNested | TrimNested => {
            pen.block(12.0, 5.0, 0.65, b);
            if icon == ExtendNested {
                pen.l(1.0, 13.0, 6.0, 13.0);
                pen.arrow(6.0, 13.0, 12.0, 13.0, a);
            } else {
                pen.l(1.0, 13.0, 12.0, 13.0);
                pen.dash(12.0, 13.0, 22.0, 13.0, pen.pt);
                pen.mark(20.0, 18.5);
            }
        }
        ArcText => {
            pen.arc(12.0, 24.0, 15.0, 40.0, 140.0, a);
            pen.text(4.5, 14.0, "A", 8.0, b);
            pen.text(12.0, 10.5, "B", 8.0, b);
            pen.text(19.5, 14.0, "C", 8.0, b);
        }
        TextExplode => {
            pen.text(12.0, 12.5, "A", 13.0, b);
            for (x0, y0, x1, y1) in [(5.0, 5.0, 2.0, 2.0), (19.0, 5.0, 22.0, 2.0), (5.0, 20.0, 2.0, 23.0), (19.0, 20.0, 22.0, 23.0)] {
                pen.la(x0, y0, x1, y1);
            }
        }
        TextCase => {
            pen.text(7.0, 11.0, "A", 13.0, b);
            pen.text(18.0, 14.0, "a", 11.0, a);
        }
        TextRotate => {
            pen.text(12.0, 12.5, "A", 11.0, b);
            pen.turn(12.0, 12.0, 9.5, 200.0, 70.0, a);
        }
        TextFit => {
            pen.text(12.0, 9.5, "AB", 10.0, b);
            pen.la(2.0, 15.0, 2.0, 22.0);
            pen.la(22.0, 15.0, 22.0, 22.0);
            pen.la(4.0, 18.5, 20.0, 18.5);
            pen.arrow_tip(2.5, 18.5, -1.0, 0.0, a);
            pen.arrow_tip(21.5, 18.5, 1.0, 0.0, a);
        }
        AutoNumber => {
            pen.text(4.5, 12.0, "1", 9.0, b);
            pen.text(12.0, 12.0, "2", 9.0, a);
            pen.text(19.5, 12.0, "3", 9.0, b);
            pen.l(2.0, 19.0, 22.0, 19.0);
        }
        EncloseText => {
            pen.c(12.0, 12.0, 9.5, a);
            pen.text(12.0, 12.5, "A", 11.0, b);
        }
        RemoteText => {
            pen.page(2.0, 2.0, 11.0, 15.0, b);
            pen.arrow(9.0, 19.0, 15.0, 19.0, a);
            pen.text(19.0, 14.0, "A", 11.0, b);
        }
        TextMask | TextUnmask => {
            if icon == TextMask {
                pen.fill_rect(2.0, 6.0, 20.0, 12.0, a.gamma_multiply(0.35));
                pen.rect(2.0, 6.0, 20.0, 12.0, a);
            } else {
                pen.dash_rect(2.0, 6.0, 20.0, 12.0, a);
            }
            pen.text(12.0, 12.5, "Ab", 10.0, b);
        }
        _ => return false,
    }
    true
}

fn modify_layout(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        MoveCopyRotate => {
            pen.rect(2.0, 13.0, 8.0, 8.0, b);
            pen.rect(13.0, 3.0, 8.0, 8.0, a);
            pen.arrow(8.0, 12.0, 13.0, 8.0, pen.pt);
            pen.turn(17.0, 7.0, 6.0, 280.0, 350.0, a);
        }
        StretchMultiple => {
            pen.poly(&[(2.0, 20.0), (7.0, 20.0), (7.0, 13.0), (16.0, 13.0), (16.0, 6.0), (22.0, 6.0)], false, b);
            pen.dash_rect(4.0, 10.0, 6.0, 6.0, a);
            pen.dash_rect(13.0, 3.0, 6.0, 6.0, a);
            pen.arrow(16.0, 16.0, 22.0, 16.0, pen.pt);
        }
        Flatten => {
            pen.block(4.0, 1.0, 1.0, b);
            pen.arrow(12.0, 15.0, 12.0, 20.0, a);
            pen.thick(2.0, 22.0, 22.0, 22.0, 2.0, a);
        }
        ExtendedClip => {
            pen.c(12.0, 12.0, 8.0, b);
            pen.poly(&[(5.0, 4.0), (19.0, 6.0), (17.0, 13.0), (21.0, 20.0), (7.0, 19.0), (9.0, 12.0)], true, pen.pt);
        }
        ExtendedOffset => {
            pen.poly(&[(2.0, 20.0), (7.0, 6.0), (17.0, 6.0), (22.0, 20.0)], false, b);
            pen.poly(&[(6.5, 21.0), (10.0, 11.0), (14.0, 11.0), (17.5, 21.0)], false, a);
        }
        ShapeToBlock => {
            pen.curve([(1.0, 16.0), (3.0, 4.0), (7.0, 20.0), (10.0, 8.0)], b);
            pen.arrow(10.0, 13.0, 14.0, 13.0, a);
            pen.block(14.5, 7.0, 0.55, b);
        }
        AlignSpace => {
            pen.rect(2.0, 2.0, 10.0, 10.0, b);
            pen.rect(12.0, 12.0, 10.0, 10.0, b);
            pen.dash(5.0, 5.0, 19.0, 19.0, a);
            pen.dot(5.0, 5.0);
            pen.dot(19.0, 19.0);
        }
        SyncViewports => {
            pen.rect(2.0, 2.0, 11.0, 9.0, b);
            pen.rect(11.0, 13.0, 11.0, 9.0, b);
            pen.turn(12.0, 12.0, 7.0, 100.0, 170.0, pen.tint(GREEN));
            pen.turn(12.0, 12.0, 7.0, 280.0, 350.0, pen.tint(GREEN));
        }
        MergeLayout => {
            pen.page(2.0, 2.0, 9.0, 11.0, b);
            pen.page(13.0, 2.0, 9.0, 11.0, b);
            pen.arrow(6.5, 14.0, 10.0, 17.0, a);
            pen.arrow(17.5, 14.0, 14.0, 17.0, a);
            pen.rect(5.0, 18.0, 14.0, 5.0, a);
        }
        ViewportScale => {
            pen.rect(2.0, 3.0, 20.0, 15.0, b);
            pen.text(12.0, 10.5, "1:2", 9.0, a);
            pen.l(5.0, 21.0, 19.0, 21.0);
        }
        BreakLine => {
            pen.poly(&[(1.0, 13.0), (8.5, 13.0)], false, b);
            pen.poly(&[(8.5, 13.0), (10.5, 5.0), (13.5, 21.0), (15.5, 13.0)], false, a);
            pen.poly(&[(15.5, 13.0), (23.0, 13.0)], false, b);
        }
        SuperHatch => {
            pen.rect(2.0, 2.0, 20.0, 20.0, b);
            for (i, x) in [4.0, 9.0, 14.0, 19.0].iter().enumerate() {
                for (j, y) in [4.0, 9.0, 14.0, 19.0].iter().enumerate() {
                    if (i + j) % 2 == 0 {
                        pen.fill_rect(*x - 1.5, *y - 1.5, 3.0, 3.0, a);
                    }
                }
            }
        }
        _ => return false,
    }
    true
}

fn dims_tools(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        AttachLeader | AttachLeaders | DetachLeaders => {
            pen.note(11.0, 3.0, 12.0);
            pen.lead(2.0, 21.0, 11.0, 7.0, a);
            match icon {
                AttachLeaders => pen.lead(9.0, 22.0, 13.0, 10.0, a),
                DetachLeaders => pen.mark(15.0, 15.0),
                _ => pen.chain(16.0, 16.0, pen.tint(GREEN)),
            }
        }
        ResetDimText => {
            dim(pen, 3.0, 21.0, 16.0, 13.0, 22.0);
            pen.text(9.0, 8.0, "10", 8.0, b);
            pen.turn(18.0, 7.0, 4.0, 200.0, 90.0, pen.pt);
        }
        DimStyleImport | DimStyleExport => {
            dim(pen, 3.0, 15.0, 6.0, 3.0, 12.0);
            pen.l(3.0, 12.0, 15.0, 12.0);
            if icon == DimStyleImport {
                pen.arrow(23.0, 19.0, 9.0, 19.0, pen.pt);
            } else {
                pen.arrow(6.0, 19.0, 22.0, 19.0, pen.pt);
            }
        }
        CommandAliases => {
            pen.rect(2.0, 3.0, 19.0, 15.0, b);
            pen.text(8.0, 8.5, ">_", 8.0, a);
            pen.pencil();
        }
        AttachXdata | ListXdata => {
            pen.page(2.0, 2.0, 13.0, 19.0, b);
            pen.text(8.5, 9.0, "X", 8.0, a);
            pen.knock(13.0, 11.0, 11.0, 13.0);
            if icon == AttachXdata {
                pen.thick(18.5, 13.0, 18.5, 22.0, 2.0, pen.tint(GREEN));
                pen.thick(14.0, 17.5, 23.0, 17.5, 2.0, pen.tint(GREEN));
            } else {
                for y in [14.0, 17.5, 21.0] {
                    pen.la(15.0, y, 23.0, y);
                }
            }
        }
        SysVarEditor => {
            pen.grid(2.0, 3.0, 20.0, 18.0, 2, 4, b);
            pen.text(7.0, 11.5, "(x)", 6.5, a);
        }
        ExtendedPlan => {
            pen.rect(7.0, 3.0, 15.0, 13.0, b);
            pen.arrow(3.0, 21.0, 3.0, 12.0, a);
            pen.arrow(3.0, 21.0, 12.0, 21.0, a);
            pen.text(14.5, 9.5, "P", 8.0, b);
        }
        MakeLinetype => {
            pen.dash(2.0, 5.0, 22.0, 5.0, b);
            pen.poly(&[(2.0, 12.0), (9.0, 12.0)], false, a);
            pen.dot(12.0, 12.0);
            pen.poly(&[(15.0, 12.0), (22.0, 12.0)], false, a);
            pen.pencil();
        }
        MakeShape => {
            pen.curve([(2.0, 14.0), (5.0, 2.0), (10.0, 20.0), (14.0, 4.0)], b);
            pen.pencil();
        }
        ShowUrls | ChangeUrls | ReplaceUrls => {
            pen.globe(10.0, 10.0, 8.0, b);
            pen.knock(12.0, 12.0, 12.0, 12.0);
            match icon {
                ShowUrls => {
                    pen.c(17.0, 17.0, 3.5, a);
                    pen.thick(19.5, 19.5, 23.0, 23.0, 2.0, a);
                }
                ChangeUrls => pen.pencil(),
                _ => {
                    pen.arrow(13.0, 15.0, 23.0, 15.0, a);
                    pen.arrow(23.0, 20.0, 13.0, 20.0, pen.pt);
                }
            }
        }
        _ => return false,
    }
    true
}
