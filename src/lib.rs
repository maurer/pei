use std::collections::HashSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Hash)]
pub enum Location {
    Beach,
    Dungeon,
    Graveyard,
    Lookout,
    Alleyways,
    AridLake,
    Desert,
    FloodedMine,
    Marshes,
    Pen,
    Arcade,
    BurialChambers,
    Cage,
    Cells,
    Excavation,
    Iceberg,
    Leyline,
    Peninsula,
    Port,
    Springs,
    Canyon,
    Chateau,
    CitySquare,
    Courthouse,
    Gorge,
    Grotto,
    Lighthouse,
    RelicChambers,
    Strand,
    Volcano,
    AncientCity,
    Barrows,
    Channel,
    Conservatory,
    HauntedMansion,
    IvoryTemple,
    Maze,
    SpiderLair,
    SulphurVents,
    ToxicSewer,
    Academy,
    Atoll,
    AshenWood,
    Cemetery,
    Fields,
    JungleValley,
    Mausoleum,
    Phantasmagoria,
    Thicket,
    UndergroundSea,
    Wharf,
    ArachnidNest,
    Bazaar,
    BoneCrypt,
    CoralRuins,
    Dunes,
    Gardens,
    LavaChamber,
    Ramparts,
    Residence,
    Tribunal,
    UndergroundRiver,
    Armoury,
    Courtyard,
    Geode,
    InfestedValley,
    Laboratory,
    MineralPools,
    MudGeyser,
    OvergrownRuin,
    Shore,
    TropicalIsland,
    Arena,
    Estuary,
    MoonTemple,
    Museum,
    Plateau,
    Scriptorium,
    Sepulchre,
    Temple,
    Tower,
    Vault,
    WastePool,
    ArachnidTomb,
    Belfry,
    Bog,
    CursedCrypt,
    Orchard,
    Pier,
    Precinct,
    Shipyard,
    Siege,
    Wasteland,
    Collonade,
    Coves,
    Factory,
    Mesa,
    Lair,
    Pit,
    PrimordialPool,
    Promenade,
    SpiderForest,
    Waterways,
    CastleRuins,
    CrystalOre,
    DefiledCathedral,
    Necropolis,
    OvergrownShrine,
    Racecourse,
    Summit,
    TortureChamber,
    Villa,
    Arsenal,
    Core,
    DesertSpring,
    Ghetto,
    Malformation,
    Park,
    Shrine,
    Terrace,
    AcidLakes,
    Colosseum,
    CrimsonTemple,
    DarkForest,
    Dig,
    Palace,
    Caldera,
    Plaza,
    Basilica,
    Carcass,
    LavaLake,
    Reef,
    SunkenCity,
    VaalPyramid,
    Colonnade,
}

pub use Location::*;

