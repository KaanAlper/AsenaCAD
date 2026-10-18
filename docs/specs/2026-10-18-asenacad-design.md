# AsenaCAD: tasarım

> **Tarih:** 2026-10-18 · **Durum:** onaylandı (brainstorming), uygulama planı bekliyor · **Hedef:** Autodesk AutoCAD (Windows, güncel sürüm), birebir

## 1. Amaç

AsenaCAD, AutoCAD'le **arayüzü, komutları ve çalışma biçimi birebir aynı** olan, açık kaynak,
saf Rust bir CAD uygulamasıdır. Farklılaşma hedeflenmez: AutoCAD bilen biri hiçbir şeyi yeniden
öğrenmeden çalışabilmelidir. AutoCAD'in sunmadığı tek fark, platformlardır (Linux, FreeBSD, web)
ve agent kontrolü (MCP, kontrol kanalı, CLI): bunlar temelden hazır gelir.

**Kapsam dışı (şimdilik):** 3D modelleme, SketchUp tarzı Push/Pull, AutoCAD Architecture
toolset (WALLADD, DOORADD…), Türkçe arayüz. Hepsi M4 sonrasına bırakıldı.

## 2. Temel: CADCraft fork'u

| Karar           | Seçim                                                                                     |
|-----------------|-------------------------------------------------------------------------------------------|
| Taban           | `storytold/cadcraft` `main`'i (MIT/Apache-2.0, ~57 bin satır Rust, 295 komut, pre-alpha)  |
| Repo            | `KaanAlper/AsenaCAD`, **public**, yerelde `~/Mimarlık/AsenaCAD`                           |
| Upstream ilişkisi | **Fork + ayrıl.** `upstream` remote olarak durur; düzenli merge yok, gerektiğinde `cherry-pick` |
| Stack           | CADCraft'ınki: Rust, egui + wgpu, WASM web build; JS/TS yok, Tauri/Electron yok            |

CADCraft'tan aynen korunan kurallar: never-crash (non-test kodda `unwrap`/`expect`/`panic!`
yasak, `unsafe` yasak, `catch_unwind` koruması), clean-room, "her şey bir komut"
(`CommandSpec`), katman kuralı (`cargo xtask layers`: L0 `geom`/`dxf` → … → L5 `ui-egui`/`mcp`),
`cargo xtask ci` geçmeden commit yok.

### Yeniden adlandırma ve marka

- Ürün adı **AsenaCAD**; binary'ler `asenacad`, `asenacad-cli`, `asenacad-web`; app id
  `io.github.kaanalper.asenacad`.
- Kütüphane crate'leri (`crates/geom` … `crates/mcp`, paket adları `cadcraft-geom` …
  `cadcraft-mcp`) **değişmez**: upstream'den cherry-pick'i kolay tutar.
- Çıkarılanlar: `docs/brand/` (ArtCraft ticari markası, MIT değil), About ekranındaki ve macOS
  menüsündeki ArtCraft site/Discord bağlantıları, private `craftrules` reposuna yapılan atıflar.
