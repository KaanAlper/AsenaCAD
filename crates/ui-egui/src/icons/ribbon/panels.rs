//! Annotation, Layers, Block, Groups, Utilities, Clipboard and View panel tools.

use egui::Color32;

use super::{GOLD, GREEN, ICE, YELLOW};
use crate::icons::{Icon, Pen};

pub(super) fn paint(pen: &Pen, icon: Icon) {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        TextToFront | DimsToFront | LeadersToFront | AnnotationsToFront => {
            pen.fill_rect(2.0, 2.0, 12.0, 12.0, b.gamma_multiply(0.25));
            pen.rect(2.0, 2.0, 12.0, 12.0, b);
            match icon {
                TextToFront => pen.text(12.5, 13.0, "A", 14.0, a),
                DimsToFront => dim(pen, 6.0, 18.0, 13.0, 9.0, 19.0),
                LeadersToFront => leader(pen, 6.0, 19.0),
                _ => {
                    pen.text(11.0, 10.5, "A", 10.0, a);
                    dim(pen, 6.0, 18.0, 18.5, 15.0, 21.0);
                }
            }
            pen.order_arrow(true, true);
        }
        HatchToBack => {
            pen.rect(2.0, 2.0, 10.0, 10.0, a);
            pen.la(2.0, 7.0, 7.0, 2.0);
            pen.la(2.0, 12.0, 12.0, 2.0);
            pen.la(7.0, 12.0, 12.0, 7.0);
            pen.knock(7.0, 7.0, 10.0, 10.0);
            pen.fill_rect(7.0, 7.0, 10.0, 10.0, b.gamma_multiply(0.3));
            pen.rect(7.0, 7.0, 10.0, 10.0, b);
            pen.order_arrow(false, true);
        }
        Dimension => {
            dim(pen, 3.0, 19.0, 7.0, 3.0, 13.0);
            pen.pointer(13.0, 11.0);
        }
        DimJogged => {
            pen.arc(3.0, 21.0, 15.0, 0.0, 90.0, b);
            pen.poly(&[(5.0, 19.0), (8.5, 15.5), (7.0, 13.5)], false, a);
            pen.arrow(7.0, 13.5, 13.6, 10.4, a);
            pen.dot(5.0, 19.0);
        }
        CollectLeaders => {
            pen.la(2.5, 21.0, 7.0, 14.0);
            pen.la(7.0, 14.0, 8.5, 14.0);
            pen.arrow_tip(2.5, 21.0, -0.55, 1.0, a);
            for x in [10.8, 15.4, 20.0] {
                pen.c(x, 14.0, 2.3, b);
            }
            pen.c(15.4, 9.4, 2.3, b);
        }
        TextStyle => {
            pen.text(8.0, 9.5, "A", 15.0, b);
            pen.brush();
        }
        DimStyle => {
            dim(pen, 2.0, 15.0, 6.0, 2.0, 12.0);
            pen.brush();
        }
        MLeaderStyle => leader_style(pen),
        TableStyle => {
            pen.rect(2.0, 2.0, 14.0, 11.0, b);
            pen.fill_rect(2.0, 2.0, 14.0, 3.5, a.gamma_multiply(0.6));
            pen.l(2.0, 9.0, 16.0, 9.0);
            pen.l(9.0, 5.5, 9.0, 13.0);
            pen.brush();
        }
        LayerOff | LayerIso | LayerFreeze | LayerLock | LayerMatch | MakeCurrent | LayerPrev | LayerOn | LayerUniso | LayerThaw | LayerUnlock
        | LayerCur | CopyToLayer | LayerWalk | LayerVpFreeze | LayerMerge | LayerDelete | LockedLayerFade => layer(pen, icon),
        BlockCreate | Insert | BlockEdit | AttDef | DetectBlocks | ConvertBlocks | BlockReplace | BasePoint => block(pen, icon),
        AttEdit | AttEditMultiple | AttManager | AttSync => attribute(pen, icon),
        Group | Ungroup | GroupEdit | GroupSelect | GroupManager | GroupBox => group(pen, icon),
        MeasureRadius => {
            pen.c(11.0, 13.0, 8.5, b);
            pen.arrow(11.0, 13.0, 17.0, 7.0, a);
            pen.dot(11.0, 13.0);
        }
        MeasureAngle => {
            pen.l(3.0, 20.0, 21.0, 20.0);
            pen.l(3.0, 20.0, 15.0, 6.0);
            pen.arc(3.0, 20.0, 12.0, 8.0, 41.0, a);
            pen.arrow_tip(15.0, 20.0, 0.0, 1.0, a);
            pen.arrow_tip(10.9, 10.9, -0.75, -0.66, a);
            pen.dot(3.0, 20.0);
        }
        Volume => {
            let hex = [(12.0, 3.0), (20.0, 7.0), (20.0, 16.0), (12.0, 20.0), (4.0, 16.0), (4.0, 7.0)];
            pen.fill_poly(&hex, a.gamma_multiply(0.3));
            pen.poly(&hex, true, a);
            pen.poly(&[(4.0, 7.0), (12.0, 11.0), (20.0, 7.0)], false, a);
            pen.la(12.0, 11.0, 12.0, 20.0);
        }
        SelectAll => {
            pen.dash_rect(2.0, 2.0, 20.0, 20.0, b);
            pen.c(8.0, 8.0, 3.0, a);
            pen.rect(13.0, 5.0, 6.0, 6.0, a);
            pen.la(5.0, 18.5, 19.0, 14.0);
        }
        QuickCalc => {
            pen.rect(5.0, 2.0, 14.0, 20.0, b);
            pen.fill_rect(7.0, 4.0, 10.0, 4.0, a);
            for i in 0..3 {
                for j in 0..3 {
                    pen.fill_rect(7.0 + i as f32 * 3.7, 10.5 + j as f32 * 3.7, 2.4, 2.4, b);
                }
            }
        }
        IdPoint => {
            pen.l(7.0, 13.0, 7.0, 22.0);
            pen.l(2.5, 17.5, 11.5, 17.5);
            pen.l(7.0, 17.5, 13.0, 10.0);
            pen.rect(12.0, 3.0, 10.0, 7.0, b);
            pen.la(14.0, 5.5, 20.0, 5.5);
            pen.la(14.0, 7.8, 18.0, 7.8);
            pen.dot(7.0, 17.5);
        }
        PointStyle => {
            pen.fill_c(7.0, 7.0, 1.6, b);
            pen.l(17.0, 3.5, 17.0, 10.5);
            pen.l(13.5, 7.0, 20.5, 7.0);
            pen.la(4.5, 14.5, 9.5, 19.5);
            pen.la(4.5, 19.5, 9.5, 14.5);
            pen.c(17.0, 17.0, 3.0, a);
            pen.fill_c(17.0, 17.0, 1.0, a);
        }
        Paste | PasteBlock | PasteHyperlink | PasteOrig | PasteSpecial => paste(pen, icon),
        Cut => {
            pen.c(7.0, 18.0, 3.0, b);
            pen.c(17.0, 18.0, 3.0, b);
            pen.la(8.8, 15.6, 16.5, 2.5);
            pen.la(15.2, 15.6, 7.5, 2.5);
            pen.fill_c(12.0, 9.8, 1.0, b);
        }
        CopyClip => {
            sheet(pen, 3.0, 2.0, b);
            sheet(pen, 10.0, 8.0, a);
        }
        ViewBase => {
            let hex = [(7.0, 2.0), (12.0, 4.5), (12.0, 10.0), (7.0, 12.5), (2.0, 10.0), (2.0, 4.5)];
            pen.poly(&hex, true, b);
            pen.poly(&[(2.0, 4.5), (7.0, 7.0), (12.0, 4.5)], false, b);
            pen.l(7.0, 7.0, 7.0, 12.5);
            pen.arrow(11.0, 13.0, 14.5, 16.5, a);
            pen.rect(15.0, 15.0, 7.0, 7.0, a);
        }
        SectionViewStyle => {
            pen.rect(2.0, 3.0, 14.0, 12.0, b);
            pen.la(2.0, 9.0, 8.0, 3.0);
            pen.la(2.0, 15.0, 14.0, 3.0);
            pen.la(8.0, 15.0, 16.0, 7.0);
            pen.la(14.0, 15.0, 16.0, 13.0);
            pen.brush();
        }
        DetailViewStyle => {
            pen.c(9.5, 9.5, 7.5, b);
            pen.arc(10.0, 10.0, 4.0, 90.0, 180.0, a);
            pen.la(6.0, 10.0, 6.0, 15.0);
            pen.la(10.0, 6.0, 15.0, 6.0);
            pen.brush();
        }
        DimLinear | DimAligned | DimBaseline | DimContinue | QDim => {
            let (y0, y1) = (8.0, 18.0);
            pen.l(4.0, y1, 4.0, 5.0);
            pen.l(20.0, y1, 20.0, 5.0);
            pen.la(4.0, y0, 20.0, y0);
            pen.arrow_tip(4.0, y0, -1.0, 0.0, a);
            pen.arrow_tip(20.0, y0, 1.0, 0.0, a);
            match icon {
                DimBaseline => pen.la(4.0, 4.0, 13.0, 4.0),
                DimContinue => {
                    pen.l(12.0, y1, 12.0, 5.0);
                }
                QDim => pen.text(12.0, 15.0, "Q", 7.0, b),
                DimAligned => {}
                _ => pen.text(12.0, 14.5, "1.5", 6.0, b),
            }
        }
        DimRadius => {
            pen.arc(12.0, 14.0, 8.0, 0.0, 180.0, b);
            pen.la(12.0, 14.0, 17.6, 8.4);
            pen.arrow_tip(17.6, 8.4, 1.0, -1.0, a);
            pen.dot(12.0, 14.0);
        }
        DimDiameter => {
            pen.c(12.0, 12.0, 8.0, b);
            pen.la(6.4, 17.6, 17.6, 6.4);
            pen.arrow_tip(6.4, 17.6, -1.0, 1.0, a);
            pen.arrow_tip(17.6, 6.4, 1.0, -1.0, a);
        }
        DimAngular => {
            pen.l(4.0, 20.0, 20.0, 20.0);
            pen.l(4.0, 20.0, 16.0, 6.0);
            pen.arc(4.0, 20.0, 11.0, 0.0, 49.0, a);
        }
        DimArc => {
            pen.arc(12.0, 18.0, 8.0, 20.0, 160.0, b);
            pen.arc(12.0, 18.0, 12.0, 25.0, 155.0, a);
        }
        DimOrdinate => {
            pen.l(4.0, 20.0, 4.0, 4.0);
            pen.l(4.0, 20.0, 20.0, 20.0);
            pen.la(10.0, 20.0, 10.0, 8.0);
            pen.la(16.0, 20.0, 16.0, 11.0);
        }
        MLeader | AddLeader | RemoveLeader | AlignLeaders => {
            pen.la(3.0, 20.0, 10.0, 9.0);
            pen.la(10.0, 9.0, 14.0, 9.0);
            pen.arrow_tip(3.0, 20.0, -0.55, 1.0, a);
            pen.l(15.0, 6.0, 22.0, 6.0);
            pen.l(15.0, 10.0, 21.0, 10.0);
            match icon {
                AddLeader => pen.text(19.0, 18.0, "+", 9.0, pen.pt),
                RemoveLeader => pen.text(19.0, 18.0, "−", 9.0, pen.pt),
                _ => {}
            }
        }
        Launcher => {
            pen.l(6.0, 6.0, 17.0, 17.0);
            pen.poly(&[(17.0, 10.0), (17.0, 17.0), (10.0, 17.0)], false, b);
        }
        Pin => {
            pen.rect(8.0, 3.0, 8.0, 9.0, b);
            pen.l(5.0, 12.0, 19.0, 12.0);
            pen.l(12.0, 12.0, 12.0, 21.0);
        }
        Unpin => {
            pen.rect(3.0, 8.0, 9.0, 8.0, b);
            pen.l(12.0, 5.0, 12.0, 19.0);
            pen.l(12.0, 12.0, 21.0, 12.0);
        }
        _ => {}
    }
}