pub fn connections(loc: Location) -> &'static [Location] {
    match loc {
        Colonnade => &[DefiledCathedral, Siege],
        Caldera => &[CrystalOre, Palace],
        VaalPyramid => &[Tribunal, Estuary, Museum],
        Beach => &[AridLake, Desert],
        Dungeon => &[FloodedMine],
        Graveyard => &[Marshes],
        Lookout => &[Alleyways, Pen],
        Alleyways => &[Port, Arcade, Cage],
        AridLake => &[Peninsula, Beach],
        Desert => &[Iceberg, Beach],
        FloodedMine => &[Cells, Excavation, Leyline, Dungeon],
        Marshes => &[BurialChambers, Springs, Graveyard],
        Pen => &[Cage, Lookout],
        Arcade => &[Courthouse, Alleyways],
        BurialChambers => &[RelicChambers, Marshes],
        Cage => &[Courthouse, Pen, CitySquare, Alleyways],
        Cells => &[Grotto, FloodedMine],
        Excavation => &[FloodedMine, Grotto, Gorge],
        Iceberg => &[Strand, Desert],
        Leyline => &[FloodedMine, Gorge, Volcano],
        Peninsula => &[AridLake, Strand, Canyon],
        Port => &[Lighthouse, Alleyways],
        Springs => &[Chateau, Marshes],
        Canyon => &[SpiderLair, Peninsula],
        Chateau => &[RelicChambers, Springs, ToxicSewer],
        CitySquare => &[HauntedMansion, Conservatory, Cage],
        Courthouse => &[Cage, HauntedMansion, Arcade, IvoryTemple],
        Gorge => &[Leyline, SulphurVents, Excavation, Channel],
        Grotto => &[Cells, Channel, Excavation, Maze],
        Lighthouse => &[IvoryTemple, Port],
        RelicChambers => &[Barrows, BurialChambers, Chateau],
        Strand => &[AncientCity, Iceberg, Peninsula],
        Volcano => &[Leyline, SulphurVents],
        AncientCity => &[Fields, Strand],
        Barrows => &[RelicChambers, Cemetery, Mausoleum, AshenWood],
        Channel => &[Grotto, Gorge, Atoll, UndergroundSea],
        Conservatory => &[Academy, CitySquare],
        HauntedMansion => &[Academy, CitySquare, Wharf, Courthouse],
        IvoryTemple => &[Thicket, Wharf, Lighthouse, Courthouse],
        Maze => &[Grotto, UndergroundSea],
        SpiderLair => &[Fields, Canyon, JungleValley, Thicket],
        SulphurVents => &[Volcano, Gorge, Atoll, AshenWood],
        ToxicSewer => &[Chateau, Cemetery, Phantasmagoria],
        Academy => &[Tribunal, Gardens, Conservatory, HauntedMansion],
        Atoll => &[SulphurVents, Channel, Ramparts],
        AshenWood => &[LavaChamber, SulphurVents, Barrows],
        Cemetery => &[ToxicSewer, Barrows, BoneCrypt],
        Fields => &[Residence, AncientCity, SpiderLair],
        JungleValley => &[CoralRuins, ArachnidNest, SpiderLair],
        Mausoleum => &[LavaChamber, Barrows],
        Phantasmagoria => &[BoneCrypt, ToxicSewer, Dunes],
        Thicket => &[ArachnidNest, SpiderLair, IvoryTemple],
        UndergroundSea => &[Maze, Channel, Ramparts, UndergroundRiver],
        Wharf => &[Bazaar, HauntedMansion, IvoryTemple],
        ArachnidNest => &[InfestedValley, JungleValley, Thicket],
        Bazaar => &[Laboratory, Wharf],
        BoneCrypt => &[Cemetery, Phantasmagoria, MudGeyser],
        CoralRuins => &[InfestedValley, JungleValley],
        Dunes => &[Phantasmagoria, MudGeyser, Shore],
        Gardens => &[TropicalIsland, Academy],
        LavaChamber => &[Armoury, AshenWood, Mausoleum, OvergrownRuin],
        Ramparts => &[Atoll, Armoury, UndergroundSea],
        Residence => &[Courtyard, Fields],
        Tribunal => &[VaalPyramid, Academy],
        UndergroundRiver => &[UndergroundSea, Geode, MineralPools],
        Armoury => &[Ramparts, LavaChamber, Vault],
        Courtyard => &[Temple, Arena, Residence],
        Geode => &[Vault, Sepulchre, UndergroundRiver],
        InfestedValley => &[Plateau, ArachnidNest, CoralRuins],
        Laboratory => &[Scriptorium, Museum, Bazaar],
        MineralPools => &[UndergroundRiver, MoonTemple],
        MudGeyser => &[BoneCrypt, WastePool, Dunes],
        OvergrownRuin => &[LavaChamber, WastePool, Vault],
        Shore => &[Dunes, Tower, TropicalIsland],
        TropicalIsland => &[Shore, Estuary, Gardens],
        Arena => &[Courtyard, Pier, Wasteland],
        Estuary => &[Shipyard, TropicalIsland, ArachnidTomb, VaalPyramid],
        MoonTemple => &[MineralPools, Belfry, Temple],
        Museum => &[VaalPyramid, Siege, Laboratory],
        Plateau => &[Wasteland, InfestedValley],
        Scriptorium => &[Siege, Laboratory],
        Sepulchre => &[Geode, Belfry, Orchard],
        Temple => &[MoonTemple, CursedCrypt, Courtyard],
        Tower => &[Precinct, Shore, Shipyard],
        Vault => &[Armoury, OvergrownRuin, Orchard, Geode],
        WastePool => &[OvergrownRuin, Bog, Orchard, MudGeyser],
        ArachnidTomb => &[SpiderForest, Estuary],
        Belfry => &[Sepulchre, Waterways, MoonTemple],
        Bog => &[WastePool, Lair, PrimordialPool],
        CursedCrypt => &[Mesa, Temple],
        Orchard => &[Vault, WastePool, Sepulchre, Waterways],
        Pier => &[Pit, Arena],
        Precinct => &[Promenade, Tower],
        Shipyard => &[Tower, Coves, Estuary],
        Siege => &[Colonnade, Museum, Scriptorium],
        Wasteland => &[Pit, Factory, Arena],
        Collonade => &[DefiledCathedral, Siege],
        Coves => &[CastleRuins, Shipyard],
        Factory => &[Villa, CrystalOre, Wasteland],
        Mesa => &[OvergrownShrine, CursedCrypt],
        Lair => &[Bog, Necropolis, Racecourse],
        Pit => &[OvergrownShrine, Villa, Pier, Wasteland],
        PrimordialPool => &[Bog, Summit],
        Promenade => &[Precinct, Racecourse],
        SpiderForest => &[CastleRuins, ArachnidTomb],
        Waterways => &[Orchard, Belfry, TortureChamber],
        CastleRuins => &[Coves, Ghetto, SpiderForest],
        CrystalOre => &[Caldera, Factory],
        DefiledCathedral => &[Colonnade, Shrine],
        Necropolis => &[Lair, Arsenal],
        OvergrownShrine => &[Mesa, DesertSpring, Pit],
        Racecourse => &[Lair, Promenade, Terrace],
        Summit => &[PrimordialPool, Core],
        TortureChamber => &[Waterways, Malformation],
        Villa => &[Pit, Factory, Park],
        Arsenal => &[Necropolis, Terrace, Colosseum],
        Core => &[Summit, Plaza],
        DesertSpring => &[OvergrownShrine, CrimsonTemple],
        Ghetto => &[CastleRuins, Dig],
        Malformation => &[TortureChamber, DarkForest],
        Park => &[Palace, Villa],
        Shrine => &[AcidLakes, Dig, DefiledCathedral],
        Terrace => &[Racecourse, Arsenal, Colosseum],
        AcidLakes => &[Carcass, Shrine],
        Colosseum => &[Arsenal, Terrace, LavaLake],
        CrimsonTemple => &[DesertSpring, Basilica, Palace],
        DarkForest => &[Malformation, Plaza, Reef],
        Dig => &[Ghetto, Carcass, Shrine],
        Palace => &[Caldera, Park, CrimsonTemple, Basilica],
        Plaza => &[Core, SunkenCity, DarkForest],
        Basilica => &[Palace, CrimsonTemple],
        Carcass => &[AcidLakes, Dig],
        LavaLake => &[Colosseum],
        Reef => &[SunkenCity, DarkForest],
        SunkenCity => &[Plaza, Reef],
    }
}