- Korunanlar: `contributors/` (About ▸ Contributors'ta CADCraft katkıcılarının künyesi),
  isteğe bağlı public `storytold/craft-fonts` build girdisi, kütüphane crate adları
  (`cadcraft-geom` …) ve büyük harfli `CADCRAFT*` tanımlayıcıları (env değişkenleri, DXF xdata
  adları; CADCraft çizimleriyle uyum için).
- Eklenenler: AsenaCAD'in kendi ikonu (kodla/SVG ile, orijinal), `NOTICE` ve `ATTRIBUTION.md`'de
  CADCraft'a kaynak gösterimi (MIT/Apache gereği).
- `craftrules` (private, erişilemez) yerine geçen standartlar `docs/standards/` altına yazılır.

## 3. Agent yapısı

ArtCraft deseni: tek kaynak `AGENTS.md`, yanında yalnızca `@AGENTS.md` içeren `CLAUDE.md` (Claude
Code bu satırla AGENTS.md'yi içe aktarır; symlink Windows checkout'larında çalışmadığı için düz
dosya); konu başlıklı kurallar `.claude/rules/` altında. storytold'un public repolarında skill/agent yok (onların düzeneği private
`craftrules` ve gitignore'daki `plan/` içinde), bu yüzden bunları biz yazarız.

```
AsenaCAD/
├── AGENTS.md                  CADCraft'ınkinden uyarlanır: never-crash, clean-room, komut, katman, gözlem
├── CLAUDE.md                  tek satır: @AGENTS.md
├── crates/<crate>/AGENTS.md   varsa, yanına @AGENTS.md içeren CLAUDE.md
├── .claude/
│   ├── rules/
│   │   ├── parity-method.md   AutoCAD gözlem ve karşılaştırma kuralları
│   │   ├── clean-room.md      neye bakılır, ne asla kopyalanmaz, ne asla commit'lenmez
│   │   └── cherry-pick.md     upstream'den commit alma yöntemi
│   ├── skills/
│   │   ├── autocad-observe/   Windows AutoCAD'de script çalıştır, görüntü + komut çıktısı al
│   │   ├── parity-compare/    aynı script'i AsenaCAD'de çalıştır, karşılaştır, parity satırını güncelle
│   │   ├── add-command/       CommandSpec + prompt makinesi + test + parity kaydı
│   │   └── ui-match/          bir paneli AutoCAD görüntüsünden ölçüp egui'de kur
│   └── agents/
│       ├── parity-checker.md     kod yazmaz; komut/paneli yan yana dener, fark listesi çıkarır
│       └── clean-room-auditor.md commit öncesi Autodesk varlığı/metni sızıntısı denetimi
├── docs/
│   ├── standards/             never-crash, progress-docs (craftrules'un yerine)
│   ├── specs/                 bu doküman
│   └── *-parity.md            CADCraft'tan gelen denklik tabloları, Windows AutoCAD'e göre yeniden ölçülür
└── plan/                      .gitignore'da: STATUS.md, gözlem notları, plan/autocad/screenshots/
```

Genel Rust incelemesi ve build hataları için mevcut `rust-reviewer` ve `rust-build-resolver`
agent'ları kullanılır; bunlar yeniden yazılmaz.

## 4. Denklik yöntemi

Her iş aynı döngüyle yapılır ve sonucu ölçülür:

1. **Gözlem:** Windows'taki AutoCAD'de işlem bir script'le yapılır (`acad.exe /b test.scr`),
   ekran görüntüsü ve komut satırı metni `plan/autocad/` altına kaydedilir.
2. **Uygulama:** aynı script AsenaCAD'de kontrol kanalından (`cmdline.input`) çalıştırılır,
   `ui.screenshot` alınır.
3. **Karşılaştırma:** prompt metinleri, seçenek anahtar kelimeleri, varsayılanlar, sonuç geometrisi
   ve arayüz yerleşimi karşılaştırılır; sonuç ilgili `docs/*-parity.md` satırına işlenir.

### Windows'a erişim: AsenaScale

Referans makine: sahibin Windows PC'si (AsenaScale cihaz adı yerel `plan/autocad/env.sh` içinde,
repoya yazılmaz). AsenaScale host'unda SSH
`exec` ve `mc shot` (ekran görüntüsü) zaten var, ama sadece telefon istemcisi kullanıyor. M0'da
AsenaScale'in Linux CLI'ına iki komut eklenir:

| Komut                                   | Ne yapar                                                    |
|-----------------------------------------|-------------------------------------------------------------|
| `asenascale exec CİHAZ -- KOMUT`        | Windows'ta komut çalıştırır, stdout/stderr/çıkış kodunu döner |
| `asenascale shot CİHAZ [--window ID] -o DOSYA` | Tüm ekranın ya da bir pencerenin PNG'sini kaydeder   |

Bu iş `~/Projects/AsenaScale` reposunda yapılır, AsenaCAD reposuna girmez.

### Clean-room sınırı