/// A small linear dimension: extension lines from `y1` up to `y0`, dimension line at `dy`.
fn dim(pen: &Pen, x0: f32, x1: f32, dy: f32, y0: f32, y1: f32) {
    let a = pen.acc;
    pen.la(x0, y1, x0, y0);
    pen.la(x1, y1, x1, y0);
    pen.la(x0 + 2.0, dy, x1 - 2.0, dy);
    pen.arrow_tip(x0, dy, -1.0, 0.0, a);
    pen.arrow_tip(x1, dy, 1.0, 0.0, a);
}

/// A small leader with its arrow at (x, y) and a landing to the upper right.
fn leader(pen: &Pen, x: f32, y: f32) {
    let a = pen.acc;
    pen.la(x, y, x + 6.0, y - 8.0);
    pen.la(x + 6.0, y - 8.0, x + 11.0, y - 8.0);
    pen.arrow_tip(x, y, -0.6, 0.8, a);
}

fn leader_style(pen: &Pen) {
    leader(pen, 2.0, 14.0);
    pen.l(14.0, 4.0, 18.0, 4.0);
    pen.l(14.0, 7.0, 17.0, 7.0);
    pen.brush();
}

fn layer(pen: &Pen, icon: Icon) {
    use Icon::*;
    let (b, a) = (pen.base, pen.acc);
    let (x, y) = (18.0, 17.0);
    pen.layers(if icon == LockedLayerFade { b.gamma_multiply(0.45) } else { b });
    match icon {
        LayerOff => pen.fill_c(x, y, 3.5, Color32::from_rgb(0x8a, 0x8a, 0x5a)),
        LayerIso => pen.fill_c(x, y, 3.5, a),
        LayerFreeze => pen.text(x, y, "❄", 9.0, ICE),
        LayerLock => pen.rect(x - 3.0, y - 1.0, 6.0, 5.0, GOLD),
        LayerMatch => pen.la(x - 4.0, y, x + 4.0, y),
        MakeCurrent => pen.poly(&[(x - 3.5, y), (x - 1.0, y + 3.0), (x + 4.0, y - 3.0)], false, GREEN),
        LayerPrev => pen.arc(x, y, 3.5, 90.0, 360.0, a),
        LayerOn => {
            pen.fill_c(x, y - 0.5, 3.8, pen.tint(YELLOW));
            pen.fill_rect(x - 1.5, y + 3.0, 3.0, 2.0, b);
        }
        LayerUniso => {
            pen.c(x, y, 3.2, a);
            for (dx, dy) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)] {
                pen.la(x + dx * 4.3, y + dy * 4.3, x + dx * 5.8, y + dy * 5.8);
            }
        }
        LayerThaw => {
            let c = pen.tint(YELLOW);
            pen.fill_c(x, y, 2.4, c);
            for i in 0..8 {
                let t = (i as f32 * 45.0).to_radians();
                pen.poly(&[(x + 3.6 * t.cos(), y + 3.6 * t.sin()), (x + 5.4 * t.cos(), y + 5.4 * t.sin())], false, c);
            }
        }
        LayerUnlock => {
            let c = pen.tint(GOLD);
            pen.rect(x - 3.0, y, 6.0, 5.0, c);
            pen.arc(x + 2.5, y - 1.5, 2.2, 0.0, 180.0, c);
            pen.poly(&[(x + 4.7, y - 1.5), (x + 4.7, y)], false, c);
        }
        LayerCur => {
            pen.rect(x - 1.0, y + 0.5, 5.0, 5.0, a);
            pen.arrow(x - 1.0, y + 0.5, x - 6.0, y - 4.5, a);
        }
        CopyToLayer => {
            pen.rect(x - 4.5, y - 4.0, 5.5, 5.5, b);
            pen.knock(x - 1.5, y - 1.0, 6.5, 6.5);
            pen.rect(x - 1.5, y - 1.0, 5.5, 5.5, a);
        }
        LayerWalk => {
            pen.la(x + 2.0, y - 2.5, x + 2.0, y + 2.5);
            pen.arrow_tip(x + 2.0, y - 4.5, 0.0, -1.0, a);
            pen.arrow_tip(x + 2.0, y + 4.5, 0.0, 1.0, a);
        }
        LayerVpFreeze => {
            pen.knock(x - 5.0, y - 3.5, 10.0, 8.0);
            pen.rect(x - 5.0, y - 3.5, 10.0, 8.0, b);
            let c = pen.tint(ICE);
            for i in 0..3 {
                let t = (i as f32 * 60.0 + 90.0).to_radians();
                pen.poly(&[(x - 2.8 * t.cos(), y + 0.5 - 2.8 * t.sin()), (x + 2.8 * t.cos(), y + 0.5 + 2.8 * t.sin())], false, c);
            }
        }
        LayerMerge => {
            pen.la(x - 4.0, y - 4.0, x, y);
            pen.la(x + 4.0, y - 4.0, x, y);
            pen.arrow(x, y, x, y + 5.5, a);
        }
        LayerDelete => {
            pen.la(x - 3.0, y - 3.0, x + 3.0, y + 3.0);
            pen.la(x + 3.0, y - 3.0, x - 3.0, y + 3.0);
        }
        LockedLayerFade => {
            let c = pen.tint(GOLD);
            pen.fill_rect(x - 3.0, y, 6.0, 5.0, c);
            pen.arc(x, y, 2.2, 0.0, 180.0, c);
        }
        _ => {}
    }
}