pub fn contiguous(start: Location, blob: &[Location]) -> bool {
    let mut unconnected_set: HashSet<Location> = blob.iter().cloned().collect();
    let mut connected_set: HashSet<Location> = [start].into_iter().cloned().collect();
    let mut one_step: HashSet<Location> = connections(start).iter().cloned().collect();

    unconnected_set.remove(&start);

    let mut progress = true;
    while !unconnected_set.is_empty() && progress {
        progress = false;
        let new_nodes: Vec<Location> = one_step.intersection(&unconnected_set).cloned().collect();
        for new in new_nodes {
            progress = true;
            connected_set.insert(new);
            unconnected_set.remove(&new);
            one_step.remove(&new);
            for step in connections(new) {
                if !connected_set.contains(step) {
                    one_step.insert(*step);
                }
            }
        }
    }
    unconnected_set.is_empty()
}

pub fn static_elder(elder: Location, guardian: Location, elder_influence: &[Location]) -> bool {
    // An elder section is static+valid if:
    // 1.) There are 30 nodes, so the elder can't take another node.
    // 2.) The elder node and the guardian node are in the elder_influence set.
    // 3.) elder_influence is contiguous
    // 4.) Every node in elder_influence is either:
    //     a.) On the critical path (were it removed, elder and guardian would not be connected)
    //     b.) Not vulnerable to shaper (all connections are elder influenced)
    //     c.) Is elder
    //     d.) Is the guardian

    // Saturated (#1)
    if elder_influence.len() != 30 {
        return false;
    }
    // Contained (#2)
    if !(elder_influence.contains(&elder) && elder_influence.contains(&guardian)) {
        return false;
    }
    // Contiguous (#3)
    if !contiguous(elder, elder_influence) {
        return false;
    }
    // Invulnerable (#4)
    for node in elder_influence {
        // c/d Is elder/guardian, can't die
        if *node == elder || *node == guardian {
            continue;
        }
        // b No shaper connections possible
        if connections(*node)
            .iter()
            .all(|connect| elder_influence.contains(connect))
        {
            continue;
        }
        // a Critical path
        let sub: Vec<Location> = elder_influence
            .iter()
            .cloned()
            .filter(|x| x != node)
            .collect();
        if contiguous(elder, &sub) {
            return false;
        }
    }
    return true;
}

pub struct CriticalIterator {
    guardian: Location,
    elder: Location,
    stack: Vec<&'static [Location]>,
    blob: HashSet<Location>,
}

impl Iterator for CriticalIterator {
    type Item = HashSet<Location>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }
        let out = self.blob.clone();
        self.lateral();
        if self.stack.len() != 0 {
            self.walk();
        }
        return Some(out);
    }
}

fn crit_incompat(blob: &HashSet<Location>, possible: Location) -> bool {
    if blob.contains(&possible) {
        return true;
    }
    let mut conns = 0;
    for adj in connections(possible) {
        if blob.contains(adj) {
            conns += 1;
        }
    }
    conns != 1
}