- **Aynı olabilir:** davranış, yerleşim, komut adları, prompt dizileri, kısayollar, menü/ribbon
  yapısı, dosya formatı (yayınlanmış DXF referansı, ODA'nın public DWG spesifikasyonu).
- **Asla kopyalanmaz:** Autodesk ikonları, fontları (SHX), hatch desenleri (`.pat`), linetype'ları
  (`.lin`), şablonları, CUI/PGP dosyaları, yardım metinleri; AutoCAD kurulumunun içi açılmaz.
- **Asla commit'lenmez:** AutoCAD ekran görüntüleri ve AutoCAD'in ürettiği dosyalar
  (`plan/` gitignore'da; repo public olduğu için kritik).

## 5. Milestone'lar

| Milestone | İçerik                                                                                       | Bitti sayılması için                                                      |
|-----------|----------------------------------------------------------------------------------------------|---------------------------------------------------------------------------|
| **M0**    | AsenaScale `exec`/`shot`; fork, yeniden adlandırma, marka temizliği; agent dosyaları, skills, agents; CI | `cargo xtask ci` yeşil; `asenacad` açılıyor; AutoCAD'den script'le görüntü alınıyor; repoda ArtCraft markası kalmamış |
| **M1**    | Arayüz denkliği (Windows ribbon düzeni): Application menüsü, Quick Access araç çubuğu, ribbon sekme ve panelleri, komut satırı, durum çubuğu, Properties, Layer Properties Manager, Model/Layout sekmeleri | Her öğe AutoCAD görüntüsüyle yan yana karşılaştırılmış ve `ui-parity.md`'de işaretli |
| **M2**    | Precise drafting: obje snap izleme, dynamic input, TRIM/EXTEND/OFFSET'in bütün seçenekleri    | Her komutun prompt dizisi ve sonucu AutoCAD'le aynı                        |
| **M3**    | Blok editörü (`BEDIT`), xref'ler, plot stilleri (CTB/STB)                                     | CADCraft'ın alpha kapısındaki ilgili satırlar geçer                        |
| **M4**    | DWG: AsenaCAD'in kaydettiği dosya AutoCAD'de kayıpsız açılır                                  | Test DWG seti AutoCAD'de açılır, `AUDIT` hatasız                           |

M1'in önde olmasının nedeni: CADCraft'ın arayüzü AutoCAD **for Mac**'e göre kurulu (menü çubuğu,
Tool Sets); hedef Windows AutoCAD'in ribbon düzeni. Arayüz önce oturursa sonraki her komut doğru
yere yerleşir.

## 6. Test ve kalite

- CADCraft'ın kapısı aynen: `cargo xtask ci` (fmt, clippy `-D warnings`, test, assets, layers, wasm).
- Her çökme düzeltmesi bir regresyon testiyle gelir; düşmanca girdi testleri (`hostile_*`) korunur.
- Her yeni/değişen komut: JSON formu için birim testi + parity kaydı.
- Arayüz işleri: `ui.screenshot` görüntüsü AutoCAD görüntüsüyle karşılaştırılmadan bitmiş sayılmaz.

## 7. Riskler ve açık noktalar

| Risk                                                                 | Önlem                                                                                 |
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------|
| AsenaScale `exec` ile açılan AutoCAD'in görünür masaüstü oturumunda çalışıp çalışmadığı | M0'da ilk doğrulanacak şey; çalışmazsa Windows'ta zamanlanmış görev (`schtasks /it`) ile başlatılır |
| Linux makinenin anahtarının Windows host'ta onaylanması gerekir        | İlk bağlantıda Windows'ta çıkan izin penceresinde kullanıcı *Evet* der (tek seferlik) |
| CADCraft'ın tablo/rakamları Mac AutoCAD'e göre ölçülmüş               | M1 başında parity dokümanları Windows AutoCAD'e göre yeniden ölçülür                  |
| `acadrust` (DWG köprüsü) MPL-2.0                                      | Değiştirilmeden bağımlılık olarak kalır (CADCraft'taki gibi); lisansı `ATTRIBUTION.md`'de |
| Ayrılan fork upstream'in düzeltmelerini kaçırır                       | `cherry-pick.md` kuralı: upstream'in commit'leri ayda bir taranır, işe yarayanlar alınır |