fn block(pen: &Pen, icon: Icon) {
    use Icon::*;
    let (b, a) = (pen.base, pen.acc);
    if icon == BlockReplace {
        pen.block(1.0, 1.0, 0.6, b);
        pen.block(12.0, 12.0, 0.6, a);
        pen.arc(16.0, 8.0, 5.0, 160.0, 10.0, a);
        pen.arrow_tip(20.9, 9.8, 0.3, 1.0, a);
        pen.arc(8.0, 16.0, 5.0, 340.0, 190.0, b);
        pen.arrow_tip(3.1, 14.2, -0.3, -1.0, b);
        return;
    }
    pen.block(1.0, 4.0, 1.0, b);
    let (x, y) = (18.0, 17.5);
    if matches!(icon, DetectBlocks | ConvertBlocks | BasePoint) {
        pen.knock(x - 5.0, y - 4.5, 11.0, 11.0);
    }
    match icon {
        BlockCreate => pen.text(19.0, 5.0, "✳", 9.0, pen.pt),
        Insert => {
            pen.la(14.0, 20.0, 22.0, 20.0);
            pen.arrow_tip(13.0, 20.0, -1.0, 0.0, a);
        }
        BlockEdit => pen.la(14.0, 22.0, 22.0, 14.0),
        AttDef => pen.text(19.0, 19.0, "A", 9.0, a),
        DetectBlocks => {
            pen.c(x - 0.5, y - 0.5, 3.6, a);
            pen.thick(x + 2.2, y + 2.2, x + 5.0, y + 5.0, 1.8, a);
        }
        ConvertBlocks => {
            pen.arc(x, y, 3.8, 20.0, 160.0, a);
            pen.arrow_tip(x - 3.8, y + 0.8, -0.2, 1.0, a);
            pen.arc(x, y, 3.8, 200.0, 340.0, a);
            pen.arrow_tip(x + 3.8, y - 0.8, 0.2, -1.0, a);
        }
        BasePoint => {
            pen.l(x, y - 4.0, x, y + 4.0);
            pen.l(x - 4.0, y, x + 4.0, y);
            pen.fill_c(x, y, 2.0, pen.pt);
        }
        _ => {}
    }
}