impl CriticalIterator {
    pub fn new(elder: Location, guardian: Location) -> Self {
        let mut pre = CriticalIterator {
            guardian: guardian,
            elder: elder,
            stack: Vec::new(),
            blob: [elder].iter().cloned().collect(),
        };
        pre.walk();
        pre
    }
    fn forward(&mut self) -> bool {
        assert!(self.head() != self.guardian);
        let mut conns = connections(self.head());
        while conns.len() != 0 {
            if crit_incompat(&self.blob, conns[0]) {
                conns = &conns[1..];
            } else {
                self.stack.push(conns);
                self.blob.insert(conns[0]);
                return true;
            }
        }
        return false;
    }
    fn lateral(&mut self) {
        let mut conns = self.stack.pop().unwrap();
        self.blob.remove(&conns[0]);
        conns = &conns[1..];
        while conns.len() != 0 {
            if crit_incompat(&self.blob, conns[0]) {
                conns = &conns[1..];
            } else {
                self.stack.push(conns);
                self.blob.insert(conns[0]);
                return;
            }
        }
        if self.stack.len() > 0 {
            self.lateral();
        }
    }
    fn walk(&mut self) {
        while self.head() != self.guardian {
            if self.stack.len() < 28 {
                if !self.forward() {
                    self.lateral()
                }
            } else {
                self.lateral()
            }
            if self.stack.len() == 0 {
                return;
            }
        }
    }
    fn head(&self) -> Location {
        if self.stack.len() == 0 {
            self.elder
        } else {
            self.stack[self.stack.len() - 1][0]
        }
    }
}

struct FillOutIterator {
    blob: HashSet<Location>,
    stack: Vec<Vec<Location>>,
    purge: Vec<Location>,
}

fn blob_out(blob: &HashSet<Location>) -> Vec<Location> {
    let mut out = HashSet::new();
    for loc in blob {
        for out_node in connections(*loc) {
            if !blob.contains(out_node) {
                out.insert(*out_node);
            }
        }
    }
    out.into_iter().collect()
}

impl FillOutIterator {
    pub fn new(critical: HashSet<Location>) -> Self {
        let mut pre = FillOutIterator {
            blob: critical,
            stack: Vec::new(),
            purge: Vec::new(),
        };
        pre.walk();
        pre
    }
    fn forward(&mut self) -> bool {
        let mut conns = blob_out(&self.blob);
        while conns.len() != 0 {
            let next = conns.pop().unwrap();
            if !self.blob.contains(&next) {
                self.stack.push(conns);
                self.blob.insert(next);
                self.purge.push(next);
                return true;
            }
        }
        return false;
    }
    fn lateral(&mut self) {
        let mut conns = self.stack.pop().unwrap();
        self.blob.remove(&self.purge.pop().unwrap());
        while conns.len() != 0 {
            let next = conns.pop().unwrap();
            if !self.blob.contains(&next) {
                self.stack.push(conns);
                self.blob.insert(next);
                self.purge.push(next);
                return;
            }
        }
        if self.stack.len() > 0 {
            self.lateral();
        }
    }
    fn walk(&mut self) {
        while self.blob.len() < 30 {
            self.forward();
        }
    }
}

impl Iterator for FillOutIterator {
    type Item = HashSet<Location>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }
        let out = self.blob.clone();
        self.lateral();
        if self.stack.len() != 0 {
            self.walk();
        }
        return Some(out);
    }
}

pub fn solve(elder: Location, guardian: Location, extras: &[Location]) -> HashSet<Location> {
    // 1.) Draw the critical path. This path must have no redundancy, must connect the elder to
    //   the guardian, and must have a maximum length of 29 (and even that is probably too much in
    //   practice).
    // 2.) Take 30 - critical path length, and try to create a "fill out" blob of that size.
    // A fill out blob is one which only has connections to itself, and the critical path
    for mut critical in CriticalIterator::new(elder, guardian) {
        if critical.len() < 26 {
            // This path is _highly_ unlikely to work, and difficult to try to fill out, so we'll
            // skip it.
            continue;
        }
        for extra in extras {
            critical.insert(*extra);
        }
        let mut fill_count = 0;
        for candidate in FillOutIterator::new(critical) {
            fill_count += 1;
            if fill_count % 100000 == 0 {
                println!("Fills checked: {}", fill_count);
            }
            if static_elder(
                elder,
                guardian,
                &candidate.iter().cloned().collect::<Vec<_>>(),
            ) {
                return candidate;
            }
        }
    }
    panic!("unsolved")
}

#[test]
fn lolsolve() {
    println!("Answer: {:?}", solve(Orchard, Armoury, &[]));
}
