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
    // xl_weapon: XL_weapon,
    // l_weapon: L_weapon,
    // m_weapon: M_weapon,
    // s_weapon: S_weapon,
    // xl_turret: XL_turret,
    // l_turret: L_turret,
    // m_turret: M_turret,
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
    let toml_str = include_str!("Config.toml");
    let toml_parsed: Toml = toml::from_str(&toml_str).unwrap_or_default();

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
    // gen values
    let mut prng = rand::thread_rng();
    let weaponcount = &toml_parsed.config.weaponcount;
    let mut tcount = *weaponcount;
    let select = &toml_parsed.config.weapontypes;
    let mut current = Current::default();
    println!("{:#?}", current);
    for i in 1..*weaponcount {
        tcount += 100;
        let selected = select.choose(&mut rand::thread_rng()).expect("weapontype select failed");
        if selected == &"lgun".to_string() {
            current = current_large_weapon(&toml_parsed, tcount);
        }
        gen_bullet(&toml_parsed, &current, &mut bullet_string);
        gen_macro(&toml_parsed, &current, &mut macro_string);
        gen_index(&current, &mut index_string, &mut index_out_path);
        gen_tstring(&mut t_string, &current);
        gen_ware(&current, &mut ware_string, &toml_parsed);
    }
    // output
    bullet_string.push_str("</macros>");
    macro_string.push_str("</macros>");
    index_string.push_str("</add>\n</diff>\n");
    t_string.push_str("</add>\n</diff>\n");
    ware_string.push_str("</add>\n</diff>\n");
    output(&mut macro_out_path, "bullet.xml".to_string(), &bullet_string);
    output(&mut macro_out_path, "macros.xml".to_string(), &macro_string);
    output(&mut index_out_path, "index.xml".to_string(), &index_string);
    output(&mut t_out_path, "0001-L044.xml".to_string(), &t_string);
    output(&mut ware_out_path, "wares.xml".to_string(), &ware_string);
}
fn current_large_weapon(toml_parsed: &Toml, tcount: i32) -> (Current) {
    let mut prng = rand::thread_rng();

    let current = Current {
        name: toml_parsed.l_weapon.name_pool[0].clone(),
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
        scale: toml_parsed.l_weapon.scale.choose(&mut rand::thread_rng()).expect("scale select failed"),
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
fn gen_ware(current: &Current, ware_string: &mut String, toml_parsed: &Toml) {
    ware_string.push_str(&format!(
        "<ware id=\"{}_ware\" name=\"{{{},{}}}\" description=\"{{{},{}}}\" group=\"{}\" transport=\"equipment\" volume=\"1\" tags=\"{}\">
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
        &current.name,
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
        &current.name,
        &current.license,
        &current.owner,
    ));
}
fn gen_tstring(t_string: &mut String, current: &Current) {
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>\"", current.count + 1, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>\"", current.count + 2, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>\"", current.count + 3, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>\"", current.count + 4, current.name));
    t_string.push_str(&format!("\n<t id=\"{}\">{}</t>\"", current.count + 5, current.name));
}

fn gen_index(current: &Current, index_string: &mut String, path: &mut String) {
    index_string.push_str(&format!("\n<entry name=\"{}\" value=\"{}/{}\" />\n", current.name, path, current.name));
}
fn output(path: &mut String, name: String, input: &str) {
    let mut outputfile = File::create(format!("{}{}", path, name)).expect("create file failed");
    outputfile.write_all(input.as_bytes()).unwrap();
}
fn gen_macro(toml_parsed: &Toml, current: &Current, macro_string: &mut String) {
    macro_string.push_str(&format!(
        "  \n<macro name=\"{}_macro\" class=\"{}\">
    <component ref=\"{}_comp\" />
    <properties>
      <identification name=\"{{{},{}}}\" description=\"{{{},{}}}\" mk=\"1\" />
      <bullet class=\"{}_bullet_macro\" />
      <rotationspeed max=\"{}\" />
      <rotationacceleration max=\"{}\" />
      <reload />
      <hull max=\"{}\" />
    </properties>
  </macro>",
        current.name,
        current.class,
        current.wcomponent,
        toml_parsed.config.pageid,
        current.count + 1,
        toml_parsed.config.pageid,
        current.count + 3,
        current.name,
        current.rotationspeed,
        current.rotationacceleration,
        current.hullmax
    ));
}
fn gen_bullet(toml_parsed: &Toml, current: &Current, bullet_string: &mut String) {
    // how to handlge gun vs laser
    bullet_string.push_str(&format!(
        "
   \n<macro name=\"{}_macro_bullet\" class=\"bullet\">
    <component ref=\"{}\" />
    <properties>
      <bullet speed=\"{}\" lifetime=\"{}\" amount=\"1\" barrelamount=\"1\" scale=\"{}\" />
      <reload time=\"{}\" />
      <damage min=\"{}\" max=\"{}\"  value=\"{}\" time=\"{}\" />
      <effects>
        <impact ref=\"{}\" />
        <launch ref=\"{}\" />
      </effects>
      <weapon system=\"{}\" />
    </properties>
  </macro>
</macros>",
        current.name,
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