/// The attribute motif: a value field with an "A" over a block; badges at the bottom right.
fn attribute(pen: &Pen, icon: Icon) {
    use Icon::*;
    let (b, a) = (pen.base, pen.acc);
    match icon {
        AttEditMultiple => {
            pen.rect(5.0, 7.0, 13.0, 8.0, b);
            pen.knock(2.0, 3.0, 13.0, 8.0);
            pen.rect(2.0, 3.0, 13.0, 8.0, b);
            pen.text(8.5, 7.2, "A", 8.5, a);
        }
        AttManager => {
            pen.rect(2.0, 2.0, 20.0, 20.0, b);
            pen.fill_rect(2.0, 2.0, 20.0, 4.5, b.gamma_multiply(0.5));
            for y in [10.0, 14.0, 18.0] {
                pen.text(6.0, y, "A", 5.5, a);
                pen.l(9.0, y, 19.0, y);
            }
        }
        _ => {
            pen.rect(2.0, 3.0, 15.0, 10.0, b);
            pen.text(9.5, 8.2, "A", 10.0, a);
        }
    }
    match icon {
        AttEdit | AttEditMultiple => pen.pencil(),
        AttSync => {
            let (x, y) = (18.0, 17.5);
            pen.knock(x - 5.0, y - 5.0, 11.0, 11.0);
            pen.arc(x, y, 3.8, 20.0, 160.0, a);
            pen.arrow_tip(x - 3.8, y + 0.8, -0.2, 1.0, a);
            pen.arc(x, y, 3.8, 200.0, 340.0, a);
            pen.arrow_tip(x + 3.8, y - 0.8, 0.2, -1.0, a);
        }
        _ => {}
    }
}

