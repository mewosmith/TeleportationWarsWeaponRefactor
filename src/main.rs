use rand::seq::SliceRandom;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,
    l_weapon: LargeWeapon,
    m_weapon: MediumWeapon,
    s_weapon: SmallWeapon,
    m_turret: MediumTurret,
    l_turret: LargeTurret,
    // faction_vec: Factions,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    out_path: String,
    mod_name: String,
    pageid: String,
    weaponcount: i32,
    weapontypes: Vec<String>,
    prodwares: Vec<String>,
    owner: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct SmallWeapon {
    name_pool: Vec<String>,
    group: String,
    tags: String,
    minprice: Vec<i32>,
    avgprice: Vec<i32>,
    maxprice: Vec<i32>,
    prodamt1: Vec<i32>,
    prodamt2: Vec<i32>,
    prodamt3: Vec<i32>,
    license: Vec<String>,
    class: String,
    wcomponents: Vec<String>,
    rotationspeed: Vec<i32>,
    rotationacceleration: Vec<i32>,
    hullmax: Vec<i32>,
    scale: Vec<i32>,
    bcomponent_laser: Vec<String>,
    bcomponent_proj: Vec<String>,
    speed: Vec<i32>,
    reload: Vec<i32>,
    dmin: Vec<i32>,
    dmax: Vec<i32>,
    dval: Vec<i32>,
    lifetime: Vec<i32>,
    impact: Vec<String>,
    launch: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct MediumWeapon {
    name_pool: Vec<String>,
    group: String,
    tags: String,
    minprice: Vec<i32>,
    avgprice: Vec<i32>,
    maxprice: Vec<i32>,
    prodamt1: Vec<i32>,
    prodamt2: Vec<i32>,
    prodamt3: Vec<i32>,
    license: Vec<String>,
    class: String,
    wcomponents: Vec<String>,
    rotationspeed: Vec<i32>,
    rotationacceleration: Vec<i32>,
    hullmax: Vec<i32>,
    scale: Vec<i32>,
    bcomponent_laser: Vec<String>,
    bcomponent_proj: Vec<String>,
    speed: Vec<i32>,
    reload: Vec<i32>,
    dmin: Vec<i32>,
    dmax: Vec<i32>,
    dval: Vec<i32>,
    lifetime: Vec<i32>,
    impact: Vec<String>,
    launch: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct LargeWeapon {
    name_pool: Vec<String>,
    group: String,
    tags: String,
    minprice: Vec<i32>,
    avgprice: Vec<i32>,
    maxprice: Vec<i32>,
    prodamt1: Vec<i32>,
    prodamt2: Vec<i32>,
    prodamt3: Vec<i32>,
    license: Vec<String>,
    class: String,
    wcomponents: Vec<String>,
    rotationspeed: Vec<i32>,
    rotationacceleration: Vec<i32>,
    hullmax: Vec<i32>,
    scale: Vec<i32>,
    bcomponent_laser: Vec<String>,
    bcomponent_proj: Vec<String>,
    speed: Vec<i32>,
    reload: Vec<i32>,
    dmin: Vec<i32>,
    dmax: Vec<i32>,
    dval: Vec<i32>,
    lifetime: Vec<i32>,
    impact: Vec<String>,
    launch: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct MediumTurret {
    name_pool: Vec<String>,
    group: String,
    tags: String,
    minprice: Vec<i32>,
    avgprice: Vec<i32>,
    maxprice: Vec<i32>,
    prodamt1: Vec<i32>,
    prodamt2: Vec<i32>,
    prodamt3: Vec<i32>,
    license: Vec<String>,
    class: String,
    wcomponents: Vec<String>,
    rotationspeed: Vec<i32>,
    rotationacceleration: Vec<i32>,
    hullmax: Vec<i32>,
    scale: Vec<i32>,
    bcomponent_laser: Vec<String>,
    bcomponent_proj: Vec<String>,
    speed: Vec<i32>,
    reload: Vec<i32>,
    dmin: Vec<i32>,
    dmax: Vec<i32>,
    dval: Vec<i32>,
    lifetime: Vec<i32>,
    impact: Vec<String>,
    launch: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct LargeTurret {
    name_pool: Vec<String>,
    group: String,
    tags: String,
    minprice: Vec<i32>,
    avgprice: Vec<i32>,
    maxprice: Vec<i32>,
    prodamt1: Vec<i32>,
    prodamt2: Vec<i32>,
    prodamt3: Vec<i32>,
    license: Vec<String>,
    class: String,
    wcomponents: Vec<String>,
    rotationspeed: Vec<i32>,
    rotationacceleration: Vec<i32>,
    hullmax: Vec<i32>,
    scale: Vec<i32>,
    bcomponent_laser: Vec<String>,
    bcomponent_proj: Vec<String>,
    speed: Vec<i32>,
    reload: Vec<i32>,
    dmin: Vec<i32>,
    dmax: Vec<i32>,
    dval: Vec<i32>,
    lifetime: Vec<i32>,
    impact: Vec<String>,
    launch: Vec<String>,
}
#[derive(Debug, Default)]
struct Current {
    name: String,
    count: i32,
    group: String,
    tags: String,
    minprice: i32,
    avgprice: i32,
    maxprice: i32,
    prod1: String,
    prod2: String,
    prod3: String,
    prodamt1: i32,
    prodamt2: i32,
    prodamt3: i32,
    license: String,
    owner: String,
    class: String,
    wcomponent: String,
    rotationspeed: i32,
    rotationacceleration: i32,
    hullmax: i32,
    scale: i32,
    bcomponent_laser: String,
    bcomponent_proj: String,
    speed: i32,
    reload: i32,
    dmin: i32,
    dmax: i32,
    dval: i32,
    lifetime: i32,
    impact: String,
    launch: String,
}

fn main() {
    let mut toml_str = include_str!("Config.toml");
    let mut toml_parsed: Toml = toml::from_str(&toml_str).unwrap_or_default();

    // files
    let mut bullet_string = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<macros>".to_string();
    let mut macro_string = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<macros>".to_string();
    let mut ware_string = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<diff>\n  <add sel=\"/wares\">\n".to_string();
    let mut index_string = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<diff>\n  <add sel=\"/index\">\n".to_string();
    let mut t_string = format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<diff>\n  <add sel=\"/language\">\n    <page id=\"{}\" title=\"tpweapons\" descr=\"tpweapongen\" voice=\"no\">", toml_parsed.config.pageid).to_string();
    // directory
    let out_path = &toml_parsed.config.out_path;
    let mut t_out_path = [&out_path, "t/"].concat();
    fs::create_dir_all(&t_out_path).unwrap();
    let mut index_out_path = [&out_path, "index/"].concat();
    fs::create_dir_all(&index_out_path).unwrap();
    let mut macro_out_path = [&out_path, "macros/"].concat();
    fs::create_dir_all(&macro_out_path).unwrap();
    let mut ware_out_path = [&out_path, "libraries/"].concat();
    fs::create_dir_all(&ware_out_path).unwrap();
    //count
    let weaponcount = &toml_parsed.config.weaponcount;
    let mut tcount = *weaponcount;
    let select = &toml_parsed.config.weapontypes;
    let mut current = Current::default();
    for i in 1..*weaponcount {
        let mut prng = rand::thread_rng();
        tcount += 100;
        println!("{}", tcount);
        let selected = select.choose(&mut rand::thread_rng()).expect("weapontype select failed");
        if selected == &"sgun".to_string() {
            let macroname = toml_parsed.s_weapon.name_pool.choose(&mut rand::thread_rng()).expect("name remove").to_string();
            current = current_sgun(&toml_parsed, tcount, &macroname.replace(" ", "_"));
            let index = toml_parsed.s_weapon.name_pool.iter().position(|x| *x == macroname).unwrap();
            toml_parsed.s_weapon.name_pool.remove(index);
            println!("sgun");
        }
        if selected == &"mgun".to_string() {
            let macroname = toml_parsed.m_weapon.name_pool.choose(&mut rand::thread_rng()).expect("name remove").to_string();
            current = current_mgun(&toml_parsed, tcount, &macroname.replace(" ", "_"));
            let index = toml_parsed.m_weapon.name_pool.iter().position(|x| *x == macroname).unwrap();
            toml_parsed.m_weapon.name_pool.remove(index);
            println!("mgun");
        }
        if selected == &"lgun".to_string() {
            let macroname = toml_parsed.l_weapon.name_pool.choose(&mut rand::thread_rng()).expect("name remove").to_string();
            current = current_lgun(&toml_parsed, tcount, &macroname.replace(" ", "_"));
            let index = toml_parsed.l_weapon.name_pool.iter().position(|x| *x == macroname).unwrap();
            toml_parsed.l_weapon.name_pool.remove(index);
            println!("lgun");
        }
        if selected == &"mturret".to_string() {
            let macroname = toml_parsed.m_turret.name_pool.choose(&mut rand::thread_rng()).expect("name remove").to_string();
            current = current_mturret(&toml_parsed, tcount, &macroname.replace(" ", "_"));
            let index = toml_parsed.m_turret.name_pool.iter().position(|x| *x == macroname).unwrap();
            toml_parsed.m_turret.name_pool.remove(index);
            println!("mturret");
        }
        if selected == &"lturret".to_string() {
            let macroname = toml_parsed.l_turret.name_pool.choose(&mut rand::thread_rng()).expect("name remove").to_string();
            current = current_lturret(&toml_parsed, tcount, &macroname.replace(" ", "_"));
            let index = toml_parsed.l_turret.name_pool.iter().position(|x| *x == macroname).unwrap();
            toml_parsed.l_turret.name_pool.remove(index);
            println!("lturret");
        }

        // if selected == &"smiss".to_string() {
        // current = current_large_weapon(&toml_parsed, tcount);
        // }
        // if selected == &"mmiss".to_string() {
        // current = current_large_weapon(&toml_parsed, tcount);
        // }
        // if selected == &"lmiss".to_string() {
        // current = current_large_weapon(&toml_parsed, tcount);
        // }
        gen_bullet(&toml_parsed, &current, &mut bullet_string);
        gen_macro(&toml_parsed, &current, &mut macro_string);
        gen_index(&current, &mut index_string, &toml_parsed);
        gen_tstring(&mut t_string, &current);
        gen_ware(&current, &mut ware_string, &toml_parsed);
    }
    // write
    bullet_string.push_str("\n</macros>");
    macro_string.push_str("\n</macros>");
    index_string.push_str("\n</add>\n</diff>\n");
    t_string.push_str("\n</page>\n</add>\n</diff>\n");
    ware_string.push_str("\n</add>\n</diff>\n");
    output(&mut macro_out_path, "bullets.xml".to_string(), &bullet_string);
    output(&mut macro_out_path, "macros.xml".to_string(), &macro_string);
    output(&mut index_out_path, "macros.xml".to_string(), &index_string);
    output(&mut t_out_path, "0001-L044.xml".to_string(), &t_string);
    output(&mut ware_out_path, "wares.xml".to_string(), &ware_string);
}

fn current_sgun(toml_parsed: &Toml, tcount: i32, macroname: &str) -> (Current) {
    let mut prng = rand::thread_rng();

    let current = Current {
        name: macroname.to_string(),
        count: tcount,
        group: toml_parsed.s_weapon.group.to_string(),
        tags: toml_parsed.s_weapon.tags.to_string(),
        minprice: prng.gen_range(toml_parsed.s_weapon.minprice[0], toml_parsed.s_weapon.minprice[1]),
        avgprice: prng.gen_range(toml_parsed.s_weapon.avgprice[0], toml_parsed.s_weapon.avgprice[1]),
        maxprice: prng.gen_range(toml_parsed.s_weapon.maxprice[0], toml_parsed.s_weapon.maxprice[1]),
        prod1: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose 1 failed")
            .to_string(),
        prod2: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose2 failed")
            .to_string(),
        prod3: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose3 failed")
            .to_string(),
        prodamt1: prng.gen_range(toml_parsed.s_weapon.prodamt1[0], toml_parsed.s_weapon.prodamt1[1]),
        prodamt2: prng.gen_range(toml_parsed.s_weapon.prodamt2[0], toml_parsed.s_weapon.prodamt2[1]),
        prodamt3: prng.gen_range(toml_parsed.s_weapon.prodamt3[0], toml_parsed.s_weapon.prodamt3[1]),
        license: toml_parsed
            .s_weapon
            .license
            .choose(&mut rand::thread_rng())
            .expect("choose license failed")
            .to_string(),
        owner: toml_parsed
            .config
            .owner
            .choose(&mut rand::thread_rng())
            .expect("choose owner failed")
            .to_string(),
        class: toml_parsed.s_weapon.class.to_string(),
        wcomponent: toml_parsed
            .s_weapon
            .wcomponents
            .choose(&mut rand::thread_rng())
            .expect("wcomponents choose failed")
            .to_string(),

        rotationspeed: prng.gen_range(toml_parsed.s_weapon.rotationspeed[0], toml_parsed.s_weapon.rotationspeed[1]),
        rotationacceleration: prng.gen_range(toml_parsed.s_weapon.rotationacceleration[0], toml_parsed.s_weapon.rotationacceleration[1]),
        hullmax: prng.gen_range(toml_parsed.s_weapon.hullmax[0], toml_parsed.s_weapon.hullmax[1]),
        scale: *toml_parsed.s_weapon.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
        bcomponent_laser: toml_parsed
            .s_weapon
            .bcomponent_laser
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_laser select failed")
            .to_string(),
        bcomponent_proj: toml_parsed
            .s_weapon
            .bcomponent_proj
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_proj select failed")
            .to_string(),
        speed: prng.gen_range(toml_parsed.s_weapon.speed[0], toml_parsed.s_weapon.speed[1]),
        reload: prng.gen_range(toml_parsed.s_weapon.reload[0], toml_parsed.s_weapon.reload[1]),
        dmin: prng.gen_range(toml_parsed.s_weapon.dmin[0], toml_parsed.s_weapon.dmin[1]),
        dmax: prng.gen_range(toml_parsed.s_weapon.dmax[0], toml_parsed.s_weapon.dmax[1]),
        dval: prng.gen_range(toml_parsed.s_weapon.dval[0], toml_parsed.s_weapon.dval[1]),
        lifetime: prng.gen_range(toml_parsed.s_weapon.lifetime[0], toml_parsed.s_weapon.lifetime[1]),
        impact: toml_parsed
            .s_weapon
            .impact
            .choose(&mut rand::thread_rng())
            .expect("impact select failed")
            .to_string(),
        launch: toml_parsed
            .s_weapon
            .launch
            .choose(&mut rand::thread_rng())
            .expect("launch select failed")
            .to_string(),
    };
    current
}

fn current_mgun(toml_parsed: &Toml, tcount: i32, macroname: &str) -> (Current) {
    let mut prng = rand::thread_rng();
    let current = Current {
        name: macroname.to_string(),
        count: tcount,
        group: toml_parsed.m_weapon.group.to_string(),
        tags: toml_parsed.m_weapon.tags.to_string(),
        minprice: prng.gen_range(toml_parsed.m_weapon.minprice[0], toml_parsed.m_weapon.minprice[1]),
        avgprice: prng.gen_range(toml_parsed.m_weapon.avgprice[0], toml_parsed.m_weapon.avgprice[1]),
        maxprice: prng.gen_range(toml_parsed.m_weapon.maxprice[0], toml_parsed.m_weapon.maxprice[1]),
        prod1: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose 1 failed")
            .to_string(),
        prod2: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose2 failed")
            .to_string(),
        prod3: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose3 failed")
            .to_string(),
        prodamt1: prng.gen_range(toml_parsed.m_weapon.prodamt1[0], toml_parsed.m_weapon.prodamt1[1]),
        prodamt2: prng.gen_range(toml_parsed.m_weapon.prodamt2[0], toml_parsed.m_weapon.prodamt2[1]),
        prodamt3: prng.gen_range(toml_parsed.m_weapon.prodamt3[0], toml_parsed.m_weapon.prodamt3[1]),
        license: toml_parsed
            .m_weapon
            .license
            .choose(&mut rand::thread_rng())
            .expect("choose license failed")
            .to_string(),
        owner: toml_parsed
            .config
            .owner
            .choose(&mut rand::thread_rng())
            .expect("choose owner failed")
            .to_string(),
        class: toml_parsed.m_weapon.class.to_string(),
        wcomponent: toml_parsed
            .m_weapon
            .wcomponents
            .choose(&mut rand::thread_rng())
            .expect("wcomponents choose failed")
            .to_string(),

        rotationspeed: prng.gen_range(toml_parsed.m_weapon.rotationspeed[0], toml_parsed.m_weapon.rotationspeed[1]),
        rotationacceleration: prng.gen_range(toml_parsed.m_weapon.rotationacceleration[0], toml_parsed.m_weapon.rotationacceleration[1]),
        hullmax: prng.gen_range(toml_parsed.m_weapon.hullmax[0], toml_parsed.m_weapon.hullmax[1]),
        scale: *toml_parsed.m_weapon.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
        bcomponent_laser: toml_parsed
            .m_weapon
            .bcomponent_laser
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_laser select failed")
            .to_string(),
        bcomponent_proj: toml_parsed
            .m_weapon
            .bcomponent_proj
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_proj select failed")
            .to_string(),
        speed: prng.gen_range(toml_parsed.m_weapon.speed[0], toml_parsed.m_weapon.speed[1]),
        reload: prng.gen_range(toml_parsed.m_weapon.reload[0], toml_parsed.m_weapon.reload[1]),
        dmin: prng.gen_range(toml_parsed.m_weapon.dmin[0], toml_parsed.m_weapon.dmin[1]),
        dmax: prng.gen_range(toml_parsed.m_weapon.dmax[0], toml_parsed.m_weapon.dmax[1]),
        dval: prng.gen_range(toml_parsed.m_weapon.dval[0], toml_parsed.m_weapon.dval[1]),
        lifetime: prng.gen_range(toml_parsed.m_weapon.lifetime[0], toml_parsed.m_weapon.lifetime[1]),
        impact: toml_parsed
            .m_weapon
            .impact
            .choose(&mut rand::thread_rng())
            .expect("impact select failed")
            .to_string(),
        launch: toml_parsed
            .m_weapon
            .launch
            .choose(&mut rand::thread_rng())
            .expect("launch select failed")
            .to_string(),
    };
    current
}
fn current_lgun(toml_parsed: &Toml, tcount: i32, macroname: &str) -> (Current) {
    let mut prng = rand::thread_rng();
    let current = Current {
        name: macroname.to_string(),
        count: tcount,
        group: toml_parsed.l_weapon.group.to_string(),
        tags: toml_parsed.l_weapon.tags.to_string(),
        minprice: prng.gen_range(toml_parsed.l_weapon.minprice[0], toml_parsed.l_weapon.minprice[1]),
        avgprice: prng.gen_range(toml_parsed.l_weapon.avgprice[0], toml_parsed.l_weapon.avgprice[1]),
        maxprice: prng.gen_range(toml_parsed.l_weapon.maxprice[0], toml_parsed.l_weapon.maxprice[1]),
        prod1: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose 1 failed")
            .to_string(),
        prod2: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose2 failed")
            .to_string(),
        prod3: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose3 failed")
            .to_string(),
        prodamt1: prng.gen_range(toml_parsed.l_weapon.prodamt1[0], toml_parsed.l_weapon.prodamt1[1]),
        prodamt2: prng.gen_range(toml_parsed.l_weapon.prodamt2[0], toml_parsed.l_weapon.prodamt2[1]),
        prodamt3: prng.gen_range(toml_parsed.l_weapon.prodamt3[0], toml_parsed.l_weapon.prodamt3[1]),
        license: toml_parsed
            .l_weapon
            .license
            .choose(&mut rand::thread_rng())
            .expect("choose license failed")
            .to_string(),
        owner: toml_parsed
            .config
            .owner
            .choose(&mut rand::thread_rng())
            .expect("choose owner failed")
            .to_string(),
        class: toml_parsed.l_weapon.class.to_string(),
        wcomponent: toml_parsed
            .l_weapon
            .wcomponents
            .choose(&mut rand::thread_rng())
            .expect("wcomponents choose failed")
            .to_string(),

        rotationspeed: prng.gen_range(toml_parsed.l_weapon.rotationspeed[0], toml_parsed.l_weapon.rotationspeed[1]),
        rotationacceleration: prng.gen_range(toml_parsed.l_weapon.rotationacceleration[0], toml_parsed.l_weapon.rotationacceleration[1]),
        hullmax: prng.gen_range(toml_parsed.l_weapon.hullmax[0], toml_parsed.l_weapon.hullmax[1]),
        scale: *toml_parsed.l_weapon.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
        bcomponent_laser: toml_parsed
            .l_weapon
            .bcomponent_laser
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_laser select failed")
            .to_string(),
        bcomponent_proj: toml_parsed
            .l_weapon
            .bcomponent_proj
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_proj select failed")
            .to_string(),
        speed: prng.gen_range(toml_parsed.l_weapon.speed[0], toml_parsed.l_weapon.speed[1]),
        reload: prng.gen_range(toml_parsed.l_weapon.reload[0], toml_parsed.l_weapon.reload[1]),
        dmin: prng.gen_range(toml_parsed.l_weapon.dmin[0], toml_parsed.l_weapon.dmin[1]),
        dmax: prng.gen_range(toml_parsed.l_weapon.dmax[0], toml_parsed.l_weapon.dmax[1]),
        dval: prng.gen_range(toml_parsed.l_weapon.dval[0], toml_parsed.l_weapon.dval[1]),
        lifetime: prng.gen_range(toml_parsed.l_weapon.lifetime[0], toml_parsed.l_weapon.lifetime[1]),
        impact: toml_parsed
            .l_weapon
            .impact
            .choose(&mut rand::thread_rng())
            .expect("impact select failed")
            .to_string(),
        launch: toml_parsed
            .l_weapon
            .launch
            .choose(&mut rand::thread_rng())
            .expect("launch select failed")
            .to_string(),
    };
    current
}

fn current_mturret(toml_parsed: &Toml, tcount: i32, macroname: &str) -> (Current) {
    let mut prng = rand::thread_rng();
    let current = Current {
        name: macroname.to_string(),
        count: tcount,
        group: toml_parsed.m_turret.group.to_string(),
        tags: toml_parsed.m_turret.tags.to_string(),
        minprice: prng.gen_range(toml_parsed.m_turret.minprice[0], toml_parsed.m_turret.minprice[1]),
        avgprice: prng.gen_range(toml_parsed.m_turret.avgprice[0], toml_parsed.m_turret.avgprice[1]),
        maxprice: prng.gen_range(toml_parsed.m_turret.maxprice[0], toml_parsed.m_turret.maxprice[1]),
        prod1: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose 1 failed")
            .to_string(),
        prod2: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose2 failed")
            .to_string(),
        prod3: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose3 failed")
            .to_string(),
        prodamt1: prng.gen_range(toml_parsed.m_turret.prodamt1[0], toml_parsed.m_turret.prodamt1[1]),
        prodamt2: prng.gen_range(toml_parsed.m_turret.prodamt2[0], toml_parsed.m_turret.prodamt2[1]),
        prodamt3: prng.gen_range(toml_parsed.m_turret.prodamt3[0], toml_parsed.m_turret.prodamt3[1]),
        license: toml_parsed
            .m_turret
            .license
            .choose(&mut rand::thread_rng())
            .expect("choose license failed")
            .to_string(),
        owner: toml_parsed
            .config
            .owner
            .choose(&mut rand::thread_rng())
            .expect("choose owner failed")
            .to_string(),
        class: toml_parsed.m_turret.class.to_string(),
        wcomponent: toml_parsed
            .m_turret
            .wcomponents
            .choose(&mut rand::thread_rng())
            .expect("wcomponents choose failed")
            .to_string(),

        rotationspeed: prng.gen_range(toml_parsed.m_turret.rotationspeed[0], toml_parsed.m_turret.rotationspeed[1]),
        rotationacceleration: prng.gen_range(toml_parsed.m_turret.rotationacceleration[0], toml_parsed.m_turret.rotationacceleration[1]),
        hullmax: prng.gen_range(toml_parsed.m_turret.hullmax[0], toml_parsed.m_turret.hullmax[1]),
        scale: *toml_parsed.m_turret.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
        bcomponent_laser: toml_parsed
            .m_turret
            .bcomponent_laser
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_laser select failed")
            .to_string(),
        bcomponent_proj: toml_parsed
            .m_turret
            .bcomponent_proj
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_proj select failed")
            .to_string(),
        speed: prng.gen_range(toml_parsed.m_turret.speed[0], toml_parsed.m_turret.speed[1]),
        reload: prng.gen_range(toml_parsed.m_turret.reload[0], toml_parsed.m_turret.reload[1]),
        dmin: prng.gen_range(toml_parsed.m_turret.dmin[0], toml_parsed.m_turret.dmin[1]),
        dmax: prng.gen_range(toml_parsed.m_turret.dmax[0], toml_parsed.m_turret.dmax[1]),
        dval: prng.gen_range(toml_parsed.m_turret.dval[0], toml_parsed.m_turret.dval[1]),
        lifetime: prng.gen_range(toml_parsed.m_turret.lifetime[0], toml_parsed.m_turret.lifetime[1]),
        impact: toml_parsed
            .m_turret
            .impact
            .choose(&mut rand::thread_rng())
            .expect("impact select failed")
            .to_string(),
        launch: toml_parsed
            .m_turret
            .launch
            .choose(&mut rand::thread_rng())
            .expect("launch select failed")
            .to_string(),
    };
    current
}
fn current_lturret(toml_parsed: &Toml, tcount: i32, macroname: &str) -> (Current) {
    let mut prng = rand::thread_rng();
    let current = Current {
        name: macroname.to_string(),
        count: tcount,
        group: toml_parsed.l_turret.group.to_string(),
        tags: toml_parsed.l_turret.tags.to_string(),
        minprice: prng.gen_range(toml_parsed.l_turret.minprice[0], toml_parsed.l_turret.minprice[1]),
        avgprice: prng.gen_range(toml_parsed.l_turret.avgprice[0], toml_parsed.l_turret.avgprice[1]),
        maxprice: prng.gen_range(toml_parsed.l_turret.maxprice[0], toml_parsed.l_turret.maxprice[1]),
        prod1: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose 1 failed")
            .to_string(),
        prod2: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose2 failed")
            .to_string(),
        prod3: toml_parsed
            .config
            .prodwares
            .choose(&mut rand::thread_rng())
            .expect("prod choose3 failed")
            .to_string(),
        prodamt1: prng.gen_range(toml_parsed.l_turret.prodamt1[0], toml_parsed.l_turret.prodamt1[1]),
        prodamt2: prng.gen_range(toml_parsed.l_turret.prodamt2[0], toml_parsed.l_turret.prodamt2[1]),
        prodamt3: prng.gen_range(toml_parsed.l_turret.prodamt3[0], toml_parsed.l_turret.prodamt3[1]),
        license: toml_parsed
            .l_turret
            .license
            .choose(&mut rand::thread_rng())
            .expect("choose license failed")
            .to_string(),
        owner: toml_parsed
            .config
            .owner
            .choose(&mut rand::thread_rng())
            .expect("choose owner failed")
            .to_string(),
        class: toml_parsed.l_turret.class.to_string(),
        wcomponent: toml_parsed
            .l_turret
            .wcomponents
            .choose(&mut rand::thread_rng())
            .expect("wcomponents choose failed")
            .to_string(),

        rotationspeed: prng.gen_range(toml_parsed.l_turret.rotationspeed[0], toml_parsed.l_turret.rotationspeed[1]),
        rotationacceleration: prng.gen_range(toml_parsed.l_turret.rotationacceleration[0], toml_parsed.l_turret.rotationacceleration[1]),
        hullmax: prng.gen_range(toml_parsed.l_turret.hullmax[0], toml_parsed.l_turret.hullmax[1]),
        scale: *toml_parsed.l_turret.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
        bcomponent_laser: toml_parsed
            .l_turret
            .bcomponent_laser
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_laser select failed")
            .to_string(),
        bcomponent_proj: toml_parsed
            .l_turret
            .bcomponent_proj
            .choose(&mut rand::thread_rng())
            .expect("bcomponent_proj select failed")
            .to_string(),
        speed: prng.gen_range(toml_parsed.l_turret.speed[0], toml_parsed.l_turret.speed[1]),
        reload: prng.gen_range(toml_parsed.l_turret.reload[0], toml_parsed.l_turret.reload[1]),
        dmin: prng.gen_range(toml_parsed.l_turret.dmin[0], toml_parsed.l_turret.dmin[1]),
        dmax: prng.gen_range(toml_parsed.l_turret.dmax[0], toml_parsed.l_turret.dmax[1]),
        dval: prng.gen_range(toml_parsed.l_turret.dval[0], toml_parsed.l_turret.dval[1]),
        lifetime: prng.gen_range(toml_parsed.l_turret.lifetime[0], toml_parsed.l_turret.lifetime[1]),
        impact: toml_parsed
            .l_turret
            .impact
            .choose(&mut rand::thread_rng())
            .expect("impact select failed")
            .to_string(),
        launch: toml_parsed
            .l_turret
            .launch
            .choose(&mut rand::thread_rng())
            .expect("launch select failed")
            .to_string(),
    };
    current
}

fn gen_ware(current: &Current, ware_string: &mut String, toml_parsed: &Toml) {
    ware_string.push_str(&format!(
        "\n<ware id=\"{}_ware\" name=\"{{{},{}}}\" description=\"{{{},{}}}\" group=\"{}\" transport=\"equipment\" volume=\"1\" tags=\"{}\">
        <price min=\"{}\" average=\"{}\" max=\"{}\" />
        <production time=\"10\" amount=\"1\" method=\"default\" name=\"{{20206,101}}\">
          <primary>
            <ware ware=\"{}\" amount=\"{}\" />
            <ware ware=\"{}\" amount=\"{}\" />
            <ware ware=\"{}\" amount=\"{}\" />
          </primary>
        </production>
        <component ref=\"{}_macro\" amount=\"1\" />
        <restriction licence=\"{}\" />
        <use threshold=\"0\"  />
        <owner faction=\"{}\" />
      </ware>",
        &current.name.to_lowercase(),
        toml_parsed.config.pageid,
        &current.count + 1,
        toml_parsed.config.pageid,
        &current.count + 3,
        &current.group,
        &current.tags,
        &current.minprice,
        &current.avgprice,
        &current.maxprice,
        &current.prod1,
        &current.prodamt1,
        &current.prod2,
        &current.prodamt2,
        &current.prod3,
        &current.prodamt3,
        &current.name.to_lowercase(),
        &current.license,
        &current.owner,
    ));
}
fn gen_tstring(t_string: &mut String, current: &Current) {
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>", current.count + 1, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>", current.count + 2, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>", current.count + 3, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>", current.count + 4, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>", current.count + 5, current.name));
}

fn gen_index(current: &Current, index_string: &mut String, toml_parsed: &Toml) {
    index_string.push_str(&format!(
        "\n<entry name=\"{}_macro\" value=\"{}macros\" />",
        current.name.to_lowercase(), toml_parsed.config.mod_name
    ));
    index_string.push_str(&format!(
        "\n<entry name=\"{}_bullet_macro\" value=\"{}bullets\" />",
        current.name.to_lowercase(), toml_parsed.config.mod_name
    ));
}
fn output(path: &mut String, name: String, input: &str) {
    let mut outputfile = File::create(format!("{}{}", path, name)).expect("create file failed");
    outputfile.write_all(input.as_bytes()).unwrap();
}
fn gen_macro(toml_parsed: &Toml, current: &Current, macro_string: &mut String) {
    macro_string.push_str(&format!(
        "  \n<macro name=\"{}_macro\" class=\"{}\">
    <component ref=\"{}\" />
    <properties>
      <identification name=\"{{{},{}}}\" description=\"{{{},{}}}\" mk=\"1\" />
      <bullet class=\"{}_bullet_macro\" />
      <rotationspeed max=\"{}\" />
      <rotationacceleration max=\"{}\" />
      <reload />
      <hull max=\"{}\" />
    </properties>
  </macro>",
        current.name.to_lowercase(),
        current.class,
        current.wcomponent,
        toml_parsed.config.pageid,
        current.count + 1,
        toml_parsed.config.pageid,
        current.count + 3,
        current.name.to_lowercase(),
        current.rotationspeed,
        current.rotationacceleration,
        current.hullmax
    ));
}
fn gen_bullet(toml_parsed: &Toml, current: &Current, bullet_string: &mut String) {
    // how to handlge gun vs laser
    bullet_string.push_str(&format!(
        "
   \n<macro name=\"{}_bullet_macro\" class=\"bullet\">
    <component ref=\"{}\" />
    <properties>
      <bullet speed=\"{}\" lifetime=\"{}\" amount=\"1\" barrelamount=\"1\" icon=\"weapon_railgun_mk2\" timediff=\"0.005\" angle=\"0.05\" maxhits=\"1\" ricochet=\"0\"  attach=\"0\"  scale=\"{}\" />
      <reload time=\"{}\" />
      <damage min=\"{}\" max=\"{}\"  value=\"{}\" time=\"{}\" />
      <effects>
        <impact ref=\"{}\" />
        <launch ref=\"{}\" />
      </effects>
      <weapon system=\"{}\" />
    </properties>
  </macro>",
        current.name.to_lowercase(),
        if current.scale == 1 {
            &current.bcomponent_laser
        } else {
            &current.bcomponent_proj
        },
        current.speed,
        current.lifetime,
        current.scale,
        current.reload,
        current.dmin,
        current.dmax,
        current.dval,
        current.lifetime,
        current.impact,
        current.launch,
        if current.class == "turret" { "turret_longrange" } else { "weapon_standard" },
    ));
}

// <ware id="missile_smart_heavy_mk1" name="{20105,6134}" description="{20105,6132}" group="missiles" transport="equipment" volume="1" tags="equipment missile">
//     <price min="2465" average="2900" max="3335" />
//     <production time="2" amount="1" method="default" name="{20206,101}">
//       <primary>
//         <ware ware="energycells" amount="20" />
//         <ware ware="missilecomponents" amount="3" />
//         <ware ware="smartchips" amount="4" />
//       </primary>
//     </production>
//     <production time="10" amount="1" method="xenon" name="{20206,601}">
//       <primary>
//         <ware ware="energycells" amount="20" />
//         <ware ware="ore" amount="6" />
//         <ware ware="silicon" amount="5" />
//       </primary>
//     </production>
//     <component ref="missile_smart_heavy_mk1_macro" amount="1" />
//     <container ref="sm_gen_pickup_equipment_01_macro" />
//     <use threshold="0" />
//   </ware>

// <ware id="turret_tel_m_dumbfire_01_mk1" name="{20105,4264}" description="{20105,4262}" group="turrets" transport="equipment" volume="1" tags="noplayerblueprint turret">
//     <price min="22865" average="25405" max="27946" />
//     <production time="10" amount="1" method="default" name="{20206,101}">
//       <primary>
//         <ware ware="advancedelectronics" amount="3" />
//         <ware ware="energycells" amount="10" />
//         <ware ware="turretcomponents" amount="5" />
//       </primary>
//     </production>
//     <component ref="turret_tel_m_dumbfire_01_mk1_macro" amount="1" />
//     <restriction licence="militaryequipment" />
//     <owner faction="teladi" />
//   </ware>

// <?xml version="1.0" encoding="utf-8"?>
// <!--Exported by: nick (192.168.3.120) at 12.02.2019_18-41-31-->
// <macros>
//   <macro name="missile_smart_heavy_mk1_macro" class="missile">
//     <component ref="missile_smart_heavy_mk1" />
//     <properties>
//       <identification name="{20105,6134}" basename="{20105,6131}" description="{20105,6132}" />
//       <ammunition value="1" />
//       <missile amount="1" barrelamount="1" lifetime="43.2" range="5500" guided="1" icon="missile_guided_mk1" retarget="1" tags="guided" />
//       <explosiondamage value="3378" />
//       <reload time="6" />
//       <hull max="3" />
//       <effects>
//         <explosion ref="missile_explosion_medium_01" />
//         <launch ref="missile_guided_muzzle" />
//       </effects>
//       <weapon system="missile_guided" />
//       <countermeasure resilience="0.98" />
//       <physics mass="1.7">
//         <inertia pitch="2.003" yaw="2.003" roll="2.003" />
//         <drag forward="0.34" reverse="1.36" horizontal="2.44" vertical="2.44" pitch="0.95" yaw="0.95" roll="0.95" />
//       </physics>
//     </properties>
//     <connections>
//       <connection ref="con_engine01">
//         <macro ref="engine_missile_guided_mk1_macro" connection="ship" />
//       </connection>
//     </connections>
//   </macro>
// </macros>

// <?xml version="1.0" encoding="utf-8"?>
// <!--Exported by: Michael (192.168.3.81) at 15.11.2018_16-14-20-->
// <macros>
//   <macro name="turret_arg_l_guided_01_mk1_macro" class="missileturret">
//     <component ref="turret_arg_l_guided_01_mk1" />
//     <properties>
//       <identification name="{20105,5164}" basename="{20105,5161}" shortname="{20105,5165}" makerrace="argon" description="{20105,5162}" mk="1" />
//       <ammunition tags="guided" />
//       <bullet class="missile_guided_heavy_mk1_macro" />
//       <rotationspeed max="20" />
//       <reload />
//       <storage capacity="100" />
//       <hull max="5178" threshold="0.2" />
//     </properties>
//   </macro>
// </macros>