fn group(pen: &Pen, icon: Icon) {
    use Icon::*;
    let (b, a) = (pen.base, pen.acc);
    match icon {
        Ungroup => {
            pen.c(6.0, 8.0, 3.5, b);
            pen.dash_rect(1.5, 3.5, 9.0, 9.0, b);
            pen.rect(14.0, 13.0, 7.0, 7.0, b);
            pen.dash_rect(12.5, 11.5, 10.0, 10.0, b);
            pen.arrow(10.0, 14.0, 6.5, 18.5, a);
            pen.arrow(14.0, 10.0, 18.5, 5.5, a);
        }
        GroupBox => {
            pen.c(7.5, 9.0, 3.5, b);
            pen.rect(10.5, 10.0, 7.0, 7.0, b);
            pen.rect(2.0, 3.0, 18.0, 16.0, a);
            for (x, y) in [(2.0, 3.0), (20.0, 3.0), (2.0, 19.0), (20.0, 19.0)] {
                pen.fill_rect(x - 1.5, y - 1.5, 3.0, 3.0, a);
            }
        }
        GroupManager => {
            pen.rect(2.0, 2.0, 20.0, 20.0, b);
            pen.fill_rect(2.0, 2.0, 20.0, 4.5, b.gamma_multiply(0.5));
            pen.dash_rect(5.0, 9.0, 8.0, 5.5, a);
            pen.dash_rect(5.0, 16.0, 8.0, 3.5, a);
            pen.l(15.0, 11.75, 19.0, 11.75);
            pen.l(15.0, 17.75, 19.0, 17.75);
        }
        _ => {
            pen.group(a);
            match icon {
                GroupEdit => pen.pencil(),
                GroupSelect => pen.pointer(14.0, 12.5),
                _ => {}
            }
        }
    }
}

/// A sheet with a folded corner and text lines, top-left at (x, y).
fn sheet(pen: &Pen, x: f32, y: f32, c: Color32) {
    pen.knock(x, y, 11.0, 14.0);
    pen.poly(&[(x, y), (x + 7.5, y), (x + 11.0, y + 3.5), (x + 11.0, y + 14.0), (x, y + 14.0)], true, c);
    pen.poly(&[(x + 7.5, y), (x + 7.5, y + 3.5), (x + 11.0, y + 3.5)], false, c);
    for k in 0..3 {
        let ly = y + 6.5 + k as f32 * 2.5;
        pen.l(x + 2.5, ly, x + 8.5, ly);
    }
}

fn paste(pen: &Pen, icon: Icon) {
    use Icon::*;
    let a = pen.acc;
    pen.clipboard();
    match icon {
        Paste | PasteSpecial => {
            pen.knock(10.0, 9.0, 12.0, 14.0);
            pen.rect(11.0, 10.0, 10.0, 12.0, a);
            if icon == PasteSpecial {
                pen.text(16.0, 16.0, "✳", 9.0, pen.pt);
            }
        }
        PasteBlock => {
            pen.knock(10.0, 10.0, 14.0, 13.0);
            pen.block(11.0, 11.0, 0.75, a);
        }
        PasteHyperlink => {
            pen.knock(10.0, 10.0, 14.0, 13.0);
            for (cx, cy) in [(14.5, 19.0), (19.0, 14.5)] {
                pen.ell(cx, cy, 3.8, 2.0, 45.0, 0.0, 360.0, a);
            }
        }
        _ => {
            pen.knock(10.0, 9.0, 14.0, 14.0);
            pen.arrow(12.5, 21.0, 22.5, 21.0, a);
            pen.arrow(12.5, 21.0, 12.5, 11.0, a);
            pen.dot(12.5, 21.0);
        }
    }
}
